use gpui::{rgb, Rgba};

struct FluentColorScheme {
    neutral: Rgba,
    neutral_hover: Rgba,
    on_neutral: Rgba,
    on_neutral_hover: Rgba,
    primary: Rgba,
    primary_hover: Rgba,
    outline: Rgba,
    outline_hover: Rgba,
}

impl FluentColorScheme {
    fn light() -> Self {
        Self {
            neutral: rgb(0xffffff),
            neutral_hover: rgb(0xf5f5f5),
            primary: rgb(0x0f6cbd),
            primary_hover: rgb(0x115ea3),
            on_neutral: rgb(0x242424),
            on_neutral_hover: rgb(0x242424),
            outline: rgb(0xd1d1d1),
            outline_hover: rgb(0xc7c7c7),
        }
    }

    fn dark() -> Self {
        Self {
            neutral: rgb(0x292929),
            neutral_hover: rgb(0x3d3d3d),
            primary: rgb(0x115ea3),
            primary_hover: rgb(0x0f6cbd),
            on_neutral: rgb(0xffffff),
            on_neutral_hover: rgb(0xffffff),
            outline: rgb(0x666666),
            outline_hover: rgb(0x757575),
        }
    }
}
