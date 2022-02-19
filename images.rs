use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Images {
    pub logo:  graphics::Image,
}

impl Images {
    pub fn new(ctx: &mut Context) -> GameResult<Images> {
        let logo = graphics::Image::new(ctx, "/logo.png")?;

        Ok(Images {
           logo,
        })
    }
}
