use crate::rgb_color::color_error::RGBColorError;

pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}
impl RGBColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
    pub fn from_hex_str(hex_str: &str) -> Result<Self, RGBColorError> {
        let hex_str = hex_str.trim_start_matches('#');

        if hex_str.len() != 6 {
            return Err(RGBColorError::InvalidHexStringLength);
        }

        let red =
            u8::from_str_radix(&hex_str[0..2], 16).map_err(|_e| RGBColorError::InvalidHexValue)?;
        let green =
            u8::from_str_radix(&hex_str[0..2], 16).map_err(|_e| RGBColorError::InvalidHexValue)?;
        let blue =
            u8::from_str_radix(&hex_str[0..2], 16).map_err(|_e| RGBColorError::InvalidHexValue)?;

        Ok(Self { red, green, blue })
    }

    pub fn to_hex_str(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}
