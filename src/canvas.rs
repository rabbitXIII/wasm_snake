use stdweb::traits::*;
use crate::stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};
use crate::cell::Cell;

pub struct Canvas {
  pub canvas: CanvasElement,
  pub ctx: CanvasRenderingContext2d,
  width_scaling_factor: u32,
  height_scaling_factor: u32,
  width: u32,
  height: u32,
}

impl Canvas {
  pub fn new(element_id: &str, width: u32, height: u32) -> Canvas {
    let canvas: CanvasElement = document()
      .query_selector(element_id)
      .unwrap()
      .unwrap()
      .try_into()
      .unwrap();

    let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

    let width_scaling_factor = canvas.width() / width;
    let height_scaling_factor = canvas.height() / height;

    Canvas {
      canvas,
      ctx,
      width_scaling_factor,
      height_scaling_factor,
      width,
      height,
    }
  }

  pub fn get_width(&self) -> u32 {
    return self.width;
  }

  pub fn get_height(&self) -> u32 {
    return self.height;
  }

  pub fn draw(&self, cell: Cell, color: &str) {
    assert!(cell.x < self.width);
    assert!(cell.y < self.height);

    self.ctx.set_fill_style_color(color);

    let x = cell.x * self.width_scaling_factor;
    let y = cell.y * self.height_scaling_factor;

    self.ctx.fill_rect(
      f64::from(x),
      f64::from(y),
      f64::from(self.width_scaling_factor),
      f64::from(self.height_scaling_factor),
    );
  }

  pub fn clear(&self) {
    self.ctx.set_fill_style_color("white");
    self.ctx.fill_rect(
      0.0,
      0.0,
      f64::from(self.width * self.width_scaling_factor),
      f64::from(self.height * self.height_scaling_factor),
    );
  }
}