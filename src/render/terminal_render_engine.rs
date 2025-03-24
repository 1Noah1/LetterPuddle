use std::collections::VecDeque;

use colored::{Color, Colorize};
use termion::cursor;

use crate::config::render_config::RenderConfig;
use crate::map::map::Map;
use crate::services::letter_service::LetterService;

use super::render_engine::RenderEngine;

pub struct TerminalRenderEngine {
    config: RenderConfig,
    frames: VecDeque<Map>,
}

impl RenderEngine for TerminalRenderEngine {
    #[allow(refining_impl_trait)]
    fn new(config: RenderConfig) -> TerminalRenderEngine {
        TerminalRenderEngine {
            config: config,
            frames: VecDeque::new(),
        }
    }
    fn frames_left(&self) -> bool {
        !self.frames.is_empty()
    }
    fn render(&mut self) {
        let map: Map;
        match self.frames.pop_front() {
            Some(frame) => map = frame,
            None => return,
        }
        map.vec.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                cursor::Goto(pixel.location.x as u16, pixel.location.y as u16);

                if self.config.render_letters {
                    // print  letters
                    if self.config.colored {
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
        self.frames.pop_front();
    }
    fn add_frame(&mut self, frame: Map) {
        self.frames.push_back(frame);
    }
}
