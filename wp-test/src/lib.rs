mod html_cast;
mod utils;

use html_cast::*;
use js_sys::Math;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::{Document, Element, HtmlButtonElement, NodeList};
use web_sys::{Event, HtmlElement, HtmlParagraphElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//extern "C"下に記述される関数はすべてjs関数の呼び出しだと思って良さそう
//別言語の関数だと認識されるのでコンパイル時に”本当にその関数が存在するのか”のチェックはない　ただし引数の型チェックはされる
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
    fn Number(s: &str) -> i32;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();

    let random_num = (Math::floor(Math::random() * 100.0)) as i32 + 1;
    log(&format!("random_num:{:?}", random_num));
    let mut random_num_rc = Rc::new(RefCell::new(random_num));

    let guess_submit = query_selector_to::<HtmlInputElement>(".guessSubmit").unwrap();
    let guess_field = query_selector_to::<HtmlInputElement>(".guessField").unwrap();

    let mut doc_rc = Rc::new(RefCell::new(doc));
    let mut guess_count_rc = Rc::new(RefCell::new(1));

    //event_listnerに渡すためのコールバックをクロージャで作成する
    let closure = Closure::wrap(Box::new(move |_e: Event| {
        check_guess(
            &mut doc_rc,
            &mut random_num_rc,
            &mut guess_count_rc,
            &guess_field,
        );
    }) as Box<dyn FnMut(_)>);

    //closure.as_ref()でポインタを取り出し、.unchecked_ref()でそれをjsが関数として認識できる型にキャストする
    guess_submit
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    //渡したポインタが指し示す関数がDropされないようにforget()で明示的にDrop対象から外す
    closure.forget();
    Ok(())
}

pub fn check_guess(
    doc_rc: &mut Rc<RefCell<web_sys::Document>>,
    random_num_rc: &mut Rc<RefCell<i32>>,
    guess_count_rc: &mut Rc<RefCell<i32>>,
    guess_field: &HtmlInputElement,
) {
    let random_num_ref = random_num_rc.borrow().clone();

    let low_or_hi = query_selector_to::<HtmlParagraphElement>(".lowOrHi").unwrap();
    let last_result = query_selector_to::<HtmlParagraphElement>(".lastResult").unwrap();

    let user_guess = Number(&guess_field.value());

    add_record(doc_rc, &guess_count_rc.borrow().clone(), &user_guess);

    let guess_count = guess_count_rc.borrow().clone();

    if user_guess == random_num_ref {
        last_result.set_text_content(Some("おめでとう！　正解です！"));
        last_result.style().set_css_text("Color: Green");
        low_or_hi.set_text_content(None);
        set_gameover(doc_rc, random_num_rc, guess_count_rc);
    } else if guess_count >= 10 {
        last_result.set_text_content(Some("ゲームオーバー"));
        set_gameover(doc_rc, random_num_rc, guess_count_rc);
    } else {
        last_result.set_text_content(Some("間違いです！"));
        last_result.style().set_css_text("Color: Red");

        if user_guess < random_num_ref {
            low_or_hi.set_text_content(Some("今の予想は小さすぎです！"));
        } else {
            low_or_hi.set_text_content(Some("今の予想は大きすぎです！"));
        }
    }

    *guess_count_rc.borrow_mut() = guess_count + 1;
    guess_field.set_value("");

    guess_field.focus().unwrap();
}

pub fn add_record(
    doc_rc: &mut Rc<RefCell<web_sys::Document>>,
    guess_count: &i32,
    user_guess: &i32,
) {
    let guesses = query_selector_to::<HtmlElement>(".guesses").unwrap();

    if *guess_count == 1 {
        guesses.set_text_content(Some("前回の予想: "));
    }

    guesses.set_text_content(Some(&format!(
        "{} {}",
        guesses.text_content().unwrap(),
        user_guess
    )));
}

fn set_gameover(
    doc_rc: &mut Rc<RefCell<web_sys::Document>>,
    random_num_rc: &mut Rc<RefCell<i32>>,
    guess_count_rc: &mut Rc<RefCell<i32>>,
) {
    let guess_submit = query_selector_to::<HtmlInputElement>(".guessSubmit").unwrap();
    let guess_field = query_selector_to::<HtmlInputElement>(".guessField").unwrap();

    guess_field.set_disabled(true);
    guess_submit.set_disabled(true);

    let mut reset_button = doc_rc
        .borrow()
        .create_element("button")
        .element_to::<HtmlButtonElement>();
    reset_button.set_text_content(Some("新しいゲームを始める"));
    doc_rc
        .borrow()
        .body()
        .unwrap()
        .append_child(&reset_button)
        .unwrap();
    let mut random_num_rc_clone = random_num_rc.clone();

    let mut guess_count_rc_clone = guess_count_rc.clone();
    let mut doc_rc_clone = doc_rc.clone();

    let reset_button_rc = Rc::new(RefCell::new(reset_button));
    let reset_button_rc_clone = reset_button_rc.clone();

    let box1 = Box::new(move |_e: Event| {
        reset_game(
            &mut doc_rc_clone,
            &mut guess_count_rc_clone,
            &mut random_num_rc_clone,
            &reset_button_rc_clone,
        );
    });

    let closure = Closure::wrap(box1 as Box<dyn FnMut(_)>);

    reset_button_rc
        .borrow()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
}

fn reset_game(
    doc_rc: &mut Rc<RefCell<web_sys::Document>>,
    guess_count_rc: &mut Rc<RefCell<i32>>,
    random_num_rc: &mut Rc<RefCell<i32>>,
    reset_button_rc: &Rc<RefCell<HtmlButtonElement>>,
) {
    *guess_count_rc.borrow_mut() = 1;

    let reset_paras = doc_rc
        .borrow()
        .query_selector_all(".resultParas p")
        .unwrap();
    for i in 0..reset_paras.length() {
        reset_paras.item(i).unwrap().set_text_content(Some(""));
    }

    reset_button_rc
        .borrow()
        .parent_node()
        .unwrap()
        .remove_child(&reset_button_rc.borrow())
        .unwrap();

    let guess_submit = query_selector_to::<HtmlInputElement>(".guessSubmit").unwrap();
    let guess_field = query_selector_to::<HtmlInputElement>(".guessField").unwrap();

    guess_field.set_disabled(false);
    guess_submit.set_disabled(false);
    guess_field.set_value("");
    guess_field.focus().unwrap();

    let last_result = query_selector_to::<HtmlParagraphElement>(".lastResult").unwrap();

    last_result.style().set_css_text("color: white");

    *random_num_rc.borrow_mut() = Math::floor(Math::random() * 100.0) as i32 + 1;
    log(&format!("random_num:{:?}", random_num_rc.borrow()));
}
