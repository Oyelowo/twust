/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TailwindConfig {
    pub theme: Theme,
    pub allowed_lists: Option<AllowedLists>,
    pub variants: Option<Variants>,
    pub core_plugins: Option<CorePlugins>,
    pub plugins: Option<Plugins>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CorePlugins {
    pub screens: Option<bool>,
    pub colors: Option<bool>,
    pub spacing: Option<bool>,

    pub width: Option<bool>,
    pub height: Option<bool>,

    pub min_width: Option<bool>,

    pub min_height: Option<bool>,

    pub max_width: Option<bool>,

    pub max_height: Option<bool>,

    pub padding: Option<bool>,
    pub margin: Option<bool>,
    pub negative_margin: Option<bool>,

    pub shadows: Option<bool>,

    pub z_index: Option<bool>,
    pub opacity: Option<bool>,
    pub stroke: Option<bool>,

    pub accent_color: Option<bool>,
    pub accessibility: Option<bool>,

    pub align_content: Option<bool>,
    pub align_items: Option<bool>,
    pub align_self: Option<bool>,
    pub animation: Option<bool>,
    pub appearance: Option<bool>,
    pub aspect_ratio: Option<bool>,
    pub grayscale: Option<bool>,
    pub backdrop_blur: Option<bool>,
    pub backdrop_brightness: Option<bool>,
    pub backdrop_contrast: Option<bool>,
    pub backdrop_filter: Option<bool>,
    pub backdrop_grayscale: Option<bool>,
    pub backdrop_hue_rotate: Option<bool>,
    pub backdrop_invert: Option<bool>,
    pub backdrop_opacity: Option<bool>,
    pub backdrop_saturate: Option<bool>,
    pub backdrop_sepia: Option<bool>,
    pub background_attachment: Option<bool>,
    pub background_blend_mode: Option<bool>,
    pub background_clip: Option<bool>,
    pub background_color: Option<bool>,
    pub background_image: Option<bool>,
    pub background_opacity: Option<bool>,
    pub background_origin: Option<bool>,
    pub background_position: Option<bool>,
    pub background_repeat: Option<bool>,
    pub background_size: Option<bool>,
    pub blur: Option<bool>,
    pub border_collapse: Option<bool>,
    pub border_color: Option<bool>,
    pub border_opacity: Option<bool>,
    pub border_radius: Option<bool>,
    pub border_spacing: Option<bool>,
    pub border_style: Option<bool>,
    pub border_width: Option<bool>,
    pub box_decoration_break: Option<bool>,
    pub box_shadow: Option<bool>,
    pub box_shadow_color: Option<bool>,
    pub box_sizing: Option<bool>,
    pub break_after: Option<bool>,
    pub break_before: Option<bool>,
    pub break_inside: Option<bool>,
    pub brightness: Option<bool>,
    pub caption_side: Option<bool>,
    pub caret_color: Option<bool>,
    pub clear: Option<bool>,
    pub columns: Option<bool>,
    pub container: Option<bool>,
    pub content: Option<bool>,
    pub contrast: Option<bool>,
    pub cursor: Option<bool>,
    pub display: Option<bool>,
    pub divide_color: Option<bool>,
    pub divide_opacity: Option<bool>,
    pub divide_style: Option<bool>,
    pub divide_width: Option<bool>,
    pub drop_shadow: Option<bool>,
    pub fill: Option<bool>,
    pub filter: Option<bool>,
    pub flex: Option<bool>,
    pub flex_basis: Option<bool>,
    pub flex_direction: Option<bool>,
    pub flex_grow: Option<bool>,
    pub flex_shrink: Option<bool>,
    pub flex_wrap: Option<bool>,
    pub float: Option<bool>,
    pub font_family: Option<bool>,
    pub font_size: Option<bool>,
    pub font_smoothing: Option<bool>,
    pub font_style: Option<bool>,
    pub font_variant_numeric: Option<bool>,
    pub font_weight: Option<bool>,
    pub gap: Option<bool>,
    pub gradient_color_stops: Option<bool>,
    pub grid_auto_columns: Option<bool>,
    pub grid_auto_flow: Option<bool>,
    pub grid_auto_rows: Option<bool>,
    pub grid_column: Option<bool>,
    pub grid_column_end: Option<bool>,
    pub grid_column_start: Option<bool>,
    pub grid_row: Option<bool>,
    pub grid_row_end: Option<bool>,
    pub grid_row_start: Option<bool>,
    pub grid_template_columns: Option<bool>,
    pub grid_template_rows: Option<bool>,
    pub hue_rotate: Option<bool>,
    pub hyphens: Option<bool>,
    pub top: Option<bool>,
    pub right: Option<bool>,
    pub left: Option<bool>,
    pub bottom: Option<bool>,
    pub start: Option<bool>,
    pub end: Option<bool>,
    pub inset: Option<bool>,
    pub invert: Option<bool>,
    pub isolation: Option<bool>,
    pub justify_content: Option<bool>,
    pub justify_items: Option<bool>,
    pub justify_self: Option<bool>,
    pub letter_spacing: Option<bool>,
    pub line_clamp: Option<bool>,
    pub line_height: Option<bool>,
    pub list_style_image: Option<bool>,
    pub list_style_position: Option<bool>,
    pub list_style_type: Option<bool>,
    pub mix_blend_mode: Option<bool>,
    pub object_fit: Option<bool>,
    pub object_position: Option<bool>,
    pub order: Option<bool>,
    pub outline_color: Option<bool>,
    pub outline_offset: Option<bool>,
    pub outline_style: Option<bool>,
    pub outline_width: Option<bool>,
    pub overflow: Option<bool>,
    pub overscroll_behavior: Option<bool>,
    pub place_content: Option<bool>,
    pub place_items: Option<bool>,
    pub place_self: Option<bool>,
    pub placeholder_color: Option<bool>,
    pub placeholder_opacity: Option<bool>,
    pub pointer_events: Option<bool>,
    pub position: Option<bool>,
    pub preflight: Option<bool>,
    pub resize: Option<bool>,
    pub ring_color: Option<bool>,
    pub ring_offset_color: Option<bool>,
    pub ring_offset_width: Option<bool>,
    pub ring_opacity: Option<bool>,
    pub ring_width: Option<bool>,
    pub rotate: Option<bool>,
    pub saturate: Option<bool>,
    pub scale: Option<bool>,
    pub scroll_behavior: Option<bool>,
    pub scroll_margin: Option<bool>,
    pub scroll_padding: Option<bool>,
    pub scroll_snap_align: Option<bool>,
    pub scroll_snap_stop: Option<bool>,
    pub scroll_snap_type: Option<bool>,
    pub sepia: Option<bool>,
    pub skew: Option<bool>,
    pub space: Option<bool>,
    pub stroke_width: Option<bool>,
    pub table_layout: Option<bool>,
    pub text_align: Option<bool>,
    pub text_color: Option<bool>,
    pub text_decoration: Option<bool>,
    pub text_decoration_color: Option<bool>,
    pub text_decoration_style: Option<bool>,
    pub text_decoration_thickness: Option<bool>,
    pub text_indent: Option<bool>,
    pub text_opacity: Option<bool>,
    pub text_overflow: Option<bool>,
    pub text_transform: Option<bool>,
    pub text_underline_offset: Option<bool>,
    pub touch_action: Option<bool>,
    pub transform: Option<bool>,
    pub transform_origin: Option<bool>,

    pub transition_delay: Option<bool>,
    pub transition_duration: Option<bool>,
    pub transition_property: Option<bool>,
    pub transition_timing_function: Option<bool>,
    pub translate: Option<bool>,
    pub user_select: Option<bool>,
    pub vertical_align: Option<bool>,
    pub visibility: Option<bool>,
    pub whitespace: Option<bool>,
    pub will_change: Option<bool>,
    pub word_break: Option<bool>,
    pub screen_readers: Option<bool>,
    // pub extend: HashMap<Key, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllowedLists {
    pub classes: Option<Vec<String>>,
    pub modifiers: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Theme {
    #[serde(flatten)]
    pub overrides: CustomisableClasses,
    pub extend: CustomisableClasses,
}

// Represents a color which can either be a simple string or a nested structure.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomisableClasses {
    // pub screens: Option<HashMap<String, String>>,
    pub screens: Option<HashMap<Key, ScreenValue>>,
    pub colors: Option<HashMap<Key, ColorValue>>,
    pub spacing: Option<HashMap<Key, String>>,

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

    // Modifiers
    pub aria: Option<HashMap<Key, String>>,
    pub supports: Option<HashMap<Key, String>>,
    pub data: Option<HashMap<Key, String>>,
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