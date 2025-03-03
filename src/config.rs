use crate::render_config::RenderConfig;
use std::io::stdin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    // if false, the random noise letter pattern will be used
    // if true, letters will just be A,B,C,D,E.....
    pub iterative_letters: bool,

    pub render_config: RenderConfig,
}

impl Config {
    pub fn new(iterative_letters: bool, render_letters: bool, colored: bool) -> Config {
        Config {
            iterative_letters,
            render_config: RenderConfig::new(colored, render_letters),
        }
    }
    pub fn new_std() -> Config {
        Config {
            iterative_letters: false,
            render_config: RenderConfig::new(true, true),
        }
    }

    pub fn config_from_user_preference() -> Config {
        let mut config = Config::new_std();
        let mut buf = "".to_string();

        println!("do you want to render letters or colored tiles?");
        println!("1 for letters");
        println!("2 for tiles (recommended)");
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

        println!("do you want simple or advaned letter/color pattern?");
        println!("");
        println!("1 for simple");
        println!("2 for andvanced (recommended)");
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.iterative_letters = true,
                    002 => config.iterative_letters = false,
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
    use crate::{render_config::RenderConfig, *};

    #[test]
    fn new_std() {
        assert_eq!(
            Config::new_std(),
            Config {
                iterative_letters: false,
                render_config: RenderConfig::new(true, true)
            }
        )
    }
}
