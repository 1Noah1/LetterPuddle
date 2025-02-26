use std::io::stdin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    // if false, the random noise letter pattern will be used
    // if true, letters will just be A,B,C,D,E.....
    pub iterative_letters: bool,
    // if false, no letters will be printed (' ')
    // things are only visible if the colored is true,
    // so the background tiles are rendered with color
    pub render_letters: bool,
    // letters or tiles will receive color, or not
    pub colored: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            iterative_letters: false,
            render_letters: true,
            colored: true,
        }
    }

    pub fn user_preference() -> Config {
        let mut config = Config::new();
        let mut buf = "".to_string();

        println!("do you want to render letters or colored tiles?");
        println!("1 for letters");
        println!("2 for tiles (recommended)");
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.render_letters = true,
                    002 => config.render_letters = false,
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

        if !config.render_letters {
            config.colored = true;
            return config;
        }

        println!("should the letters have colors?");
        println!("1 for yes (recommended)");
        println!("2 for no");
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i32>() {
                Ok(num) => match num {
                    001 => config.colored = true,
                    002 => config.colored = false,
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
    use crate::config::Config;

    #[test]
    fn new() {
        assert_eq!(
            Config::new(),
            Config {
                render_letters: true,
                iterative_letters: false,
                colored: true
            }
        )
    }
}
