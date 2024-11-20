use gpui::{
    div, prelude::FluentBuilder as _, relative, rgb, AnyElement, ClickEvent, Div, ElementId, Hsla,
    InteractiveElement, IntoElement, MouseButton, ParentElement, Pixels, RenderOnce, SharedString,
    StatefulInteractiveElement as _, Styled, WindowContext,
};

use crate::h_flex;

pub enum ButtonRounded {
    None,
    Small,
    Medium,
    Large,
    Size(Pixels),
}

impl From<Pixels> for ButtonRounded {
    fn from(px: Pixels) -> Self {
        ButtonRounded::Size(px)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Outline,
    Ghost,
    Link,
    Text,
}

impl ButtonStyle {
    fn is_link(&self) -> bool {
        matches!(self, Self::Link)
    }

    fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    fn no_padding(&self) -> bool {
        self.is_link() || self.is_text()
    }
}

/// A Button element.
#[derive(IntoElement)]
pub struct Button {
    pub base: Div,
    id: ElementId,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    pub(crate) selected: bool,
    style: ButtonStyle,
    rounded: ButtonRounded,
    compact: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    pub(crate) stop_propagation: bool,
    loading: bool,
}

impl From<Button> for AnyElement {
    fn from(button: Button) -> Self {
        button.into_any_element()
    }
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().flex_shrink_0(),
            id: id.into(),
            label: None,
            disabled: false,
            selected: false,
            style: ButtonStyle::Secondary,
            rounded: ButtonRounded::Medium,
            on_click: None,
            stop_propagation: true,
            loading: false,
            compact: false,
            children: Vec::new(),
        }
    }

    /// Set the border radius of the Button.
    pub fn rounded(mut self, rounded: impl Into<ButtonRounded>) -> Self {
        self.rounded = rounded.into();
        self
    }

    /// Set label to the Button, if no label is set, the button will be in Icon Button mode.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the ButtonStyle
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set true to show the loading indicator.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set the button to compact mode, then padding will be reduced.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn stop_propagation(mut self, val: bool) -> Self {
        self.stop_propagation = val;
        self
    }

    // pub fn loading_icon(mut self, icon: impl Into<Icon>) -> Self {
    //     self.loading_icon = Some(icon.into());
    //     self
    // }
}

// impl Disableable for Button {
//     fn disabled(mut self, disabled: bool) -> Self {
//         self.disabled = disabled;
//         self
//     }
// }

// impl Selectable for Button {
//     fn element_id(&self) -> &ElementId {
//         &self.id
//     }

//     fn selected(mut self, selected: bool) -> Self {
//         self.selected = selected;
//         self
//     }
// }

// impl Sizable for Button {
//     fn with_size(mut self, size: impl Into<Size>) -> Self {
//         self.size = size.into();
//         self
//     }
// }

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

impl RenderOnce for Button {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let style: ButtonStyle = self.style;
        let normal_style = style.normal(cx);

        self.base
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .overflow_hidden()
            .border_1()
            .rounded_lg()
            .text_color(normal_style.fg)
            .when(self.selected, |this| {
                let selected_style = style.selected(cx);
                this.bg(selected_style.bg)
                    .border_color(selected_style.border)
                    .text_color(selected_style.fg)
            })
            .when(!self.disabled && !self.selected, |this| {
                this.border_color(normal_style.border)
                    .bg(normal_style.bg)
                    .when(normal_style.underline, |this| this.text_decoration_1())
                    .hover(|this| {
                        let hover_style = style.hovered(cx);
                        this.bg(hover_style.bg)
                            .border_color(hover_style.border)
                            .text_color(rgb(0xff0000))
                    })
                    .active(|this| {
                        let active_style = style.active(cx);
                        this.bg(active_style.bg)
                            .border_color(active_style.border)
                            .text_color(active_style.fg)
                            .opacity(0.2)
                    })
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled && !self.loading),
                |this, on_click| {
                    let stop_propagation = self.stop_propagation;
                    this.on_mouse_down(MouseButton::Left, move |_, cx| {
                        cx.prevent_default();
                        if stop_propagation {
                            cx.stop_propagation();
                        }
                    })
                    .on_click(move |event, cx| {
                        (on_click)(event, cx);
                    })
                },
            )
            .when(self.disabled, |this| {
                let disabled_style = style.disabled(cx);
                this.cursor_not_allowed()
                    .bg(disabled_style.bg)
                    .text_color(disabled_style.fg)
                    .border_color(disabled_style.border)
                    .shadow_none()
            })
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

struct ButtonStyles {
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    underline: bool,
    shadow: bool,
}

impl ButtonStyle {
    fn bg_color(&self, _: &WindowContext) -> Hsla {
        rgb(0x005999).into()
        // match self {
        //     ButtonStyle::Primary => cx.theme().primary,
        //     ButtonStyle::Secondary => cx.theme().secondary,
        //     ButtonStyle::Danger => cx.theme().destructive,
        //     ButtonStyle::Outline | ButtonStyle::Ghost | ButtonStyle::Link | ButtonStyle::Text => {
        //         Hsla::transparent_black()
        //     }
        //     ButtonStyle::Custom(colors) => colors.color,
        // }
    }

    fn text_color(&self, _: &WindowContext) -> Hsla {
        rgb(0xc7e8ff).into()
        // match self {
        //     ButtonStyle::Primary => cx.theme().primary_foreground,
        //     ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
        //         cx.theme().secondary_foreground
        //     }
        //     ButtonStyle::Danger => cx.theme().destructive_foreground,
        //     ButtonStyle::Link => cx.theme().link,
        //     ButtonStyle::Text => cx.theme().foreground,
        //     ButtonStyle::Custom(colors) => colors.foreground,
        // }
    }

    fn border_color(&self, _: &WindowContext) -> Hsla {
        rgb(0x005999).into()
        // match self {
        //     ButtonStyle::Primary => cx.theme().primary,
        //     ButtonStyle::Secondary => cx.theme().border,
        //     ButtonStyle::Danger => cx.theme().destructive,
        //     ButtonStyle::Outline => cx.theme().border,
        //     ButtonStyle::Ghost | ButtonStyle::Link | ButtonStyle::Text => Hsla::transparent_black(),
        //     ButtonStyle::Custom(colors) => colors.border,
        // }
    }

    fn underline(&self, _: &WindowContext) -> bool {
        match self {
            ButtonStyle::Link => true,
            _ => false,
        }
    }

    fn shadow(&self, _: &WindowContext) -> bool {
        match self {
            ButtonStyle::Primary | ButtonStyle::Secondary | ButtonStyle::Danger => true,
            _ => false,
        }
    }

    fn normal(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = self.bg_color(cx);
        let border = self.border_color(cx);
        let fg = self.text_color(cx);
        let underline = self.underline(cx);
        let shadow = self.shadow(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn hovered(&self, cx: &WindowContext) -> ButtonStyles {
        let bg: Hsla = rgb(0x0263a8).into();
        // match self {
        //     ButtonStyle::Primary => cx.theme().primary_hover,
        //     ButtonStyle::Secondary | ButtonStyle::Outline => cx.theme().secondary_hover,
        //     ButtonStyle::Danger => cx.theme().destructive_hover,
        //     ButtonStyle::Ghost => {
        //         if cx.theme().mode.is_dark() {
        //             cx.theme().secondary.lighten(0.1).opacity(0.8)
        //         } else {
        //             cx.theme().secondary.darken(0.1).opacity(0.8)
        //         }
        //     }
        //     ButtonStyle::Link => cx.theme().transparent,
        //     ButtonStyle::Text => cx.theme().transparent,
        //     ButtonStyle::Custom(colors) => colors.hover,
        // };
        let border = self.border_color(cx);
        let fg = self.text_color(cx);
        //     match self {
        //     ButtonStyle::Link => cx.theme().link_hover,
        //     _ => self.text_color(cx),
        // };
        let underline = self.underline(cx);
        let shadow = self.shadow(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn active(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = rgb(0x013c66).into();
        //     match self {
        //     ButtonStyle::Primary => cx.theme().primary_active,
        //     ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
        //         cx.theme().secondary_active
        //     }
        //     ButtonStyle::Danger => cx.theme().destructive_active,
        //     ButtonStyle::Link => cx.theme().transparent,
        //     ButtonStyle::Text => cx.theme().transparent,
        //     ButtonStyle::Custom(colors) => colors.active,
        // };
        let border = self.border_color(cx);
        let fg = self.text_color(cx);
        //     match self {
        //     ButtonStyle::Link => cx.theme().link_active,
        //     ButtonStyle::Text => cx.theme().foreground.opacity(0.7),
        //     _ => self.text_color(cx),
        // };
        let underline = self.underline(cx);
        let shadow = self.shadow(cx);

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn selected(&self, cx: &WindowContext) -> ButtonStyles {
        self.active(cx)
        // let bg = match self {
        //     ButtonStyle::Primary => cx.theme().primary_active,
        //     ButtonStyle::Secondary | ButtonStyle::Outline | ButtonStyle::Ghost => {
        //         cx.theme().secondary_active
        //     }
        //     ButtonStyle::Danger => cx.theme().destructive_active,
        //     ButtonStyle::Link => cx.theme().transparent,
        //     ButtonStyle::Text => cx.theme().transparent,
        //     ButtonStyle::Custom(colors) => colors.active,
        // };
        // let border = self.border_color(cx);
        // let fg = match self {
        //     ButtonStyle::Link => cx.theme().link_active,
        //     ButtonStyle::Text => cx.theme().foreground.opacity(0.7),
        //     _ => self.text_color(cx),
        // };
        // let underline = self.underline(cx);
        // let shadow = self.shadow(cx);

        // ButtonStyles {
        //     bg,
        //     border,
        //     fg,
        //     underline,
        //     shadow,
        // }
    }

    fn disabled(&self, cx: &WindowContext) -> ButtonStyles {
        let bg = rgb(0x656567).into();
        //     match self {
        //     ButtonStyle::Link | ButtonStyle::Ghost | ButtonStyle::Outline | ButtonStyle::Text => {
        //         cx.theme().transparent
        //     }
        //     ButtonStyle::Primary => cx.theme().primary.opacity(0.15),
        //     ButtonStyle::Danger => cx.theme().destructive.opacity(0.15),
        //     ButtonStyle::Secondary => cx.theme().secondary.opacity(1.5),
        //     ButtonStyle::Custom(style) => style.color.opacity(0.15),
        // };
        let fg = rgb(0x242424).into();
        //     match self {
        //     ButtonStyle::Link | ButtonStyle::Text | ButtonStyle::Ghost => {
        //         cx.theme().link.grayscale()
        //     }
        //     _ => cx.theme().secondary_foreground.opacity(0.5).grayscale(),
        // };

        let border = bg;
        //     match self {
        //     ButtonStyle::Outline => cx.theme().border.opacity(0.5),
        //     _ => bg,
        // };

        let underline = self.underline(cx);
        let shadow = false;

        ButtonStyles {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }
}
