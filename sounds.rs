use ggez::audio;
use ggez::{Context, GameResult};

pub struct Sounds {
    pub correct: audio::Source,
    pub incorrect: audio::Source,
}

impl Sounds {
    pub fn new(ctx: &mut Context) -> GameResult<Sounds> {
        let correct = audio::Source::new(ctx, "/correct.ogg")?;
        let incorrect = audio::Source::new(ctx, "/incorrect.ogg")?;

        Ok(Sounds {
           correct, incorrect
        })
    }
}
