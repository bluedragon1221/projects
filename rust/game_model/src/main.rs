mod love {
    use super::Collision;

    pub type Image = ();

    pub type Quad = ();

    pub fn draw_rect(_collision: &Collision) {
        todo!()
    }

    pub fn draw_image(_size: &Quad, _image: &Image) {
        todo!()
    }

    pub fn gen_quad(_image: &Image) -> Quad {
        todo!()
    }
}

struct Collision {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

trait GameObject {
    const BASE_VX: i32;
    const BASE_VY: i32;
    fn new() -> Self;
    fn get_collision(&mut self) -> &mut Collision;
    fn move_object(&mut self, dt: f64);
    fn draw(&self);
}

struct Player {
    collision: Collision,

    image: Option<love::Image>,
    quad: Option<love::Quad>,

    vx: i32,
    vy: i32,

    is_moving: bool,
}

impl Player {
    fn load_image(&mut self, image: love::Image) {
        self.image = Some(image);
        self.quad = Some(love::gen_quad(&image));
    }
}

impl GameObject for Player {
    const BASE_VX: i32 = 400;
    const BASE_VY: i32 = 100;

    fn new() -> Player {
        Player {
            collision: Collision {
                x: 0,
                y: 0,
                w: 150,
                h: 75,
            },
            image: None,
            quad: None,

            vx: Self::BASE_VX,
            vy: Self::BASE_VY,

            is_moving: false,
        }
    }

    fn get_collision(&mut self) -> &mut Collision {
        &mut self.collision
    }

    fn move_object(&mut self, dt: f64) {
        self.collision.x += (self.vx as f64 * dt) as i32;
        self.collision.y += (self.vy as f64 * dt) as i32;
    }

    fn draw(&self) {
        love::draw_image(&self.quad.unwrap(), &self.image.unwrap())
    }
}
