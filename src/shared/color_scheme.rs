use gpui::{rgb, Rgba};

pub struct ColorScheme {
    // Primary
    primary: Rgba,
    on_primary: Rgba,
    primary_container: Rgba,
    on_primary_container: Rgba,

    // Secondary
    secondary: Rgba,
    on_secondary: Rgba,
    secondary_container: Rgba,
    on_secondary_container: Rgba,

    // Tertiary
    tertiary: Rgba,
    on_tertiary: Rgba,
    tertiary_container: Rgba,
    on_tertiary_container: Rgba,

    // Error
    error: Rgba,
    on_error: Rgba,
    error_container: Rgba,
    on_error_container: Rgba,

    // Success
    success: Rgba,
    on_success: Rgba,
    success_container: Rgba,
    on_success_container: Rgba,

    // Surfaces
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
            primary: rgb(0xa2c9fe),
            on_primary: rgb(0x00325b),
            primary_container: rgb(0x1d4875),
            on_primary_container: rgb(0xd3e4ff),
            secondary: rgb(0xbbc7db),
            on_secondary: rgb(0x263141),
            secondary_container: rgb(0x3c4858),
            on_secondary_container: rgb(0xd7e3f8),
            tertiary: rgb(0xd8bde3),
            on_tertiary: rgb(0x3c2947),
            tertiary_container: rgb(0x533f5f),
            on_tertiary_container: rgb(0xf4d9ff),
            error: rgb(0xffb4ab),
            on_error: rgb(0x690005),
            error_container: rgb(0x93000a),
            on_error_container: rgb(0xffdad6),
            success: rgb(0x346940),
            on_success: rgb(0xffffff),
            success_container: rgb(0xb6f1bc),
            on_success_container: rgb(0x00210a),
            background: rgb(0x111418),
            on_background: rgb(0xe1e2e8),
            surface: rgb(0x111418),
            on_surface: rgb(0xe1e2e8),
            surface_variant: rgb(0x43474e),
            on_surface_variant: rgb(0xc3c6cf),
            outline: rgb(0x8d9199),
            outline_variant: rgb(0x43474e),
            scrim: rgb(0x000000),
            inverse_surface: rgb(0xe1e2e8),
            inverse_on_surface: rgb(0x2e3035),
            inverse_primary: rgb(0x38608f),
            surface_dim: rgb(0x111418),
            surface_bright: rgb(0x37393e),
            surface_container_lowest: rgb(0x0b0e13),
            surface_container_low: rgb(0x191c20),
            surface_container: rgb(0x1d2024),
            surface_container_high: rgb(0x272a2f),
            surface_container_highest: rgb(0x32353a),
        }
    }

    pub fn light() -> Self {
        Self {
            primary: rgb(0x38608f),
            on_primary: rgb(0xffffff),
            primary_container: rgb(0xd3e4ff),
            on_primary_container: rgb(0x001c38),
            secondary: rgb(0x545f70),
            on_secondary: rgb(0xffffff),
            secondary_container: rgb(0xd7e3f8),
            on_secondary_container: rgb(0x101c2b),
            tertiary: rgb(0x6c5677),
            on_tertiary: rgb(0xffffff),
            tertiary_container: rgb(0xf4d9ff),
            on_tertiary_container: rgb(0x261431),
            error: rgb(0xba1a1a),
            on_error: rgb(0xffffff),
            error_container: rgb(0xffdad6),
            on_error_container: rgb(0x410002),
            success: rgb(0x9ad4a1),
            on_success: rgb(0x003916),
            success_container: rgb(0x1a512a),
            on_success_container: rgb(0xb6f1bc),
            background: rgb(0xf8f9ff),
            on_background: rgb(0x191c20),
            surface: rgb(0xf8f9ff),
            on_surface: rgb(0x191c20),
            surface_variant: rgb(0xdfe2eb),
            on_surface_variant: rgb(0x43474e),
            outline: rgb(0x73777f),
            outline_variant: rgb(0xc3c6cf),
            scrim: rgb(0x000000),
            inverse_surface: rgb(0x2e3035),
            inverse_on_surface: rgb(0xeff0f7),
            inverse_primary: rgb(0xa2c9fe),
            surface_dim: rgb(0xd8dae0),
            surface_bright: rgb(0xf8f9ff),
            surface_container_lowest: rgb(0xffffff),
            surface_container_low: rgb(0xf2f3fa),
            surface_container: rgb(0xecedf4),
            surface_container_high: rgb(0xe7e8ee),
            surface_container_highest: rgb(0xe1e2e8),
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

    pub fn success(&self) -> Rgba {
        self.success
    }

    pub fn on_success(&self) -> Rgba {
        self.on_success
    }

    pub fn success_container(&self) -> Rgba {
        self.success_container
    }

    pub fn on_success_container(&self) -> Rgba {
        self.on_success_container
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
