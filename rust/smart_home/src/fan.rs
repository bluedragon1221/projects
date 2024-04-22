use percentage::{Percentage, PercentageDecimal};

use crate::controller::Controller;

#[derive(Debug)]
pub struct Fan<'a> {
    controller: &'a mut Controller,
}

impl<'a> Fan<'a> {
    const SPEED_SLOT: usize = 0;

    pub fn new(controller: &mut Controller) -> Fan {
        Fan { controller }
    }

    pub fn get_controller(&self) -> &Controller {
        self.controller
    }

    pub fn get_mut_controller(&mut self) -> &mut Controller {
        self.controller
    }

    pub fn get_speed(&self) -> PercentageDecimal {
        Percentage::from_decimal(self.get_controller().get_slider(Fan::SPEED_SLOT) as f64 / 100.0)
    }

    pub fn set_speed(&mut self, speed: PercentageDecimal) {
        self.get_mut_controller()
            .set_slider(Fan::SPEED_SLOT, speed.apply_to(256.0) as u8);
    }
}
