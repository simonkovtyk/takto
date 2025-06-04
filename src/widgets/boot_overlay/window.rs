use gtk4::{Application, ApplicationWindow};
use gtk4_layer_shell::{Edge, LayerShell};

fn init(app: &Application) -> () {
  let window = ApplicationWindow::builder()
    .application(app)
    .name("boot-overlay__window")
    .build();

  window.init_layer_shell();

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, true);
}
