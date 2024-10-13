use custom_element::{CustomElement, GeneratedConstructor};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, ShadowRootInit, ShadowRootMode};
use leptos::view;

use crate::editor::{Editor, EditorMode};

pub struct UiuaEditorApp;

impl CustomElement for UiuaEditorApp {
    fn connected_callback(&mut self) {}
    fn disconnected_callback(&mut self) {}
}

impl UiuaEditorApp {
    pub(crate) fn register() {
        let constructor = UiuaEditorApp::create_app_element();
        UiuaEditorApp::register_app_element(&constructor);
    }

    fn create_app_element() -> GeneratedConstructor {
        let (closure, constructor) = custom_element::create_custom_element(
            move |instance, _args| UiuaEditorApp::new(instance),
            vec![],
        );
        closure.forget();
        constructor
    }

    fn register_app_element(constructor: &GeneratedConstructor) {
        let window = web_sys::window().unwrap();
        window
            .custom_elements()
            .define(&String::from("uiua-editor"), constructor)
            .unwrap();
    }

    // is called every time this component is created fresh
    fn new(instance: JsValue) -> Self {
        let instance: HtmlElement = instance.into();
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();
        leptos::mount_to(shadow_root.unchecked_into(), || view! { 
            <Editor mode=EditorMode::Front />
        });

        UiuaEditorApp
    }
}