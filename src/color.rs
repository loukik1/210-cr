#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        let c = Color { r, g, b };
        c
    }

    pub fn new_red() -> Color {
        let r = Color::new(255, 0, 0);
        r
    }

    pub fn new_green() -> Color {
        let g = Color::new(0, 255, 0);
        g
    }

    pub fn new_blue() -> Color {
        let b = Color::new(0, 0, 255);
        b
    }

    /**
     * Returns a new `Color` whose components are the sum of `c1` and `c2`'s components, modulo 256.
     *
     * First, try writing this function the "obvious" way with arithmetic operations. The test for
     * this method (which you can run with `cargo test part1_color` will fail) with a panic.
     *
     * Note which line of the test is causing the panic: why not the other?
     *
     * Then, look through the documentation for `u8` and see if there is a method that will help you.
     * https://doc.rust-lang.org/std/primitive.u8.html
     */
    pub fn cross(c1: &Color, c2: &Color) -> Color {
        let new_color = Color::new(c1.r.wrapping_add(c2.r), c1.g.wrapping_add(c2.g), c1.b.wrapping_add(c2.b));
        new_color
    }
}
