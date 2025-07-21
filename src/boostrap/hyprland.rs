use async_channel::{Receiver, Sender};
use gio::{glib::Priority, prelude::InputStreamExtManual, UnixInputStream};

use crate::ipc::socket::hyprland::{event::HyprlandEvent, socket::{get_hyprland_unix_stream, HyprlandSocketVariant}};
use gdk4::glib;

async fn start_event_listen (sender: Sender<HyprlandEvent>) -> () {
  let socket = get_hyprland_unix_stream(HyprlandSocketVariant::Event);
  socket.set_nonblocking(true).expect("Could not set unix stream to non-blocking");

  let stream;
  unsafe {
    stream  = UnixInputStream::take_fd(socket);
  }

  loop {
    let buffer = vec![0u8; 1024];

    let buffer = stream.read_future(buffer, Priority::default()).await.expect("Could not read unix stream");
    let value;

    unsafe {
      value = String::from_utf8_unchecked(buffer.0);
    }

    let multiline_value = value.split("\n");

    for inner in multiline_value {
      if inner.as_bytes().iter().all(|&byte| byte == 0) {
        return;
      }

      let event = HyprlandEvent::from(inner);

      sender.send(event).await
        .expect("Could not send Hyprland event");
    }
  }
}

pub fn init_hyprland_event_listener () -> Receiver<HyprlandEvent> {
  let (sender, reciever) = async_channel::bounded::<HyprlandEvent>(1);

  glib::spawn_future_local(async move {
    start_event_listen(sender).await;
  });

  return reciever;
}
