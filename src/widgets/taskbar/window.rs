use chrono::{Local, Timelike};
use gdk4::glib::{spawn_future, ControlFlow};
use gdk4::{Cursor, Display, Monitor};
use gtk4::glib::timeout_add_seconds_local;
use gtk4::{Align, Application, ApplicationWindow, Button, Image, Label, Orientation, Popover, Revealer, Window};
use gtk4_layer_shell::{Edge, LayerShell};
use gtk4::{prelude::*, Box };

use crate::dbus::systemd::{send_reboot, send_shutdown};
use crate::sockets::hyprland::send_request;
use crate::utils::gtk::{get_home_path, get_horizontal_box_spacer, image_from_path};

pub fn init(app: &Application, monitor: Monitor) {
  let window = ApplicationWindow::builder()
    .application(app)
    .default_height(40)
    .name("taskbar__window")
    .build();

  window.init_layer_shell();
  window.set_monitor(Some(&monitor));

  let box_container = Box::builder()
    .orientation(Orientation::Horizontal)
    .name("taskbar-main__box")
    .spacing(0)
    .margin_start(12)
    .margin_end(12)
    .build();

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, false);
  window.set_margin(Edge::Top, 8);
  window.set_margin(Edge::Bottom, 8);
  window.set_exclusive_zone(40);

  box_container.append(
    &create_left_side_box()
  );
  box_container.append(
    &get_horizontal_box_spacer()
  );
  box_container.append(
    &create_right_side_box()
  );

  window.set_child(
    Some(&box_container)
  );

  window.present();
}

fn create_left_side_box () -> Box {
  let main_box = create_side_box();
  let arch_logo_container = create_side_item_box();

  arch_logo_container.append(
    &create_arch_image()
  );
  main_box.append(
    &arch_logo_container
  );

  return main_box;
}

fn create_right_side_box () -> Box {
  let main_box = create_side_box();
  let clock_container = create_side_item_box();

  clock_container.append(
    &create_clock_label()
  );

  let boot_button = create_side_item_button();

  Some(&create_boot_menu(&boot_button));

  main_box.append(
    &clock_container
  );

  main_box.append(
    &boot_button
  );

  return main_box;
}

fn create_side_box () -> Box {
  return Box::builder()
    .orientation(Orientation::Horizontal)
    .css_classes(vec!("taskbar-side__box"))
    .spacing(8)
    .build();
}

fn create_side_item_box () -> Box {
  return Box::builder()
    .orientation(Orientation::Horizontal)
    .css_classes(vec!("taskbar-side-item__box"))
    .spacing(0)
    .build();
}

fn create_side_item_button () -> Button {
  return Button::builder()
    .css_classes(vec!("taskbar-side-item__button"))
    .cursor(&Cursor::builder().name("hand2").build())
    .build();
}

fn create_arch_image () -> Image {
  let image = image_from_path(
    &format!("{}/{}", get_home_path(), ".config/gtk-widgets/resources/arch.png"),
    40, 40
  );

  image.set_halign(Align::Start);

  return image;
}

fn create_boot_menu(parent_button: &Button) -> () {
  let popover = Popover::builder()
    .has_arrow(false)
    .autohide(true)
    .name("taskbar-boot__popover")
    .build();

  popover.set_offset(0, 4);

  let popover_content_box = Box::builder()
    .name("taskbar-boot-popover__box")
    .spacing(4)
    .orientation(Orientation::Vertical)
    .build();

  let exit_button = Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&Cursor::builder().name("hand2").build())
    .build();

  exit_button.connect_clicked(move |_| {
    send_request("dispatch exit");
  });

  let exit_label = Label::builder()
    .label("Exit to TTY")
    .halign(Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  exit_button.set_child(
    Some(&exit_label)
  );

  popover_content_box.append(
    &exit_button
  );

  let reboot_button = Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&Cursor::builder().name("hand2").build())
    .build();

  reboot_button.connect_clicked(move |_| {
    spawn_future(async move {
      send_reboot().await;
    });
  });

  let reboot_label = Label::builder()
    .label("Reboot")
    .halign(Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  reboot_button.set_child(
    Some(&reboot_label)
  );

  popover_content_box.append(
    &reboot_button
  );

  let shutdown_button = Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&Cursor::builder().name("hand2").build())
    .build();

  shutdown_button.connect_clicked(move |_| {
    spawn_future(async move {
      send_shutdown().await;
    });
  });

  let exit_label = Label::builder()
    .label("Shutdown")
    .halign(Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  shutdown_button.set_child(
    Some(&exit_label)
  );

  popover_content_box.append(
    &shutdown_button
  );

  let revealer = Revealer::builder()
    .transition_type(gtk4::RevealerTransitionType::SlideDown)
    .transition_duration(200)
    .reveal_child(false)
    .build();

  revealer.set_child(
    Some(&popover_content_box)
  );

  popover.set_child(
    Some(&revealer)
  );
  
  popover.set_parent(
    parent_button
  );
  
  let revealer_clone = revealer.clone();
  let parent_button_clone = parent_button.clone();

  popover.connect_show(move |_| {
    revealer_clone.set_reveal_child(true);
    parent_button_clone.add_css_class("taskbar-side-item__button--active");
  });

  let revealer_clone = revealer.clone();
  let parent_button_clone = parent_button.clone();

  popover.connect_hide(move |_| {
    revealer_clone.set_reveal_child(false);
    parent_button_clone.remove_css_class("taskbar-side-item__button--active");
  });

  parent_button.connect_clicked(move |_| {
    popover.popup();
    /*
    let mut window_ref_borrow = window_ref_clone.borrow_mut();

    if window_ref_borrow.is_some() {
      let unwrapped = window_ref_borrow.as_ref().unwrap();

      unwrapped.close();
      *window_ref_borrow = None;

      drop(window_ref_borrow);

      return;
    }

    *window_ref_borrow = Some(boot_overlay::window::init());
    */
  });

  let image = image_from_path(
    &format!("{}/{}", get_home_path(), ".config/gtk-widgets/icons/power.png"),
    24, 24
  );

  parent_button.set_child(
    Some(&image)
  );
}

fn create_clock_label () -> Label {
  let label = Label::builder()
    .label(&get_formatted_time())
    .name("taskbar-clock__label")
    .halign(Align::End)
    .build();

  let label_clone = label.clone();

  timeout_add_seconds_local(20, move || {
    label_clone.set_label(&get_formatted_time());

    ControlFlow::Continue
  });

  return label;
}

fn get_formatted_time() -> String {
  let now = Local::now();

  return format!("{:02}:{:02}", now.hour(), now.minute());
}
