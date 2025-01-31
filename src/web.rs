use wasm_bindgen::prelude::*;
use web_sys;
use winit::error::EventLoopError;

// Function to convert EventLoopError to JsValue for WebAssembly context
// Because of orphan rule we can't impl From :(
pub fn _event_loop_error_to_jsvalue(value: EventLoopError) -> JsValue {
    JsValue::from_str(&format!("{:?}", value))
}

#[wasm_bindgen(start)]
pub async fn wasm_run() {
    crate::run()
}

pub fn setup_wasm_logger() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
}

pub fn setup_wasm_window(window: &winit::window::Window) {
    use winit::dpi::PhysicalSize;

    use winit::platform::web::WindowExtWebSys;
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("wasm-example")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");

    let _ = window.request_inner_size(PhysicalSize::new(900, 800));
}
