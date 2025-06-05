use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow, Window};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

pub fn init() -> () {
  let window = Window::builder()
    .name("boot-overlay__window")
    .build();

  window.init_layer_shell();
  window.set_margin(Edge::Top, 8);
  window.set_margin(Edge::Left, 8);
  window.set_margin(Edge::Right, 8);
  window.set_margin(Edge::Bottom, 8);

  window.set_layer(Layer::Overlay);

  window.set_anchor(Edge::Left, true);
  window.set_anchor(Edge::Top, true);
  window.set_anchor(Edge::Right, true);
  window.set_anchor(Edge::Bottom, true);

  window.connect_close_request(move |_| {

  });
  
  window.present();
}

fn create_entry () -> () {

}
