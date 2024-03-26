use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, Node, Text};use wasm_bindgen::prelude::*;
//use web_sys::{HtmlInputElement, HtmlLabelElement};

#[wasm_bindgen]
pub struct TextField {
    label: HtmlElement,
    input: HtmlElement,
}

#[wasm_bindgen]
impl TextField {
    #[wasm_bindgen(constructor)]
    pub fn new(label_text: &str) -> TextField {
        let document = web_sys::window().unwrap().document().unwrap();
        let label = document.create_element("label").unwrap().dyn_into::<HtmlElement>().unwrap();
        let input = document.create_element("input").unwrap().dyn_into::<HtmlElement>().unwrap();

        label.set_inner_text(label_text);
        input.set_type("text");

        TextField { label, input }
    }

    pub fn set_label(&mut self, text: &str) {
        self.label.set_inner_text(text);
    }

    pub fn value(&self) -> String {
        self.input.value()
    }

    pub fn set_value(&mut self, value: &str) {
        self.input.set_value(value);
    }

    pub fn bind_on_change(&self, closure: &Closure<dyn FnMut()>) {
        self.input.set_oninput(Some(closure.as_ref().unchecked_ref()));
    }
}