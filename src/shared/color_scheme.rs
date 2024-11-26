use gpui::{rgb, rgba, Rgba};

pub struct ColorScheme {
    neutral: Rgba,
    neutral_hover: Rgba,
    on_neutral: Rgba,
    on_neutral_variant: Rgba,
    primary: Rgba,
    primary_hover: Rgba,
    on_primary: Rgba,
    subtle: Rgba,
    subtle_hover: Rgba,
    outline: Rgba,
    outline_hover: Rgba,
}

impl ColorScheme {
    pub fn light() -> Self {
        Self {
            neutral: rgb(0xffffff),
            neutral_hover: rgb(0xf5f5f5),
            on_neutral: rgb(0x242424),
            on_neutral_variant: rgb(0x424242),
            primary: rgb(0x0f6cbd),
            primary_hover: rgb(0x115ea3),
            on_primary: rgb(0xffffff),
            subtle: rgba(0x00000000),
            subtle_hover: rgb(0xf5f5f5),
            outline: rgb(0xd1d1d1),
            outline_hover: rgb(0xc7c7c7),
        }
    }

    pub fn dark() -> Self {
        Self {
            neutral: rgb(0x292929),
            neutral_hover: rgb(0x3d3d3d),
            on_neutral: rgb(0xffffff),
            on_neutral_variant: rgb(0xd6d6d6),
            primary: rgb(0x115ea3),
            primary_hover: rgb(0x0f6cbd),
            on_primary: rgb(0xffffff),
            subtle: rgba(0x00000000),
            subtle_hover: rgb(0x383838),
            outline: rgb(0x666666),
            outline_hover: rgb(0x757575),
        }
    }

    pub fn neutral(&self) -> Rgba {
        self.neutral
    }

    pub fn neutral_hover(&self) -> Rgba {
        self.neutral_hover
    }

    pub fn on_neutral(&self) -> Rgba {
        self.on_neutral
    }

    pub fn on_neutral_variant(&self) -> Rgba {
        self.on_neutral_variant
    }

    pub fn primary(&self) -> Rgba {
        self.primary
    }

    pub fn primary_hover(&self) -> Rgba {
        self.primary_hover
    }

    pub fn on_primary(&self) -> Rgba {
        self.on_primary
    }

    pub fn subtle(&self) -> Rgba {
        self.subtle
    }

    pub fn subtle_hover(&self) -> Rgba {
        self.subtle_hover
    }

    pub fn outline(&self) -> Rgba {
        self.outline
    }

    pub fn outline_hover(&self) -> Rgba {
        self.outline_hover
    }
}
