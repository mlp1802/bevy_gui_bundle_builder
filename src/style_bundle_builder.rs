use bevy::prelude::*;

use crate::{button_bundle_builder::ButtonBundleBuilder, node_bundle_builder::NodeBundleBuilder};

#[derive(Clone)]
pub struct StyleBuilder {
    pub style: Style,
}
impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            style: Style::default(),
        }
    }
    pub fn node_builder(self) -> NodeBundleBuilder {
        NodeBundleBuilder::new(self)
    }
    pub fn button_builder(self) -> ButtonBundleBuilder {
        ButtonBundleBuilder::new(self)
    }

    pub fn node_bundle(self) -> NodeBundle {
        self.node_builder().bundle()
    }

    pub fn button_bundle(self) -> ButtonBundle {
        self.button_builder().bundle()
    }

    pub fn height_2(mut self, h: Val) -> StyleBuilder {
        self.style.size.height = h;
        self
    }

    pub fn display(mut self, d: Display) -> StyleBuilder {
        self.style.display = d;
        self
    }

    pub fn position(mut self, p: UiRect) -> StyleBuilder {
        self.style.position = p;
        self
    }
    pub fn position_left(mut self, l: Val) -> StyleBuilder {
        self.style.position.left = l;
        self
    }

    pub fn position_top(mut self, l: Val) -> StyleBuilder {
        self.style.position.top = l;
        self
    }

    pub fn width(mut self, w: Val) -> StyleBuilder {
        self.style.size.width = w;
        self
    }

    pub fn width_pct(mut self, w: f32) -> StyleBuilder {
        self.style.size.width = Val::Percent(w);
        self
    }
    pub fn width_px(mut self, w: f32) -> StyleBuilder {
        self.style.size.width = Val::Px(w);
        self
    }
    pub fn size_px(self, x: f32, y: f32) -> StyleBuilder {
        self.width_px(x).height_px(y)
    }

    pub fn height(mut self, h: Val) -> StyleBuilder {
        self.style.size.height = h;
        self
    }

    pub fn height_pct(mut self, h: f32) -> StyleBuilder {
        self.style.size.height = Val::Percent(h);
        self
    }

    pub fn height_px(mut self, h: f32) -> StyleBuilder {
        self.style.size.height = Val::Px(h);
        self
    }

    pub fn flex_direction(mut self, f: FlexDirection) -> StyleBuilder {
        self.style.flex_direction = f;
        self
    }
    pub fn flex_direction_row(mut self) -> StyleBuilder {
        self.style.flex_direction = FlexDirection::Row;
        self
    }
    pub fn flex_direction_col(mut self) -> StyleBuilder {
        self.style.flex_direction = FlexDirection::Column;
        self
    }

    pub fn align_items(mut self, a: AlignItems) -> StyleBuilder {
        self.style.align_items = a;
        self
    }
    pub fn align_self(mut self, a: AlignSelf) -> StyleBuilder {
        self.style.align_self = a;
        self
    }
    pub fn align_items_center(mut self) -> StyleBuilder {
        self.style.align_items = AlignItems::Center;
        self
    }
    pub fn align_items_start(mut self) -> StyleBuilder {
        self.style.align_items = AlignItems::Start;
        self
    }

    pub fn align_items_end(mut self) -> StyleBuilder {
        self.style.align_items = AlignItems::End;
        self
    }
    pub fn align_self_start(mut self) -> StyleBuilder {
        self.style.align_self = AlignSelf::Start;
        self
    }
    pub fn align_self_center(mut self) -> StyleBuilder {
        self.style.align_self = AlignSelf::Center;
        self
    }

    pub fn align_self_end(mut self) -> StyleBuilder {
        self.style.align_self = AlignSelf::End;
        self
    }

    pub fn justify_content(mut self, j: JustifyContent) -> StyleBuilder {
        self.style.justify_content = j;
        self
    }
    pub fn justify_content_center(mut self) -> StyleBuilder {
        self.style.justify_content = JustifyContent::Center;
        self
    }

    pub fn justify_content_start(mut self) -> StyleBuilder {
        self.style.justify_content = JustifyContent::Start;
        self
    }

    pub fn justify_content_end(mut self) -> StyleBuilder {
        self.style.justify_content = JustifyContent::End;
        self
    }
    //margin
    pub fn margin(mut self, m: UiRect) -> StyleBuilder {
        self.style.margin = m;
        self
    }
    pub fn margin_left(mut self, l: Val) -> StyleBuilder {
        self.style.margin.left = l;
        self
    }

    pub fn margin_all_px(self, m: f32) -> StyleBuilder {
        self.margin_left_px(m)
            .margin_top_px(m)
            .margin_right_px(m)
            .margin_left_px(m)
    }

    pub fn margin_all_pct(self, m: f32) -> StyleBuilder {
        self.margin_left_pct(m)
            .margin_top_pct(m)
            .margin_right_pct(m)
            .margin_bottom_pct(m)
    }

    pub fn margin_left_pct(mut self, l: f32) -> StyleBuilder {
        self.style.margin.left = Val::Percent(l);
        self
    }
    pub fn margin_top(mut self, t: Val) -> StyleBuilder {
        self.style.margin.top = t;
        self
    }
    pub fn margin_top_pct(mut self, t: f32) -> StyleBuilder {
        self.style.margin.top = Val::Percent(t);
        self
    }
    pub fn margin_left_px(mut self, t: f32) -> StyleBuilder {
        self.style.margin.left = Val::Px(t);
        self
    }

    pub fn margin_top_px(mut self, t: f32) -> StyleBuilder {
        self.style.margin.top = Val::Px(t);
        self
    }

    pub fn margin_right(mut self, r: Val) -> StyleBuilder {
        self.style.margin.right = r;
        self
    }

    pub fn margin_right_pct(mut self, r: f32) -> StyleBuilder {
        self.style.margin.right = Val::Percent(r);
        self
    }
    pub fn margin_right_px(mut self, r: f32) -> StyleBuilder {
        self.style.margin.right = Val::Px(r);
        self
    }

    pub fn margin_bottom(mut self, b: Val) -> StyleBuilder {
        self.style.margin.bottom = b;
        self
    }
    pub fn margin_bottom_pct(mut self, b: f32) -> StyleBuilder {
        self.style.margin.bottom = Val::Percent(b);
        self
    }
    pub fn margin_bottom_px(mut self, b: f32) -> StyleBuilder {
        self.style.margin.bottom = Val::Px(b);
        self
    }

    //padding
    pub fn padding(mut self, m: UiRect) -> StyleBuilder {
        self.style.padding = m;
        self
    }
    pub fn padding_left(mut self, l: Val) -> StyleBuilder {
        self.style.padding.left = l;
        self
    }
    pub fn padding_left_pct(mut self, l: f32) -> StyleBuilder {
        self.style.padding.left = Val::Percent(l);
        self
    }
    pub fn padding_top(mut self, t: Val) -> StyleBuilder {
        self.style.padding.top = t;
        self
    }
    pub fn padding_top_pct(mut self, t: f32) -> StyleBuilder {
        self.style.padding.top = Val::Percent(t);
        self
    }
    pub fn padding_top_px(mut self, t: f32) -> StyleBuilder {
        self.style.padding.top = Val::Px(t);
        self
    }

    pub fn padding_right(mut self, r: Val) -> StyleBuilder {
        self.style.padding.right = r;
        self
    }

    pub fn padding_right_pct(mut self, r: f32) -> StyleBuilder {
        self.style.padding.right = Val::Percent(r);
        self
    }

    pub fn padding_bottom(mut self, b: Val) -> StyleBuilder {
        self.style.padding.bottom = b;
        self
    }
    pub fn padding_bottom_pct(mut self, b: f32) -> StyleBuilder {
        self.style.padding.bottom = Val::Percent(b);
        self
    }
}
pub fn lol() {
    StyleBuilder::new()
        .margin_right(Val::Percent(0.2))
        .align_self(AlignSelf::Center);
}
