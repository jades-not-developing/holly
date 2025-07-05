use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorParsingError {
    #[error("Hex string should be at least 7 characters")]
    HexTooShort,

    #[error("Missing '#' hex prefix")]
    HexPrefixMissing,

    #[error("Parser failed to slice string into 3 portions")]
    ParserSlicingError    
}

pub const RED: Color = Color { 
    r: 255,
    g: 0,
    b: 0,
};

pub const GREEN: Color = Color { 
    r: 0,
    g: 255,
    b: 0,
};

pub const BLUE: Color = Color { 
    r: 0,
    g: 0,
    b: 255,
};

pub const WHITE: Color = Color { 
    r: 255,
    g: 255,
    b: 255,
};

pub const BLACK: Color = Color { 
    r: 0,
    g: 0,
    b: 0,
};

pub const GRAY: Color = Color { 
    r: 120,
    g: 120,
    b: 120,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex<T: ToString>(hex: T) -> anyhow::Result<Self> {
        // # 00 00 00

        let string = hex.to_string();

        match string {
            s if s.len() < 7 => anyhow::bail!(ColorParsingError::HexTooShort),
            s if s.chars().nth(0) != Some('#') => anyhow::bail!(ColorParsingError::HexPrefixMissing),
            s => {

                let r = &s[1..3];
                let g = &s[3..5];
                let b = &s[5..7];

                let r_parsed: u8 = u8::from_str_radix(r, 16)?;
                let g_parsed: u8 = u8::from_str_radix(g, 16)?;
                let b_parsed: u8 = u8::from_str_radix(b, 16)?;

                Ok(Self {
                    r: r_parsed,
                    g: g_parsed,
                    b: b_parsed
                })
            }
        }
    }

    pub fn into_sdl_sys(&self) -> sdl2::sys::SDL_Color {
        return sdl2::sys::SDL_Color {
            r: self.r, 
            g: self.g, 
            b: self.b, 
            a: 255, 
        }
    }
}

impl From<Color> for sdl2::pixels::Color {
    fn from(val: Color) -> Self {
        sdl2::pixels::Color::RGB(val.r, val.g, val.b)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn can_parse_hex_color() -> anyhow::Result<()> {
        assert_eq!(
            Color::from_hex("#ff0000")?,
            Color {
                r: 255,
                g: 0,
                b: 0,
            }
        );

        assert_eq!(
            Color::from_hex("#00ff00")?,
            Color {
                r: 0,
                g: 255,
                b: 0,
            }
        );

        assert_eq!(
            Color::from_hex("#0000ff")?,
            Color {
                r: 0,
                g: 0,
                b: 255,
            }
        );

        assert_eq!(
            Color::from_hex("#0a0b0c")?,
            Color {
                r: 10,
                g: 11,
                b: 12,
            }
        );

        Ok(())
    }
}
