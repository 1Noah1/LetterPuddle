use crate::config::render_config::RenderConfig;
use std::io::stdin;

use super::animation::Animation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneralConfig {
    // if false, the random noise letter pattern will be used
    // if true, letters will just be A,B,C,D,E.....
    pub animation: Animation,

    pub render_config: RenderConfig,
}

impl GeneralConfig {
    pub fn new(animation: Animation, render_letters: bool, colored: bool) -> GeneralConfig {
        GeneralConfig {
            animation,
            render_config: RenderConfig::new(colored, render_letters),
        }
    }
    pub fn new_std() -> GeneralConfig {
        GeneralConfig {
            animation: Animation::Islands,
            render_config: RenderConfig::new(true, true),
        }
    }

    pub fn config_from_user_preference() -> GeneralConfig {
        let mut config = GeneralConfig::new_std();
        let mut buf = "".to_string();

        println!("What kind of animation do you want?");
        println!("1 Islands");
        println!("2 Rainbow");
        println!("3 Snakes");
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.animation = Animation::Islands,
                    002 => config.animation = Animation::Alphabet,
                    003 => config.animation = Animation::Snakes,
                    _ => println!(
                        "invalid input: {}, will proceed with standard option",
                        buf.as_str()
                    ),
                },
                Err(err) => println!("invalid input: {err}, will proceed with standard setting"),
            },
            Err(err) => println!("read error occured: {err}, will proceed with standard setting"),
        }
        println!();

        println!("do you want to render letters or colored tiles?");
        println!("1 for letters");
        println!("2 for tiles (recommended)");
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.render_config.render_letters = true,
                    002 => config.render_config.render_letters = false,
                    _ => println!(
                        "invalid input: {}, will proceed with standard option",
                        buf.as_str()
                    ),
                },
                Err(err) => println!("invalid input: {err}, will proceed with standard setting"),
            },
            Err(err) => println!("read error occured: {err}, will proceed with standard setting"),
        }
        println!();

        if !config.render_config.render_letters {
            config.render_config.colored = true;
            return config;
        }

        println!("should the letters have colors?");
        println!("1 for yes (recommended)");
        println!("2 for no");
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.render_config.colored = true,
                    002 => config.render_config.colored = false,
                    _ => println!(
                        "invalid input: {}, will proceed with standard option",
                        buf.as_str()
                    ),
                },
                Err(err) => {
                    println!("invalid input: {err}, will proceed with standard setting")
                }
            },
            Err(err) => println!("read error occured: {err}, will proceed with standard setting"),
        }
        println!();

        config
    }
}

mod tests {
    // it's needed for the test idk why linter calls it uknown
    #![allow(unused_imports)]

    use crate::config::{
        animation::Animation, general_config::GeneralConfig, render_config::RenderConfig,
    };

    #[test]
    fn new_std() {
        assert_eq!(
            GeneralConfig::new_std(),
            GeneralConfig {
                animation: Animation::Islands,
                render_config: RenderConfig::new(true, true)
            }
        )
    }
}
