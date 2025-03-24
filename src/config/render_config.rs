use super::platform::Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderConfig {
    // if false, no letters will be printed (' ')
    // things are only visible if the colored is true,
    // so the background tiles are rendered with color
    pub render_letters: bool,
    // letters or tiles will receive color, or not
    pub colored: bool,
    pub platform: Platform,
}

impl RenderConfig {
    pub fn new(colored: bool, render_letters: bool) -> RenderConfig {
        RenderConfig {
            colored,
            render_letters,
            platform: Platform::Terminal,
        }
    }
}
