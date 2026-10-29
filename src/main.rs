use crate::rgb_color::color::RGBColor;

mod rgb_color;

fn main() {
    println!("open source rgb software");
    match rgb_color::color::RGBColor::from_hex_str("#FFFFF") {
        Ok(color) => {
            println!("Successfully converted rgb color");

            println!("red: {}", color.red());
            println!("green: {}", color.green());
            println!("blue: {}", color.blue());

            println!("Back to HEX: {}", RGBColor::new(color.red(), color.green(), color.blue()).to_hex_str());
        }
        Err(_err) => {
            println!("Failed to parse color")
        }
    }
}
