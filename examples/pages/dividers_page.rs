use gpui::{rgb, ParentElement, Styled};
use luna::{v_flex, Divider, DividerStyle};

pub struct DividersPage;

impl gpui::Render for DividersPage {
    fn render(&mut self, _: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        v_flex().w_full().gap_4().children([
            Divider::horizontal(),
            Divider::horizontal().style(DividerStyle::Subtle),
            Divider::horizontal().style(DividerStyle::Strong),
            Divider::horizontal().style(DividerStyle::Primary),
            Divider::horizontal().style(DividerStyle::Custom(rgb(0xc239b3).into())),
        ])
    }
}
