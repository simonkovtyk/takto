use gtk::{prelude::*, Image, Widget};
use gtk::{glib, Application, ApplicationWindow};
use gtk4_layer_shell::{Edge, LayerShell};

const APP_ID: &str = "dev.simonkov.taskbar";

fn main() -> glib::ExitCode {
  let app = Application::builder().application_id(APP_ID).build();
  app.connect_activate(build_ui);
  app.run()
}

fn build_ui(app: &Application) -> () {
  let window = ApplicationWindow::builder()
    .application(app)
    .default_height(40)
    .decorated(false)
    .opacity(0.0)
    .build();


  window.init_layer_shell();

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, false);
  window.set_exclusive_zone(40);

  window.present();
}

fn add_logo(window: &mut ApplicationWindow) -> () {
  let image = Image::builder()
    .file("");

  window.child(
    
  );
}
