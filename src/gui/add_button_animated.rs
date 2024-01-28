use std::time::Instant;

use egui::{epaint, vec2, Color32, Id, Stroke, Widget};

use crate::constants::BACKGROUND_COLOR;




pub struct AddButtonAnimated {
    id: Id,
    radius:f32,
    background_color:Color32,
    foreground_color:Color32,
    line_width:f32,
}

#[derive(Debug, Clone, Copy)]
pub struct AnimationState {
    frame:u32,
}

impl AddButtonAnimated {
    pub fn new(id:Id) -> Self {
        Self {
            id,
            radius: 25.,
            background_color: BACKGROUND_COLOR,
            foreground_color: Color32::WHITE,
            line_width: 10.
        }
    }

    pub fn radius(mut self, radius:f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn line_width(mut self, line_width:f32) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn background_color(mut self, color:Color32) -> Self {
        self.background_color = color;
        self
    }

    pub fn foreground_color(mut self, color:Color32) -> Self {
        self.foreground_color = color;
        self
    }
}

impl Widget for AddButtonAnimated {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self { id, mut radius, background_color, foreground_color, mut line_width } = self;

        let (rect, response) = ui.allocate_exact_size(vec2(radius, radius), egui::Sense::click());



         if response.hovered() {
             if let Some(state) = ui.memory(|mem| mem.data.get_temp::<AnimationState>(id)) {
                radius = radius + f32::exp(state.frame as f32 * 0.16); // Adjust the exponential factor as needed
                line_width = line_width + f32::exp(state.frame as f32 * 0.16);

                 if state.frame < 8 {
                    ui.memory_mut(|mem| mem.data.insert_temp(id, AnimationState {
                        frame: state.frame + 1,
                    }));
                    
                    ui.ctx().request_repaint();
                 }
             } else {
                 ui.memory_mut(|mem| mem.data.insert_temp(id, AnimationState {
                    frame: 0,
                }));
             }
         } else {
            ui.memory_mut(|mem| mem.data.remove::<AnimationState>(id));
         }

        let painter = ui.painter();

        let center = rect.center();

        let circle = epaint::CircleShape {
            center,
            radius,
            fill: background_color,
            stroke: Stroke::NONE,
        };

        painter.add(circle);

        painter.line_segment([center + vec2(-line_width, 0.), center + vec2(line_width, 0.)], (2.0, foreground_color));
        painter.line_segment([center + vec2(0., -line_width), center + vec2(0., line_width)], (2.0, foreground_color));

        response
    }
}