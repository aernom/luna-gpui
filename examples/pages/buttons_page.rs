use gpui::{Div, ParentElement, Styled};
use luna::{h_flex, v_flex, Button, ButtonAppearance, ButtonShape};

pub fn buttons_page() -> Div {
    v_flex()
        .gap_4()
        .child(h_flex().gap_2().children([
            Button::new(1, "Primary").appearance(ButtonAppearance::Primary),
            Button::new(2, "Outline").appearance(ButtonAppearance::Outline),
            Button::new(3, "Subtle").appearance(ButtonAppearance::Subtle),
        ]))
        .child(
            h_flex().gap_2().children([
                Button::new(4, "Primary Disabled")
                    .appearance(ButtonAppearance::Primary)
                    .disabled(true),
                Button::new(5, "Outline Disabled")
                    .appearance(ButtonAppearance::Outline)
                    .disabled(true),
                Button::new(6, "Subtle Disabled")
                    .appearance(ButtonAppearance::Subtle)
                    .disabled(true),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(7, "Rounded"),
                Button::new(8, "Circular")
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Circular),
                Button::new(9, "Square")
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Square),
            ]),
        )
}
