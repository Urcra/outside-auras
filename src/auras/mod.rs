use eframe::{
    emath::Align2,
    epaint::{Color32, FontId, TextureHandle},
};

pub mod delay_display;
pub mod list_aura;
pub mod nelth_aura;

pub struct AuraItem<'a> {
    pub icon: Option<AuraIcon<'a>>,
    pub icon_text: Option<AuraText>,
    pub text: Option<AuraText>,
    pub background: Option<AuraBackground>,
}

pub struct AuraIcon<'a> {
    pub image: &'a TextureHandle,
    pub height: f32,
    pub width: f32,
}

pub struct AuraText {
    pub content: String,
    pub anchor: Align2,
    pub font: FontId,
    pub color: Color32,
    pub x_offset: f32,
    pub y_offset: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct AuraBackground {
    pub color: Color32,
    pub width: f32,
    pub height: f32,
}
