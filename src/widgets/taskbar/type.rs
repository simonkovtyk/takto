use async_channel::Receiver;

use crate::ipc::socket::hyprland::event::HyprlandEvent;

pub struct ListenerDelegate {
  pub hyprland_event: Receiver<HyprlandEvent>,
  pub clock: Receiver<String>
}
