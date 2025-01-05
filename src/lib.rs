use winit::event_loop::EventLoop;

pub mod app;
mod structs;

#[cfg(target_arch = "wasm32")]
mod web;

#[macro_export]
macro_rules! if_wasm {
    ($wasm:expr, $not_wasm:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $wasm
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            $not_wasm
        }
    }};
}

pub fn setup_logger() {
    if_wasm!(web::setup_wasm_logger(), {
        // TODO: we're filtering out wgpu_core::device::re
        let env = env_logger::Env::default().filter_or("RUST_LOG", "info");
        env_logger::Builder::from_env(env)
            // TODO: figure this out
            .filter(Some("wgpu_core::device::resource"), log::LevelFilter::Warn)
            .init();
    })
}

pub fn run() {
    setup_logger();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = app::App::new();

    log::info!("Starting event loop!");
    event_loop.run_app(&mut app).expect("Failed to run app");
}
