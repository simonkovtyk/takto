use std::{io::Write, os::unix::net::UnixStream};

fn init_socket () -> UnixStream {
  let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
    .expect("XDG runtime dir not found");

  let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
    .expect("Hyprland signature not found");

  let socket_path = format!("{}/hypr/{}/.socket.sock", xdg_runtime_dir, signature);

  return UnixStream::connect(&socket_path).expect("Could not connect to hyprland socket");
}

pub fn send_request (data: &str) -> () {
  let mut socket = init_socket();

  socket.write_all(data.as_bytes()).expect("Could not write data into hyprland socket");
  socket.flush().expect("Could not flush hyprland socket");
}
