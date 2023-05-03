use crate::style_bundle_builder::StyleBuilder;
use bevy::prelude::*;

pub struct TextBuilder {
    pub text: Text,
}

impl TextBuilder {
    pub fn new() -> TextBuilder {
        TextBuilder {
            text: Text::default(),
        }
    }

    pub fn add_text_section(mut self, s: TextSection) -> Self {
        self.text.sections.push(s);
        self
    }

    pub fn set_text_sections(mut self, s: Vec<TextSection>) -> Self {
        self.text.sections = s;
        self
    }
    pub fn add_string_style(self, value: String, style: TextStyle) -> Self {
        self.add_text_section(TextSection::new(value, style))
    }
}

pub struct TextBundleBuilder {
    bundle: TextBundle,
    style_builder: StyleBuilder,
    text_builder: TextBuilder,
}

impl TextBundleBuilder {
    pub fn new(style_builder: StyleBuilder) -> TextBundleBuilder {
        TextBundleBuilder {
            style_builder,
            bundle: TextBundle::default(),
            text_builder: TextBuilder::new(),
        }
    }

    pub fn z_index_local(mut self, z: i32) -> TextBundleBuilder {
        self.bundle.z_index = ZIndex::Local(z);

        self
    }

    pub fn change_style<F: Fn(StyleBuilder) -> StyleBuilder>(
        mut self,
        closure: F,
    ) -> TextBundleBuilder {
        let new_style = closure(self.style_builder);
        self.style_builder = new_style;
        self
    }
    pub fn background_color(mut self, bg_color: BackgroundColor) -> TextBundleBuilder {
        self.bundle.background_color = bg_color;
        self
    }
    pub fn z_index_global(mut self, z: i32) -> TextBundleBuilder {
        self.bundle.z_index = ZIndex::Global(z);
        self
    }

    pub fn text(mut self, t: Text) -> TextBundleBuilder {
        self.bundle.text = t;
        self
    }

    pub fn string(mut self, t: String, style: TextStyle) -> TextBundleBuilder {
        self.bundle.text = Text::from_section(t, style);
        self
    }

    pub fn text_bundle(self) -> TextBundle {
        let mut node_bundle = self.bundle.clone();
        node_bundle.style = self.style_builder.style.clone();
        node_bundle.text = self.text_builder.text;
        node_bundle
    }

    pub fn add_text_section(mut self, s: TextSection) -> Self {
        self.text_builder = self.text_builder.add_text_section(s);
        self
    }

    pub fn set_text_sections(mut self, s: Vec<TextSection>) -> Self {
        self.text_builder = self.text_builder.set_text_sections(s);
        self
    }
    pub fn clear_text_sections(mut self) -> Self {
        self.text_builder = self.text_builder.set_text_sections(vec![]);
        self
    }

    pub fn add_string_style(self, value: String, style: TextStyle) -> Self {
        self.add_text_section(TextSection::new(value, style))
    }
}
