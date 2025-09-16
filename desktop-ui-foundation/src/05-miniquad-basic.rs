
use miniquad::*;

fn main() {
    let conf = conf::Conf::default();

    start(conf, || Box::new(Stage::new()));
}

struct Stage {
    ctx: GlContext,
}

impl Stage {
    fn new() -> Self {
        Self { 
            ctx: GlContext::default() 
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        
    }

    fn draw(&mut self) {
        self.ctx.clear(Some((0.0, 1.0, 1.0, 0.0)), None, None);
    }
}
