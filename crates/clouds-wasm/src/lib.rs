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

    #[wasm_bindgen(js_namespace = ["window", "CloudsWeb"], js_name = onLog)]
    fn clouds_on_log(level: &str, msg: &str);
    #[wasm_bindgen(js_namespace = ["window", "CloudsWeb"], js_name = onError)]
    fn clouds_on_error(msg: &str);
}

struct ConsoleLogger;
impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("[{}] {}", record.level(), record.args());
            match record.level() {
                log::Level::Error => {
                    error(&msg);
                    clouds_on_log("error", &msg);
                }
                log::Level::Warn => {
                    warn(&msg);
                    clouds_on_log("warn", &msg);
                }
                _ => {
                    log(&msg);
                    clouds_on_log("info", &msg);
                }
            }
        }
    }
    fn flush(&self) {}
}
static LOGGER: ConsoleLogger = ConsoleLogger;

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

/// Standard library filesystem provider bridging std::fs into VFS & PreparedFiles.
#[unsafe(no_mangle)]
pub extern "C" fn __clouds_fs_read(path_ptr: *const u8, path_len: usize, out_len: *mut usize) -> *mut u8 {
    let slice = unsafe { std::slice::from_raw_parts(path_ptr, path_len) };
    let path_str = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let path = std::path::Path::new(path_str);
    let clean_str = path_str.trim_start_matches('.').trim_start_matches('/');
    let clean = std::path::Path::new(clean_str);

    let bytes = horizon_core::prepared_files::read(path)
        .or_else(|_| horizon_core::prepared_files::read(clean));

    if let Ok(bytes) = bytes {
        let len = bytes.len();
        let mut b = bytes.into_boxed_slice();
        let ptr = b.as_mut_ptr();
        unsafe { *out_len = len; }
        std::mem::forget(b);
        ptr
    } else {
        std::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __clouds_fs_write(
    path_ptr: *const u8,
    path_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(path_ptr, path_len) };
    let path_str = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return,
    };
    let data_slice = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
    horizon_core::prepared_files::vfs_write(path_str, data_slice.to_vec());
}

#[unsafe(no_mangle)]
pub extern "C" fn __clouds_fs_readdir(path_ptr: *const u8, path_len: usize, out_len: *mut usize) -> *mut u8 {
    let slice = unsafe { std::slice::from_raw_parts(path_ptr, path_len) };
    let path_str = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let path = std::path::Path::new(path_str);
    let mut entries = horizon_core::prepared_files::list_dir(path);
    if entries.is_empty() {
        let clean = path_str.trim_start_matches('.').trim_start_matches('/');
        entries = horizon_core::prepared_files::list_dir(std::path::Path::new(clean));
    }
    if entries.is_empty() {
        unsafe { *out_len = 0; }
        return std::ptr::null_mut();
    }
    let mut payload = Vec::new();
    for entry in entries {
        payload.extend_from_slice(entry.as_bytes());
        payload.push(0);
    }
    let len = payload.len();
    let mut b = payload.into_boxed_slice();
    let ptr = b.as_mut_ptr();
    unsafe { *out_len = len; }
    std::mem::forget(b);
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn __clouds_fs_stat(
    path_ptr: *const u8,
    path_len: usize,
    out_size: *mut u64,
    out_is_dir: *mut bool,
) -> bool {
    let slice = unsafe { std::slice::from_raw_parts(path_ptr, path_len) };
    let path_str = match std::str::from_utf8(slice) {
        Ok(s) => s,
        Err(_) => return false,
    };
    let path = std::path::Path::new(path_str);
    let clean_str = path_str.trim_start_matches('.').trim_start_matches('/');
    let clean_path = std::path::Path::new(clean_str);

    if let Ok(bytes) = horizon_core::prepared_files::read(path) {
        unsafe {
            *out_size = bytes.len() as u64;
            *out_is_dir = false;
        }
        return true;
    }
    if let Ok(bytes) = horizon_core::prepared_files::read(clean_path) {
        unsafe {
            *out_size = bytes.len() as u64;
            *out_is_dir = false;
        }
        return true;
    }

    let entries = horizon_core::prepared_files::list_dir(path);
    if !entries.is_empty() {
        unsafe {
            *out_size = 0;
            *out_is_dir = true;
        }
        return true;
    }
    let entries_clean = horizon_core::prepared_files::list_dir(clean_path);
    if !entries_clean.is_empty() {
        unsafe {
            *out_size = 0;
            *out_is_dir = true;
        }
        return true;
    }

    if clean_str.is_empty() || path_str == "." {
        unsafe {
            *out_size = 0;
            *out_is_dir = true;
        }
        return true;
    }

    false
}

#[unsafe(no_mangle)]
pub extern "C" fn __clouds_fs_free(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len));
        }
    }
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
pub async fn clouds_start(
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
        clouds_on_error(&msg);
    }));

    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Info);

    log("[Clouds] Requesting browser WebGPU adapter and device...");
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::BROWSER_WEBGPU;
    let instance = wgpu::Instance::new(descriptor);

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
            ..Default::default()
        })
        .await
        .map_err(|e| JsValue::from_str(&format!("Browser WebGPU requestAdapter failed: {e}")))?;

    let limits = adapter.limits();
    let features = adapter.features()
        & (wgpu::Features::FLOAT32_FILTERABLE
            | wgpu::Features::SUBGROUP
            | wgpu::Features::TEXTURE_ATOMIC
            | wgpu::Features::CLEAR_TEXTURE);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Clouds WebGPU Device"),
            required_features: features,
            required_limits: limits,
            ..Default::default()
        })
        .await
        .map_err(|e| JsValue::from_str(&format!("Browser WebGPU requestDevice failed: {e}")))?;

    horizon_client_render::gfx::set_preloaded_gpu(instance, adapter, device, queue);
    log("[Clouds] WebGPU adapter and device preloaded successfully.");

    let name = username.unwrap_or_else(|| "Clouds".to_string());
    horizon_client::web_options::set_options(
        Some(name),
        None,
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
