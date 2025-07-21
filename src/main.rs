use crate::boostrap::{app::init_app, hyprland::init_hyprland_event_listener, time::init_clock_listener};

pub mod widgets;
pub mod utils;
pub mod components;
pub mod ipc;
pub mod boostrap;

#[tokio::main]
async fn main() -> () {
  let hyprland_event_receiver = init_hyprland_event_listener();
  let clock_receiver = init_clock_listener();

  init_app();
}

