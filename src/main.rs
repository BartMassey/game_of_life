#![feature(nll)]

mod cellgrid;
use cellgrid::BoundsError;
use std::io::{Error, ErrorKind, Result};

impl From<BoundsError> for Error {
    fn from(be: BoundsError) -> Self {
        Error::new(
            ErrorKind::Other,
            format!("Bounds error occured: {:?}", be.kind),
        )
    }
}

mod frame_counter;
mod screen;
mod universe;
use frame_counter::FrameCounter;
use screen::Screen;
use universe::Universe;

fn main() -> Result<()> {
    let mut universe = {
        let (width, height) = Screen::get_size();
        Universe::new(width, height - 1)?
    };
    let mut screen = Screen::new();
    let mut frame_counter = FrameCounter::new();
    loop {
        universe.tick(|current| {
            screen.update(|buff| {
                buff.push_str(frame_counter.as_string().as_str());
                buff.push('\n');
                for (_, _, alive) in current.iter() {
                    buff.push(if alive { '®' } else { ' ' });
                }
            });
        })?;
        frame_counter.step();
    }
}
