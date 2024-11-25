use gpui::{
    div, prelude::FluentBuilder as _, px, relative, AbsoluteLength, AnyElement, ClickEvent, Div,
    ElementId, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    Rgba, SharedString, StatefulInteractiveElement as _, Styled, WindowContext,
};

use crate::{h_flex, Theme};

/// A Button element.
#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    style: ButtonAppearance,
    shape: ButtonShape,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().flex_shrink_0(),
            id: id.into(),
            label: None,
            children: Vec::new(),
            disabled: false,
            style: ButtonAppearance::default(),
            shape: ButtonShape::default(),
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: ButtonAppearance) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let style: ButtonAppearance = self.style;

        self.base
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .px_3()
            .py_2()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(style.text_color(cx))
            .bg(style.bg(cx))
            .border_color(style.border_color(cx))
            .rounded(self.shape.radius())
            .cursor_pointer()
            .when(!self.disabled, |button| {
                button.active(|style| style.opacity(0.8))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(on_click),
            )
            // .when(self.disabled, |this| {
            //     let disabled_style = style.disabled(cx);
            //     this.cursor_not_allowed()
            //         .bg(disabled_style.bg)
            //         .text_color(disabled_style.fg)
            //         .border_color(disabled_style.border)
            //         .shadow_none()
            // })
            .child(
                h_flex()
                    .id("label")
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .text_base()
                    .when_some(self.label, |this, label| {
                        this.child(div().flex_none().line_height(relative(1.)).child(label))
                    })
                    .children(self.children),
            )
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<Button> for AnyElement {
    fn from(button: Button) -> Self {
        button.into_any_element()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonAppearance {
    #[default]
    Primary,
    Outline,
    Subtle,
    Transparent,
}

impl ButtonAppearance {
    fn bg(&self, cx: &WindowContext) -> Rgba {
        match self {
            ButtonAppearance::Primary => todo!(),
            ButtonAppearance::Outline => todo!(),
            ButtonAppearance::Subtle => todo!(),
            ButtonAppearance::Transparent => todo!(),
        }
    }

    fn text_color(&self, cx: &WindowContext) -> Rgba {
        match self {
            ButtonAppearance::Primary => todo!(),
            ButtonAppearance::Outline => todo!(),
            ButtonAppearance::Subtle => todo!(),
            ButtonAppearance::Transparent => todo!(),
        }
    }

    fn border_color(&self, cx: &WindowContext) -> Rgba {
        match self {
            ButtonAppearance::Primary => todo!(),
            ButtonAppearance::Outline => todo!(),
            ButtonAppearance::Subtle => todo!(),
            ButtonAppearance::Transparent => todo!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonShape {
    #[default]
    Rounded,
    Circular,
    Square,
}

impl ButtonShape {
    fn radius(&self) -> impl Clone + Into<AbsoluteLength> {
        match self {
            ButtonShape::Rounded => px(8.),
            ButtonShape::Circular => px(9999.),
            ButtonShape::Square => px(0.),
        }
    }
}
