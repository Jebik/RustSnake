use crate::gl::{
    GLuint, 
    GL_RGB, 
    GL_UNSIGNED_BYTE, 
    GL_CLAMP_TO_EDGE, 
    GL_LINEAR, 
    GL_UNPACK_ALIGNMENT, 
    GL_TEXTURE_2D, 
    GL_TEXTURE_SWIZZLE_A, 
    GL_ALPHA, 
    GL_TEXTURE_WRAP_S,
    GL_TEXTURE_WRAP_T, 
    GL_TEXTURE_MIN_FILTER, 
    GL_TEXTURE_MAG_FILTER,
    glGenTextures, 
    glPixelStorei, 
    glTexParameteri, glTexImage2D
};
use crate::graphics::GraphicsContext;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Texture {
    pub texture: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(
        ctx: &mut GraphicsContext,
        bytes: &[u8],
        width: u32,
        height: u32,
    ) -> Texture {
        assert_eq!((3*width*height) as usize, bytes.len());
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
                width as i32,
                height as i32,
                0,
                GL_RGB,
                GL_UNSIGNED_BYTE,
                bytes.as_ptr() as *const _,
            );

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);
        }
        ctx.cache.restore_texture_binding(0);

        Texture {
            texture,
            width,
            height,
        }
    }
}
