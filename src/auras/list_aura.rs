use super::AuraItem;
use eframe::{
    egui,
    epaint::{Color32, Pos2, Rect, Rounding, Shadow, Vec2},
};

pub trait ListAura {
    fn required_size(&self) -> Vec2;

    fn item_height(&self) -> f32;

    fn item_width(&self) -> f32;

    fn items(&self) -> Vec<AuraItem>;

    fn eframe_update(&mut self, ctx: &egui::Context, window_frame: &mut eframe::Frame) {
        let my_frame = egui::containers::Frame::none()
            .fill(Color32::WHITE)
            .shadow(Shadow::NONE);

        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                let ui_max = self.required_size();

                window_frame.set_window_size(ui_max);

                let dimensions = Rect {
                    min: Pos2::new(0.0, 0.0),
                    max: ui_max.to_pos2(),
                };

                let painter = ui.painter_at(dimensions);

                let uv = Rect::from_min_max([0.0, 0.0].into(), [1.0, 1.0].into());

                let item_height = self.item_height();

                for (i, item) in self.items().iter().enumerate() {
                    let i = i as f32;

                    if let Some(b) = &item.background {
                        let dimensions_background = Rect {
                            min: Pos2::new(0.0, i * item_height),
                            max: Pos2::new(b.width, i * item_height + b.height),
                        };

                        painter.rect_filled(dimensions_background, Rounding::none(), b.color);
                    }

                    if let Some(ic) = &item.icon {
                        let ih = ic.height;
                        let iw = ic.width;
                        let dimensions = Rect {
                            min: Pos2::new(0.0, i * item_height),
                            max: Pos2::new(iw, item_height * i + ih),
                        };

                        painter.image(ic.image.id(), dimensions, uv, Color32::WHITE);
                    }

                    if let Some(t) = &item.icon_text {
                        let x_off = t.x_offset;
                        let y_off = t.y_offset;

                        painter.text(
                            [x_off, (i * item_height) + y_off].into(),
                            t.anchor,
                            t.content.clone(),
                            t.font.clone(),
                            t.color,
                        );
                    }

                    if let Some(t) = &item.text {
                        let x_off = t.x_offset;
                        let y_off = t.y_offset;

                        painter.text(
                            [x_off, (i * item_height) + y_off].into(),
                            t.anchor,
                            t.content.clone(),
                            t.font.clone(),
                            t.color,
                        );
                    }
                }
            });
    }
}
