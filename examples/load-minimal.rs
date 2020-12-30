extern crate gl;
extern crate khronos_egl as egl;
extern crate libloading;

use std::sync::Arc;
use egl::{
	EGL1_2,
	EGL1_4
};

fn main() {
	let minimal_egl = unsafe { Arc::new(egl::DynamicInstance::load_minimal().expect("unable to load libEGL.so")) };

	println!("EGL version is {}", minimal_egl.version());

	// Select the rendering API.
	if let Some(egl1_2) = minimal_egl.upcast::<EGL1_2>() {
		 egl1_2.bind_api(egl::OPENGL_API).expect("unable to select OpenGL API");
	}

	// Setup Open GL.
	gl::load_with(|name| minimal_egl.get_proc_address(name).unwrap() as *const std::ffi::c_void);

	match minimal_egl.upcast::<EGL1_4>() {
		Some(egl1_4) => {
			foo_with_1_4(egl1_4)
		},
		None => {
			foo_without_1_4(&minimal_egl)
		}
	}
}

fn foo_with_1_4<V: egl::api::EGL1_4>(egl: &egl::Instance<V>) {
	// do something that requires at least EGL 1.4.
}

fn foo_without_1_4<V>(egl: &egl::Instance<V>) {
	// do something without any specific EGL version (other that 1.0).
}