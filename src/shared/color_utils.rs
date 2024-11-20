use gpui::Hsla;

pub trait Colorize {
    fn opacity(&self, opacity: f32) -> Hsla;
    fn divide(&self, divisor: f32) -> Hsla;
    fn invert(&self) -> Hsla;
    fn invert_l(&self) -> Hsla;
    fn lighten(&self, amount: f32) -> Hsla;
    fn darken(&self, amount: f32) -> Hsla;
    fn apply(&self, base_color: Hsla) -> Hsla;
}

impl Colorize for Hsla {
    /// Returns a new color with the given opacity.
    ///
    /// The opacity is a value between 0.0 and 1.0, where 0.0 is fully transparent and 1.0 is fully opaque.
    fn opacity(&self, factor: f32) -> Hsla {
        Hsla {
            a: self.a * factor.clamp(0.0, 1.0),
            ..*self
        }
    }

    /// Returns a new color with each channel divided by the given divisor.
    ///
    /// The divisor in range of 0.0 .. 1.0
    fn divide(&self, divisor: f32) -> Hsla {
        Hsla {
            a: divisor,
            ..*self
        }
    }

    /// Return inverted color
    fn invert(&self) -> Hsla {
        Hsla {
            h: (self.h + 1.8) % 3.6,
            s: 1.0 - self.s,
            l: 1.0 - self.l,
            a: self.a,
        }
    }

    /// Return inverted lightness
    fn invert_l(&self) -> Hsla {
        Hsla {
            l: 1.0 - self.l,
            ..*self
        }
    }

    /// Return a new color with the lightness increased by the given factor.
    fn lighten(&self, factor: f32) -> Hsla {
        let l = self.l + (1.0 - self.l) * factor.clamp(0.0, 1.0).min(1.0);

        Hsla { l, ..*self }
    }

    /// Return a new color with the darkness increased by the given factor.
    fn darken(&self, factor: f32) -> Hsla {
        let l = self.l * (1.0 - factor.clamp(0.0, 1.0).min(1.0));

        Hsla { l, ..*self }
    }

    /// Return a new color with the same lightness and alpha but different hue and saturation.
    fn apply(&self, new_color: Hsla) -> Hsla {
        Hsla {
            h: new_color.h,
            s: new_color.s,
            l: self.l,
            a: self.a,
        }
    }
}
