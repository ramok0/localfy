use std::ops::RangeInclusive;
use std::hash::Hash;
use egui::{emath, Rect, Response, Color32, Ui, Label, RichText, Vec2, FontId, Pos2, InnerResponse, Id, Sense};

use crate::constants::{SECONDARY_HOVER_COLOR, TEXT_COLOR};

pub fn place_text_at(ui:&mut egui::Ui, font:FontId, pos:Pos2, text:String, is_max:bool) {
    let size = ui.painter().layout_no_wrap(text.clone(), font.clone(), TEXT_COLOR).size();

    let mut text_min;
    let mut text_max;

    if is_max {
        text_max = pos;
        text_min = text_max - size - Vec2 {x:10., y:0.};
    } else {
        text_min = pos;
        text_max = text_min + size + Vec2 {x:10., y:0.};
    }

    text_max.y = pos.y;
    text_min.y = pos.y;

    ui.put(
        Rect { min: text_min, max: text_max },
        Label::new(RichText::new(text).font(font).color(TEXT_COLOR))
    );
}

pub fn slider<Num: emath::Numeric>(ui:&mut egui::Ui, max_rect:Rect, value: &mut Num, range:RangeInclusive<Num>, hash:impl Hash, ) -> Response {
    let id = ui.id().with(hash);
    let hovered: Option<bool> = ui.memory_mut(|mem| mem.data.get_temp(id));
   
    let inner = ui.scope(|ui| {
        if let Some(hovered) = hovered {
            if hovered {
                ui.style_mut().visuals.selection.bg_fill = SECONDARY_HOVER_COLOR;
             //   ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::WHITE;
            }
        }
    
        let response = ui.put(max_rect, egui::Slider::new(value, range)
        .show_value(false)
        .trailing_fill(true));
    

        ui.memory_mut(|mem| mem.data.insert_temp(id, response.hovered()));


        response
    });

    inner.inner
}

pub fn centerer(ui: &mut Ui, hash:impl Hash, add_contents: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
      let id = ui.id().with(hash);
      let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
      if let Some(last_width) = last_width {
        ui.add_space((ui.available_width() - last_width) / 2.0);
      }
      let res = ui
        .scope(|ui| {
          add_contents(ui);
        })
        .response;
      let width = res.rect.width();
      ui.memory_mut(|mem| mem.data.insert_temp(id, width));

      // Repaint if width changed
      match last_width {
        None => ui.ctx().request_repaint(),
        Some(last_width) if last_width != width => ui.ctx().request_repaint(),
        Some(_) => {}
      }
    });
  }

  pub fn scope_click<R>(ui:&mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> InnerResponse<R> {
    scope_dyn_click(ui, Box::new(add_contents), ui.next_auto_id())
}

fn scope_dyn_click<'c, R>(
  ui:&mut egui::Ui,
    add_contents: Box<dyn FnOnce(&mut Ui) -> R + 'c>,
    id_source: Id,
) -> InnerResponse<R> {
    let child_rect = ui.available_rect_before_wrap();
    let mut child_ui = ui.child_ui_with_id_source(child_rect, *ui.layout(), id_source);
    let ret = add_contents(&mut child_ui);
    let response = ui.allocate_rect(child_ui.min_rect(), Sense::click());
    InnerResponse::new(ret, response)
}

pub fn croix(ui:&mut egui::Ui, center:Pos2) {
    ui.painter().line_segment([center + Vec2 {x: -5., y: -5.}, center + Vec2 {x: 5., y: 5.}], egui::Stroke::new(1., Color32::GREEN));
    ui.painter().line_segment([center + Vec2 {x: 5., y: -5.}, center + Vec2 {x: -5., y: 5.}], egui::Stroke::new(1., Color32::GREEN));
}