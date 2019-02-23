extern crate sparc;

use sparc::parser;

fn main() {
    assert_eq!(
        parser::hex_color("#2F14DF"),
        Ok((
            "",
            parser::Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );

    println!("Hello, world!");
}
