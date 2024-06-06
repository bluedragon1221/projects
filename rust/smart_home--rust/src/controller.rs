#[derive(Debug)]
pub struct Controller {
    button: bool,
    sliders: [u8; 2],
}

impl Controller {
    pub fn new() -> Self {
        Self {
            button: false,
            sliders: [0, 0],
        }
    }

    pub fn toggle(&mut self) {
        self.button = !&self.button
    }

    pub fn get_button(&self) -> bool {
        self.button
    }

    pub fn set_slider(&mut self, id: usize, value: u8) {
        self.sliders[id] = value
    }

    pub fn get_slider(&self, id: usize) -> u8 {
        self.sliders[id]
    }
}
