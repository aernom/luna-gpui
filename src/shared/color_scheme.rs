use gpui::{rgba, Rgba};

pub struct ColorScheme {
    primary: Rgba,
    on_primary: Rgba,
    primary_container: Rgba,
    on_primary_container: Rgba,
    secondary: Rgba,
    on_secondary: Rgba,
    secondary_container: Rgba,
    on_secondary_container: Rgba,
    tertiary: Rgba,
    on_tertiary: Rgba,
    tertiary_container: Rgba,
    on_tertiary_container: Rgba,
    error: Rgba,
    on_error: Rgba,
    error_container: Rgba,
    on_error_container: Rgba,
    background: Rgba,
    on_background: Rgba,
    surface: Rgba,
    on_surface: Rgba,
    surface_variant: Rgba,
    on_surface_variant: Rgba,
    outline: Rgba,
    outline_variant: Rgba,
    scrim: Rgba,
    inverse_surface: Rgba,
    inverse_on_surface: Rgba,
    inverse_primary: Rgba,
    surface_dim: Rgba,
    surface_bright: Rgba,
    surface_container_lowest: Rgba,
    surface_container_low: Rgba,
    surface_container: Rgba,
    surface_container_high: Rgba,
    surface_container_highest: Rgba,
}

impl ColorScheme {
    pub fn dark() -> Self {
        Self {
            primary: rgba(0xA2C9FEFF),
            on_primary: rgba(0x00325BFF),
            primary_container: rgba(0x1D4875FF),
            on_primary_container: rgba(0xD3E4FFFF),
            secondary: rgba(0xBBC7DBFF),
            on_secondary: rgba(0x263141FF),
            secondary_container: rgba(0x3C4858FF),
            on_secondary_container: rgba(0xD7E3F8FF),
            tertiary: rgba(0xD8BDE3FF),
            on_tertiary: rgba(0x3C2947FF),
            tertiary_container: rgba(0x533F5FFF),
            on_tertiary_container: rgba(0xF4D9FFFF),
            error: rgba(0xFFB4ABFF),
            on_error: rgba(0x690005FF),
            error_container: rgba(0x93000AFF),
            on_error_container: rgba(0xFFDAD6FF),
            background: rgba(0x111418FF),
            on_background: rgba(0xE1E2E8FF),
            surface: rgba(0x111418FF),
            on_surface: rgba(0xE1E2E8FF),
            surface_variant: rgba(0x43474EFF),
            on_surface_variant: rgba(0xC3C6CFFF),
            outline: rgba(0x8D9199FF),
            outline_variant: rgba(0x43474EFF),
            scrim: rgba(0x000000FF),
            inverse_surface: rgba(0xE1E2E8FF),
            inverse_on_surface: rgba(0x2E3035FF),
            inverse_primary: rgba(0x38608FFF),
            surface_dim: rgba(0x111418FF),
            surface_bright: rgba(0x37393EFF),
            surface_container_lowest: rgba(0x0B0E13FF),
            surface_container_low: rgba(0x191C20FF),
            surface_container: rgba(0x1D2024FF),
            surface_container_high: rgba(0x272A2FFF),
            surface_container_highest: rgba(0x32353AFF),
        }
    }

    pub fn light() -> Self {
        Self {
            primary: rgba(0x38608FFF),
            on_primary: rgba(0xFFFFFFFF),
            primary_container: rgba(0xD3E4FFFF),
            on_primary_container: rgba(0x001C38FF),
            secondary: rgba(0x545F70FF),
            on_secondary: rgba(0xFFFFFFFF),
            secondary_container: rgba(0xD7E3F8FF),
            on_secondary_container: rgba(0x101C2BFF),
            tertiary: rgba(0x6C5677FF),
            on_tertiary: rgba(0xFFFFFFFF),
            tertiary_container: rgba(0xF4D9FFFF),
            on_tertiary_container: rgba(0x261431FF),
            error: rgba(0xBA1A1AFF),
            on_error: rgba(0xFFFFFFFF),
            error_container: rgba(0xFFDAD6FF),
            on_error_container: rgba(0x410002FF),
            background: rgba(0xF8F9FFFF),
            on_background: rgba(0x191C20FF),
            surface: rgba(0xF8F9FFFF),
            on_surface: rgba(0x191C20FF),
            surface_variant: rgba(0xDFE2EBFF),
            on_surface_variant: rgba(0x43474EFF),
            outline: rgba(0x73777FFF),
            outline_variant: rgba(0xC3C6CFFF),
            scrim: rgba(0x000000FF),
            inverse_surface: rgba(0x2E3035FF),
            inverse_on_surface: rgba(0xEFF0F7FF),
            inverse_primary: rgba(0xA2C9FEFF),
            surface_dim: rgba(0xD8DAE0FF),
            surface_bright: rgba(0xF8F9FFFF),
            surface_container_lowest: rgba(0xFFFFFFFF),
            surface_container_low: rgba(0xF2F3FAFF),
            surface_container: rgba(0xECEDF4FF),
            surface_container_high: rgba(0xE7E8EEFF),
            surface_container_highest: rgba(0xE1E2E8FF),
        }
    }

    pub fn primary(&self) -> Rgba {
        self.primary
    }

    pub fn on_primary(&self) -> Rgba {
        self.on_primary
    }

    pub fn primary_container(&self) -> Rgba {
        self.primary_container
    }

    pub fn on_primary_container(&self) -> Rgba {
        self.on_primary_container
    }

    pub fn secondary(&self) -> Rgba {
        self.secondary
    }

    pub fn on_secondary(&self) -> Rgba {
        self.on_secondary
    }

    pub fn secondary_container(&self) -> Rgba {
        self.secondary_container
    }

    pub fn on_secondary_container(&self) -> Rgba {
        self.on_secondary_container
    }

    pub fn tertiary(&self) -> Rgba {
        self.tertiary
    }

    pub fn on_tertiary(&self) -> Rgba {
        self.on_tertiary
    }

    pub fn tertiary_container(&self) -> Rgba {
        self.tertiary_container
    }

    pub fn on_tertiary_container(&self) -> Rgba {
        self.on_tertiary_container
    }

    pub fn error(&self) -> Rgba {
        self.error
    }

    pub fn on_error(&self) -> Rgba {
        self.on_error
    }

    pub fn error_container(&self) -> Rgba {
        self.error_container
    }

    pub fn on_error_container(&self) -> Rgba {
        self.on_error_container
    }

    pub fn background(&self) -> Rgba {
        self.background
    }

    pub fn on_background(&self) -> Rgba {
        self.on_background
    }

    pub fn surface(&self) -> Rgba {
        self.surface
    }

    pub fn on_surface(&self) -> Rgba {
        self.on_surface
    }

    pub fn surface_variant(&self) -> Rgba {
        self.surface_variant
    }

    pub fn on_surface_variant(&self) -> Rgba {
        self.on_surface_variant
    }

    pub fn outline(&self) -> Rgba {
        self.outline
    }

    pub fn outline_variant(&self) -> Rgba {
        self.outline_variant
    }

    pub fn scrim(&self) -> Rgba {
        self.scrim
    }

    pub fn inverse_surface(&self) -> Rgba {
        self.inverse_surface
    }

    pub fn inverse_on_surface(&self) -> Rgba {
        self.inverse_on_surface
    }

    pub fn inverse_primary(&self) -> Rgba {
        self.inverse_primary
    }

    pub fn surface_dim(&self) -> Rgba {
        self.surface_dim
    }

    pub fn surface_bright(&self) -> Rgba {
        self.surface_bright
    }

    pub fn surface_container_lowest(&self) -> Rgba {
        self.surface_container_lowest
    }

    pub fn surface_container_low(&self) -> Rgba {
        self.surface_container_low
    }

    pub fn surface_container(&self) -> Rgba {
        self.surface_container
    }

    pub fn surface_container_high(&self) -> Rgba {
        self.surface_container_high
    }

    pub fn surface_container_highest(&self) -> Rgba {
        self.surface_container_highest
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::dark()
    }
}
