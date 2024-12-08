use gpui::{
    div, prelude::FluentBuilder, px, rgba, AbsoluteLength, AnyElement, ClickEvent, Div, ElementId,
    FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce, Rgba, SharedString,
    StatefulInteractiveElement, Styled, WindowContext,
};

use crate::Theme;

#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    appearance: ButtonAppearance,
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
            appearance: ButtonAppearance::default(),
            shape: ButtonShape::default(),
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn appearance(mut self, style: ButtonAppearance) -> Self {
        self.appearance = style;
        self
    }

    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .px(px(12.))
            .py(px(5.))
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .rounded(self.shape.radius())
            .border_1()
            .cursor_pointer()
            .when(!self.disabled, |this| {
                let colors = self.appearance.base(cx);
                this.text_color(colors.text)
                    .bg(colors.bg)
                    .border_color(colors.outline)
                    .hover(|this| {
                        let colors = self.appearance.hover(cx);
                        this.text_color(colors.text)
                            .bg(colors.bg)
                            .border_color(colors.outline)
                    })
                    .active(|style| style.opacity(0.8))
            })
            .when(self.disabled, |this| {
                let colors = self.appearance.disabled(cx);
                this.text_color(colors.text)
                    .bg(colors.bg)
                    .border_color(colors.outline)
                    .cursor_not_allowed()
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(on_click),
            )
            .children(self.children)
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

pub struct ButtonStyle {
    bg: Rgba,
    text: Rgba,
    outline: Rgba,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonAppearance {
    #[default]
    Primary,
    Outline,
    Subtle,
}

impl ButtonAppearance {
    fn base(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).color_scheme();

        match self {
            ButtonAppearance::Primary => ButtonStyle {
                bg: colors.primary(),
                text: colors.on_primary(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Outline => ButtonStyle {
                bg: colors.neutral(),
                text: colors.on_neutral(),
                outline: colors.neutral_stroke(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: colors.subtle(),
                text: colors.on_neutral_variant(),
                outline: rgba(0xffffff00),
            },
        }
    }

    fn hover(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).color_scheme();

        match self {
            ButtonAppearance::Primary => ButtonStyle {
                bg: colors.primary_hover(),
                text: colors.on_primary(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Outline => ButtonStyle {
                bg: colors.neutral_hover(),
                text: colors.on_neutral(),
                outline: colors.neutral_stroke_hover(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: colors.subtle_hover(),
                text: colors.on_neutral(),
                outline: rgba(0xffffff00),
            },
        }
    }

    fn disabled(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).color_scheme();

        match self {
            ButtonAppearance::Primary => ButtonStyle {
                bg: colors.neutral_disabled(),
                text: colors.on_neutral_disabled(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Outline => ButtonStyle {
                bg: colors.neutral_disabled(),
                text: colors.on_neutral_disabled(),
                outline: colors.neutral_stroke_disabled(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: rgba(0xffffff00),
                text: colors.on_neutral_disabled(),
                outline: rgba(0xffffff00),
            },
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
            ButtonShape::Rounded => px(4.),
            ButtonShape::Circular => px(9999.),
            ButtonShape::Square => px(0.),
        }
    }
}
