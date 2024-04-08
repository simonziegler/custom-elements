use custom_elements::{inject_style, CustomElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, HtmlElement, HtmlInputElement, Node, Text, Event, ShadowRoot};
use wasm_bindgen::closure::Closure;


struct MateriaTextField {
    parent: Option<HtmlElement>,
    label: Option<Text>,
    input: Option<HtmlInputElement>,
}

impl MateriaTextField {
    fn new() -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let parent: Option<HtmlElement> = None;
        let label: Option<Text> = Some(document.create_text_node("label"));
        let input: Option<HtmlInputElement> = Some(document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap());
        Self { parent, label, input }
    }

    fn view(&self) -> Node {
        log("view");
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element("p").unwrap();

        if let Some(label) = &self.label {
            el.append_child(label).unwrap();
        }

        if let Some(input) = &self.input {
            el.append_child(input).unwrap();
        }
        // el.append_child(&self.label).unwrap();
        // el.append_child(&self.input).unwrap();

        let _l = document.create_element("label").unwrap();
        let _tf = document.create_element("input").unwrap();
        

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
                custom_event.init_custom_event_with_can_bubble_and_cancelable_and_detail(
                    &e.type_(),
                    true,
                    true,
                    &JsValue::from(input.value()),
                );

                let shadow_root = get_shadow_root_from_input(&input);
                log(&format!("Shadow root is some {}", shadow_root.is_some()));
                
                shadow_root.unwrap().host().dispatch_event(&custom_event).unwrap();
                // Cannot do this because the closure no longer implements FnMut, and only FnOnce
                //self.parent.unwrap().dispatch_event(&custom_event).unwrap();
                
                log("Event dispatched from shadow root");

                if let Some(shadow_root) = input.get_root_node().dyn_into::<ShadowRoot>().ok() {
                    if let Some(_host) = shadow_root.host().dyn_into::<Element>().ok() {
                        let host = shadow_root.host().dyn_into::<Element>().unwrap();
                        //let custom_element = host.dyn_into::<MateriaTextField>();
                    
                        // Set the value of the custom element to the value of the input element
                        //custom_element.set_value(&input.value());


                        // Assuming the host is also an input element
                        if let Ok(host_input) = host.dyn_into::<Element>() {
                            let custom_element = host_input.dyn_into::<HtmlElement>().unwrap();

                            // Set the value of the custom element to the value of the input element
                            //custom_element.set_value(&input_element.value());
                            //custom_element.set_value(&input.value());
                            //custom_element.dispatch_event(&e).unwrap();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        el.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref()).unwrap();

        closure.forget(); // Important! Otherwise, the closure will be deallocated.

        el.unchecked_into() 
    }
}



impl Default for MateriaTextField {
    fn default() -> Self {
        log("MateriaTextField new");
        Self::new()
    }
}


// Implementing Drop for MateriaTextField to log when it's dropped
impl Drop for MateriaTextField {
    fn drop(&mut self) {
        log("MateriaTextField dropped");
    }
}

// Here's the interesting part: configuring the Custom Element
impl CustomElement for MateriaTextField {
    fn inject_children(&mut self, this: &HtmlElement) {
        inject_style(&this, "p { color: red; }");
        let node = self.view();
        this.append_child(&node).unwrap_throw();
        self.parent = Some(this.clone());
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
            if let Some(label) = &self.label {
                label.set_data(&new_value.unwrap_or_else(|| "label".to_string()));
            }

            // self.label
            //     .set_data(&new_value.unwrap_or_else(|| "label".to_string()));
        }
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        log("connected");// Emit change event when the input value changes
 
        // if let Some(input) = &self.input {
        //     let input = input.clone();

        //     input.set_oninput(Some(Box::new(move |_| {
        //         self.emit_change_event(&input);
        //     })));
        // }

        // let input = self.input.clone();
        
        // input.set_oninput(Some(Box::new(move |_| {
        //     self.emit_change_event(&input);
        // })));
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        log("disconnected");

        self.parent = None;
        self.label = None;
        self.input = None;
    }

    fn adopted_callback(&mut self, _this: &HtmlElement) {
        log("adopted");
    }
}


// wasm_bindgen entry point defines the Custom Element, then creates a few of them
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // define the Custom Element
    MateriaTextField::define("mt-text-field");

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}