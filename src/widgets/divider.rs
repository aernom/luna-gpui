use gpui::{
    div, prelude::FluentBuilder, px, Axis, Div, Hsla, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, WindowContext,
};

use crate::ThemeProvider;

#[derive(IntoElement)]
pub struct Divider {
    base: Div,
    label: Option<SharedString>,
    axis: Axis,
    style: DividerStyle,
}

impl Divider {
    pub fn vertical() -> Self {
        Self {
            base: div().h_full(),
            axis: Axis::Vertical,
            label: None,
            style: DividerStyle::default(),
        }
    }

    pub fn horizontal() -> Self {
        Self {
            base: div().w_full(),
            axis: Axis::Horizontal,
            label: None,
            style: DividerStyle::default(),
        }
    }

    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = style;
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
                    .bg(self.style.fill(cx)),
            )
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum DividerStyle {
    #[default]
    Default,
    Subtle,
    Strong,
    Primary,
    Custom(Hsla),
}

impl DividerStyle {
    fn fill(&self, cx: &WindowContext) -> Hsla {
        let colors = cx.theme().color_scheme();

        match self {
            DividerStyle::Default => colors.neutral_stroke_dim().into(),
            DividerStyle::Subtle => colors.neutral_stroke_subtle().into(),
            DividerStyle::Strong => colors.neutral_stroke().into(),
            DividerStyle::Primary => colors.primary_stroke().into(),
            DividerStyle::Custom(hsla) => *hsla,
        }
    }
}
