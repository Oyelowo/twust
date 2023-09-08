use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TailwindConfig {
    pub theme: Theme,
    pub variants: Variants,
    pub plugins: Plugins,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(flatten)]
    pub overrides: CustomisableClasses,
    pub extend: CustomisableClasses,
}

// Represents a color which can either be a simple string or a nested structure.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorValue {
    Simple(String),
    Shades(HashMap<String, String>),
}

// #[derive(Debug, Deserialize)]
// struct Theme {
//     colors: Option<HashMap<String, ColorValue>>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScreenValue {
    Simple(String),
    Range(RangeBreakpoint),
    MultiRange(Vec<RangeBreakpoint>),
    Raw(RawBreakpoint),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeBreakpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawBreakpoint {
    raw: String,
}

pub type Key = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomisableClasses {
    // pub screens: Option<HashMap<String, String>>,
    pub screens: Option<HashMap<Key, ScreenValue>>,
    pub colors: Option<HashMap<Key, ColorValue>>,
    pub spacing: Option<HashMap<Key, String>>,

    pub border_widths: Option<HashMap<Key, String>>,

    pub width: Option<HashMap<Key, String>>,
    pub height: Option<HashMap<Key, String>>,

    pub min_width: Option<HashMap<Key, String>>,

    pub min_height: Option<HashMap<Key, String>>,

    pub max_width: Option<HashMap<Key, String>>,

    pub max_height: Option<HashMap<Key, String>>,

    pub padding: Option<HashMap<Key, String>>,
    pub margin: Option<HashMap<Key, String>>,
    pub negative_margin: Option<HashMap<Key, String>>,

    pub shadows: Option<HashMap<Key, String>>,

    pub z_index: Option<HashMap<Key, String>>,
    pub opacity: Option<HashMap<Key, String>>,
    pub stroke: Option<HashMap<Key, ColorValue>>,

    pub accent_color: Option<HashMap<Key, ColorValue>>,
    pub accessibility: Option<HashMap<Key, String>>,

    pub align_content: Option<HashMap<Key, String>>,
    pub align_items: Option<HashMap<Key, String>>,
    pub align_self: Option<HashMap<Key, String>>,
    pub animation: Option<HashMap<Key, String>>,
    pub appearance: Option<HashMap<Key, String>>,
    pub aspect_ratio: Option<HashMap<Key, String>>,
    pub grayscale: Option<HashMap<Key, String>>,
    pub backdrop_blur: Option<HashMap<Key, String>>,
    pub backdrop_brightness: Option<HashMap<Key, String>>,
    pub backdrop_contrast: Option<HashMap<Key, String>>,
    pub backdrop_filter: Option<HashMap<Key, String>>,
    pub backdrop_grayscale: Option<HashMap<Key, String>>,
    pub backdrop_hue_rotate: Option<HashMap<Key, String>>,
    pub backdrop_invert: Option<HashMap<Key, String>>,
    pub backdrop_opacity: Option<HashMap<Key, String>>,
    pub backdrop_saturate: Option<HashMap<Key, String>>,
    pub backdrop_sepia: Option<HashMap<Key, String>>,
    pub background_attachment: Option<HashMap<Key, String>>,
    pub background_blend_mode: Option<HashMap<Key, String>>,
    pub background_clip: Option<HashMap<Key, String>>,
    pub background_color: Option<HashMap<Key, ColorValue>>,
    pub background_image: Option<HashMap<Key, String>>,
    pub background_opacity: Option<HashMap<Key, String>>,
    pub background_origin: Option<HashMap<Key, String>>,
    pub background_position: Option<HashMap<Key, String>>,
    pub background_repeat: Option<HashMap<Key, String>>,
    pub background_size: Option<HashMap<Key, String>>,
    pub blur: Option<HashMap<Key, String>>,
    pub border_collapse: Option<HashMap<Key, String>>,
    pub border_color: Option<HashMap<Key, ColorValue>>,
    pub border_opacity: Option<HashMap<Key, String>>,
    pub border_radius: Option<HashMap<Key, String>>,
    pub border_spacing: Option<HashMap<Key, String>>,
    pub border_style: Option<HashMap<Key, String>>,
    pub border_width: Option<HashMap<Key, String>>,
    pub box_decoration_break: Option<HashMap<Key, String>>,
    pub box_shadow: Option<HashMap<Key, String>>,
    pub box_shadow_color: Option<HashMap<Key, ColorValue>>,
    pub box_sizing: Option<HashMap<Key, String>>,
    pub break_after: Option<HashMap<Key, String>>,
    pub break_before: Option<HashMap<Key, String>>,
    pub break_inside: Option<HashMap<Key, String>>,
    pub brightness: Option<HashMap<Key, String>>,
    pub caption_side: Option<HashMap<Key, String>>,
    pub caret_color: Option<HashMap<Key, ColorValue>>,
    pub clear: Option<HashMap<Key, String>>,
    pub columns: Option<HashMap<Key, String>>,
    pub container: Option<HashMap<Key, String>>,
    pub content: Option<HashMap<Key, String>>,
    pub contrast: Option<HashMap<Key, String>>,
    pub cursor: Option<HashMap<Key, String>>,
    pub display: Option<HashMap<Key, String>>,
    pub divide_color: Option<HashMap<Key, ColorValue>>,
    pub divide_opacity: Option<HashMap<Key, String>>,
    pub divide_style: Option<HashMap<Key, String>>,
    pub divide_width: Option<HashMap<Key, String>>,
    pub drop_shadow: Option<HashMap<Key, String>>,
    pub fill: Option<HashMap<Key, ColorValue>>,
    pub filter: Option<HashMap<Key, String>>,
    pub flex: Option<HashMap<Key, String>>,
    pub flex_basis: Option<HashMap<Key, String>>,
    pub flex_direction: Option<HashMap<Key, String>>,
    pub flex_grow: Option<HashMap<Key, String>>,
    pub flex_shrink: Option<HashMap<Key, String>>,
    pub flex_wrap: Option<HashMap<Key, String>>,
    pub float: Option<HashMap<Key, String>>,
    pub font_family: Option<HashMap<Key, String>>,
    pub font_size: Option<HashMap<Key, String>>,
    pub font_smoothing: Option<HashMap<Key, String>>,
    pub font_style: Option<HashMap<Key, String>>,
    pub font_variant_numeric: Option<HashMap<Key, String>>,
    pub font_weight: Option<HashMap<Key, String>>,
    pub gap: Option<HashMap<Key, String>>,
    pub gradient_color_stops: Option<HashMap<Key, ColorValue>>,
    pub grid_auto_columns: Option<HashMap<Key, String>>,
    pub grid_auto_flow: Option<HashMap<Key, String>>,
    pub grid_auto_rows: Option<HashMap<Key, String>>,
    pub grid_column: Option<HashMap<Key, String>>,
    pub grid_column_end: Option<HashMap<Key, String>>,
    pub grid_column_start: Option<HashMap<Key, String>>,
    pub grid_row: Option<HashMap<Key, String>>,
    pub grid_row_end: Option<HashMap<Key, String>>,
    pub grid_row_start: Option<HashMap<Key, String>>,
    pub grid_template_columns: Option<HashMap<Key, String>>,
    pub grid_template_rows: Option<HashMap<Key, String>>,
    pub hue_rotate: Option<HashMap<Key, String>>,
    pub hyphens: Option<HashMap<Key, String>>,
    pub top: Option<HashMap<Key, String>>,
    pub right: Option<HashMap<Key, String>>,
    pub left: Option<HashMap<Key, String>>,
    pub bottom: Option<HashMap<Key, String>>,
    pub start: Option<HashMap<Key, String>>,
    pub end: Option<HashMap<Key, String>>,
    pub inset: Option<HashMap<Key, String>>,
    pub invert: Option<HashMap<Key, String>>,
    pub isolation: Option<HashMap<Key, String>>,
    pub justify_content: Option<HashMap<Key, String>>,
    pub justify_items: Option<HashMap<Key, String>>,
    pub justify_self: Option<HashMap<Key, String>>,
    pub letter_spacing: Option<HashMap<Key, String>>,
    pub line_clamp: Option<HashMap<Key, String>>,
    pub line_height: Option<HashMap<Key, String>>,
    pub list_style_image: Option<HashMap<Key, String>>,
    pub list_style_position: Option<HashMap<Key, String>>,
    pub list_style_type: Option<HashMap<Key, String>>,
    pub mix_blend_mode: Option<HashMap<Key, String>>,
    pub object_fit: Option<HashMap<Key, String>>,
    pub object_position: Option<HashMap<Key, String>>,
    pub order: Option<HashMap<Key, String>>,
    pub outline_color: Option<HashMap<Key, ColorValue>>,
    pub outline_offset: Option<HashMap<Key, String>>,
    pub outline_style: Option<HashMap<Key, String>>,
    pub outline_width: Option<HashMap<Key, String>>,
    pub overflow: Option<HashMap<Key, String>>,
    pub overscroll_behavior: Option<HashMap<Key, String>>,
    pub place_content: Option<HashMap<Key, String>>,
    pub place_items: Option<HashMap<Key, String>>,
    pub place_self: Option<HashMap<Key, String>>,
    pub placeholder_color: Option<HashMap<Key, ColorValue>>,
    pub placeholder_opacity: Option<HashMap<Key, String>>,
    pub pointer_events: Option<HashMap<Key, String>>,
    pub position: Option<HashMap<Key, String>>,
    pub preflight: Option<HashMap<Key, String>>,
    pub resize: Option<HashMap<Key, String>>,
    pub ring_color: Option<HashMap<Key, ColorValue>>,
    pub ring_offset_color: Option<HashMap<Key, ColorValue>>,
    pub ring_offset_width: Option<HashMap<Key, String>>,
    pub ring_opacity: Option<HashMap<Key, String>>,
    pub ring_width: Option<HashMap<Key, String>>,
    pub rotate: Option<HashMap<Key, String>>,
    pub saturate: Option<HashMap<Key, String>>,
    pub scale: Option<HashMap<Key, String>>,
    pub scroll_behavior: Option<HashMap<Key, String>>,
    pub scroll_margin: Option<HashMap<Key, String>>,
    pub scroll_padding: Option<HashMap<Key, String>>,
    pub scroll_snap_align: Option<HashMap<Key, String>>,
    pub scroll_snap_stop: Option<HashMap<Key, String>>,
    pub scroll_snap_type: Option<HashMap<Key, String>>,
    pub sepia: Option<HashMap<Key, String>>,
    pub skew: Option<HashMap<Key, String>>,
    pub space: Option<HashMap<Key, String>>,
    pub stroke_width: Option<HashMap<Key, String>>,
    pub table_layout: Option<HashMap<Key, String>>,
    pub text_align: Option<HashMap<Key, String>>,
    pub text_color: Option<HashMap<Key, ColorValue>>,
    pub text_decoration: Option<HashMap<Key, String>>,
    pub text_decoration_color: Option<HashMap<Key, ColorValue>>,
    pub text_decoration_style: Option<HashMap<Key, String>>,
    pub text_decoration_thickness: Option<HashMap<Key, String>>,
    pub text_indent: Option<HashMap<Key, String>>,
    pub text_opacity: Option<HashMap<Key, String>>,
    pub text_overflow: Option<HashMap<Key, String>>,
    pub text_transform: Option<HashMap<Key, String>>,
    pub text_underline_offset: Option<HashMap<Key, String>>,
    pub touch_action: Option<HashMap<Key, String>>,
    pub transform: Option<HashMap<Key, String>>,
    pub transform_origin: Option<HashMap<Key, String>>,

    pub transition_delay: Option<HashMap<Key, String>>,
    pub transition_duration: Option<HashMap<Key, String>>,
    pub transition_property: Option<HashMap<Key, String>>,
    pub transition_timing_function: Option<HashMap<Key, String>>,
    pub translate: Option<HashMap<Key, String>>,
    pub user_select: Option<HashMap<Key, String>>,
    pub vertical_align: Option<HashMap<Key, String>>,
    pub visibility: Option<HashMap<Key, String>>,
    pub whitespace: Option<HashMap<Key, String>>,
    pub will_change: Option<HashMap<Key, String>>,
    pub word_break: Option<HashMap<Key, String>>,

    pub screen_readers: Option<HashMap<Key, String>>,
    // pub extend: HashMap<Key, HashMap<String, String>>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Extend {
//     pub screens: Option<HashMap<Key, String>>,
//     // add other fields as needed
// }

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Variants {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Plugins {}
