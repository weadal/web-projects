use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub trait CastHtmlElements {
    fn element_to<T: JsCast>(&self) -> T;
}
impl CastHtmlElements for Result<Option<Element>, JsValue> {
    fn element_to<T: JsCast>(&self) -> T {
        self.clone().unwrap().unwrap().dyn_into::<T>().unwrap()
    }
}

impl CastHtmlElements for Result<Element, JsValue> {
    fn element_to<T: JsCast>(&self) -> T {
        self.clone().unwrap().dyn_into::<T>().unwrap()
    }
}

pub fn query_selector_to<T: JsCast>(selector: &str) -> Option<T> {
    let w = web_sys::window()?;
    let doc = w.document()?;
    let element = doc.query_selector(selector).unwrap()?;
    element.dyn_into::<T>().ok()
}
