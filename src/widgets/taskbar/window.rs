use gtk4_layer_shell::LayerShell;
use gtk4::prelude::*;
use crate::utils;
use crate::widgets::taskbar::boot_menu::create_boot_menu;
use crate::widgets::taskbar::clock::create_clock_label;
use crate::widgets::taskbar::os_logo::create_os_image;

pub fn init(
  app: &gtk4::Application,
  monitor: gdk4::Monitor
) {
  let window = gtk4::ApplicationWindow::builder()
    .application(app)
    .default_height(40)
    .name("taskbar__window")
    .build();

  window.init_layer_shell();
  window.set_monitor(Some(&monitor));

  let box_container = gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .name("taskbar-main__box")
    .spacing(0)
    .margin_start(12)
    .margin_end(12)
    .build();

  window.set_anchor(gtk4_layer_shell::Edge::Left, true);
  window.set_anchor(gtk4_layer_shell::Edge::Top, true);
  window.set_anchor(gtk4_layer_shell::Edge::Right, true);
  window.set_anchor(gtk4_layer_shell::Edge::Bottom, false);
  window.set_margin(gtk4_layer_shell::Edge::Top, 8);
  window.set_margin(gtk4_layer_shell::Edge::Bottom, 8);
  window.set_exclusive_zone(40);

  box_container.append(
    &create_left_side_box()
  );
  box_container.append(
    &utils::gtk::get_horizontal_box_spacer()
  );
  box_container.append(
    &create_right_side_box()
  );

  window.set_child(
    Some(&box_container)
  );

  window.present();
}

fn create_left_side_box () -> gtk4::Box {
  let main_box = create_side_box();
  let os_logo_container = create_side_item_box();

  os_logo_container.append(
    &create_os_image()
  );
  main_box.append(
    &os_logo_container
  );

  return main_box;
}

fn create_right_side_box () -> gtk4::Box {
  let main_box = create_side_box();
  let clock_container = create_side_item_box();

  clock_container.append(
    &create_clock_label()
  );

  let boot_button = create_side_item_button();

  let _ = &create_boot_menu(&boot_button);

  /*
  let notifications_button = create_side_item_button();

  let _ = &create_notification_menu(&notifications_button, notification_reciever);

  main_box.append(
    &notifications_button
  );
  */

  main_box.append(
    &clock_container
  );

  main_box.append(
    &boot_button
  );  

  return main_box;
}

fn create_side_box () -> gtk4::Box {
  return gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .css_classes(vec!("taskbar-side__box"))
    .spacing(8)
    .build();
}

fn create_side_item_box () -> gtk4::Box {
  return gtk4::Box::builder()
    .orientation(gtk4::Orientation::Horizontal)
    .css_classes(vec!("taskbar-side-item__box"))
    .spacing(0)
    .build();
}

fn create_side_item_button () -> gtk4::Button {
  return gtk4::Button::builder()
    .css_classes(vec!("taskbar-side-item__button"))
    .cursor(
      &gdk4::Cursor::builder()
        .name("hand2")
        .build()
    )
    .build();
}

