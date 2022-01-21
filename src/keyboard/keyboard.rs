use crate::PaudleMsg;

use super::key::{Key, KeyType, BACKSPACE, ENTER};
use super::keyboard_status::KeyboardStatus;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Keyboard;

#[derive(Properties, PartialEq)]
pub struct KeyboardProperties {
    pub keys: KeyboardStatus,
    pub key_press: Callback<PaudleMsg>,
}

impl Component for Keyboard {
    type Message = ();

    type Properties = KeyboardProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let key = |c: char| {
            html! { <Key def={ctx.props().keys.get_status(c)} /> }
        };

        let row_one = ['Й', 'Ц', 'У', 'К', 'Е', 'Н', 'Г', 'Ш', 'Щ', 'З', 'Х', 'Ъ']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();
        let row_two = ['Ф', 'Ы', 'В', 'А', 'П', 'Р', 'О', 'Л', 'Д', 'Ж', 'Э']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();
        let row_three = ['Я', 'Ч', 'С', 'М', 'И', 'Т', 'Ь', 'Б', 'Ю']
            .into_iter()
            .map(key)
            .collect::<Vec<_>>();

        let key_press = ctx.props().key_press.clone();
        let click = ctx.link().batch_callback(move |e: MouseEvent| {
            if let Some(t) = e.target() {
                if let Ok(div) = t.dyn_into::<HtmlElement>() {
                    if let Some(key) = div.get_attribute("data-key-id") {
                        if key.chars().count() == 1 {
                            if let Some(c) = key.chars().next() {
                                key_press.emit(PaudleMsg::TypeLetter(c));
                            }
                        }
                        if key == ENTER {
                            key_press.emit(PaudleMsg::Submit);
                        }
                        if key == BACKSPACE {
                            key_press.emit(PaudleMsg::Backspace);
                        }
                    }
                }
            }

            None
        });

        html! {
        <div class="wrapper">
          <div onclick={click} class="keyboard">
            <div class="keyboard-row">
              {row_one}
            </div>
            <div class="keyboard-row">
              {row_two}
            </div>
            <div class="keyboard-row">
              <Key def={KeyType::Enter} />
              {row_three}
              <Key def={KeyType::Backspace} />
            </div>
          </div>
        </div>
            }
    }
}
