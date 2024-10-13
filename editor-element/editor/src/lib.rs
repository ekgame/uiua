use wasm_bindgen::prelude::*;

mod editor_app;
mod editor;

use editor_app::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    web_sys::console::log_1(&"Hello from start!".into());
}

#[wasm_bindgen(js_name = registerUiuaEditorElement)]
pub fn register_uiua_editor_element() {
    UiuaEditorApp::register();
    web_sys::console::log_1(&"Hello from register_uiua_editor_element!".into());
}