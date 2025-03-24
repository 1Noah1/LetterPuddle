pub mod app_manager;
pub mod config;
pub mod map;
pub mod pixel;
pub mod render;
pub mod services;

use core::time;
use std::{
    thread::{self},
    time::Instant,
};

use app_manager::AppManager;

fn main() {
    let app = AppManager::new();
    app.run();
}
