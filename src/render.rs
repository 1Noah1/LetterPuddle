use colored::{Color, Colorize};
use termion::cursor;

use crate::letter_service::LetterService;
use crate::map::Map;
use crate::render_config::RenderConfig;

pub struct Render;

impl Render {
    pub fn draw_map(config: &RenderConfig, map: &Map) {
        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x as u16, pixel.location.y as u16);

                if config.render_letters {
                    // print  letters
                    if config.colored {
                        match LetterService::get_color(pixel.char) {
                            Color::Blue => print!("{}", pixel.char.to_string().blue()),
                            Color::Red => print!("{}", pixel.char.to_string().red()),
                            Color::Magenta => print!("{}", pixel.char.to_string().magenta()),
                            Color::Green => print!("{}", pixel.char.to_string().green()),
                            Color::Cyan => print!("{}", pixel.char.to_string().cyan()),
                            Color::Yellow => print!("{}", pixel.char.to_string().yellow()),
                            _ => print!("{}", pixel.char.to_string().white()),
                        }
                    } else {
                        print!("{}", pixel.char.to_string().white())
                    }
                } else {
                    // print color only
                    match LetterService::get_color(pixel.char) {
                        Color::Blue => print!("{}", " ".to_string().on_blue()),
                        Color::Red => print!("{}", " ".to_string().on_red()),
                        Color::Magenta => print!("{}", " ".to_string().on_magenta()),
                        Color::Green => print!("{}", " ".to_string().on_green()),
                        Color::Cyan => print!("{}", " ".to_string().on_cyan()),
                        Color::Yellow => print!("{}", " ".to_string().on_yellow()),
                        _ => print!("{}", pixel.char),
                    }
                }

                // one symbol draw
                // let symbol = '#';
                // match LetterService::get_colors(pixel.char) {
                // Color::Blue => print!("{}", symbol.to_string().blue()),
                // Color::Red => print!("{}", symbol.to_string().red()),
                // Color::Magenta => print!("{}", symbol.to_string().magenta()),
                // Color::Green => print!("{}", symbol.to_string().green()),
                // Color::Cyan => print!("{}", symbol.to_string().cyan()),
                // Color::Yellow => print!("{}", symbol.to_string().yellow()),
                // _ => print!("{}", pixel.char)
                // }
            });
            println!();
        });
    }
}
