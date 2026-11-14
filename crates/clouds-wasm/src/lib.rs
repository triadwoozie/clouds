//! WebAssembly entry point for Clouds (WebGPU Minecraft Java Edition client in browser).

use wasm_bindgen::prelude::*;
use std::sync::Arc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

/// High-resolution monotonic timer provider used by standard library Instant and SystemTime.
#[unsafe(no_mangle)]
pub extern "C" fn __clouds_now_ms() -> f64 {
    if let Some(win) = web_sys::window() {
        if let Some(perf) = win.performance() {
            return perf.now();
        }
    }
    js_sys::Date::now()
}

/// Store a file in the virtual filesystem.
/// If `path == "horizon-data.bin"`, automatically loads it into `PreparedFiles`.
#[wasm_bindgen]
pub fn clouds_vfs_write(path: String, data: Vec<u8>) -> Result<(), JsValue> {
    if path == "horizon-data.bin" || path.ends_with("/horizon-data.bin") {
        let bytes: Arc<[u8]> = Arc::from(data.clone());
        match horizon_core::prepared_files::PreparedFiles::from_bytes(bytes) {
            Ok(Some(files)) => {
                horizon_core::prepared_files::set_global_prepared(Arc::new(files));
                log("[Clouds] Loaded horizon-data.bin container into memory.");
            }
            Ok(None) => {
                error("[Clouds] horizon-data.bin header or version invalid.");
            }
            Err(e) => {
                error(&format!("[Clouds] Failed to parse horizon-data.bin: {e}"));
            }
        }
    }
    horizon_core::prepared_files::vfs_write(&path, data);
    Ok(())
}

/// Read a file from the virtual filesystem.
#[wasm_bindgen]
pub fn clouds_vfs_read(path: String) -> Option<Vec<u8>> {
    horizon_core::prepared_files::vfs_read(std::path::Path::new(&path))
}

/// Start the Clouds game engine inside the browser.
#[wasm_bindgen]
pub fn clouds_start(
    username: Option<String>,
    world_name: Option<String>,
    render_distance: Option<u32>,
    creative: Option<bool>,
) -> Result<(), JsValue> {
    // Install panic hook FIRST so any early errors or asserts are reported
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<Any>".to_string()
        };
        let msg = format!("[Clouds Engine Panic] {payload}{location}");
        error(&msg);
    }));

    let name = username.unwrap_or_else(|| "Clouds".to_string());
    horizon_client::web_options::set_options(
        Some(name),
        world_name,
        render_distance,
        creative.unwrap_or(false),
    );

    log("[Clouds] Initializing Clouds game client...");

    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        horizon_client::entry::main();
    }));

    if let Err(e) = res {
        let desc = if let Some(s) = e.downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = e.downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        error(&format!("[Clouds Engine Error] {desc}"));
        return Err(JsValue::from_str(&format!("Clouds engine panic: {desc}")));
    }

    Ok(())
}
