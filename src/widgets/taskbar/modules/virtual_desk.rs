use gio::{glib::Priority, prelude::InputStreamExtManual, UnixInputStream};
use gtk4::prelude::{BoxExt, ButtonExt, WidgetExt};

use gdk4::glib;

use crate::ipc::{socket::hyprland::{event::{HyprlandEvent, HyprlandEventName}, socket::{get_hyprland_unix_stream, send_to_hyprland_request_socket, HyprlandSocketVariant}}};

const VDESK_COUNT: u8 = 2;

struct WorkspaceState {
  pub id: u8,
  pub name: String,
  pub focused: bool,
  pub populated: bool,
  pub workspaces: Vec<u8>,
  pub windows: u16
}

pub fn create_virtual_desk () -> gtk4::Box {
  let container = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .name("taskbar-vdesk")
    .build();

  let container_clone = container.clone();
  glib::spawn_future_local(async move {
    listen_to_change(&container_clone).await;
  });

  send_to_hyprland_request_socket("hyprland printstate -j");

  for vdesk in 0..VDESK_COUNT {
    let virtual_desk_item = create_virtual_desk_item(vdesk + 1);

    if vdesk == 0 {
      virtual_desk_item.add_css_class("taskbar-virtual-desk-item__button--active");
    }

    container.append(
      &virtual_desk_item
    );
  }

  return container;
}

fn handle_hyprland_event (current_workspace_index: &mut u8, event: HyprlandEvent, container: &gtk4::Box) -> () {
  let new_workspace_index = event.data.expect("No workspace id given");
  let parsed_new_workspace_index = new_workspace_index.parse::<u8>().expect("Could not parse new workspace id") - 1;

  if *current_workspace_index == parsed_new_workspace_index {
    return;
  }

  *current_workspace_index = parsed_new_workspace_index;
  let mut iter_index = 0;
  let mut current_child = container.first_child().unwrap();

  loop {
    if iter_index == *current_workspace_index {
      current_child.add_css_class("taskbar-virtual-desk-item__button--active");
    } else {
      let has_class = current_child.has_css_class("taskbar-virtual-desk-item__button--active");

      if has_class {
        current_child.remove_css_class("taskbar-virtual-desk-item__button--active");
      }
    }

    if let Some(next_sibling) = current_child.next_sibling() {
      current_child = next_sibling;
      iter_index += 1;
    } else {
      break;
    }
  }
}

async fn listen_to_change (container: &gtk4::Box) -> () {
  let mut current_workspace_index: u8 = 0;
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

    multiline_value.for_each(|inner| {
      if inner.as_bytes().iter().all(|&byte| byte == 0) {
        return;
      }

      let event = HyprlandEvent::from(inner);

      match event.name {
        HyprlandEventName::VDesk => {
          handle_hyprland_event(&mut current_workspace_index, event, container);
        },
        _ => {}
      }
    });
  }
}

fn create_virtual_desk_item (index: u8) -> gtk4::Button {
  let button = gtk4::Button::builder()
    .label(index.to_string())
    .css_classes(vec!["taskbar-virtual-desk-item__button"])
    .cursor(&gdk4::Cursor::builder().name("hand2").build())
    .build();

  button.connect_clicked(move |_click| {
    let dispatch = format!("dispatch vdesk {}", index);

    send_to_hyprland_request_socket(&dispatch);
  });

  return button;
}
