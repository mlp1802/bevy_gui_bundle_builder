use bevy::prelude::*;
pub struct TextStyleBuilder {
    text_style: TextStyle,
}

impl TextStyleBuilder {
    pub fn new() -> TextStyleBuilder {
        TextStyleBuilder {
            text_style: TextStyle::default(),
        }
    }

    pub fn color(mut self, c: Color) -> TextStyleBuilder {
        self.text_style.color = c;
        self
    }
    pub fn font_size(mut self, s: f32) -> TextStyleBuilder {
        self.text_style.font_size = s;
        self
    }
    pub fn font(mut self, f: Handle<Font>) -> TextStyleBuilder {
        self.text_style.font = f;
        self
    }

    pub fn get_text_style(self) -> TextStyle {
        self.text_style
    }
}
