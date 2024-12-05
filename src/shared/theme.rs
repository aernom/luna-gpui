use std::ops::Deref;

use gpui::{AppContext, Global};

use super::ColorScheme;

pub struct Theme {
    color_scheme: ColorScheme,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            color_scheme: ColorScheme::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            color_scheme: ColorScheme::light(),
        }
    }

    pub fn of(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::dark(),
        }
    }
}

impl Global for Theme {}

pub trait ThemeProvider {
    fn theme(&self) -> &Theme;
}

impl<T> ThemeProvider for T
where
    T: Deref<Target = gpui::AppContext>,
{
    fn theme(&self) -> &Theme {
        Theme::of(self.deref())
    }
}
