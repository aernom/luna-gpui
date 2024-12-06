mod color_scheme;

pub use color_scheme::*;

use gpui::{AppContext, Global};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Brightness {
    Dark,
    Light,
}

pub struct Theme {
    brightness: Brightness,
    color_scheme: ColorScheme,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            brightness: Brightness::Dark,
            color_scheme: ColorScheme::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            brightness: Brightness::Light,
            color_scheme: ColorScheme::light(),
        }
    }

    pub fn of(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn brightness(&self) -> &Brightness {
        &self.brightness
    }

    pub fn color_scheme(&self) -> &ColorScheme {
        &self.color_scheme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
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
