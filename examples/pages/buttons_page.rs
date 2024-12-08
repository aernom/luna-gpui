use gpui::{ParentElement, Styled};
use luna::{h_flex, v_flex, Button, ButtonAppearance, ButtonShape};

pub struct ButtonsPage;

impl gpui::Render for ButtonsPage {
    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex()
            .gap_4()
            .child(
                h_flex().gap_2().children([
                    Button::new(1)
                        .appearance(ButtonAppearance::Primary)
                        .child("Primary"),
                    Button::new(2)
                        .appearance(ButtonAppearance::Outline)
                        .child("Outline"),
                    Button::new(3)
                        .appearance(ButtonAppearance::Subtle)
                        .child("Subtle"),
                ]),
            )
            .child(
                h_flex().gap_2().children([
                    Button::new(4)
                        .appearance(ButtonAppearance::Primary)
                        .disabled(true)
                        .child("Primary Disabled"),
                    Button::new(5)
                        .appearance(ButtonAppearance::Outline)
                        .disabled(true)
                        .child("Outline Disabled"),
                    Button::new(6)
                        .appearance(ButtonAppearance::Subtle)
                        .disabled(true)
                        .child("Subtle Disabled"),
                ]),
            )
            .child(
                h_flex().gap_2().children([
                    Button::new(7)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Rounded)
                        .child("Rounded"),
                    Button::new(8)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Circular)
                        .child("Circular"),
                    Button::new(9)
                        .appearance(ButtonAppearance::Outline)
                        .shape(ButtonShape::Square)
                        .child("Square"),
                ]),
            )
    }
}
