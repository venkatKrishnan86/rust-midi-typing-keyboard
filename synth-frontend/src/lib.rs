use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use components::atoms::{black_keys::BlackKey, white_keys::create_white_keys};

mod components;

const WHITE_KEYS_CSS: &str = include_str!("UI_components/keys/white_keys.css");
const BLACK_KEYS_CSS: &str = include_str!("UI_components/keys/black_keys.css");

#[styled_component(MIDIKeyboard)]
pub fn midi_keyboard() -> Html {
    let white_keys_style = Style::new(WHITE_KEYS_CSS).unwrap();
    let black_keys_style = Style::new(BLACK_KEYS_CSS).unwrap();
    html! {
        <>
            <div class={black_keys_style}>
                <div id="corner-left" class="filler" ></div>
                <BlackKey label='W' />
                <BlackKey label='E' />
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <BlackKey label='T' />
                <BlackKey label='Y' />
                <BlackKey label='U' />
                <div class="filler"></div>
                <div id="corner-left" class="filler"></div>
                <div id="corner-right" class="filler"></div>
            </div>
            <div class={white_keys_style}>
                {create_white_keys()}
            </div>
        </>
    }
}

