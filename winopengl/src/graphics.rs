use std::{ffi::CString, mem};
mod texture;
use std::{error::Error, fmt::Display};
pub use texture::{FilterMode, Texture, TextureAccess, TextureFormat, TextureParams, TextureWrap};
use crate::gl::{GLuint, glGetUniformLocation, GL_FLOAT, GLint, GLenum, GL_ARRAY_BUFFER, glBindBuffer, glActiveTexture, glBindTexture, GL_TEXTURE0, GL_TEXTURE_2D, GL_ELEMENT_ARRAY_BUFFER, glGetIntegerv, GL_FRAMEBUFFER_BINDING, glGenVertexArrays, glBindVertexArray, glUseProgram, glEnable, glDisable, glFrontFace, GL_SCISSOR_TEST, GL_DEPTH_TEST, GL_CCW, glScissor, glUniform1i, glVertexAttribPointer, GL_FALSE, glVertexAttribDivisor, glEnableVertexAttribArray, glDisableVertexAttribArray, GL_COLOR_BUFFER_BIT, glClearColor, GL_DEPTH_BUFFER_BIT, glClearDepthf, glClear, glViewport, glBindFramebuffer, GL_FRAMEBUFFER, glDrawElementsInstanced, GL_TRIANGLES, GL_UNSIGNED_SHORT, glUniform2fv, GL_VERTEX_SHADER, GL_FRAGMENT_SHADER, glAttachShader, glCreateProgram, glLinkProgram, GL_LINK_STATUS, glGetProgramiv, GL_INFO_LOG_LENGTH, glGetProgramInfoLog, glCreateShader, glShaderSource, glCompileShader, glGetShaderiv, GL_COMPILE_STATUS, glGetShaderInfoLog, glGetAttribLocation, GL_STATIC_DRAW, glGenBuffers, glBufferData, glBufferSubData};
use crate::graphics::GraphicsContext as Context;

const FLOAT2_SIZE:usize = 8;

fn get_uniform_location(program: GLuint, name: &str) -> Option<i32> {
    let cname = CString::new(name).unwrap_or_else(|e| panic!("{}", e));
    let location = unsafe { glGetUniformLocation(program, cname.as_ptr()) };

    if location == -1 {
        return None;
    }

    Some(location)
}

#[derive(Clone)]
pub struct UniformDesc {
    name: String,
    array_count: usize,
}

#[derive(Clone)]
pub struct UniformBlockLayout {
    pub uniforms: Vec<UniformDesc>,
}

impl UniformDesc {
    pub fn new(name: &str) -> UniformDesc {
        UniformDesc {
            name: name.to_string(),
            array_count: 1,
        }
    }
}

#[derive(Clone)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VertexStep {
    PerVertex,
    PerInstance,
}

impl Default for VertexStep {
    fn default() -> VertexStep {
        VertexStep::PerVertex
    }
}

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub name: &'static str,
    pub buffer_index: usize,
}

impl VertexAttribute {
    pub const fn new(name: &'static str) -> VertexAttribute {
        Self::with_buffer(name, 0)
    }

    pub const fn with_buffer(
        name: &'static str,
        buffer_index: usize,
    ) -> VertexAttribute {
        VertexAttribute {
            name,
            buffer_index,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Clone, Debug)]
pub enum ShaderError {
    CompilationError {
        shader_type: ShaderType,
        error_message: String,
    },
    LinkError(String),
    /// Shader strings should never contains \00 in the middle
    FFINulError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for ShaderError {
    fn from(e: std::ffi::NulError) -> ShaderError {
        ShaderError::FFINulError(e)
    }
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Display the same way as Debug
    }
}

impl Error for ShaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Shader(usize);

impl Shader {
    pub fn new(
        ctx: &mut Context,
        vertex_shader: &str,
        fragment_shader: &str,
        meta: ShaderMeta,
    ) -> Result<Shader, ShaderError> {
        let shader = load_shader_internal(vertex_shader, fragment_shader, meta)?;
        ctx.shaders.push(shader);
        Ok(Shader(ctx.shaders.len() - 1))
    }
}

type UniformLocation = Option<GLint>;

pub struct ShaderImage {
    gl_loc: UniformLocation,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ShaderUniform {
    gl_loc: UniformLocation,
    _offset: usize,
    _size: usize,
    array_count: i32,
}

struct ShaderInternal {
    program: GLuint,
    images: Vec<ShaderImage>,
    uniforms: Vec<ShaderUniform>,
}

#[derive(Default, Copy, Clone)]
struct CachedAttribute {
    attribute: VertexAttributeInternal,
    gl_vbuf: GLuint,
}

struct GlCache {
    stored_index_buffer: GLuint,
    stored_vertex_buffer: GLuint,
    stored_texture: GLuint,
    index_buffer: GLuint,
    vertex_buffer: GLuint,
    textures: [GLuint; MAX_SHADERSTAGE_IMAGES],
    cur_pipeline: Option<Pipeline>,
    attributes: [Option<CachedAttribute>; MAX_VERTEX_ATTRIBUTES],
}

impl GlCache {
    fn bind_buffer(&mut self, target: GLenum, buffer: GLuint) {
        if target == GL_ARRAY_BUFFER 
        {
            if self.vertex_buffer != buffer {
                self.vertex_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        } 
        else 
        {
            if self.index_buffer != buffer {
                self.index_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        }
    }

    fn store_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.stored_vertex_buffer = self.vertex_buffer;
        } else {
            self.stored_index_buffer = self.index_buffer;
        }
    }

    fn restore_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            if self.stored_vertex_buffer != 0 {
                self.bind_buffer(target, self.stored_vertex_buffer);
                self.stored_vertex_buffer = 0;
            }
        } else if self.stored_index_buffer != 0 {
                self.bind_buffer(target, self.stored_index_buffer);
                self.stored_index_buffer = 0;
        }        
    }

    fn bind_texture(&mut self, slot_index: usize, texture: GLuint) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + slot_index as GLuint);
            if self.textures[slot_index] != texture {
                glBindTexture(GL_TEXTURE_2D, texture);
                self.textures[slot_index] = texture;
            }
        }
    }

    fn store_texture_binding(&mut self, slot_index: usize) {
        self.stored_texture = self.textures[slot_index];
    }

    fn restore_texture_binding(&mut self, slot_index: usize) {
        self.bind_texture(slot_index, self.stored_texture);
    }

    fn clear_buffer_bindings(&mut self) {
        self.bind_buffer(GL_ARRAY_BUFFER, 0);
        self.vertex_buffer = 0;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        self.index_buffer = 0;
    }

    fn clear_texture_bindings(&mut self) {
        for ix in 0..MAX_SHADERSTAGE_IMAGES {
            if self.textures[ix] != 0 {
                self.bind_texture(ix, 0);
                self.textures[ix] = 0;
            }
        }
    }
}

pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_SHADERSTAGE_IMAGES: usize = 12;

pub struct Features {
    pub instancing: bool,
}

impl Default for Features {
    fn default() -> Features {
        Features { instancing: true }
    }
}

pub struct GraphicsContext {
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    default_framebuffer: GLuint,
    cache: GlCache,
    width:i32,
    height:i32,

    pub(crate) features: Features,
    pub(crate) display: Option<*mut dyn crate::NativeDisplay>,
}

impl GraphicsContext {
    pub fn new(width:i32, height:i32) -> GraphicsContext {
        unsafe {
            let mut default_framebuffer: GLuint = 0;
            glGetIntegerv(
                GL_FRAMEBUFFER_BINDING,
                &mut default_framebuffer as *mut _ as *mut _,
            );
            let mut vao = 0;

            glGenVertexArrays(1, &mut vao as *mut _);
            glBindVertexArray(vao);
            GraphicsContext {
                default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                features: Default::default(),
                width,
                height,
                cache: GlCache {
                    stored_index_buffer: 0,
                    stored_vertex_buffer: 0,
                    index_buffer: 0,
                    vertex_buffer: 0,
                    cur_pipeline: None,
                    stored_texture: 0,
                    textures: [0; MAX_SHADERSTAGE_IMAGES],
                    attributes: [None; MAX_VERTEX_ATTRIBUTES],
                },
                display: None,
            }
        }
    }

    pub fn features(&self) -> &Features {
        &self.features
    }
}

impl Context {
    pub fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        self.cache.cur_pipeline = Some(*pipeline);

        {
            let pipeline = &self.pipelines[pipeline.0];
            let shader = &mut self.shaders[pipeline.shader.0];
            unsafe {
                glUseProgram(shader.program);
            }

            unsafe {
                glEnable(GL_SCISSOR_TEST);
            }

            unsafe {
                glDisable(GL_DEPTH_TEST);
            }
            unsafe {
                glFrontFace(GL_CCW);
            }
        }
    }

    pub fn apply_bindings(&mut self, bindings: &Bindings) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, shader_image) in shader.images.iter().enumerate() {
            let bindings_image = bindings
                .images
                .get(n)
                .unwrap_or_else(|| panic!("Image count in bindings and shader did not match!"));
            if let Some(gl_loc) = shader_image.gl_loc {
                unsafe {
                    self.cache.bind_texture(n, bindings_image.texture);
                    glUniform1i(gl_loc, n as i32);
                }
            }
        }

        self.cache.bind_buffer(
            GL_ELEMENT_ARRAY_BUFFER,
            bindings.index_buffer.gl_buf,
        );

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(Some(attribute)) = pip_attribute {
                let vb = bindings.vertex_buffers[attribute.buffer_index];

                if cached_attr.map_or(true, |cached_attr| {
                    attribute != cached_attr.attribute || cached_attr.gl_vbuf != vb.gl_buf
                }) {
                    self.cache.bind_buffer(GL_ARRAY_BUFFER, vb.gl_buf);

                    unsafe {
                        glVertexAttribPointer(
                            attr_index as GLuint,
                            attribute.size,
                            attribute.type_,
                            GL_FALSE as u8,
                            attribute.stride,
                            attribute.offset as *mut _,
                        );
                        if self.features.instancing {
                            glVertexAttribDivisor(attr_index as GLuint, attribute.divisor as u32);
                        }
                        glEnableVertexAttribArray(attr_index as GLuint);
                    };

                    let cached_attr = &mut self.cache.attributes[attr_index];
                    *cached_attr = Some(CachedAttribute {
                        attribute,
                        gl_vbuf: vb.gl_buf,
                    });
                }
            } else if cached_attr.is_some() {
                    unsafe {
                        glDisableVertexAttribArray(attr_index as GLuint);
                    }
                    *cached_attr = None;
            }
        }
    }

    pub fn apply_uniforms<U>(&mut self, uniforms: &U) {
        self.apply_uniforms_from_bytes(uniforms as *const _ as *const u8, std::mem::size_of::<U>())
    }

    #[doc(hidden)]
    /// Apply uniforms data from array of bytes with very special layout.
    /// Hidden because `apply_uniforms` is the recommended and safer way to work with uniforms.
    pub fn apply_uniforms_from_bytes(&mut self, uniform_ptr: *const u8, size: usize) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for (_, uniform) in shader.uniforms.iter().enumerate() {
            assert!(
                offset <= size - FLOAT2_SIZE / 4,
                "Uniforms struct does not match shader uniforms layout"
            );

            unsafe {
                let data = (uniform_ptr as *const f32).offset(offset as isize);

                if let Some(gl_loc) = uniform.gl_loc 
                {
                    glUniform2fv(gl_loc, uniform.array_count, data);
                }
            }
            offset += FLOAT2_SIZE / 4 * uniform.array_count as usize;
        }
    }

    pub fn clear(&self) {
        let mut bits = 0;
        bits |= GL_COLOR_BUFFER_BIT;
        unsafe {
            glClearColor(0.0, 0.0, 0.0, 0.0)
        }

        bits |= GL_DEPTH_BUFFER_BIT;
        unsafe {
            glClearDepthf(1.);
        }

        if bits != 0 {
            unsafe {
                glClear(bits);
            }
        }
    }

    /// start rendering to the default frame buffer
    pub fn begin_default_pass(&mut self) {
        self.begin_pass();
    }

    /// start rendering to an offscreen framebuffer
    pub fn begin_pass(&mut self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
            glViewport(0, 0, self.width, self.height);
            glScissor(0, 0, self.width, self.height);
        }
        self.clear();
    }

    pub fn end_render_pass(&mut self) {
        unsafe {
            glBindFramebuffer(GL_FRAMEBUFFER, self.default_framebuffer);
            self.cache.bind_buffer(GL_ARRAY_BUFFER, 0);
            self.cache.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn commit_frame(&mut self) {
        self.cache.clear_buffer_bindings();
        self.cache.clear_texture_bindings();
    }

    pub fn draw(&self, base_element: i32, num_elements: i32, num_instances: i32) {
        assert!(
            self.cache.cur_pipeline.is_some(),
            "Drawing without any binded pipeline"
        );

        if !self.features.instancing && num_instances != 1 {
            println!("Instanced rendering is not supported by the GPU");
            println!("Ignoring this draw call");
            return;
        }

        unsafe {
            glDrawElementsInstanced(
                GL_TRIANGLES,
                num_elements,
                GL_UNSIGNED_SHORT,
                (2 as i32 * base_element) as *mut _,
                num_instances,
            );
        }
    }
}

fn load_shader_internal(
    vertex_shader: &str,
    fragment_shader: &str,
    meta: ShaderMeta,
) -> Result<ShaderInternal, ShaderError> {
    unsafe {
        let vertex_shader = load_shader(GL_VERTEX_SHADER, vertex_shader)?;
        let fragment_shader = load_shader(GL_FRAGMENT_SHADER, fragment_shader)?;

        let program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);

        let mut link_status = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut link_status as *mut _);
        if link_status == 0 {
            let mut max_length: i32 = 0;
            glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut max_length as *mut _);

            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetProgramInfoLog(
                program,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );
            assert!(max_length >= 1);
            let error_message =
                std::string::String::from_utf8_lossy(&error_message[0..max_length as usize - 1]);
            return Err(ShaderError::LinkError(error_message.to_string()));
        }

        glUseProgram(program);

        #[rustfmt::skip]
        let images = meta.images.iter().map(|name| ShaderImage {
            gl_loc: get_uniform_location(program, name),
        }).collect();

        #[rustfmt::skip]
        let uniforms = meta.uniforms.uniforms.iter().scan(0, |offset, uniform| {
            let res = ShaderUniform {
                gl_loc: get_uniform_location(program, &uniform.name),
                _offset: *offset,
                _size: FLOAT2_SIZE,
                array_count: uniform.array_count as _,
            };
            *offset += FLOAT2_SIZE * uniform.array_count;
            Some(res)
        }).collect();

        Ok(ShaderInternal {
            program,
            images,
            uniforms,
        })
    }
}

pub fn load_shader(shader_type: GLenum, source: &str) -> Result<GLuint, ShaderError> {
    unsafe {
        let shader = glCreateShader(shader_type);
        assert!(shader != 0);

        let cstring = CString::new(source)?;
        let csource = [cstring];
        glShaderSource(shader, 1, csource.as_ptr() as *const _, std::ptr::null());
        glCompileShader(shader);

        let mut is_compiled = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut is_compiled as *mut _);
        if is_compiled == 0 {
            let mut max_length: i32 = 0;
            glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut max_length as *mut _);

            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetShaderInfoLog(
                shader,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );

            assert!(max_length >= 1);
            let mut error_message =
                std::string::String::from_utf8_lossy(&error_message[0..max_length as usize - 1])
                    .into_owned();

            // On Wasm + Chrome, for unknown reason, string with zero-terminator is returned. On Firefox there is no zero-terminators in JavaScript string.
            if error_message.ends_with('\0') {
                error_message.pop();
            }

            return Err(ShaderError::CompilationError {
                shader_type: match shader_type {
                    GL_VERTEX_SHADER => ShaderType::Vertex,
                    GL_FRAGMENT_SHADER => ShaderType::Fragment,
                    _ => unreachable!(),
                },
                error_message,
            });
        }

        Ok(shader)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pipeline(usize);
impl Pipeline {
    pub fn new(
        ctx: &mut Context,
        attributes: &[VertexAttribute],
        shader: Shader,
    ) -> Pipeline {
        Self::with_params(ctx, attributes, shader)
    }

    pub fn with_params(
        ctx: &mut Context,
        attributes: &[VertexAttribute],
        shader: Shader,
    ) -> Pipeline {
        #[derive(Clone, Copy, Default)]
        struct BufferCacheData {
            stride: i32,
            offset: i64,
        }

        let mut buffer_cache: Vec<BufferCacheData> =
            vec![BufferCacheData::default(); 1];

        for VertexAttribute {
            buffer_index,
            ..
        } in attributes
        {
            let mut cache = buffer_cache
                .get_mut(*buffer_index)
                .unwrap_or_else(|| panic!());

            cache.stride += 8;
            // WebGL 1 limitation
            assert!(cache.stride <= 255);
        }

        let program = ctx.shaders[shader.0].program;

        let mut vertex_layout: Vec<Option<VertexAttributeInternal>> = vec![None; 2];

        for VertexAttribute {
            name,
            buffer_index,
        } in attributes
        {
            let mut buffer_data = &mut buffer_cache
                .get_mut(*buffer_index)
                .unwrap_or_else(|| panic!());
            let cname = CString::new(*name).unwrap_or_else(|e| panic!("{}", e));
            let attr_loc = unsafe { glGetAttribLocation(program, cname.as_ptr() as *const _) };
            let attr_loc = if attr_loc == -1 { None } else { Some(attr_loc) };
            let divisor = 0;

            for i in 0..1 {
                if let Some(attr_loc) = attr_loc {
                    let attr_loc = attr_loc as GLuint + i as GLuint;

                    let attr = VertexAttributeInternal {
                        attr_loc,
                        size: 2,
                        type_: GL_FLOAT,
                        offset: buffer_data.offset,
                        stride: buffer_data.stride,
                        buffer_index: *buffer_index,
                        divisor,
                    };

                    assert!(
                        attr_loc < vertex_layout.len() as u32,
                        "attribute: {} outside of allocated attributes array len: {}",
                        name,
                        vertex_layout.len()
                    );
                    vertex_layout[attr_loc as usize] = Some(attr);
                }
                buffer_data.offset += 8 as i64
            }
        }

        let pipeline = PipelineInternal {
            layout: vertex_layout,
            shader,
        };

        ctx.pipelines.push(pipeline);
        Pipeline(ctx.pipelines.len() - 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct VertexAttributeInternal {
    attr_loc: GLuint,
    size: i32,
    type_: GLuint,
    offset: i64,
    stride: i32,
    buffer_index: usize,
    divisor: i32,
}

struct PipelineInternal {
    layout: Vec<Option<VertexAttributeInternal>>,
    shader: Shader,
}

/// Geometry bindings
#[derive(Clone, Debug)]
pub struct Bindings {
    pub vertex_buffers: Vec<Buffer>,
    pub index_buffer: Buffer,
    pub images: Vec<Texture>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
}

#[derive(Clone, Copy, Debug)]
pub struct Buffer {
    gl_buf: GLuint,
}
impl Buffer {
    pub fn immutable<T>(ctx: &mut Context, buffer_type: BufferType, data: &[T]) -> Buffer {

        let gl_target = match buffer_type {
            BufferType::VertexBuffer => GL_ARRAY_BUFFER,
            BufferType::IndexBuffer => GL_ELEMENT_ARRAY_BUFFER,
        };
        let gl_usage = GL_STATIC_DRAW;
        let size = mem::size_of_val(data);
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.cache.store_buffer_binding(gl_target);
            ctx.cache.bind_buffer(gl_target, gl_buf);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            glBufferSubData(gl_target, 0, size as _, data.as_ptr() as *const _);
            ctx.cache.restore_buffer_binding(gl_target);
        }

        Buffer {
            gl_buf,
        }
    }
}