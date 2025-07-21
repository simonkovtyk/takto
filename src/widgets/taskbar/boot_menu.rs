use gtk4::{prelude::*, glib};

use crate::{ipc::{self, socket::hyprland::socket::send_to_hyprland_request_socket}, utils};

pub fn create_boot_menu(parent_button: &gtk4::Button) -> () {
  let popover = gtk4::Popover::builder()
    .autohide(true)
    .name("taskbar-boot__popover")
    .build();

  let popover_content_box = gtk4::Box::builder()
    .name("taskbar-boot-popover__box")
    .spacing(4)
    .orientation(gtk4::Orientation::Vertical)
    .build();

  let exit_button = gtk4::Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&gdk4::Cursor::builder().name("hand2").build())
    .build();

  exit_button.connect_clicked(move |_| {
    send_to_hyprland_request_socket("dispatch exit");
  });

  let exit_label = gtk4::Label::builder()
    .label("Exit to TTY")
    .halign(gtk4::Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  exit_button.set_child(
    Some(&exit_label)
  );

  popover_content_box.append(
    &exit_button
  );

  let reboot_button = gtk4::Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&gdk4::Cursor::builder().name("hand2").build())
    .build();

  reboot_button.connect_clicked(move |_| {
    glib::spawn_future(async move {
      ipc::dbus::systemd::send_reboot().await;
    });
  });

  let reboot_label = gtk4::Label::builder()
    .label("Reboot")
    .halign(gtk4::Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  reboot_button.set_child(
    Some(&reboot_label)
  );

  popover_content_box.append(
    &reboot_button
  );

  let shutdown_button = gtk4::Button::builder()
    .hexpand(true)
    .css_classes(vec!("taskbar-boot-popover__button"))
    .cursor(&gdk4::Cursor::builder().name("hand2").build())
    .build();

  shutdown_button.connect_clicked(move |_| {
    glib::spawn_future(async move {
      ipc::dbus::systemd::send_shutdown().await;
    });
  });

  let exit_label = gtk4::Label::builder()
    .label("Shutdown")
    .halign(gtk4::Align::Start)
    .css_classes(vec!("taskbar-boot-popover__label"))
    .build();

  shutdown_button.set_child(
    Some(&exit_label)
  );

  popover_content_box.append(
    &shutdown_button
  );

  let revealer = gtk4::Revealer::builder()
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

  let image = utils::gtk::image_from_binary_data(include_bytes!("../../../assets/power.png"));

  parent_button.set_child(
    Some(&image)
  );
}
