enum Mode {
    Fill, Outline
}

function draw_rect(mode: Mode, x: i32, y: i32, w: i32, h: i32) {
    todo!()
}

struct Game {
    screen_width: i32,
    screen_height: i32,

    gravity: i32,
    jump_velocity: i32,
}

struct Collision {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Collision {
    fn is_touching(obj1: Collision, obj2: Collision) {
        ! (obj1.x + obj1.width) < obj2.x ||
          obj1.x > (obj2.x + obj2.width) ||
          (obj1.y + obj1.height) < obj2.y ||
          obj1.y > (obj2.y + obj2.height)
    }

    fn draw_rect(&self) {
        draw_rect(Mode.Fill, self.x, self.y, self.width, self.height)
    }
}

enum CharacterState {
    Standing, Jumping, Dead
}

struct PlatCharacter<'a> {
    game: &'a Game,
    collision: Collision,
    vx: i32,
    direction: i32,

    vy: i32,
    mass: i32,
    state: CharacterState
}

impl PlatCharacter<'_> {
    fn move(&mut self) {
        
    }
}

