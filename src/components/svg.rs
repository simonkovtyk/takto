use gio::glib::subclass::types::ObjectSubclassExt;
use gtk4::glib;

mod imp {
  use std::cell::RefCell;

use gio::glib::subclass::types::ObjectSubclassExt;
use gtk4::{glib, prelude::{SnapshotExt, WidgetExt}};

  pub struct Svg {
    pub svg_handle: RefCell<Option<rsvg::SvgHandle>>
  }

  impl Default for Svg {
    fn default() -> Self {
      Self { svg_handle: RefCell::new(None) }
    }
  }

  #[glib::object_subclass]
  impl glib::subclass::types::ObjectSubclass for Svg {
    const NAME: &'static str = "Svg";
    type Type = super::Svg;
    type ParentType = gtk4::Widget;
  }

  impl glib::subclass::object::ObjectImpl for Svg {}

  impl gtk4::subclass::widget::WidgetImpl for Svg {
    fn snapshot(&self, snapshot: &gtk4::Snapshot) -> () {
      if self.svg_handle.borrow().is_none() {
        return;
      }

      let widget = self.obj();

      let widget_width = widget.width() as f32;
      let widget_height = widget.height() as f32;

      let cairo_bounds = graphene::Rect::new(0.0, 0.0, widget_width, widget_height);
      let cairo_context = snapshot.append_cairo(&cairo_bounds);

      let borrowed_svg_handle = self.svg_handle.borrow();

      let cairo_renderer = rsvg::CairoRenderer::new(&borrowed_svg_handle.as_ref().unwrap());

      cairo_renderer.render_document(&cairo_context, &cairo::Rectangle::new(0.0, 0.0, widget_width as f64, widget_height as f64)).expect("Cairo could not render the svg");
    }
  }
}

glib::wrapper! {
  pub struct Svg(ObjectSubclass<imp::Svg>) @extends gtk4::Widget; 
}

impl Svg {
  pub fn new () -> Self {
    glib::Object::new()
  }

  pub fn builder () -> SvgBuilder {
    SvgBuilder::new()
  }
}

impl From<&[u8]> for Svg {
  fn from(value: &[u8]) -> Self {
    let bytes = glib::Bytes::from(value);
    let stream = gio::MemoryInputStream::from_bytes(&bytes);
    let handle = rsvg::Loader::new()
      .read_stream(&stream, None::<&gio::File>, None::<&gio::Cancellable>)
      .expect("Input stream is invalid");

    let instance = Svg::new();
    let imp = imp::Svg::from_obj(&instance);

    *imp.svg_handle.borrow_mut() = Some(handle);

    return instance;
  }
}

struct SvgBuilder {
  bytes: Option<Vec<u8>>
}

impl SvgBuilder {
  pub fn new () -> Self {
    Self {
      bytes: None
    }
  }

  pub fn from_bytes (mut self: Self, bytes: &[u8]) -> Self {
    self.bytes = Some(bytes.to_vec());

    return self;
  }

  pub fn build (self: Self) -> Svg {
    return Svg::from(self.bytes.unwrap().as_slice());
  }
}
