mod light;
use percentage::Percentage;

use crate::light::Light;

mod controller;
use crate::controller::Controller;

mod color;
use crate::color::ColorValue;

mod fan;
use fan::Fan;

fn main() {
    let mut light_switch = Controller::new();
    let mut light = Light::new(&mut light_switch);
    light.turn_on(true); // turn on the light
    light.set_color(ColorValue::BLUE); // turn the light blue
    light.set_brightness(Percentage::from_decimal(0.5)); // set to half brightness
    println!("{:#?}", light);

    let mut fan_remote = Controller::new();
    let mut fan = Fan::new(&mut fan_remote);
    fan.set_speed(Percentage::from_decimal(1.0)); // set fan to full speed
}
