enum Color {
    _Red,
    _Green,
    _Blue,
    _Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::_Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::_Red => false,
            Color::_Green => false,
            Color::_Blue => true,
            Color::_Yellow => true,
        }
    }
}
fn print_color(color: Color) {
    match color {
        Color::_Red => println!("red"),
        Color::_Green => println!("green"),
        Color::_Blue => println!("blue"),
        Color::_Yellow => println!("yellow"),
    }
}

fn main() {
    print_color(Color::_Yellow);

    let foo = Color::_Yellow;
    dbg!(foo.is_green_parts());
    dbg!(foo.is_green());
}
