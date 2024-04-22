use crate::color::ColorValue;
use crate::controller::Controller;
use percentage::{Percentage, PercentageDecimal};

#[derive(Debug)]
pub struct Light<'a> {
    controller: &'a mut Controller,
}

impl<'a> Light<'a> {
    const COLOR_SLOT: usize = 0;
    const BRIGHTNESS_SLOT: usize = 1;

    pub fn new(controller: &mut Controller) -> Light {
        Light { controller }
    }

    pub fn get_controller(&self) -> &Controller {
        self.controller
    }

    pub fn get_mut_controller(&mut self) -> &mut Controller {
        self.controller
    }

    pub fn get_color(&mut self) -> ColorValue {
        // Scale the number up. Slider has range 0..=255, but ColorValue has range 0..=360
        ColorValue(self.get_controller().get_slider(Light::COLOR_SLOT) as u16 * (360 / 255))
    }

    pub fn get_brightness(&mut self) -> PercentageDecimal {
        Percentage::from_decimal(
            self.get_controller().get_slider(Light::BRIGHTNESS_SLOT) as f64 / 100.0,
        )
    }

    pub fn set_color(&mut self, color: ColorValue) {
        // Scale number down for the same reason
        self.get_mut_controller().set_slider(
            Light::COLOR_SLOT,
            (Into::<u16>::into(color) * (255 / 360)) as u8,
        );
    }

    pub fn set_brightness(&mut self, brightness: PercentageDecimal) {
        // Scale number up for the same reason
        self.get_mut_controller()
            .set_slider(Light::BRIGHTNESS_SLOT, brightness.apply_to(256.0) as u8);
    }

    pub fn turn_on(&mut self, flip: bool) {
        if flip != self.get_controller().get_button() {
            self.get_mut_controller().toggle();
        };
    }
}
