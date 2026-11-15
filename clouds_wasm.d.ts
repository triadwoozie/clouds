/* tslint:disable */
/* eslint-disable */

/**
 * Start the Clouds game engine inside the browser.
 */
export function clouds_start(username?: string | null, world_name?: string | null, render_distance?: number | null, creative?: boolean | null): Promise<void>;

/**
 * Read a file from the virtual filesystem.
 */
export function clouds_vfs_read(path: string): Uint8Array | undefined;

/**
 * Store a file in the virtual filesystem.
 * If `path == "horizon-data.bin"`, automatically loads it into `PreparedFiles`.
 */
export function clouds_vfs_write(path: string, data: Uint8Array): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __clouds_fs_free: (a: number, b: number) => void;
    readonly __clouds_fs_read: (a: number, b: number, c: number) => number;
    readonly __clouds_fs_readdir: (a: number, b: number, c: number) => number;
    readonly __clouds_fs_stat: (a: number, b: number, c: number, d: number) => number;
    readonly __clouds_fs_write: (a: number, b: number, c: number, d: number) => void;
    readonly __clouds_now_ms: () => number;
    readonly clouds_start: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly clouds_vfs_read: (a: number, b: number) => [number, number];
    readonly clouds_vfs_write: (a: number, b: number, c: number, d: number) => [number, number];
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___js_sys_eb28580b882a1eda___Array__web_sys_a03ec40506ea4b57___features__gen_ResizeObserver__ResizeObserver______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___js_sys_eb28580b882a1eda___Function_fn_wasm_bindgen_393f27d5d35dd1ba___JsValue_____wasm_bindgen_393f27d5d35dd1ba___sys__Undefined___js_sys_eb28580b882a1eda___Function_fn_wasm_bindgen_393f27d5d35dd1ba___JsValue_____wasm_bindgen_393f27d5d35dd1ba___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue__core_38742bd6726ee3e0___result__Result_____wasm_bindgen_393f27d5d35dd1ba___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___sys__JsNullable_wgpu_d0b76334d909e266___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_38742bd6726ee3e0___result__Result_____wasm_bindgen_393f27d5d35dd1ba___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___sys__JsNullable_wgpu_d0b76334d909e266___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_38742bd6726ee3e0___result__Result_____wasm_bindgen_393f27d5d35dd1ba___JsError___true__16: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___sys__JsNullable_wgpu_d0b76334d909e266___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_38742bd6726ee3e0___result__Result_____wasm_bindgen_393f27d5d35dd1ba___JsError___true__17: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__12: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__13: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__14: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__15: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wasm_bindgen_393f27d5d35dd1ba___JsValue______true__9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke___wgpu_d0b76334d909e266___backend__webgpu__webgpu_sys__gen_GpuDeviceLostInfo__GpuDeviceLostInfo______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_393f27d5d35dd1ba___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
