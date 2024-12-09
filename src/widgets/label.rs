use gpui::{
    div, prelude::FluentBuilder, relative, AnyElement, Div, FontWeight, Hsla, IntoElement,
    ParentElement, RenderOnce, StyleRefinement, Styled, UnderlineStyle, WindowContext,
};

use crate::ThemeProvider;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum LabelSize {
    #[default]
    Default,
    Large,
    Small,
    XSmall,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    #[default]
    TextLabel,
    /// Sets the line height to 1.
    UiLabel,
}

/// A common set of traits all labels must implement.
pub trait LabelCommon {
    /// Sets the size of the label using a [`LabelSize`].
    fn size(self, size: LabelSize) -> Self;

    /// Sets the font weight of the label.
    fn weight(self, weight: FontWeight) -> Self;

    /// Sets the line height style of the label using a [`LineHeightStyle`].
    fn line_height_style(self, line_height_style: LineHeightStyle) -> Self;

    /// Sets the color of the label using a [`Color`].
    fn color(self, color: impl Into<Hsla>) -> Self;

    /// Sets the strikethrough property of the label.
    fn strikethrough(self, strikethrough: bool) -> Self;

    /// Sets the italic property of the label.
    fn italic(self, italic: bool) -> Self;

    /// Sets the underline property of the label
    fn underline(self, underline: bool) -> Self;

    /// Sets the label to render as a single line.
    fn single_line(self) -> Self;
}

#[derive(IntoElement)]
pub struct LabelLike {
    base: Div,
    size: LabelSize,
    weight: Option<FontWeight>,
    line_height_style: LineHeightStyle,
    color: Option<Hsla>,
    strikethrough: bool,
    italic: bool,
    children: Vec<AnyElement>,
    underline: bool,
    single_line: bool,
}

impl Default for LabelLike {
    fn default() -> Self {
        Self::new()
    }
}

impl LabelLike {
    pub fn new() -> Self {
        Self {
            base: div(),
            size: LabelSize::Default,
            weight: None,
            line_height_style: LineHeightStyle::default(),
            color: None,
            strikethrough: false,
            italic: false,
            children: Vec::new(),
            underline: false,
            single_line: false,
        }
    }
}

// Style methods.
impl Styled for LabelLike {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl LabelCommon for LabelLike {
    fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.line_height_style = line_height_style;
        self
    }

    fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }
}

impl ParentElement for LabelLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for LabelLike {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        self.base
            .map(|this| match self.size {
                LabelSize::Large => this.text_lg(),
                LabelSize::Default => this.text_base(),
                LabelSize::Small => this.text_sm(),
                LabelSize::XSmall => this.text_xs(),
            })
            .when_some(self.color, |this, color| this.text_color(color))
            .when_some(self.weight, |this, weight| this.font_weight(weight))
            .when(self.line_height_style == LineHeightStyle::UiLabel, |this| {
                this.line_height(relative(1.))
            })
            .when(self.italic, |this| this.italic())
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: gpui::Pixels(1.),
                    color: None,
                    wavy: false,
                });
                this
            })
            .when(self.strikethrough, |this| this.line_through())
            .when(self.single_line, |this| this.whitespace_nowrap())
            .children(self.children)
    }
}
