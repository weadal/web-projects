use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlParagraphElement};

pub trait CastHtmlElements {
    type Html;
    type Para;
    type Input;
    type Button;

    fn to_html_element(&self) -> Self::Html;
    fn to_html_paragraph_element(&self) -> Self::Para;
    fn to_html_input_element(&self) -> Self::Input;
    fn to_html_button_element(&self) -> Self::Button;
}
impl CastHtmlElements for Result<Option<Element>, JsValue> {
    type Html = HtmlElement;
    type Para = HtmlParagraphElement;
    type Input = HtmlInputElement;
    type Button = HtmlButtonElement;

    fn to_html_element(&self) -> HtmlElement {
        self.clone()
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
    }

    fn to_html_paragraph_element(&self) -> Self::Para {
        self.clone()
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlParagraphElement>()
            .unwrap()
    }
    fn to_html_input_element(&self) -> HtmlInputElement {
        self.clone()
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
    }
    fn to_html_button_element(&self) -> HtmlButtonElement {
        self.clone()
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap()
    }
}

impl CastHtmlElements for Result<Element, JsValue> {
    type Html = HtmlElement;
    type Para = HtmlParagraphElement;
    type Input = HtmlInputElement;
    type Button = HtmlButtonElement;

    fn to_html_element(&self) -> HtmlElement {
        self.clone().unwrap().dyn_into::<HtmlElement>().unwrap()
    }

    fn to_html_paragraph_element(&self) -> Self::Para {
        self.clone()
            .unwrap()
            .dyn_into::<HtmlParagraphElement>()
            .unwrap()
    }
    fn to_html_input_element(&self) -> HtmlInputElement {
        self.clone()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
    }
    fn to_html_button_element(&self) -> HtmlButtonElement {
        self.clone()
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap()
    }
}
