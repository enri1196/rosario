/// Mounts the client application and then registers its offline worker.
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
    register_service_worker();
}

/// Registers the service worker at the current static-site scope.
#[cfg(target_arch = "wasm32")]
fn register_service_worker() {
    use wasm_bindgen_futures::{spawn_local, JsFuture};

    let Some(window) = web_sys::window() else {
        return;
    };
    let registration = window
        .navigator()
        .service_worker()
        .register("service-worker.js");

    spawn_local(async move {
        let _ = JsFuture::from(registration).await;
    });
}

/// Leaves service-worker registration disabled for host builds and tests.
#[cfg(not(target_arch = "wasm32"))]
fn register_service_worker() {}
