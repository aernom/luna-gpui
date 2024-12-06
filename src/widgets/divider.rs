use gpui::{
    div, prelude::FluentBuilder, px, Axis, Div, Hsla, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, WindowContext,
};

use crate::ThemeProvider;

/// A divider that can be either vertical or horizontal.
#[derive(IntoElement)]
pub struct Divider {
    base: Div,
    label: Option<SharedString>,
    axis: Axis,
    fill: Option<Hsla>,
}

impl Divider {
    pub fn vertical() -> Self {
        Self {
            base: div().h_full(),
            axis: Axis::Vertical,
            label: None,
            fill: None,
        }
    }

    pub fn horizontal() -> Self {
        Self {
            base: div().w_full(),
            axis: Axis::Horizontal,
            label: None,
            fill: None,
        }
    }

    pub fn fill(mut self, fill: impl Into<Hsla>) -> Self {
        self.fill = Some(fill.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Styled for Divider {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Divider {
    fn render(self, cx: &mut WindowContext) -> impl gpui::IntoElement {
        self.base
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .child(
                div()
                    .absolute()
                    .map(|div| match self.axis {
                        Axis::Vertical => div.w(px(1.)).h_full(),
                        Axis::Horizontal => div.h(px(1.)).w_full(),
                    })
                    .bg(self
                        .fill
                        .unwrap_or(cx.theme().color_scheme().neutral_stroke().into())),
            )
    }
}
