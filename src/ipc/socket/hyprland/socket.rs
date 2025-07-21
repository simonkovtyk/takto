use std::{io::Write, os::unix::net::UnixStream};

pub enum HyprlandSocketVariant {
  Request,
  Event
}

pub fn get_hyprland_unix_socket_path (socket_variant: HyprlandSocketVariant) -> String {
  let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")
    .expect("Could not find XDG runtime dir environment variable");

  let signature = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")
    .expect("Could not get Hyprland instance signature environment variable");
  
  let socket_filename = match socket_variant {
    HyprlandSocketVariant::Request => "socket",
    HyprlandSocketVariant::Event => "socket2"
  };

  return format!("{}/hypr/{}/.{}.sock", xdg_runtime_dir, signature, socket_filename);
}

pub fn get_hyprland_unix_stream (socket_variant: HyprlandSocketVariant) -> UnixStream {
  let unix_socket_path: String = get_hyprland_unix_socket_path(socket_variant);

  return UnixStream::connect(&unix_socket_path)
    .expect("Could not connect to Hyprland socket stream");
}

pub fn send_to_hyprland_request_socket (data: &str) -> () {
  let mut socket = get_hyprland_unix_stream(HyprlandSocketVariant::Request);

  socket.write_all(data.as_bytes())
    .expect("Could not write data into Hyprland request socket stream");
  socket.flush()
    .expect("Could not flush Hyprland request socket stream");
}
