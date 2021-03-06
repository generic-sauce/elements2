#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use crate::prelude::*;

mod prelude;
mod render;
mod js;
mod backend;

fn main_loop(f: impl FnMut() + 'static, fps: u32) {
	let cb = Closure::<dyn FnMut()>::wrap(Box::new(f));
	let leaked_cb = Box::leak(Box::new(cb)); // TODO
	setInterval(leaked_cb, 1000 as f64 / fps as f64);
}


#[wasm_bindgen]
pub fn client_main() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));

	let texture_filenames: Vec<_> = texture_filenames_iter().collect();
	let texture_filenames = JsValue::from_serde(&texture_filenames).unwrap();
	js_init(texture_filenames);

	let mut runnable = Runnable::Menu;

	let input_backend = WebInputBackend;
	let graphics_backend = WebGraphicsBackend::new();
	let mut app = App::<WebBackend>::new(graphics_backend, input_backend, runnable.build_menu());
	main_loop(move || app.tick_draw(&mut runnable), 60);
}

#[cfg(not(target_arch = "wasm32"))] compile_error!("This only compiles for wasm32 target");
