use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, HtmlInputElement, Node, Text, Event, EventTarget, CustomEvent, CustomEventInit, ShadowRoot};
use wasm_bindgen::closure::Closure;

// The boring part: a basic DOM component
struct MyWebComponent {
    name_node: Text,
}

impl MyWebComponent {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let name_node = document.create_text_node("friend");
        Self { name_node }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element("p").unwrap();
        let t1 = document.create_text_node("Welcome to my web component, ");
        let t3 = document.create_text_node("!");
        el.append_child(&t1).unwrap();
        el.append_child(&self.name_node).unwrap();
        el.append_child(&t3).unwrap();

        el.unchecked_into()
    }
}

impl Default for MyWebComponent {
    fn default() -> Self {
        Self::new()
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for MyWebComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "p { color: green; }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["name"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "name" {
            self.name_node
                .set_data(&new_value.unwrap_or_else(|| "friend".to_string()));
        }
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        //log("connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        //log("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        //log("adopted");
    }
}











struct MyTextField {
    label: Text,
    input: HtmlInputElement,
}

impl MyTextField {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let label = document.create_text_node("label");
        let input = document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
        Self { label, input }
    }

    fn view(&self) -> Node {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el: web_sys::Element = document.create_element("p").unwrap();
        el.append_child(&self.label).unwrap();
        el.append_child(&self.input).unwrap();

        let _l: web_sys::Element = document.create_element("label").unwrap();
        let _tf: web_sys::Element = document.create_element("input").unwrap();

        
        fn get_shadow_root_from_input(input_element: &Element) -> Option<ShadowRoot> {
            let mut node: Option<Node> = Some(input_element.clone().into());
            while let Some(current_node) = node.take() { // Use take() to replace the current value with None
                if let Ok(shadow_root) = current_node.clone().dyn_into::<ShadowRoot>() {
                    return Some(shadow_root);
                }
                node = current_node.parent_node();
            }
            None
        }

        let closure = Closure::wrap(Box::new(move |e: Event| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) {
                log(&format!("Input event: {}", input.value()));
                
                // Emit the custom event from the web component
                let custom_event = web_sys::CustomEvent::new("change").unwrap();
                let shadow_root = get_shadow_root_from_input(&input);
                log(&format!("Shadow root is some {}", shadow_root.is_some()));
                
                shadow_root.unwrap().dispatch_event(&custom_event).unwrap();
                log("Event dispatched from shadow root");
            }
        }) as Box<dyn FnMut(_)>);

        el.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();

        closure.forget(); // Important! Otherwise, the closure will be deallocated.

        el.unchecked_into() 
    }

    
    pub fn get_shadow_root_from_input(input_element: &Element) -> Option<ShadowRoot> {
        let mut node: Option<Node> = Some(input_element.clone().into());
        while let Some(current_node) = node.take() { // Use take() to replace the current value with None
            if let Ok(shadow_root) = current_node.clone().dyn_into::<ShadowRoot>() {
                return Some(shadow_root);
            }
            node = current_node.parent_node();
        }
        None
    }
}



impl Default for MyTextField {
    fn default() -> Self {
        Self::new()
    }
}


// Implementing Drop for MyTextField to log when it's dropped
impl Drop for MyTextField {
    fn drop(&mut self) {
        log("MyTextField dropped");
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for MyTextField {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "p { color: red; }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["label"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        log("attribute changed");        
        if name == "label" {
            self.label
                .set_data(&new_value.unwrap_or_else(|| "label".to_string()));
        }
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        log("connected");// Emit change event when the input value changes
        // let input = self.input.clone();
        // input.set_oninput(Some(Box::new(move |_| {
        //     self.emit_change_event(&input);
        // })));
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        log("disconnected");
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        log("adopted");
    }
}


// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    MyWebComponent::define("ce-vanilla");
    MyTextField::define("ce-text-field");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}