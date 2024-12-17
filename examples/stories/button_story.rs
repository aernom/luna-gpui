use gpui::{svg, white, Div, ParentElement, Styled};
use luna::{h_flex, v_flex, Button, ButtonAppearance, ButtonShape};

pub fn buttons_page() -> Div {
    v_flex()
        .gap_4()
        .child(
            h_flex().gap_2().children([
                Button::new(1)
                    .child("Primary")
                    .appearance(ButtonAppearance::Primary),
                Button::new(2)
                    .child("Outline")
                    .appearance(ButtonAppearance::Outline),
                Button::new(3)
                    .child("Subtle")
                    .appearance(ButtonAppearance::Subtle),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(4)
                    .child("Primary Disabled")
                    .appearance(ButtonAppearance::Primary)
                    .disabled(true),
                Button::new(5)
                    .child("Outline Disabled")
                    .appearance(ButtonAppearance::Outline)
                    .disabled(true),
                Button::new(6)
                    .child("Subtle Disabled")
                    .appearance(ButtonAppearance::Subtle)
                    .disabled(true),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(7).child("Rounded"),
                Button::new(8)
                    .child("Circular")
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Circular),
                Button::new(9)
                    .child("Square")
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Square),
            ]),
        )
        .child(
            Button::new(10)
                .child(svg().path("send.svg").w_5().h_5().text_color(white()))
                .child("Cookie"),
        )
}
