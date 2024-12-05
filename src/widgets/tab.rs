use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, ClickEvent, Div, ElementId, FontWeight,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, Styled, WindowContext,
};

use crate::Theme;

#[derive(IntoElement)]
pub struct Tab {
    pub base: Div,
    id: ElementId,
    label: SharedString,
    selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            label: label.into(),
            selected: false,
            on_click: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Tab {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let colors = Theme::of(cx).color_scheme();

        self.base
            .id(self.id)
            .px_3()
            .py(px(10.))
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .rounded_md()
            .cursor_pointer()
            .hover(|style| style.bg(colors.subtle_hover()))
            .active(|style| style.opacity(0.8))
            .when(self.selected, |tab| {
                tab.child(
                    div()
                        .absolute()
                        .h(px(3.))
                        .bottom_0()
                        .left_3()
                        .right_3()
                        .bg(colors.primary()),
                )
            })
            .when_some(self.on_click, |this, on_click| this.on_click(on_click))
            .child(self.label)
    }
}

impl Styled for Tab {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<Tab> for AnyElement {
    fn from(tab: Tab) -> Self {
        tab.into_any_element()
    }
}
