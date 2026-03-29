//! WebGL2 GL interface shim for `wasm32-unknown-unknown`.
//!
//! Translates OpenGL ES function calls (as dispatched by Skia's Ganesh renderer
//! via `GrGLMakeAssembledInterface`) into WebGL2 API calls via `web-sys`.
//!
//! The approach mirrors Emscripten's `libglemu.js`: OpenGL integer object handles
//! are mapped to JavaScript WebGL objects stored in thread-local tables.
//!
//! # Usage
//! Call `Interface::new_web_sys(ctx)` with a `WebGl2RenderingContext` obtained
//! from the browser. The context is stored in a thread-local and used by all
//! subsequently-dispatched GL calls for the lifetime of the interface.
//!
//! # Limitations
//! - Only one WebGL context is supported per thread at a time.
//! - A subset of GL ES 3.0 is implemented (the subset Skia's Ganesh needs).
//! - `glTexImage2D` / `glTexSubImage2D` pixel size is estimated; Skia's usage
//!   is almost always RGBA8 so this is correct in practice.

#![allow(clippy::missing_safety_doc, clippy::too_many_arguments)]

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use wasm_bindgen::JsCast;
use web_sys::{
    WebGl2RenderingContext as Gl, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlQuery,
    WebGlRenderbuffer, WebGlSampler, WebGlShader, WebGlSync, WebGlTexture, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

// -------------------------------------------------------------------
// GL type aliases (OpenGL ES / Skia conventions)
// -------------------------------------------------------------------
type GLenum = u32;
type GLboolean = u8;
type GLbitfield = u32;
type GLint = i32;
type GLuint = u32;
type GLsizei = i32;
type GLfloat = f32;
type GLclampf = f32;
type GLintptr = i32; // WASM is 32-bit
type GLsizeiptr = i32;

const GL_TRUE: GLboolean = 1;
const GL_FALSE: GLboolean = 0;
const GL_EXTENSIONS: GLenum = 0x1F03;
const GL_NUM_EXTENSIONS: GLenum = 0x821D;

// -------------------------------------------------------------------
// Thread-local WebGL state
// -------------------------------------------------------------------

struct MappedBuffer {
    target: GLenum,
    offset: GLintptr,
    data: Vec<u8>,
}

struct WebGlState {
    ctx: Gl,
    textures: HashMap<GLuint, WebGlTexture>,
    buffers: HashMap<GLuint, WebGlBuffer>,
    framebuffers: HashMap<GLuint, WebGlFramebuffer>,
    renderbuffers: HashMap<GLuint, WebGlRenderbuffer>,
    shaders: HashMap<GLuint, WebGlShader>,
    programs: HashMap<GLuint, WebGlProgram>,
    vaos: HashMap<GLuint, WebGlVertexArrayObject>,
    uniform_locs: HashMap<GLuint, WebGlUniformLocation>,
    queries: HashMap<GLuint, WebGlQuery>,
    syncs: HashMap<GLuint, WebGlSync>,
    samplers: HashMap<GLuint, WebGlSampler>,
    mapped_buffer: Option<MappedBuffer>,
    next_id: GLuint,
    // Cached NUL-terminated C strings returned by glGetString
    version_cstr: Vec<u8>,
    vendor_cstr: Vec<u8>,
    renderer_cstr: Vec<u8>,
    shading_lang_cstr: Vec<u8>,
    extension_strs: Vec<Vec<u8>>,
}

impl WebGlState {
    fn new(ctx: Gl) -> Self {
        let extension_strs: Vec<Vec<u8>> = ctx
            .get_supported_extensions()
            .map(|arr| arr.to_vec())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| v.as_string())
            .map(|s| {
                let mut b = s.into_bytes();
                b.push(0);
                b
            })
            .collect();

        let vendor_cstr = nul_string(
            ctx.get_parameter(Gl::VENDOR)
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "WebGL".into()),
        );
        let renderer_cstr = nul_string(
            ctx.get_parameter(Gl::RENDERER)
                .ok()
                .and_then(|v| v.as_string())
                .unwrap_or_else(|| "WebGL2 Renderer".into()),
        );
        Self {
            ctx,
            textures: HashMap::new(),
            buffers: HashMap::new(),
            framebuffers: HashMap::new(),
            renderbuffers: HashMap::new(),
            shaders: HashMap::new(),
            programs: HashMap::new(),
            vaos: HashMap::new(),
            uniform_locs: HashMap::new(),
            queries: HashMap::new(),
            syncs: HashMap::new(),
            samplers: HashMap::new(),
            mapped_buffer: None,
            next_id: 1,
            // Expose an ES 3.0 version string so Skia's GLES code path enables
            // ES3 renderable formats (e.g. RGBA8) on wasm32-unknown.
            version_cstr: b"OpenGL ES 3.0\0".to_vec(),
            vendor_cstr,
            renderer_cstr,
            shading_lang_cstr: b"OpenGL ES GLSL ES 3.00\0".to_vec(),
            extension_strs,
        }
    }

    fn alloc_id(&mut self) -> GLuint {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

fn nul_string(mut s: String) -> Vec<u8> {
    s.push('\0');
    s.into_bytes()
}

thread_local! {
    static GL_CONTEXTS: RefCell<HashMap<u32, WebGlState>> = RefCell::new(HashMap::new());
    static GL_CURRENT:  Cell<u32>                         = const { Cell::new(0) };
    static GL_NEXT_ID:  Cell<u32>                         = const { Cell::new(1) };
}

macro_rules! with_gl {
    ($s:ident, $body:expr) => {
        GL_CONTEXTS.with(|ctxs| {
            let mut borrow = ctxs.borrow_mut();
            let id = GL_CURRENT.with(|c| c.get());
            if let Some($s) = borrow.get_mut(&id) {
                $body
            }
        })
    };
    ($s:ident => $body:expr) => {
        GL_CONTEXTS.with(|ctxs| {
            let mut borrow = ctxs.borrow_mut();
            let id = GL_CURRENT.with(|c| c.get());
            if let Some($s) = borrow.get_mut(&id) {
                $body
            } else {
                Default::default()
            }
        })
    };
}

macro_rules! with_gl_ref {
    ($s:ident => $body:expr) => {
        GL_CONTEXTS.with(|ctxs| {
            let borrow = ctxs.borrow();
            let id = GL_CURRENT.with(|c| c.get());
            if let Some($s) = borrow.get(&id) {
                $body
            } else {
                Default::default()
            }
        })
    };
}

// -------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------

/// Returns true if `pname` is valid for WebGL2 `getParameter`.
///
/// Invalid names generate `INVALID_ENUM` in the browser console before we can
/// clear the error, so guard them before calling into WebGL.
fn is_valid_webgl2_get_parameter(pname: GLenum) -> bool {
    // DRAW_BUFFERi: DRAW_BUFFER0 (0x8825) … DRAW_BUFFER15 (0x8834)
    if (0x8825..=0x8834).contains(&pname) {
        return true;
    }

    matches!(
        pname,
        0x84E0 // ACTIVE_TEXTURE
            | 0x846E // ALIASED_LINE_WIDTH_RANGE
            | 0x846D // ALIASED_POINT_SIZE_RANGE
            | 0x0D55 // ALPHA_BITS
            | 0x8894 // ARRAY_BUFFER_BINDING
            | 0x0BE2 // BLEND
            | 0x8005 // BLEND_COLOR
            | 0x80CA // BLEND_DST_ALPHA
            | 0x80C8 // BLEND_DST_RGB
            | 0x8009 // BLEND_EQUATION_RGB (= BLEND_EQUATION)
            | 0x883D // BLEND_EQUATION_ALPHA
            | 0x80CB // BLEND_SRC_ALPHA
            | 0x80C9 // BLEND_SRC_RGB
            | 0x0D54 // BLUE_BITS
            | 0x0C22 // COLOR_CLEAR_VALUE
            | 0x0C23 // COLOR_WRITEMASK
            | 0x86A3 // COMPRESSED_TEXTURE_FORMATS
            | 0x8F36 // COPY_READ_BUFFER_BINDING
            | 0x8F37 // COPY_WRITE_BUFFER_BINDING
            | 0x0B44 // CULL_FACE
            | 0x0B45 // CULL_FACE_MODE
            | 0x8B8D // CURRENT_PROGRAM
            | 0x0D56 // DEPTH_BITS
            | 0x0B73 // DEPTH_CLEAR_VALUE
            | 0x0B74 // DEPTH_FUNC
            | 0x0B70 // DEPTH_RANGE
            | 0x0B71 // DEPTH_TEST
            | 0x0B72 // DEPTH_WRITEMASK
            | 0x0BD0 // DITHER
            | 0x8CA6 // DRAW_FRAMEBUFFER_BINDING (= FRAMEBUFFER_BINDING)
            | 0x8895 // ELEMENT_ARRAY_BUFFER_BINDING
            | 0x8B8B // FRAGMENT_SHADER_DERIVATIVE_HINT
            | 0x0B46 // FRONT_FACE
            | 0x8192 // GENERATE_MIPMAP_HINT
            | 0x0D53 // GREEN_BITS
            | 0x8B9B // IMPLEMENTATION_COLOR_READ_FORMAT
            | 0x8B9A // IMPLEMENTATION_COLOR_READ_TYPE
            | 0x0B21 // LINE_WIDTH
            | 0x8073 // MAX_3D_TEXTURE_SIZE
            | 0x88FF // MAX_ARRAY_TEXTURE_LAYERS
            | 0x8CDF // MAX_COLOR_ATTACHMENTS
            | 0x8A33 // MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS
            | 0x8B4D // MAX_COMBINED_TEXTURE_IMAGE_UNITS
            | 0x8A2E // MAX_COMBINED_UNIFORM_BLOCKS
            | 0x8A31 // MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS
            | 0x851C // MAX_CUBE_MAP_TEXTURE_SIZE
            | 0x8824 // MAX_DRAW_BUFFERS
            | 0x8D6B // MAX_ELEMENT_INDEX
            | 0x80E9 // MAX_ELEMENTS_INDICES
            | 0x80E8 // MAX_ELEMENTS_VERTICES
            | 0x9125 // MAX_FRAGMENT_INPUT_COMPONENTS
            | 0x8A2D // MAX_FRAGMENT_UNIFORM_BLOCKS
            | 0x8B49 // MAX_FRAGMENT_UNIFORM_COMPONENTS
            | 0x8DFD // MAX_FRAGMENT_UNIFORM_VECTORS
            | 0x8905 // MAX_PROGRAM_TEXEL_OFFSET
            | 0x84E8 // MAX_RENDERBUFFER_SIZE
            | 0x8D57 // MAX_SAMPLES
            | 0x9111 // MAX_SERVER_WAIT_TIMEOUT
            | 0x8872 // MAX_TEXTURE_IMAGE_UNITS
            | 0x84FD // MAX_TEXTURE_LOD_BIAS
            | 0x0D33 // MAX_TEXTURE_SIZE
            | 0x8C8A // MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS
            | 0x8C8B // MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS
            | 0x8C80 // MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS
            | 0x8A30 // MAX_UNIFORM_BLOCK_SIZE
            | 0x8A2F // MAX_UNIFORM_BUFFER_BINDINGS
            | 0x8B4B // MAX_VARYING_COMPONENTS
            | 0x8DFC // MAX_VARYING_VECTORS
            | 0x8869 // MAX_VERTEX_ATTRIBS
            | 0x9122 // MAX_VERTEX_OUTPUT_COMPONENTS
            | 0x8B4C // MAX_VERTEX_TEXTURE_IMAGE_UNITS
            | 0x8A2B // MAX_VERTEX_UNIFORM_BLOCKS
            | 0x8B4A // MAX_VERTEX_UNIFORM_COMPONENTS
            | 0x8DFB // MAX_VERTEX_UNIFORM_VECTORS
            | 0x0D3A // MAX_VIEWPORT_DIMS
            | 0x821B // GL_MAJOR_VERSION
            | 0x821C // GL_MINOR_VERSION
            | 0x8904 // MIN_PROGRAM_TEXEL_OFFSET
            | 0x86A2 // NUM_COMPRESSED_TEXTURE_FORMATS
            | 0x821D // NUM_EXTENSIONS
            | 0x0D05 // PACK_ALIGNMENT
            | 0x0D02 // PACK_ROW_LENGTH
            | 0x0D04 // PACK_SKIP_PIXELS
            | 0x0D03 // PACK_SKIP_ROWS
            | 0x88ED // PIXEL_PACK_BUFFER_BINDING
            | 0x88EF // PIXEL_UNPACK_BUFFER_BINDING
            | 0x8038 // POLYGON_OFFSET_FACTOR
            | 0x8037 // POLYGON_OFFSET_FILL
            | 0x2A00 // POLYGON_OFFSET_UNITS
            | 0x8C89 // RASTERIZER_DISCARD
            | 0x0C02 // READ_BUFFER
            | 0x8CAA // READ_FRAMEBUFFER_BINDING
            | 0x0D52 // RED_BITS
            | 0x8CA7 // RENDERBUFFER_BINDING
            | 0x809E // SAMPLE_ALPHA_TO_COVERAGE
            | 0x80A8 // SAMPLE_BUFFERS
            | 0x80AB // SAMPLE_COVERAGE_INVERT
            | 0x80AA // SAMPLE_COVERAGE_VALUE
            | 0x80A9 // SAMPLES
            | 0x0C10 // SCISSOR_BOX
            | 0x0C11 // SCISSOR_TEST
            | 0x8801 // STENCIL_BACK_FAIL
            | 0x8800 // STENCIL_BACK_FUNC
            | 0x8802 // STENCIL_BACK_PASS_DEPTH_FAIL
            | 0x8803 // STENCIL_BACK_PASS_DEPTH_PASS
            | 0x8CA3 // STENCIL_BACK_REF
            | 0x8CA4 // STENCIL_BACK_VALUE_MASK
            | 0x8CA5 // STENCIL_BACK_WRITEMASK
            | 0x0D57 // STENCIL_BITS
            | 0x0B91 // STENCIL_CLEAR_VALUE
            | 0x0B94 // STENCIL_FAIL
            | 0x0B92 // STENCIL_FUNC
            | 0x0B95 // STENCIL_PASS_DEPTH_FAIL
            | 0x0B96 // STENCIL_PASS_DEPTH_PASS
            | 0x0B97 // STENCIL_REF
            | 0x0B90 // STENCIL_TEST
            | 0x0B93 // STENCIL_VALUE_MASK
            | 0x0B98 // STENCIL_WRITEMASK
            | 0x0D50 // SUBPIXEL_BITS
            | 0x8069 // TEXTURE_BINDING_2D
            | 0x8C1D // TEXTURE_BINDING_2D_ARRAY
            | 0x806A // TEXTURE_BINDING_3D
            | 0x8514 // TEXTURE_BINDING_CUBE_MAP
            | 0x8E24 // TRANSFORM_FEEDBACK_ACTIVE
            | 0x8E25 // TRANSFORM_FEEDBACK_BINDING
            | 0x8C8F // TRANSFORM_FEEDBACK_BUFFER_BINDING
            | 0x8E23 // TRANSFORM_FEEDBACK_PAUSED
            | 0x8A28 // UNIFORM_BUFFER_BINDING
            | 0x8A34 // UNIFORM_BUFFER_OFFSET_ALIGNMENT
            | 0x0CF5 // UNPACK_ALIGNMENT
            | 0x9243 // UNPACK_COLORSPACE_CONVERSION_WEBGL
            | 0x9240 // UNPACK_FLIP_Y_WEBGL
            | 0x806E // UNPACK_IMAGE_HEIGHT
            | 0x9241 // UNPACK_PREMULTIPLY_ALPHA_WEBGL
            | 0x0CF2 // UNPACK_ROW_LENGTH
            | 0x806D // UNPACK_SKIP_IMAGES
            | 0x0CF4 // UNPACK_SKIP_PIXELS
            | 0x0CF3 // UNPACK_SKIP_ROWS
            | 0x85B5 // VERTEX_ARRAY_BINDING
            | 0x1F00 // VENDOR
            | 0x1F02 // VERSION
            | 0x0BA2 // VIEWPORT
    )
}

/// Estimate the byte length of a `glTexImage2D` / `glTexSubImage2D` pixel buffer.
///
/// This is a best-effort calculation; Skia's Ganesh renderer almost exclusively
/// uses RGBA8 (format=GL_RGBA, type=GL_UNSIGNED_BYTE) making it width×height×4.
fn pixel_byte_size(width: i32, height: i32, format: u32, type_: u32) -> usize {
    let components: usize = match format {
        0x1902 | 0x1909 | 0x1903 | 0x1906 => 1, // DEPTH_COMPONENT, LUMINANCE, RED, ALPHA
        0x190A | 0x8227 => 2,                   // LUMINANCE_ALPHA, RG
        0x1907 | 0x8C41 => 3,                   // RGB, SRGB
        _ => 4,                                 // RGBA, SRGB_ALPHA, etc.
    };
    let bytes_per_component: usize = match type_ {
        0x1400 | 0x1401 => 1,          // BYTE, UNSIGNED_BYTE
        0x140B | 0x8D61 => 2,          // HALF_FLOAT, HALF_FLOAT_OES
        0x1402 | 0x1403 => 2,          // SHORT, UNSIGNED_SHORT
        0x1404..=0x1406 => 4,          // INT, UNSIGNED_INT, FLOAT
        0x8363 | 0x8033 | 0x8034 => 2, // packed 16-bit types
        _ => 1,
    };
    (width.max(0) as usize) * (height.max(0) as usize) * components * bytes_per_component
}

// -------------------------------------------------------------------
// GL function implementations
// -------------------------------------------------------------------

unsafe extern "C" fn gl_active_texture(texture: GLenum) {
    with_gl!(s, s.ctx.active_texture(texture));
}

unsafe extern "C" fn gl_attach_shader(program: GLuint, shader: GLuint) {
    with_gl!(s, {
        if let (Some(p), Some(sh)) = (s.programs.get(&program), s.shaders.get(&shader)) {
            s.ctx.attach_shader(p, sh);
        }
    });
}

unsafe extern "C" fn gl_bind_attrib_location(program: GLuint, index: GLuint, name: *const c_char) {
    let name_str = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    with_gl!(s, {
        if let Some(p) = s.programs.get(&program) {
            s.ctx.bind_attrib_location(p, index, name_str);
        }
    });
}

unsafe extern "C" fn gl_bind_buffer(target: GLenum, buffer: GLuint) {
    with_gl!(s, {
        let b = (buffer != 0).then(|| s.buffers.get(&buffer)).flatten();
        s.ctx.bind_buffer(target, b);
    });
}

unsafe extern "C" fn gl_bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    with_gl!(s, {
        let fb = (framebuffer != 0)
            .then(|| s.framebuffers.get(&framebuffer))
            .flatten();
        s.ctx.bind_framebuffer(target, fb);
    });
}

unsafe extern "C" fn gl_bind_renderbuffer(target: GLenum, renderbuffer: GLuint) {
    with_gl!(s, {
        let rb = (renderbuffer != 0)
            .then(|| s.renderbuffers.get(&renderbuffer))
            .flatten();
        s.ctx.bind_renderbuffer(target, rb);
    });
}

unsafe extern "C" fn gl_bind_texture(target: GLenum, texture: GLuint) {
    with_gl!(s, {
        let t = (texture != 0).then(|| s.textures.get(&texture)).flatten();
        s.ctx.bind_texture(target, t);
    });
}

unsafe extern "C" fn gl_bind_vertex_array(array: GLuint) {
    with_gl!(s, {
        let v = (array != 0).then(|| s.vaos.get(&array)).flatten();
        s.ctx.bind_vertex_array(v);
    });
}

unsafe extern "C" fn gl_blend_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    with_gl!(s, s.ctx.blend_color(r, g, b, a));
}

unsafe extern "C" fn gl_blend_equation(mode: GLenum) {
    with_gl!(s, s.ctx.blend_equation(mode));
}

unsafe extern "C" fn gl_blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
    with_gl!(s, s.ctx.blend_equation_separate(mode_rgb, mode_alpha));
}

unsafe extern "C" fn gl_blend_func(sfactor: GLenum, dfactor: GLenum) {
    with_gl!(s, s.ctx.blend_func(sfactor, dfactor));
}

unsafe extern "C" fn gl_blend_func_separate(
    src_rgb: GLenum,
    dst_rgb: GLenum,
    src_alpha: GLenum,
    dst_alpha: GLenum,
) {
    with_gl!(
        s,
        s.ctx
            .blend_func_separate(src_rgb, dst_rgb, src_alpha, dst_alpha)
    );
}

unsafe extern "C" fn gl_blit_framebuffer(
    src_x0: GLint,
    src_y0: GLint,
    src_x1: GLint,
    src_y1: GLint,
    dst_x0: GLint,
    dst_y0: GLint,
    dst_x1: GLint,
    dst_y1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    with_gl!(
        s,
        s.ctx.blit_framebuffer(
            src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
        )
    );
}

unsafe extern "C" fn gl_buffer_data(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum,
) {
    with_gl!(s, {
        if data.is_null() {
            s.ctx.buffer_data_with_i32(target, size, usage);
        } else {
            let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) };
            let array = js_sys::Uint8Array::from(bytes);
            s.ctx
                .buffer_data_with_array_buffer_view(target, &array, usage);
        }
    });
}

unsafe extern "C" fn gl_buffer_sub_data(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
) {
    with_gl!(s, {
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, size as usize) };
        let array = js_sys::Uint8Array::from(bytes);
        s.ctx
            .buffer_sub_data_with_i32_and_array_buffer_view(target, offset, &array);
    });
}

unsafe extern "C" fn gl_check_framebuffer_status(target: GLenum) -> GLenum {
    with_gl!(s => s.ctx.check_framebuffer_status(target))
}

unsafe extern "C" fn gl_clear(mask: GLbitfield) {
    with_gl!(s, s.ctx.clear(mask));
}

unsafe extern "C" fn gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    with_gl!(s, s.ctx.clear_color(r, g, b, a));
}

unsafe extern "C" fn gl_clear_stencil(s_val: GLint) {
    with_gl!(s, s.ctx.clear_stencil(s_val));
}

unsafe extern "C" fn gl_color_mask(r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
    with_gl!(s, s.ctx.color_mask(r != 0, g != 0, b != 0, a != 0));
}

unsafe extern "C" fn gl_compile_shader(shader: GLuint) {
    with_gl!(s, {
        if let Some(sh) = s.shaders.get(&shader) {
            s.ctx.compile_shader(sh);
        }
    });
}

unsafe extern "C" fn gl_copy_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl!(
        s,
        s.ctx
            .copy_tex_sub_image_2d(target, level, xoffset, yoffset, x, y, width, height)
    );
}

unsafe extern "C" fn gl_compressed_tex_image_2d(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    image_size: GLsizei,
    data: *const c_void,
) {
    with_gl!(s, {
        if data.is_null() || image_size <= 0 {
            s.ctx.compressed_tex_image_2d_with_u8_array(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                &[],
            );
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, image_size as usize) };
        s.ctx.compressed_tex_image_2d_with_u8_array(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            bytes,
        );
    });
}

unsafe extern "C" fn gl_compressed_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    image_size: GLsizei,
    data: *const c_void,
) {
    with_gl!(s, {
        if data.is_null() || image_size <= 0 {
            let mut empty = Vec::<u8>::new();
            s.ctx.compressed_tex_sub_image_2d_with_u8_array(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                empty.as_mut_slice(),
            );
            return;
        }
        let bytes = unsafe { std::slice::from_raw_parts(data as *const u8, image_size as usize) };
        let mut owned = bytes.to_vec();
        s.ctx.compressed_tex_sub_image_2d_with_u8_array(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            owned.as_mut_slice(),
        );
    });
}

unsafe extern "C" fn gl_create_program() -> GLuint {
    with_gl!(s => {
        match s.ctx.create_program() {
            Some(p) => { let id = s.alloc_id(); s.programs.insert(id, p); id }
            None => 0,
        }
    })
}

unsafe extern "C" fn gl_create_shader(shader_type: GLenum) -> GLuint {
    with_gl!(s => {
        match s.ctx.create_shader(shader_type) {
            Some(sh) => { let id = s.alloc_id(); s.shaders.insert(id, sh); id }
            None => 0,
        }
    })
}

unsafe extern "C" fn gl_cull_face(mode: GLenum) {
    with_gl!(s, s.ctx.cull_face(mode));
}

unsafe extern "C" fn gl_delete_buffers(n: GLsizei, buffers: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *buffers.add(i) };
            if let Some(b) = s.buffers.remove(&id) {
                s.ctx.delete_buffer(Some(&b));
            }
        }
    });
}

unsafe extern "C" fn gl_delete_framebuffers(n: GLsizei, framebuffers: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *framebuffers.add(i) };
            if let Some(fb) = s.framebuffers.remove(&id) {
                s.ctx.delete_framebuffer(Some(&fb));
            }
        }
    });
}

unsafe extern "C" fn gl_delete_program(program: GLuint) {
    with_gl!(s, {
        if let Some(p) = s.programs.remove(&program) {
            s.ctx.delete_program(Some(&p));
        }
    });
}

unsafe extern "C" fn gl_delete_queries(n: GLsizei, ids: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *ids.add(i) };
            if let Some(q) = s.queries.remove(&id) {
                s.ctx.delete_query(Some(&q));
            }
        }
    });
}

unsafe extern "C" fn gl_delete_renderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *renderbuffers.add(i) };
            if let Some(rb) = s.renderbuffers.remove(&id) {
                s.ctx.delete_renderbuffer(Some(&rb));
            }
        }
    });
}

unsafe extern "C" fn gl_delete_shader(shader: GLuint) {
    with_gl!(s, {
        if let Some(sh) = s.shaders.remove(&shader) {
            s.ctx.delete_shader(Some(&sh));
        }
    });
}

unsafe extern "C" fn gl_delete_textures(n: GLsizei, textures: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *textures.add(i) };
            if let Some(t) = s.textures.remove(&id) {
                s.ctx.delete_texture(Some(&t));
            }
        }
    });
}

unsafe extern "C" fn gl_delete_vertex_arrays(n: GLsizei, arrays: *const GLuint) {
    with_gl!(s, {
        for i in 0..n as usize {
            let id = unsafe { *arrays.add(i) };
            if let Some(v) = s.vaos.remove(&id) {
                s.ctx.delete_vertex_array(Some(&v));
            }
        }
    });
}

unsafe extern "C" fn gl_depth_mask(flag: GLboolean) {
    with_gl!(s, s.ctx.depth_mask(flag != 0));
}

unsafe extern "C" fn gl_disable(cap: GLenum) {
    with_gl!(s, s.ctx.disable(cap));
}

unsafe extern "C" fn gl_disable_vertex_attrib_array(index: GLuint) {
    with_gl!(s, s.ctx.disable_vertex_attrib_array(index));
}

unsafe extern "C" fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    with_gl!(s, s.ctx.draw_arrays(mode, first, count));
}

unsafe extern "C" fn gl_draw_elements(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
) {
    with_gl!(
        s,
        s.ctx
            .draw_elements_with_i32(mode, count, type_, indices as i32)
    );
}

unsafe extern "C" fn gl_enable(cap: GLenum) {
    with_gl!(s, s.ctx.enable(cap));
}

unsafe extern "C" fn gl_enable_vertex_attrib_array(index: GLuint) {
    with_gl!(s, s.ctx.enable_vertex_attrib_array(index));
}

unsafe extern "C" fn gl_finish() {
    with_gl!(s, s.ctx.finish());
}

unsafe extern "C" fn gl_flush() {
    with_gl!(s, s.ctx.flush());
}

unsafe extern "C" fn gl_framebuffer_renderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    with_gl!(s, {
        let rb = (renderbuffer != 0)
            .then(|| s.renderbuffers.get(&renderbuffer))
            .flatten();
        s.ctx
            .framebuffer_renderbuffer(target, attachment, renderbuffertarget, rb);
    });
}

unsafe extern "C" fn gl_framebuffer_texture_2d(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    with_gl!(s, {
        let t = (texture != 0).then(|| s.textures.get(&texture)).flatten();
        s.ctx
            .framebuffer_texture_2d(target, attachment, textarget, t, level);
    });
}

unsafe extern "C" fn gl_front_face(mode: GLenum) {
    with_gl!(s, s.ctx.front_face(mode));
}

macro_rules! gen_objects {
    ($fn_name:ident, $create:ident, $map:ident) => {
        unsafe extern "C" fn $fn_name(n: GLsizei, out: *mut GLuint) {
            with_gl!(s, {
                for i in 0..n as usize {
                    let id = match s.ctx.$create() {
                        Some(obj) => {
                            let id = s.alloc_id();
                            s.$map.insert(id, obj);
                            id
                        }
                        None => 0,
                    };
                    unsafe { *out.add(i) = id };
                }
            });
        }
    };
}

gen_objects!(gl_gen_buffers, create_buffer, buffers);
gen_objects!(gl_gen_framebuffers, create_framebuffer, framebuffers);
gen_objects!(gl_gen_queries, create_query, queries);
gen_objects!(gl_gen_renderbuffers, create_renderbuffer, renderbuffers);
gen_objects!(gl_gen_textures, create_texture, textures);
gen_objects!(gl_gen_vertex_arrays, create_vertex_array, vaos);

unsafe extern "C" fn gl_get_booleanv(pname: GLenum, params: *mut GLboolean) {
    with_gl!(s, {
        unsafe { *params = GL_FALSE };
        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }
        match s.ctx.get_parameter(pname) {
            Ok(v) => unsafe {
                *params = if v.as_bool().unwrap_or(false) {
                    GL_TRUE
                } else {
                    GL_FALSE
                };
            },
            Err(_) => {
                let _ = s.ctx.get_error();
            }
        }
    });
}

unsafe extern "C" fn gl_get_error() -> GLenum {
    with_gl!(s => s.ctx.get_error())
}

unsafe extern "C" fn gl_get_framebuffer_attachment_parameter_iv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl!(s, {
        if params.is_null() {
            return;
        }
        unsafe { *params = 0 };
        if let Ok(v) = s
            .ctx
            .get_framebuffer_attachment_parameter(target, attachment, pname)
        {
            if let Some(n) = v.as_f64() {
                unsafe { *params = n as GLint };
            } else if let Some(b) = v.as_bool() {
                unsafe { *params = b as GLint };
            } else if v.is_object() {
                // WebGL returns a WebGLRenderbuffer/WebGLTexture object for
                // FRAMEBUFFER_ATTACHMENT_OBJECT_NAME. GL expects an integer object name.
                // Skia uses this as a presence check, so return a non-zero sentinel.
                unsafe { *params = 1 };
            }
        }
    });
}

unsafe extern "C" fn gl_get_integerv(pname: GLenum, params: *mut GLint) {
    with_gl!(s, {
        unsafe { *params = 0 };
        match pname {
            GL_NUM_EXTENSIONS => {
                unsafe { *params = s.extension_strs.len() as GLint };
                return;
            }
            0x84E2 => {
                // GL_MAX_TEXTURE_UNITS (legacy alias)
                if let Some(v) = get_parameter_int(&s.ctx, Gl::MAX_TEXTURE_IMAGE_UNITS) {
                    unsafe { *params = v };
                }
                return;
            }
            0x8B4A => {
                // GL_MAX_VERTEX_UNIFORM_COMPONENTS_ARB
                if let Some(v) = get_parameter_int(&s.ctx, Gl::MAX_VERTEX_UNIFORM_VECTORS) {
                    unsafe { *params = v * 4 };
                }
                return;
            }
            0x8B49 => {
                // GL_MAX_FRAGMENT_UNIFORM_COMPONENTS_ARB
                if let Some(v) = get_parameter_int(&s.ctx, Gl::MAX_FRAGMENT_UNIFORM_VECTORS) {
                    unsafe { *params = v * 4 };
                }
                return;
            }
            0x8B4B => {
                // GL_MAX_VARYING_FLOATS_ARB
                if let Some(v) = get_parameter_int(&s.ctx, Gl::MAX_VARYING_VECTORS) {
                    unsafe { *params = v * 4 };
                }
                return;
            }
            0x8871 => {
                // GL_MAX_TEXTURE_COORDS
                if let Some(v) = get_parameter_int(&s.ctx, Gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS) {
                    unsafe { *params = v };
                }
                return;
            }
            0x821B => {
                // GL_MAJOR_VERSION — report 3 to match "OpenGL ES 3.0" version string
                unsafe { *params = 3 };
                return;
            }
            0x821C => {
                // GL_MINOR_VERSION
                unsafe { *params = 0 };
                return;
            }
            _ => {}
        }

        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }

        match s.ctx.get_parameter(pname) {
            Ok(v) => write_glint_params_from_js(pname, params, &v),
            Err(_) => {
                let _ = s.ctx.get_error();
            }
        }
    });
}

unsafe extern "C" fn gl_get_program_info_log(
    program: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut c_char,
) {
    with_gl!(s, {
        if let Some(p) = s.programs.get(&program) {
            let log = s.ctx.get_program_info_log(p).unwrap_or_default();
            copy_info_log(log.as_bytes(), max_length, length, info_log);
        }
    });
}

unsafe extern "C" fn gl_get_program_iv(program: GLuint, pname: GLenum, params: *mut GLint) {
    with_gl!(s, {
        if let Some(p) = s.programs.get(&program) {
            let v = s.ctx.get_program_parameter(p, pname);
            let val = js_value_to_glint(&v);
            unsafe { *params = val };
        }
    });
}

unsafe extern "C" fn gl_get_renderbuffer_parameter_iv(
    target: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    with_gl!(s, {
        let v = s.ctx.get_renderbuffer_parameter(target, pname);
        unsafe { *params = js_value_to_glint(&v) };
    });
}

unsafe extern "C" fn gl_get_shader_info_log(
    shader: GLuint,
    max_length: GLsizei,
    length: *mut GLsizei,
    info_log: *mut c_char,
) {
    with_gl!(s, {
        if let Some(sh) = s.shaders.get(&shader) {
            let log = s.ctx.get_shader_info_log(sh).unwrap_or_default();
            copy_info_log(log.as_bytes(), max_length, length, info_log);
        }
    });
}

unsafe extern "C" fn gl_get_shaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    with_gl!(s, {
        if let Some(sh) = s.shaders.get(&shader) {
            let v = s.ctx.get_shader_parameter(sh, pname);
            unsafe { *params = js_value_to_glint(&v) };
        }
    });
}

unsafe extern "C" fn gl_get_shader_precision_format(
    _shader_type: GLenum,
    _precision_type: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    // WebGL does not expose precision details; return safe defaults.
    unsafe {
        *range = 127;
        *range.add(1) = 127;
        *precision = 23;
    }
}

unsafe extern "C" fn gl_get_string(name: GLenum) -> *const u8 {
    with_gl_ref!(s => match name {
        0x1F00 => s.vendor_cstr.as_ptr(),          // GL_VENDOR
        0x1F01 => s.renderer_cstr.as_ptr(),        // GL_RENDERER
        0x1F02 => s.version_cstr.as_ptr(),         // GL_VERSION
        0x8B8C => s.shading_lang_cstr.as_ptr(),    // GL_SHADING_LANGUAGE_VERSION
        GL_EXTENSIONS => c"".as_ptr().cast(),      // GL_EXTENSIONS (invalid in ES3; use glGetStringi)
        _ => std::ptr::null(),
    })
}

unsafe extern "C" fn gl_get_stringi(name: GLenum, index: GLuint) -> *const u8 {
    with_gl_ref!(s => {
        if name == GL_EXTENSIONS {
            // GL_EXTENSIONS
            s.extension_strs
                .get(index as usize)
                .map_or(std::ptr::null(), |v| v.as_ptr())
        } else {
            std::ptr::null()
        }
    })
}

unsafe extern "C" fn gl_get_uniform_location(program: GLuint, name: *const c_char) -> GLint {
    let name_str = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    with_gl!(s => {
        if let Some(p) = s.programs.get(&program) {
            match s.ctx.get_uniform_location(p, name_str) {
                Some(loc) => { let id = s.alloc_id(); s.uniform_locs.insert(id, loc); id as GLint }
                None => -1,
            }
        } else {
            -1
        }
    })
}

unsafe extern "C" fn gl_invalidate_framebuffer(
    target: GLenum,
    num_attachments: GLsizei,
    attachments: *const GLenum,
) {
    with_gl!(s, {
        let arr = js_sys::Array::new();
        for i in 0..num_attachments as usize {
            let att = unsafe { *attachments.add(i) };
            arr.push(&wasm_bindgen::JsValue::from_f64(att as f64));
        }
        let _ = s.ctx.invalidate_framebuffer(target, &arr);
    });
}

unsafe extern "C" fn gl_is_texture(texture: GLuint) -> GLboolean {
    with_gl!(s => {
        s.textures.get(&texture)
            .map(|t| if s.ctx.is_texture(Some(t)) { GL_TRUE } else { GL_FALSE })
            .unwrap_or(GL_FALSE)
    })
}

unsafe extern "C" fn gl_line_width(width: GLfloat) {
    with_gl!(s, s.ctx.line_width(width));
}

unsafe extern "C" fn gl_link_program(program: GLuint) {
    with_gl!(s, {
        if let Some(p) = s.programs.get(&program) {
            s.ctx.link_program(p);
        }
    });
}

unsafe extern "C" fn gl_pixel_store_i(pname: GLenum, param: GLint) {
    with_gl!(s, s.ctx.pixel_storei(pname, param));
}

unsafe extern "C" fn gl_read_pixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
) {
    with_gl!(s, {
        let len = pixel_byte_size(width, height, format, type_);
        let slice = unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, len) };
        let _ =
            s.ctx
                .read_pixels_with_opt_u8_array(x, y, width, height, format, type_, Some(slice));
    });
}

unsafe extern "C" fn gl_renderbuffer_storage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl!(
        s,
        s.ctx
            .renderbuffer_storage(target, internalformat, width, height)
    );
}

unsafe extern "C" fn gl_renderbuffer_storage_multisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl!(
        s,
        s.ctx
            .renderbuffer_storage_multisample(target, samples, internalformat, width, height)
    );
}

unsafe extern "C" fn gl_scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_gl!(s, s.ctx.scissor(x, y, width, height));
}

unsafe extern "C" fn gl_shader_source(
    shader: GLuint,
    count: GLsizei,
    strings: *const *const c_char,
    lengths: *const GLint,
) {
    with_gl!(s, {
        if let Some(sh) = s.shaders.get(&shader) {
            let mut source = String::new();
            for i in 0..count as usize {
                let ptr = unsafe { *strings.add(i) };
                let chunk = if lengths.is_null() || unsafe { *lengths.add(i) } < 0 {
                    unsafe { CStr::from_ptr(ptr) }
                        .to_string_lossy()
                        .into_owned()
                } else {
                    let len = unsafe { *lengths.add(i) } as usize;
                    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
                    String::from_utf8_lossy(bytes).into_owned()
                };
                source.push_str(&chunk);
            }
            s.ctx.shader_source(sh, &source);
        }
    });
}

unsafe extern "C" fn gl_stencil_func(func: GLenum, ref_: GLint, mask: GLuint) {
    with_gl!(s, s.ctx.stencil_func(func, ref_, mask));
}

unsafe extern "C" fn gl_stencil_func_separate(
    face: GLenum,
    func: GLenum,
    ref_: GLint,
    mask: GLuint,
) {
    with_gl!(s, s.ctx.stencil_func_separate(face, func, ref_, mask));
}

unsafe extern "C" fn gl_stencil_mask(mask: GLuint) {
    with_gl!(s, s.ctx.stencil_mask(mask));
}

unsafe extern "C" fn gl_stencil_mask_separate(face: GLenum, mask: GLuint) {
    with_gl!(s, s.ctx.stencil_mask_separate(face, mask));
}

unsafe extern "C" fn gl_stencil_op(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    with_gl!(s, s.ctx.stencil_op(fail, zfail, zpass));
}

unsafe extern "C" fn gl_stencil_op_separate(
    face: GLenum,
    fail: GLenum,
    zfail: GLenum,
    zpass: GLenum,
) {
    with_gl!(s, s.ctx.stencil_op_separate(face, fail, zfail, zpass));
}

unsafe extern "C" fn gl_tex_image_2d(
    target: GLenum,
    level: GLint,
    internal_format: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl!(s, {
        let data: Option<&[u8]> = if pixels.is_null() {
            None
        } else {
            let len = pixel_byte_size(width, height, format, type_);
            Some(unsafe { std::slice::from_raw_parts(pixels as *const u8, len) })
        };
        let _ = s
            .ctx
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                type_,
                data,
            );
    });
}

unsafe extern "C" fn gl_tex_parameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    with_gl!(s, s.ctx.tex_parameterf(target, pname, param));
}

unsafe extern "C" fn gl_tex_parameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    with_gl!(s, {
        if !params.is_null() {
            let p0 = unsafe { *params };
            s.ctx.tex_parameterf(target, pname, p0);
        }
    });
}

unsafe extern "C" fn gl_tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    with_gl!(s, s.ctx.tex_parameteri(target, pname, param));
}

unsafe extern "C" fn gl_tex_parameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    with_gl!(s, {
        if !params.is_null() {
            let p0 = unsafe { *params };
            s.ctx.tex_parameteri(target, pname, p0);
        }
    });
}

unsafe extern "C" fn gl_tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    with_gl!(s, {
        let len = pixel_byte_size(width, height, format, type_);
        let bytes = unsafe { std::slice::from_raw_parts(pixels as *const u8, len) };
        let _ = s
            .ctx
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                target,
                level,
                xoffset,
                yoffset,
                width,
                height,
                format,
                type_,
                Some(bytes),
            );
    });
}

// --- Uniform setters -------------------------------------------------

macro_rules! uniform_scalar {
    ($fn_name:ident, $method:ident, $T:ty) => {
        unsafe extern "C" fn $fn_name(location: GLint, v0: $T) {
            with_gl!(s, {
                if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
                    s.ctx.$method(Some(loc), v0);
                }
            });
        }
    };
    ($fn_name:ident, $method:ident, $T:ty, $v1:ident) => {
        unsafe extern "C" fn $fn_name(location: GLint, v0: $T, $v1: $T) {
            with_gl!(s, {
                if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
                    s.ctx.$method(Some(loc), v0, $v1);
                }
            });
        }
    };
}

macro_rules! uniform_vec {
    ($fn_name:ident, $method:ident, $T:ty, $n:expr) => {
        unsafe extern "C" fn $fn_name(location: GLint, count: GLsizei, value: *const $T) {
            with_gl!(s, {
                if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
                    let data = unsafe { std::slice::from_raw_parts(value, count as usize * $n) };
                    s.ctx.$method(Some(loc), data);
                }
            });
        }
    };
}

macro_rules! uniform_matrix {
    ($fn_name:ident, $method:ident, $n:expr) => {
        unsafe extern "C" fn $fn_name(
            location: GLint,
            count: GLsizei,
            transpose: GLboolean,
            value: *const GLfloat,
        ) {
            with_gl!(s, {
                if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
                    let data =
                        unsafe { std::slice::from_raw_parts(value, count as usize * $n * $n) };
                    s.ctx.$method(Some(loc), transpose != 0, data);
                }
            });
        }
    };
}

uniform_scalar!(gl_uniform1f, uniform1f, GLfloat);
uniform_scalar!(gl_uniform1i, uniform1i, GLint);
uniform_vec!(gl_uniform1fv, uniform1fv_with_f32_array, GLfloat, 1);
uniform_vec!(gl_uniform1iv, uniform1iv_with_i32_array, GLint, 1);

unsafe extern "C" fn gl_uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform2f(Some(loc), v0, v1);
        }
    });
}
unsafe extern "C" fn gl_uniform2i(location: GLint, v0: GLint, v1: GLint) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform2i(Some(loc), v0, v1);
        }
    });
}
uniform_vec!(gl_uniform2fv, uniform2fv_with_f32_array, GLfloat, 2);
uniform_vec!(gl_uniform2iv, uniform2iv_with_i32_array, GLint, 2);

unsafe extern "C" fn gl_uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform3f(Some(loc), v0, v1, v2);
        }
    });
}
unsafe extern "C" fn gl_uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform3i(Some(loc), v0, v1, v2);
        }
    });
}
uniform_vec!(gl_uniform3fv, uniform3fv_with_f32_array, GLfloat, 3);
uniform_vec!(gl_uniform3iv, uniform3iv_with_i32_array, GLint, 3);

unsafe extern "C" fn gl_uniform4f(
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform4f(Some(loc), v0, v1, v2, v3);
        }
    });
}
unsafe extern "C" fn gl_uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    with_gl!(s, {
        if let Some(loc) = s.uniform_locs.get(&(location as GLuint)) {
            s.ctx.uniform4i(Some(loc), v0, v1, v2, v3);
        }
    });
}
uniform_vec!(gl_uniform4fv, uniform4fv_with_f32_array, GLfloat, 4);
uniform_vec!(gl_uniform4iv, uniform4iv_with_i32_array, GLint, 4);

uniform_matrix!(gl_uniform_matrix2fv, uniform_matrix2fv_with_f32_array, 2);
uniform_matrix!(gl_uniform_matrix3fv, uniform_matrix3fv_with_f32_array, 3);
uniform_matrix!(gl_uniform_matrix4fv, uniform_matrix4fv_with_f32_array, 4);

unsafe extern "C" fn gl_use_program(program: GLuint) {
    with_gl!(s, {
        let p = (program != 0).then(|| s.programs.get(&program)).flatten();
        s.ctx.use_program(p);
    });
}

unsafe extern "C" fn gl_vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
) {
    with_gl!(
        s,
        s.ctx.vertex_attrib_pointer_with_i32(
            index,
            size,
            type_,
            normalized != 0,
            stride,
            pointer as i32,
        )
    );
}

unsafe extern "C" fn gl_viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    with_gl!(s, s.ctx.viewport(x, y, width, height));
}

unsafe extern "C" fn gl_begin_query(target: GLenum, id: GLuint) {
    with_gl!(s, {
        if let Some(q) = s.queries.get(&id) {
            s.ctx.begin_query(target, q);
        }
    });
}

unsafe extern "C" fn gl_end_query(target: GLenum) {
    with_gl!(s, s.ctx.end_query(target));
}

unsafe extern "C" fn gl_get_query_object_uiv(id: GLuint, pname: GLenum, params: *mut GLuint) {
    with_gl!(s, {
        if let Some(q) = s.queries.get(&id) {
            let v = s.ctx.get_query_parameter(q, pname);
            let val = if let Some(n) = v.as_f64() {
                n as GLuint
            } else if let Some(b) = v.as_bool() {
                b as GLuint
            } else {
                return;
            };
            unsafe { *params = val };
        }
    });
}

// --- Additional GL functions needed by Skia's GLES assembler --------

unsafe extern "C" fn gl_generate_mipmap(target: GLenum) {
    with_gl!(s, s.ctx.generate_mipmap(target));
}

unsafe extern "C" fn gl_tex_storage_2d(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    with_gl!(
        s,
        s.ctx
            .tex_storage_2d(target, levels, internalformat, width, height)
    );
}

unsafe extern "C" fn gl_vertex_attrib1f(index: GLuint, x: GLfloat) {
    with_gl!(s, s.ctx.vertex_attrib1f(index, x));
}

unsafe extern "C" fn gl_vertex_attrib2fv(index: GLuint, v: *const GLfloat) {
    with_gl!(s, {
        let arr = unsafe { std::slice::from_raw_parts(v, 2) };
        s.ctx.vertex_attrib2fv_with_f32_array(index, arr);
    });
}

unsafe extern "C" fn gl_vertex_attrib3fv(index: GLuint, v: *const GLfloat) {
    with_gl!(s, {
        let arr = unsafe { std::slice::from_raw_parts(v, 3) };
        s.ctx.vertex_attrib3fv_with_f32_array(index, arr);
    });
}

unsafe extern "C" fn gl_vertex_attrib4fv(index: GLuint, v: *const GLfloat) {
    with_gl!(s, {
        let arr = unsafe { std::slice::from_raw_parts(v, 4) };
        s.ctx.vertex_attrib4fv_with_f32_array(index, arr);
    });
}

unsafe extern "C" fn gl_get_buffer_parameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    with_gl!(s, {
        let v = s.ctx.get_buffer_parameter(target, pname);
        if let Some(n) = v.as_f64() {
            unsafe { *params = n as GLint };
        }
    });
}

unsafe extern "C" fn gl_is_enabled(cap: GLenum) -> GLboolean {
    with_gl!(s => if s.ctx.is_enabled(cap) { GL_TRUE } else { GL_FALSE })
}

unsafe extern "C" fn gl_depth_func(func: GLenum) {
    with_gl!(s, s.ctx.depth_func(func));
}

unsafe extern "C" fn gl_depth_rangef(n: GLclampf, f: GLclampf) {
    with_gl!(s, s.ctx.depth_range(n, f));
}

unsafe extern "C" fn gl_get_floatv(pname: GLenum, params: *mut GLfloat) {
    with_gl!(s, {
        unsafe { *params = 0.0 };
        if !is_valid_webgl2_get_parameter(pname) {
            return;
        }
        match s.ctx.get_parameter(pname) {
            Ok(v) => write_glfloat_params_from_js(pname, params, &v),
            Err(_) => {
                let _ = s.ctx.get_error();
            }
        }
    });
}

// -------------------------------------------------------------------
// Internal helpers
// -------------------------------------------------------------------

fn js_value_to_glint(v: &wasm_bindgen::JsValue) -> GLint {
    if let Some(n) = v.as_f64() {
        n as GLint
    } else if let Some(b) = v.as_bool() {
        b as GLint
    } else {
        0
    }
}

fn get_parameter_int(ctx: &Gl, pname: GLenum) -> Option<GLint> {
    ctx.get_parameter(pname).ok().map(|v| js_value_to_glint(&v))
}

fn write_glint_params_from_js(pname: GLenum, params: *mut GLint, value: &wasm_bindgen::JsValue) {
    let count = expected_gl_param_count(pname);
    for i in 0..count {
        unsafe { *params.add(i) = 0 };
    }
    if let Some(n) = value.as_f64() {
        unsafe { *params = n as GLint };
        return;
    }
    if let Some(b) = value.as_bool() {
        unsafe { *params = b as GLint };
        return;
    }
    if let Some(len) = js_array_len(value) {
        let len = len.min(count as u32);
        for i in 0..len {
            if let Some(n) = js_array_num(value, i) {
                unsafe { *params.add(i as usize) = n as GLint };
            }
        }
    }
}

fn write_glfloat_params_from_js(
    pname: GLenum,
    params: *mut GLfloat,
    value: &wasm_bindgen::JsValue,
) {
    let count = expected_gl_param_count(pname);
    for i in 0..count {
        unsafe { *params.add(i) = 0.0 };
    }
    if let Some(n) = value.as_f64() {
        unsafe { *params = n as GLfloat };
        return;
    }
    if let Some(len) = js_array_len(value) {
        let len = len.min(count as u32);
        for i in 0..len {
            if let Some(n) = js_array_num(value, i) {
                unsafe { *params.add(i as usize) = n as GLfloat };
            }
        }
    }
}

fn expected_gl_param_count(pname: GLenum) -> usize {
    match pname {
        0x0BA2 | // GL_VIEWPORT
        0x0C10 => 4, // GL_SCISSOR_BOX
        0x0D3A | // GL_MAX_VIEWPORT_DIMS
        0x846D | // GL_ALIASED_POINT_SIZE_RANGE
        0x846E => 2, // GL_ALIASED_LINE_WIDTH_RANGE
        _ => 1,
    }
}

fn js_array_len(value: &wasm_bindgen::JsValue) -> Option<u32> {
    js_sys::Reflect::get(value, &wasm_bindgen::JsValue::from_str("length"))
        .ok()
        .and_then(|v| v.as_f64())
        .map(|n| n as u32)
}

fn js_array_num(value: &wasm_bindgen::JsValue, index: u32) -> Option<f64> {
    js_sys::Reflect::get_u32(value, index)
        .ok()
        .and_then(|v| v.as_f64())
}

fn copy_info_log(bytes: &[u8], max_length: GLsizei, length: *mut GLsizei, buf: *mut c_char) {
    if max_length <= 0 || buf.is_null() {
        return;
    }
    let copy_len = bytes.len().min(max_length as usize - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, copy_len);
        *buf.add(copy_len) = 0;
        if !length.is_null() {
            *length = copy_len as GLsizei;
        }
    }
}

// -------------------------------------------------------------------
// ES 3.0 — instanced drawing
// -------------------------------------------------------------------

unsafe extern "C" fn gl_draw_arrays_instanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instance_count: GLsizei,
) {
    with_gl!(
        s,
        s.ctx
            .draw_arrays_instanced(mode, first, count, instance_count)
    )
}

unsafe extern "C" fn gl_draw_elements_instanced(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instance_count: GLsizei,
) {
    with_gl!(s, {
        s.ctx
            .draw_elements_instanced_with_i32(mode, count, type_, indices as i32, instance_count)
    })
}

unsafe extern "C" fn gl_vertex_attrib_divisor(index: GLuint, divisor: GLuint) {
    with_gl!(s, s.ctx.vertex_attrib_divisor(index, divisor))
}

unsafe extern "C" fn gl_draw_range_elements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
) {
    with_gl!(s, {
        s.ctx
            .draw_range_elements_with_i32(mode, start, end, count, type_, indices as i32)
    })
}

unsafe extern "C" fn gl_vertex_attrib_i_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
) {
    with_gl!(s, {
        s.ctx
            .vertex_attrib_i_pointer_with_i32(index, size, type_, stride, pointer as i32)
    })
}

// -------------------------------------------------------------------
// ES 3.0 — framebuffer / buffer
// -------------------------------------------------------------------

unsafe extern "C" fn gl_draw_buffers(n: GLsizei, bufs: *const GLenum) {
    if n <= 0 || bufs.is_null() {
        return;
    }
    let array = js_sys::Array::new();
    for i in 0..n as usize {
        array.push(&wasm_bindgen::JsValue::from_f64(*bufs.add(i) as f64));
    }
    with_gl!(s, s.ctx.draw_buffers(&array))
}

unsafe extern "C" fn gl_read_buffer(src: GLenum) {
    with_gl!(s, s.ctx.read_buffer(src))
}

unsafe extern "C" fn gl_copy_buffer_sub_data(
    read_target: GLenum,
    write_target: GLenum,
    read_offset: GLintptr,
    write_offset: GLintptr,
    size: GLsizeiptr,
) {
    with_gl!(s, {
        s.ctx.copy_buffer_sub_data_with_i32_and_i32_and_i32(
            read_target,
            write_target,
            read_offset,
            write_offset,
            size,
        )
    })
}

unsafe extern "C" fn gl_invalidate_sub_framebuffer(
    target: GLenum,
    num_attachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    if num_attachments <= 0 || attachments.is_null() {
        return;
    }
    let array = js_sys::Array::new();
    for i in 0..num_attachments as usize {
        array.push(&wasm_bindgen::JsValue::from_f64(*attachments.add(i) as f64));
    }
    with_gl!(s, {
        let _ = s
            .ctx
            .invalidate_sub_framebuffer(target, &array, x, y, width, height);
    })
}

// -------------------------------------------------------------------
// ES 3.0 — buffer mapping (emulated)
// -------------------------------------------------------------------

unsafe extern "C" fn gl_map_buffer_range(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    _access: GLbitfield,
) -> *mut c_void {
    if length <= 0 {
        return std::ptr::null_mut();
    }
    GL_CONTEXTS.with(|ctxs| {
        let mut borrow = ctxs.borrow_mut();
        let id = GL_CURRENT.with(|c| c.get());
        if let Some(s) = borrow.get_mut(&id) {
            let data = vec![0u8; length as usize];
            let ptr = data.as_ptr() as *mut c_void;
            s.mapped_buffer = Some(MappedBuffer {
                target,
                offset,
                data,
            });
            ptr
        } else {
            std::ptr::null_mut()
        }
    })
}

unsafe extern "C" fn gl_unmap_buffer(target: GLenum) -> GLboolean {
    GL_CONTEXTS.with(|ctxs| {
        let mut borrow = ctxs.borrow_mut();
        let id = GL_CURRENT.with(|c| c.get());
        if let Some(s) = borrow.get_mut(&id) {
            if let Some(mapped) = s.mapped_buffer.take() {
                if mapped.target != target {
                    return GL_FALSE;
                }
                let view = unsafe { js_sys::Uint8Array::view(&mapped.data) };
                s.ctx
                    .buffer_sub_data_with_i32_and_array_buffer_view(target, mapped.offset, &view);
            } else {
                return GL_FALSE;
            }
            GL_TRUE
        } else {
            GL_FALSE
        }
    })
}

unsafe extern "C" fn gl_flush_mapped_buffer_range(
    _target: GLenum,
    _offset: GLintptr,
    _length: GLsizeiptr,
) {
    // no-op; data is fully uploaded on gl_unmap_buffer
}

// -------------------------------------------------------------------
// ES 3.0 — sync objects
// -------------------------------------------------------------------

unsafe extern "C" fn gl_fence_sync(condition: GLenum, _flags: GLbitfield) -> *mut c_void {
    GL_CONTEXTS.with(|ctxs| {
        let mut borrow = ctxs.borrow_mut();
        let id = GL_CURRENT.with(|c| c.get());
        if let Some(s) = borrow.get_mut(&id) {
            match s.ctx.fence_sync(condition, 0) {
                Some(sync) => {
                    let handle = s.alloc_id();
                    s.syncs.insert(handle, sync);
                    handle as usize as *mut c_void
                }
                None => std::ptr::null_mut(),
            }
        } else {
            std::ptr::null_mut()
        }
    })
}

unsafe extern "C" fn gl_client_wait_sync(
    sync: *mut c_void,
    flags: GLbitfield,
    timeout: u64,
) -> GLenum {
    let handle = sync as usize as GLuint;
    GL_CONTEXTS.with(|ctxs| {
        let mut borrow = ctxs.borrow_mut();
        let id = GL_CURRENT.with(|c| c.get());
        if let Some(s) = borrow.get_mut(&id) {
            if let Some(sync_obj) = s.syncs.get(&handle) {
                s.ctx.client_wait_sync_with_u32(
                    sync_obj,
                    flags,
                    timeout.min(u32::MAX as u64) as u32,
                )
            } else {
                0x911C // GL_WAIT_FAILED
            }
        } else {
            0x911C
        }
    })
}

unsafe extern "C" fn gl_delete_sync(sync: *mut c_void) {
    if sync.is_null() {
        return;
    }
    let handle = sync as usize as GLuint;
    with_gl!(s, {
        if let Some(sync_obj) = s.syncs.remove(&handle) {
            s.ctx.delete_sync(Some(&sync_obj));
        }
    })
}

unsafe extern "C" fn gl_is_sync(sync: *mut c_void) -> GLboolean {
    if sync.is_null() {
        return GL_FALSE;
    }
    let handle = sync as usize as GLuint;
    with_gl!(s => {
        match s.syncs.get(&handle) {
            Some(sync_obj) => {
                if s.ctx.is_sync(Some(sync_obj)) { GL_TRUE } else { GL_FALSE }
            }
            None => GL_FALSE,
        }
    })
}

unsafe extern "C" fn gl_wait_sync(sync: *mut c_void, flags: GLbitfield, _timeout: u64) {
    if sync.is_null() {
        return;
    }
    let handle = sync as usize as GLuint;
    with_gl!(s, {
        if let Some(sync_obj) = s.syncs.get(&handle) {
            // timeout must be TIMEOUT_IGNORED (-1) per WebGL spec
            s.ctx.wait_sync_with_f64(sync_obj, flags, -1.0);
        }
    })
}

// -------------------------------------------------------------------
// ES 3.0 — sampler objects
// -------------------------------------------------------------------

unsafe extern "C" fn gl_gen_samplers(count: GLsizei, samplers: *mut GLuint) {
    if count <= 0 || samplers.is_null() {
        return;
    }
    with_gl!(s, {
        for i in 0..count as usize {
            let handle = if let Some(obj) = s.ctx.create_sampler() {
                let id = s.alloc_id();
                s.samplers.insert(id, obj);
                id
            } else {
                0
            };
            *samplers.add(i) = handle;
        }
    })
}

unsafe extern "C" fn gl_delete_samplers(count: GLsizei, samplers: *const GLuint) {
    if count <= 0 || samplers.is_null() {
        return;
    }
    with_gl!(s, {
        for i in 0..count as usize {
            let id = *samplers.add(i);
            if let Some(obj) = s.samplers.remove(&id) {
                s.ctx.delete_sampler(Some(&obj));
            }
        }
    })
}

unsafe extern "C" fn gl_bind_sampler(unit: GLuint, sampler: GLuint) {
    with_gl!(s, {
        let obj = s.samplers.get(&sampler);
        s.ctx.bind_sampler(unit, obj);
    })
}

unsafe extern "C" fn gl_sampler_parameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    with_gl!(s, {
        if let Some(obj) = s.samplers.get(&sampler) {
            s.ctx.sampler_parameteri(obj, pname, param);
        }
    })
}

unsafe extern "C" fn gl_sampler_parameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    with_gl!(s, {
        if let Some(obj) = s.samplers.get(&sampler) {
            s.ctx.sampler_parameterf(obj, pname, param);
        }
    })
}

unsafe extern "C" fn gl_sampler_parameter_iv(sampler: GLuint, pname: GLenum, params: *const GLint) {
    if params.is_null() {
        return;
    }
    with_gl!(s, {
        if let Some(obj) = s.samplers.get(&sampler) {
            s.ctx.sampler_parameteri(obj, pname, *params);
        }
    })
}

// -------------------------------------------------------------------
// ES 3.0 — queries / misc
// -------------------------------------------------------------------

unsafe extern "C" fn gl_get_queryiv(_target: GLenum, pname: GLenum, params: *mut GLint) {
    // Return reasonable defaults. GL_CURRENT_QUERY = 0 (no active query).
    // GL_QUERY_COUNTER_BITS = 0 (not supported).
    if !params.is_null() {
        *params = match pname {
            0x8865 => 0, // GL_CURRENT_QUERY
            0x8864 => 0, // GL_QUERY_COUNTER_BITS
            _ => 0,
        };
    }
}

unsafe extern "C" fn gl_get_internalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    buf_size: GLsizei,
    params: *mut GLint,
) {
    if buf_size <= 0 || params.is_null() {
        return;
    }
    for i in 0..buf_size as usize {
        unsafe { *params.add(i) = 0 };
    }

    if target != Gl::RENDERBUFFER {
        return;
    }

    // A conservative fallback for common color-renderbuffer formats. If WebGL reports empty sample
    // lists for these, Skia can incorrectly conclude the format is never renderable.
    let fallback_num_samples = match internalformat {
        0x8051 | // GL_RGB8
        0x8056 | // GL_RGBA4
        0x8058 | // GL_RGBA8
        0x8059 | // GL_RGB10_A2
        0x8D62 | // GL_RGB565
        0x8C43 => 1, // GL_SRGB8_ALPHA8
        _ => 0,
    };

    // WebGL2 only supports GL_SAMPLES (0x80A9) and GL_NUM_SAMPLE_COUNTS (0x9380)
    // for getInternalformatParameter. All other pnames generate INVALID_ENUM.
    const GL_SAMPLES: GLenum = 0x80A9;
    const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
    if pname == GL_NUM_SAMPLE_COUNTS {
        with_gl!(s, {
            match s
                .ctx
                .get_internalformat_parameter(target, internalformat, GL_SAMPLES)
            {
                Ok(v) => {
                    if let Some(arr) = v.dyn_ref::<js_sys::Int32Array>() {
                        let n = arr.length() as GLint;
                        unsafe { *params = if n > 0 { n } else { fallback_num_samples } };
                    } else if let Some(n) = v.as_f64() {
                        unsafe { *params = if n > 0.0 { 1 } else { fallback_num_samples } };
                    } else {
                        unsafe { *params = fallback_num_samples };
                    }
                }
                Err(_) => {
                    let _ = s.ctx.get_error();
                    unsafe { *params = fallback_num_samples };
                }
            }
        });
        return;
    }

    if pname != GL_SAMPLES {
        return;
    }
    with_gl!(s, {
        match s
            .ctx
            .get_internalformat_parameter(target, internalformat, pname)
        {
            Ok(v) => {
                // NUM_SAMPLE_COUNTS returns a plain integer; SAMPLES returns an Int32Array
                if let Some(arr) = v.dyn_ref::<js_sys::Int32Array>() {
                    let len = (arr.length() as GLsizei).min(buf_size) as usize;
                    if len > 0 {
                        for i in 0..len {
                            unsafe { *params.add(i) = arr.get_index(i as u32) };
                        }
                    } else {
                        unsafe { *params = fallback_num_samples };
                    }
                } else if let Some(n) = v.as_f64() {
                    unsafe {
                        *params = if n > 0.0 {
                            n as GLint
                        } else {
                            fallback_num_samples
                        }
                    };
                } else {
                    unsafe { *params = fallback_num_samples };
                }
            }
            Err(_) => {
                // INVALID_ENUM from unsupported internalformat — clear the error flag
                let _ = s.ctx.get_error();
                unsafe { *params = fallback_num_samples };
            }
        }
    })
}

// -------------------------------------------------------------------
// ES 3.0 — program binary (stub; WebGL doesn't expose binary format)
// -------------------------------------------------------------------

unsafe extern "C" fn gl_get_program_binary(
    _program: GLuint,
    _buf_size: GLsizei,
    length: *mut GLsizei,
    _binary_format: *mut GLenum,
    _binary: *mut c_void,
) {
    if !length.is_null() {
        *length = 0;
    }
}

unsafe extern "C" fn gl_program_binary(
    _program: GLuint,
    _binary_format: GLenum,
    _binary: *const c_void,
    _length: GLsizei,
) {
}

unsafe extern "C" fn gl_program_parameteri(_program: GLuint, _pname: GLenum, _value: GLint) {}

// -------------------------------------------------------------------
// Proc address table
// -------------------------------------------------------------------

/// `GrGLGetProc`-compatible callback used by `GrGLMakeAssembledInterface`.
///
/// Returns a function pointer for each GL function name that Skia's Ganesh
/// renderer requests.  Unknown names return null; Skia ignores missing
/// optional extensions.
pub unsafe extern "C" fn web_sys_get_proc(_ctx: *mut c_void, name: *const c_char) -> *const c_void {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    match name {
        "glActiveTexture" => gl_active_texture as *const c_void,
        "glAttachShader" => gl_attach_shader as *const c_void,
        "glBindAttribLocation" => gl_bind_attrib_location as *const c_void,
        "glBindBuffer" => gl_bind_buffer as *const c_void,
        "glBindFramebuffer" => gl_bind_framebuffer as *const c_void,
        "glBindRenderbuffer" => gl_bind_renderbuffer as *const c_void,
        "glBindTexture" => gl_bind_texture as *const c_void,
        "glBindVertexArray" | "glBindVertexArrayOES" => gl_bind_vertex_array as *const c_void,
        "glBlendColor" => gl_blend_color as *const c_void,
        "glBlendEquation" => gl_blend_equation as *const c_void,
        "glBlendEquationSeparate" => gl_blend_equation_separate as *const c_void,
        "glBlendFunc" => gl_blend_func as *const c_void,
        "glBlendFuncSeparate" => gl_blend_func_separate as *const c_void,
        "glBlitFramebuffer" => gl_blit_framebuffer as *const c_void,
        "glBufferData" => gl_buffer_data as *const c_void,
        "glBufferSubData" => gl_buffer_sub_data as *const c_void,
        "glCheckFramebufferStatus" => gl_check_framebuffer_status as *const c_void,
        "glClear" => gl_clear as *const c_void,
        "glClearColor" => gl_clear_color as *const c_void,
        "glClearStencil" => gl_clear_stencil as *const c_void,
        "glColorMask" => gl_color_mask as *const c_void,
        "glCompileShader" => gl_compile_shader as *const c_void,
        "glCompressedTexImage2D" => gl_compressed_tex_image_2d as *const c_void,
        "glCompressedTexSubImage2D" => gl_compressed_tex_sub_image_2d as *const c_void,
        "glCopyTexSubImage2D" => gl_copy_tex_sub_image_2d as *const c_void,
        "glCreateProgram" => gl_create_program as *const c_void,
        "glCreateShader" => gl_create_shader as *const c_void,
        "glCullFace" => gl_cull_face as *const c_void,
        "glDeleteBuffers" => gl_delete_buffers as *const c_void,
        "glDeleteFramebuffers" => gl_delete_framebuffers as *const c_void,
        "glDeleteProgram" => gl_delete_program as *const c_void,
        "glDeleteQueries" | "glDeleteQueriesEXT" => gl_delete_queries as *const c_void,
        "glDeleteRenderbuffers" => gl_delete_renderbuffers as *const c_void,
        "glDeleteShader" => gl_delete_shader as *const c_void,
        "glDeleteTextures" => gl_delete_textures as *const c_void,
        "glDeleteVertexArrays" | "glDeleteVertexArraysOES" => {
            gl_delete_vertex_arrays as *const c_void
        }
        "glDepthMask" => gl_depth_mask as *const c_void,
        "glDisable" => gl_disable as *const c_void,
        "glDisableVertexAttribArray" => gl_disable_vertex_attrib_array as *const c_void,
        "glDrawArrays" => gl_draw_arrays as *const c_void,
        "glDrawElements" => gl_draw_elements as *const c_void,
        "glEnable" => gl_enable as *const c_void,
        "glEnableVertexAttribArray" => gl_enable_vertex_attrib_array as *const c_void,
        "glFinish" => gl_finish as *const c_void,
        "glFlush" => gl_flush as *const c_void,
        "glFramebufferRenderbuffer" => gl_framebuffer_renderbuffer as *const c_void,
        "glFramebufferTexture2D" => gl_framebuffer_texture_2d as *const c_void,
        "glFrontFace" => gl_front_face as *const c_void,
        "glGenBuffers" => gl_gen_buffers as *const c_void,
        "glGenFramebuffers" => gl_gen_framebuffers as *const c_void,
        "glGenQueries" | "glGenQueriesEXT" => gl_gen_queries as *const c_void,
        "glGenRenderbuffers" => gl_gen_renderbuffers as *const c_void,
        "glGenTextures" => gl_gen_textures as *const c_void,
        "glGenVertexArrays" | "glGenVertexArraysOES" => gl_gen_vertex_arrays as *const c_void,
        "glGetBooleanv" => gl_get_booleanv as *const c_void,
        "glGetError" => gl_get_error as *const c_void,
        "glGetFramebufferAttachmentParameteriv" => {
            gl_get_framebuffer_attachment_parameter_iv as *const c_void
        }
        "glGetIntegerv" => gl_get_integerv as *const c_void,
        "glGetProgramInfoLog" => gl_get_program_info_log as *const c_void,
        "glGetProgramiv" => gl_get_program_iv as *const c_void,
        "glGetRenderbufferParameteriv" => gl_get_renderbuffer_parameter_iv as *const c_void,
        "glGetShaderInfoLog" => gl_get_shader_info_log as *const c_void,
        "glGetShaderiv" => gl_get_shaderiv as *const c_void,
        "glGetShaderPrecisionFormat" => gl_get_shader_precision_format as *const c_void,
        "glGetString" => gl_get_string as *const c_void,
        "glGetStringi" => gl_get_stringi as *const c_void,
        "glGetUniformLocation" => gl_get_uniform_location as *const c_void,
        "glInvalidateFramebuffer" => gl_invalidate_framebuffer as *const c_void,
        "glIsTexture" => gl_is_texture as *const c_void,
        "glLineWidth" => gl_line_width as *const c_void,
        "glLinkProgram" => gl_link_program as *const c_void,
        "glPixelStorei" => gl_pixel_store_i as *const c_void,
        "glReadPixels" => gl_read_pixels as *const c_void,
        "glRenderbufferStorage" => gl_renderbuffer_storage as *const c_void,
        "glRenderbufferStorageMultisample" => gl_renderbuffer_storage_multisample as *const c_void,
        "glScissor" => gl_scissor as *const c_void,
        "glShaderSource" => gl_shader_source as *const c_void,
        "glStencilFunc" => gl_stencil_func as *const c_void,
        "glStencilFuncSeparate" => gl_stencil_func_separate as *const c_void,
        "glStencilMask" => gl_stencil_mask as *const c_void,
        "glStencilMaskSeparate" => gl_stencil_mask_separate as *const c_void,
        "glStencilOp" => gl_stencil_op as *const c_void,
        "glStencilOpSeparate" => gl_stencil_op_separate as *const c_void,
        "glTexImage2D" => gl_tex_image_2d as *const c_void,
        "glTexParameterf" => gl_tex_parameterf as *const c_void,
        "glTexParameterfv" => gl_tex_parameterfv as *const c_void,
        "glTexParameteri" => gl_tex_parameteri as *const c_void,
        "glTexParameteriv" => gl_tex_parameteriv as *const c_void,
        "glTexSubImage2D" => gl_tex_sub_image_2d as *const c_void,
        "glUniform1f" => gl_uniform1f as *const c_void,
        "glUniform1fv" => gl_uniform1fv as *const c_void,
        "glUniform1i" => gl_uniform1i as *const c_void,
        "glUniform1iv" => gl_uniform1iv as *const c_void,
        "glUniform2f" => gl_uniform2f as *const c_void,
        "glUniform2fv" => gl_uniform2fv as *const c_void,
        "glUniform2i" => gl_uniform2i as *const c_void,
        "glUniform2iv" => gl_uniform2iv as *const c_void,
        "glUniform3f" => gl_uniform3f as *const c_void,
        "glUniform3fv" => gl_uniform3fv as *const c_void,
        "glUniform3i" => gl_uniform3i as *const c_void,
        "glUniform3iv" => gl_uniform3iv as *const c_void,
        "glUniform4f" => gl_uniform4f as *const c_void,
        "glUniform4fv" => gl_uniform4fv as *const c_void,
        "glUniform4i" => gl_uniform4i as *const c_void,
        "glUniform4iv" => gl_uniform4iv as *const c_void,
        "glUniformMatrix2fv" => gl_uniform_matrix2fv as *const c_void,
        "glUniformMatrix3fv" => gl_uniform_matrix3fv as *const c_void,
        "glUniformMatrix4fv" => gl_uniform_matrix4fv as *const c_void,
        "glUseProgram" => gl_use_program as *const c_void,
        "glVertexAttribPointer" => gl_vertex_attrib_pointer as *const c_void,
        "glViewport" => gl_viewport as *const c_void,
        "glBeginQuery" | "glBeginQueryEXT" => gl_begin_query as *const c_void,
        "glEndQuery" | "glEndQueryEXT" => gl_end_query as *const c_void,
        "glGetQueryObjectuiv" | "glGetQueryObjectuivEXT" => {
            gl_get_query_object_uiv as *const c_void
        }
        "glGenerateMipmap" => gl_generate_mipmap as *const c_void,
        "glTexStorage2D" => gl_tex_storage_2d as *const c_void,
        "glVertexAttrib1f" => gl_vertex_attrib1f as *const c_void,
        "glVertexAttrib2fv" => gl_vertex_attrib2fv as *const c_void,
        "glVertexAttrib3fv" => gl_vertex_attrib3fv as *const c_void,
        "glVertexAttrib4fv" => gl_vertex_attrib4fv as *const c_void,
        "glGetBufferParameteriv" => gl_get_buffer_parameteriv as *const c_void,
        "glIsEnabled" => gl_is_enabled as *const c_void,
        "glDepthFunc" => gl_depth_func as *const c_void,
        "glDepthRangef" => gl_depth_rangef as *const c_void,
        "glGetFloatv" => gl_get_floatv as *const c_void,
        // ES 3.0
        "glDrawArraysInstanced" => gl_draw_arrays_instanced as *const c_void,
        "glDrawElementsInstanced" => gl_draw_elements_instanced as *const c_void,
        "glVertexAttribDivisor" => gl_vertex_attrib_divisor as *const c_void,
        "glDrawRangeElements" => gl_draw_range_elements as *const c_void,
        "glVertexAttribIPointer" => gl_vertex_attrib_i_pointer as *const c_void,
        "glDrawBuffers" => gl_draw_buffers as *const c_void,
        "glReadBuffer" => gl_read_buffer as *const c_void,
        "glCopyBufferSubData" => gl_copy_buffer_sub_data as *const c_void,
        "glInvalidateSubFramebuffer" => gl_invalidate_sub_framebuffer as *const c_void,
        "glMapBufferRange" => gl_map_buffer_range as *const c_void,
        "glUnmapBuffer" => gl_unmap_buffer as *const c_void,
        "glFlushMappedBufferRange" => gl_flush_mapped_buffer_range as *const c_void,
        "glFenceSync" => gl_fence_sync as *const c_void,
        "glClientWaitSync" => gl_client_wait_sync as *const c_void,
        "glDeleteSync" => gl_delete_sync as *const c_void,
        "glIsSync" => gl_is_sync as *const c_void,
        "glWaitSync" => gl_wait_sync as *const c_void,
        "glGenSamplers" => gl_gen_samplers as *const c_void,
        "glDeleteSamplers" => gl_delete_samplers as *const c_void,
        "glBindSampler" => gl_bind_sampler as *const c_void,
        "glSamplerParameteri" => gl_sampler_parameteri as *const c_void,
        "glSamplerParameterf" => gl_sampler_parameterf as *const c_void,
        "glSamplerParameteriv" => gl_sampler_parameter_iv as *const c_void,
        "glGetQueryiv" => gl_get_queryiv as *const c_void,
        "glGetInternalformativ" => gl_get_internalformativ as *const c_void,
        "glGetProgramBinary" => gl_get_program_binary as *const c_void,
        "glProgramBinary" => gl_program_binary as *const c_void,
        "glProgramParameteri" => gl_program_parameteri as *const c_void,
        _ => std::ptr::null(),
    }
}

/// Register `ctx` as a new context, make it current, and return `id`
///
/// The returned `id` can be passed to `make_current` / `drop_context` when
/// multiple HTML canvases are in use simultaneously.
pub fn register_gl_context(ctx: web_sys::WebGl2RenderingContext) -> u32 {
    let id = GL_NEXT_ID.with(|n| {
        let v = n.get();
        n.set(v + 1);
        v
    });
    GL_CONTEXTS.with(|ctxs| ctxs.borrow_mut().insert(id, WebGlState::new(ctx)));
    GL_CURRENT.with(|c| c.set(id));

    id
}

/// Make the context registered under `id` the active one on this thread.
pub fn set_gl_context(id: u32) {
    assert!(
        GL_CONTEXTS.with(|c| c.borrow().contains_key(&id)),
        "unknown WebGL context id {id}"
    );
    GL_CURRENT.with(|c| c.set(id));
}

/// Free the context registered under `id`.
///
/// The associated `Interface` must not be used again until a different
/// context is made current.
pub fn drop_gl_context(id: u32) {
    GL_CONTEXTS.with(|ctxs| ctxs.borrow_mut().remove(&id));
    GL_CURRENT.with(|c| {
        if c.get() == id {
            c.set(0);
        }
    });
}
