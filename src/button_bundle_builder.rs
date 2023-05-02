use bevy::prelude::*;

use crate::style_bundle_builder::StyleBuilder;

//pub fn top_down_panel(with_pct: f32, margin_pct: f32) -> NodeBundle {
//    NodeBundle {
//        style: Style {
//            display: Display::Flex,
//            size: Size {
//                width: Val::Percent(with_pct),
//                height: Val::Percent(100.0),
//            },
//            align_self: AlignSelf::Center,
//            margin: UiRect::left(Val::Percent(margin_pct)),
//            position_type: PositionType::Absolute,
//            position: UiRect {
//                left: Val::Percent(3.0),
//                right: Val::Percent(3.0),
//                ..default()
//            },
//            flex_direction: FlexDirection::Column, //top to bottoma
//            align_items: AlignItems::Center,
//            justify_content: JustifyContent::Start,
//            ..default()
//        },
//        ..default()
//    }
//}

pub struct ButtonBundleBuilder {
    bundle: ButtonBundle,
    style_builder: StyleBuilder,
}
impl ButtonBundleBuilder {
    pub fn new(style_builder: StyleBuilder) -> ButtonBundleBuilder {
        ButtonBundleBuilder {
            bundle: ButtonBundle::default(),
            style_builder,
        }
    }

    pub fn background_color(mut self, color: Color) -> ButtonBundleBuilder {
        self.bundle.background_color = color.into();
        self
    }

    pub fn z_index_local(mut self, z: i32) -> ButtonBundleBuilder {
        self.bundle.z_index = ZIndex::Local(z);
        self
    }

    pub fn z_index_global(mut self, z: i32) -> ButtonBundleBuilder {
        self.bundle.z_index = ZIndex::Global(z);
        self
    }
    pub fn bundle(&self) -> ButtonBundle {
        let mut button_bundle = self.bundle.clone();
        button_bundle.style = self.style_builder.style.clone();
        button_bundle
    }
    pub fn change_style<F: Fn(StyleBuilder) -> StyleBuilder>(
        mut self,
        closure: F,
    ) -> ButtonBundleBuilder {
        let new_style = closure(self.style_builder);
        self.style_builder = new_style;
        self
    }
}
