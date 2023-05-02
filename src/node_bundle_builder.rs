use crate::style_bundle_builder::StyleBuilder;
use bevy::prelude::*;

pub struct NodeBundleBuilder {
    bundle: NodeBundle,
    style_builder: StyleBuilder,
}

impl NodeBundleBuilder {
    pub fn new(style_builder: StyleBuilder) -> NodeBundleBuilder {
        NodeBundleBuilder {
            bundle: NodeBundle::default(),
            style_builder,
        }
    }
    pub fn z_index_local(mut self, z: i32) -> NodeBundleBuilder {
        self.bundle.z_index = ZIndex::Local(z);
        self
    }

    pub fn change_style<F: Fn(StyleBuilder) -> StyleBuilder>(
        mut self,
        closure: F,
    ) -> NodeBundleBuilder {
        let new_style = closure(self.style_builder);
        self.style_builder = new_style;
        self
    }
    pub fn background_color(mut self, bg_color: Color) -> NodeBundleBuilder {
        self.bundle.background_color = bg_color.into();
        self
    }
    pub fn z_index_global(mut self, z: i32) -> NodeBundleBuilder {
        self.bundle.z_index = ZIndex::Global(z);
        self
    }
    pub fn bundle(&self) -> NodeBundle {
        let mut node_bundle = self.bundle.clone();
        node_bundle.style = self.style_builder.style.clone();
        node_bundle
    }
}
