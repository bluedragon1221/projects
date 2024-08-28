use lua_marshalling::LuaMarshalling;

#[derive(LuaMarshalling)]
pub struct Karel {
    x: i32,
    y: i32,
    direction: String
}

impl Karel {
    pub fn turn_left(&mut self) {
        let a = match self.direction.as_str() {
            "left"  => "down",
            "down"  => "right",
            "right" => "up",
            "up"    => "left",
            _ => panic!("Not a direction!")
        };
        self.direction = a.to_owned();
    }

    pub fn move_direction(&mut self, dir: String) {
        match dir.as_str() {
            "right" if self.x < 5 => self.x += 1,
            "up"    if self.y > 1 => self.y -= 1,
            "down"  if self.y < 5 => self.y += 1,
            "left"  if self.x > 1 => self.x -= 1,
            _ => eprintln!("Can't move in that direction")
        };
    }
}

pub mod extern_ffi {
    use super::Karel;

    pub fn make_karel() -> Karel {
        Karel {
            x: 1,
            y: 1,
            direction: String::from("left"),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
