use gpui::{AppContext, Global};

use super::ColorScheme;

pub struct Theme {
    color_scheme: ColorScheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::dark(),
        }
    }
}

impl Global for Theme {}

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
