use crate::gl::{GLuint, GL_RGB, GL_UNSIGNED_BYTE, GL_CLAMP_TO_EDGE, GL_LINEAR, glGenTextures, glPixelStorei, GL_UNPACK_ALIGNMENT, glTexParameteri, GL_TEXTURE_2D, GL_TEXTURE_SWIZZLE_A, GL_ALPHA, glTexImage2D, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_MAG_FILTER};
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

#[derive(Debug, Copy, Clone)]
pub struct TextureParams {
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(
        ctx: &mut GraphicsContext,
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
        Self::new(ctx, Some(bytes), params)
    }
}
