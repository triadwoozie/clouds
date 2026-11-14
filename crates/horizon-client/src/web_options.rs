//! Runtime launch configuration for web/wasm targets.
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

pub static PLAYER_NAME: RwLock<Option<String>> = RwLock::new(None);
pub static WORLD_NAME: RwLock<Option<String>> = RwLock::new(None);
pub static RENDER_DISTANCE: RwLock<Option<u32>> = RwLock::new(None);
pub static CREATIVE: AtomicBool = AtomicBool::new(false);

pub fn set_options(name: Option<String>, world: Option<String>, dist: Option<u32>, creative: bool) {
    if let Ok(mut lock) = PLAYER_NAME.write() {
        *lock = name;
    }
    if let Ok(mut lock) = WORLD_NAME.write() {
        *lock = world;
    }
    if let Ok(mut lock) = RENDER_DISTANCE.write() {
        *lock = dist;
    }
    CREATIVE.store(creative, Ordering::Relaxed);
}

#[cfg(target_arch = "wasm32")]
pub fn is_web_pointer_locked() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.pointer_lock_element())
        .is_some()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_web_pointer_locked() -> bool {
    false
}
