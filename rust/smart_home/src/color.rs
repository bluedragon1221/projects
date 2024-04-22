#[derive(Debug)]
pub struct ColorValue(pub u16);

impl ColorValue {
    pub const YELLOW: ColorValue = ColorValue(50);
    pub const BLUE: ColorValue = ColorValue(200);
    pub const RED: ColorValue = ColorValue(0);
}

impl Into<u16> for ColorValue {
    fn into(self) -> u16 {
        self.0
    }
}
