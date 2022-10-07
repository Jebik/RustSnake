#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/*
use crate::gl::{
    GLuint, 
    GLint, 
    GLenum, 
    glGetUniformLocation, 
    glActiveTexture, 
    glBindTexture, 
    glGetIntegerv, 
    glGenVertexArrays, 
    glUseProgram,
    glEnable, 
    glDisable, 
    glFrontFace, 
    glScissor, 
    glUniform1i, 
    glVertexAttribPointer, 
    glVertexAttribDivisor, 
    glEnableVertexAttribArray, 
    glDisableVertexAttribArray, 
    glGenBuffers, 
    glBufferData, 
    glBufferSubData, 
    glGetProgramInfoLog, 
    glCreateShader, 
    glShaderSource, 
    glCompileShader, 
    glGetShaderiv, 
    glGetShaderInfoLog, 
    glGetAttribLocation,
    glGetProgramiv, 
    glAttachShader, 
    glCreateProgram, 
    glLinkProgram,
    glUniform2fv, 
    glDrawElementsInstanced,  
    glClearDepthf, 
    glClear, 
    glViewport, 
    glBindFramebuffer,  
    glClearColor, 
    GL_FLOAT,
    GL_ARRAY_BUFFER, 
    GL_TEXTURE0, 
    GL_TEXTURE_2D, 
    GL_ELEMENT_ARRAY_BUFFER, 
    GL_FRAMEBUFFER_BINDING, 
    GL_SCISSOR_TEST, 
    GL_DEPTH_TEST, 
    GL_CCW, GL_FALSE,
    GL_COLOR_BUFFER_BIT,
    GL_DEPTH_BUFFER_BIT,
    GL_FRAMEBUFFER,
    GL_TRIANGLES, 
    GL_UNSIGNED_SHORT, 
    GL_VERTEX_SHADER, 
    GL_FRAGMENT_SHADER,
    GL_LINK_STATUS, 
    GL_INFO_LOG_LENGTH,
    GL_COMPILE_STATUS, 
    GL_STATIC_DRAW};
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

*/


pub type GLenum = ::std::os::raw::c_uint;
pub type GLboolean = ::std::os::raw::c_uchar;
pub type GLbitfield = ::std::os::raw::c_uint;
pub type GLvoid = ::std::os::raw::c_void;
pub type GLint = ::std::os::raw::c_int;
pub type GLubyte = ::std::os::raw::c_uchar;
pub type GLuint = ::std::os::raw::c_uint;
pub type GLsizei = ::std::os::raw::c_int;
pub type GLchar = ::std::os::raw::c_char;

pub type khronos_ssize_t = ::std::os::raw::c_long;
pub type khronos_intptr_t = ::std::os::raw::c_long;

pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;

pub type GLfloat = f32;
pub type GLclampf = f32;

pub const GL_INFO_LOG_LENGTH: u32 = 0x8B84;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_INCR: u32 = 0x1E02;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const GL_DEPTH_TEST: u32 = 0x0B71;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_TEXTURE0: u32 = 0x84C0;
pub const GL_CLAMP_TO_EDGE: u32 = 0x812F;
pub const GL_ONE: u32 = 1;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_TEXTURE_2D: u32 = 0x0DE1;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const GL_LINEAR: u32 = 0x2601;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const GL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const GL_CCW: u32 = 0x0901;
pub const GL_RGB: u32 = 0x1907;
pub const GL_FALSE: u32 = 0;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_SCISSOR_TEST: u32 = 0x0C11;
pub const GL_FRAMEBUFFER: u32 = 0x8D40;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_FRAMEBUFFER_BINDING: u32 = 0x8CA6;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_ALPHA: u32 = 6406;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_TEXTURE_SWIZZLE_A: u32 = 36421;
pub const GL_VERSION: u32 = 0x1F02;
pub const WGL_NUMBER_PIXEL_FORMATS_ARB: u32 = 0x2000;
pub const WGL_SUPPORT_OPENGL_ARB: u32 = 0x2010;
pub const WGL_DRAW_TO_WINDOW_ARB: u32 = 0x2001;
pub const WGL_PIXEL_TYPE_ARB: u32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB: u32 = 0x202b;
pub const WGL_ACCELERATION_ARB: u32 = 0x2003;
pub const WGL_NO_ACCELERATION_ARB: u32 = 0x2025;
pub const WGL_ACCUM_BITS_ARB: u32 = 0x201d;
pub const WGL_ACCUM_RED_BITS_ARB: u32 = 0x201e;
pub const WGL_ACCUM_GREEN_BITS_ARB: u32 = 0x201f;
pub const WGL_ACCUM_BLUE_BITS_ARB: u32 = 0x2020;
pub const WGL_ACCUM_ALPHA_BITS_ARB: u32 = 0x2021;
pub const WGL_STEREO_ARB: u32 = 0x2012;
pub const WGL_DOUBLE_BUFFER_ARB: u32 = 0x2011;
pub const WGL_SAMPLES_ARB: u32 = 0x2042;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;
pub const WGL_LOSE_CONTEXT_ON_RESET_ARB: u32 = 0x8252;

macro_rules! gl_loader {
    (
        $(
            fn $fn:ident ( $($arg:ident : $t:ty),* ) -> $res:ty
        ),*
    ) => {
        mod __pfns {
            use super::*;

            $(
                pub static mut $fn: Option<extern "C" fn ($($arg: $t),*) -> $res> = None;
            )*
        }

        $(
            pub unsafe fn $fn($($arg: $t),*) -> $res {
                __pfns::$fn.unwrap()( $($arg),* )
            }
        )*

        pub fn load_gl_funcs<T: FnMut(&str) -> Option<unsafe extern "C" fn() -> ()>>(mut getprocaddr: T) {
            $(
                unsafe {
                    let fn_name = stringify!($fn);
                    __pfns::$fn = ::std::mem::transmute_copy(&getprocaddr(fn_name));
                }
            )*
        }
    };
}

gl_loader!(
    fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte,
    fn glGetString(name: GLenum) -> *const GLubyte,
    fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) -> (),
    fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
    fn glUniform1i(location: GLint, v0: GLint) -> (),
    fn glUseProgram(program: GLuint) -> (),
    fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    ) -> (),
    fn glLinkProgram(program: GLuint) -> (),
    fn glPixelStorei(pname: GLenum, param: GLint) -> (),
    fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint,
    fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),    
    fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint,    
    fn glDisableVertexAttribArray(index: GLuint) -> (),
    fn glCompileShader(shader: GLuint) -> (),
    fn glVertexAttribDivisor(index: GLuint, divisor: GLuint) -> (),
    fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void
    ) -> (),    
    fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
    fn glActiveTexture(texture: GLenum) -> (),    
    fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> (),
    fn glBindTexture(target: GLenum, texture: GLuint) -> (),
    fn glCreateShader(type_: GLenum) -> GLuint,
    fn glClearDepthf(d: GLfloat) -> (),
    fn glCreateProgram() -> GLuint,    
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
    fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        instancecount: GLsizei
    ) -> (),    
    fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ::std::os::raw::c_void
    ) -> (),
    fn glDisable(cap: GLenum) -> (),
    fn glBindBuffer(target: GLenum, buffer: GLuint) -> (),
    fn glBindVertexArray(array: GLuint) -> (),
    fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),    
    fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void,
        usage: GLenum
    ) -> (),    
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) -> (),
    fn glGetIntegerv(pname: GLenum, params: *mut GLint) -> (),
    fn glEnable(cap: GLenum) -> (),
    fn glAttachShader(program: GLuint, shader: GLuint) -> (),
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) -> (),
    fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    ) -> (),
    fn glEnableVertexAttribArray(index: GLuint) -> (),
    fn glClear(mask: GLbitfield) -> (),
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid
    ) -> (),
    fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
    fn glFrontFace(mode: GLenum) -> (),
    fn glGenTextures(n: GLsizei, textures: *mut GLuint) -> ()
);

// note that glGetString only works after first glSwapBuffer,
// not just after context creation
pub unsafe fn is_gl2() -> bool {
    let version_string = glGetString(super::gl::GL_VERSION);
    let version_string = std::ffi::CStr::from_ptr(version_string as _)
        .to_str()
        .unwrap();

    version_string.starts_with("2") || version_string.starts_with("OpenGL ES 2")
}
