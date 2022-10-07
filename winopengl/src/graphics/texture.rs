use crate::gl::{GLuint, GL_RGB, GL_UNSIGNED_BYTE, GL_CLAMP_TO_EDGE, GL_LINEAR, glGenTextures, glPixelStorei, GL_UNPACK_ALIGNMENT, glTexParameteri, GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA, glTexImage2D, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_MAG_FILTER, glTexSubImage2D, glGetIntegerv, glGenFramebuffers, glBindFramebuffer, glFramebufferTexture2D, GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_DRAW_FRAMEBUFFER_BINDING, glReadPixels, glDeleteFramebuffers};
use crate::graphics::GraphicsContext;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Texture {
    pub texture: GLuint,
    pub width: u32,
    pub height: u32,
}
impl Default for TextureParams {
    fn default() -> Self {
        TextureParams {
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}

#[derive(Debug, Copy, Clone)]
pub struct TextureParams {
    pub width: u32,
    pub height: u32,
}

impl Texture {
    /// Shorthand for `new(ctx, TextureAccess::RenderTarget, params)`
    pub fn new_render_texture(ctx: &mut GraphicsContext, params: TextureParams) -> Texture {
        Self::new(ctx, TextureAccess::RenderTarget, None, params)
    }

    pub fn new(
        ctx: &mut GraphicsContext,
        _access: TextureAccess,
        bytes: Option<&[u8]>,
        params: TextureParams,
    ) -> Texture {
        if let Some(bytes_data) = bytes {
            assert_eq!(
                (3*params.width*params.height) as usize,
                bytes_data.len()
            );
        }
        ctx.cache.store_texture_binding(0);

        let mut texture: GLuint = 0;

        unsafe {
            glGenTextures(1, &mut texture as *mut _);
            ctx.cache.bind_texture(0, texture);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA as _);

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGB as i32,
                params.width as i32,
                params.height as i32,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                match bytes {
                    Some(bytes) => bytes.as_ptr() as *const _,
                    Option::None => std::ptr::null(),
                },
            );

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        }
        ctx.cache.restore_texture_binding(0);

        Texture {
            texture,
            width: params.width,
            height: params.height,
        }
    }

    /// Upload texture to GPU with given TextureParams
    pub fn from_data_and_format(ctx: &mut GraphicsContext, bytes: &[u8], params: TextureParams) -> Texture {
        Self::new(ctx, TextureAccess::Static, Some(bytes), params)
    }

    /// Upload RGBA8 texture to GPU
    pub fn from_rgba8(ctx: &mut GraphicsContext, width: u16, height: u16, bytes: &[u8]) -> Texture {
        assert_eq!(width as usize * height as usize * 4, bytes.len());

        Self::from_data_and_format(
            ctx,
            bytes,
            TextureParams {
                width: width as _,
                height: height as _,
            },
        )
    }

    pub fn resize(&mut self, ctx: &mut GraphicsContext, width: u32, height: u32, bytes: Option<&[u8]>) {
        ctx.cache.store_texture_binding(0);

        self.width = width;
        self.height = height;

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGB as i32,
                self.width as i32,
                self.height as i32,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                match bytes {
                    Some(bytes) => bytes.as_ptr() as *const _,
                    Option::None => std::ptr::null(),
                },
            );
        }

        ctx.cache.restore_texture_binding(0);
    }

    /// Update whole texture content
    /// bytes should be width * height * 4 size - non rgba8 textures are not supported yet anyway
    pub fn update(&self, ctx: &mut GraphicsContext, bytes: &[u8]) {
        assert_eq!(self.size(self.width, self.height), bytes.len());

        self.update_texture_part(
            ctx,
            0 as _,
            0 as _,
            self.width as _,
            self.height as _,
            bytes,
        )
    }

    pub fn update_texture_part(
        &self,
        ctx: &mut GraphicsContext,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        bytes: &[u8],
    ) {
        assert_eq!(self.size(width as _, height as _), bytes.len());
        assert!(x_offset + width <= self.width as _);
        assert!(y_offset + height <= self.height as _);

        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(0, self.texture);

        unsafe {
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1); // miniquad always uses row alignment of 1

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA as _);

            glTexSubImage2D(
                GL_TEXTURE_2D,
                0,
                x_offset as _,
                y_offset as _,
                width as _,
                height as _,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                bytes.as_ptr() as *const _,
            );
        }

        ctx.cache.restore_texture_binding(0);
    }

    /// Read texture data into CPU memory
    pub fn read_pixels(&self, bytes: &mut [u8]) {

        let mut fbo = 0;
        unsafe {
            let mut binded_fbo: i32 = 0;
            glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &mut binded_fbo);
            glGenFramebuffers(1, &mut fbo);
            glBindFramebuffer(GL_FRAMEBUFFER, fbo);
            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                GL_TEXTURE_2D,
                self.texture,
                0,
            );

            glReadPixels(
                0,
                0,
                self.width as _,
                self.height as _,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                bytes.as_mut_ptr() as _,
            );

            glBindFramebuffer(GL_FRAMEBUFFER, binded_fbo as _);
            glDeleteFramebuffers(1, &fbo);
        }
    }

    #[inline]
    fn size(&self, width: u32, height: u32) -> usize {
        (3*width*height) as usize
    }
}
