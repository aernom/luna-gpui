use gpui::{div, Div, ParentElement, VisualContext as _, WindowContext};
use luna::{InputEvent, TextInput};

pub fn inputs_page(cx: &mut WindowContext) -> Div {
    let input1 = cx.new_view(|cx| {
        let mut input = TextInput::new(cx);
        input.set_text("Hello, this is LUNA.", cx);
        input
    });

    cx.subscribe(&input1, |_, event, _cx| {
        match event {
            InputEvent::Change(text) => println!("Change: {}", text),
            InputEvent::PressEnter => println!("PressEnter"),
            InputEvent::Focus => println!("Focus"),
            InputEvent::Blur => println!("Blur"),
        };
    })
    .detach();

    div().child(input1)
}
