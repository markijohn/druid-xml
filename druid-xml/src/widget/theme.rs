use druid::{Insets, Key, Color, RoundedRectRadii};

use crate::simple_style::BorderStyle;

pub const STYLE_UPDATED_NONE:u64 = 0;
pub const STYLE_UPDATED_PAINT:u64 = 1;
pub const STYLE_UPDATED_LAYOUT:u64 = 2;

pub const STYLE_UPDATED: Key<u64> = Key::new("org.druid_xml.style_updated"); //0:none, 1:paint, 2:layout&paint
pub const PADDING: Key<Insets> = Key::new("org.druid_xml.padding");
pub const FONT_SIZE: Key<f64> = Key::new("org.druid_xml.font_size");
pub const COLOR: Key<Color> = Key::new("org.druid_xml.color");
pub const BORDER_WIDTH: Key<RoundedRectRadii> = Key::new("org.druid_xml.border");
pub const BORDER_RADIUS: Key<RoundedRectRadii> = Key::new("org.druid_xml.border");
pub const BACKGROUND_COLOR: Key<Color> = Key::new("org.druid_xml.background_color");

pub const DEFAULT_FONT_SIZE:f64 = 14.;
pub const DEFAULT_TEXT_COLOR:Color = Color::rgba8(0, 0, 0, 255);
pub const DEFAULT_BACKGROUND_COLOR:Color = Color::rgba8(0, 0,0, 0);

pub fn default_border() -> BorderStyle {
    BorderStyle::new(0., 0., Color::rgba8(0,0,0,0))
}