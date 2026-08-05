pub mod interfaces {
    use crate::gpu::gl;

    #[cfg(target_os = "emscripten")]
    pub fn make_web_gl() -> Option<gl::Interface> {
        use skia_bindings as sb;
        gl::Interface::from_ptr(unsafe { sb::C_GrGLInterfaces_MakeWebGL() } as *mut _)
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn make_web_gl() -> Option<gl::Interface> {
        None
    }
}
