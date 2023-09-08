use std::collections::HashMap;

use super::macros::{define_tailwind_color_field, define_tailwind_field};
use super::TailwindField;
use crate::tailwind::class_type::TAILWIND_CSS;
use crate::tailwind::tailwind_config::TailwindConfig;

// LayOut
//
// Aspect Ratio
// Container
// Columns
// Break After
// Break Before
// Break Inside
// Box Decoration Break
// Box Sizing
// Display
// Floats
// Clear
// Isolation
// Object Fit
// Object Position
// Overflow
// Overscroll Behavior
// Position
// Top / Right / Bottom / Left
// Visibility
// Z-Index

// Tailwind doesn’t include a large set of aspect ratio values out of the box since it’s easier to
// just use arbitrary values.
// if you need a one-off custom def, u can use the square-bracket: class="aspect-[4/3]"
// https://tailwindcss.com/docs/aspect-ratio
define_tailwind_field!({
    name: AspectRatio,
    prefix: "aspect",
    inherited: aspect_ratio,
    field_name: aspect_ratio,
    variants: []
});

// Configurable, non-configurable, non-changeable

//
// Configurable but not all values are predefined by tailwindcss
// Which means you can change the behaviour within the config
// but the namings are constant i.e: ["container", "mx-auto", "max-width", "min-width"].
define_tailwind_field!({
    name: Container,
    prefix: "container",
    inherited: container,
    field_name: container,
    variants: []
});

define_tailwind_field!({
    name: Columns,
    prefix: "columns",
    inherited: columns,
    field_name: columns,
    variants: []
});

define_tailwind_field!({
    name: BreakAfter,
    prefix: "break-after",
    inherited: break_after,
    field_name: break_after,
    variants: []
});

define_tailwind_field!({
    name: BreakBefore,
    prefix: "break-before",
    inherited: break_before,
    field_name: break_before,
    variants: []
});

define_tailwind_field!({
    name: BreakInside,
    prefix: "break-inside",
    inherited: break_inside,
    field_name: break_inside,
    variants: []
});

define_tailwind_field!({
    name: BoxDecorationBreak,
    prefix: "box-decoration",
    inherited: box_decoration_break,
    field_name: box_decoration_break,
    variants: []
});

define_tailwind_field!({
    name: BoxSizing,
    prefix: "box-sizing",
    inherited: box_sizing,
    field_name: box_sizing,
    variants: []
});

define_tailwind_field!({
    name: Display,
    prefix: "display",
    inherited: display,
    field_name: display,
    variants: []
});

define_tailwind_field!({
    name: Float,
    prefix: "float",
    inherited: float,
    field_name: float,
    variants: []
});

define_tailwind_field!({
    name: Clear,
    prefix: "clear",
    inherited: clear,
    field_name: clear,
    variants: []
});

define_tailwind_field!({
    name: Isolation,
    prefix: "isolation",
    inherited: isolation,
    field_name: isolation,
    variants: []
});

define_tailwind_field!({
    name: ObjectFit,
    prefix: "object",
    inherited: object_fit,
    field_name: object_fit,
    variants: []
});

define_tailwind_field!({
    name: ObjectPosition,
    prefix: "object",
    inherited: object_position,
    field_name: object_position,
    variants: []
});

define_tailwind_field!({
    name: Overflow,
    prefix: "overflow",
    inherited: overflow,
    field_name: overflow,
    variants: []
});

define_tailwind_field!({
    name: OverscrollBehavior,
    prefix: "overscroll-behavior",
    inherited: overscroll_behavior,
    field_name: overscroll_behavior,
    variants: []
});

define_tailwind_field!({
    name: Position,
    prefix: "position",
    inherited: position,
    field_name: position,
    variants: []
});

define_tailwind_field!({
    name: Top,
    prefix: "top",
    inherited: spacing,
    field_name: top,
    variants: []
});

define_tailwind_field!({
    name: Right,
    prefix: "right",
    inherited: spacing,
    field_name: right,
    variants: []
});

define_tailwind_field!({
    name: Bottom,
    prefix: "bottom",
    inherited: spacing,
    field_name: bottom,
    variants: []
});

define_tailwind_field!({
    name: Left,
    prefix: "left",
    inherited: spacing,
    field_name: left,
    variants: []
});

define_tailwind_field!({
    name: Start,
    prefix: "start",
    inherited: spacing,
    field_name: start,
    variants: []
});

define_tailwind_field!({
    name: End,
    prefix: "end",
    inherited: spacing,
    field_name: end,
    variants: []
});

define_tailwind_field!({
    name: Inset,
    prefix: "inset",
    inherited: spacing,
    field_name: inset,
    variants: ["x", "y"]
});

define_tailwind_field!({
    name: Visibility,
    prefix: "visibility",
    inherited: visibility,
    field_name: visibility,
    variants: []
});

// z-index
define_tailwind_field!({
    name: ZIndex,
    prefix: "z",
    inherited: z_index,
    field_name: z_index,
    variants: []
});

// 2. Flexbox & Grid
// Flex Basis
// Flex Direction
// Flex Wrap
// Flex
// Flex Grow
// Flex Shrink
// Order
// Grid Template Columns
// Grid Column Start / End
// Grid Template Rows
// Grid Row Start / End
// Grid Auto Flow
// Grid Auto Columns
// Grid Auto Rows
// Gap
// Justify Content
// Justify Items
// Justify Self
// Align Content
// Align Items
// Align Self
// Place Content
// Place Items
// Place Self
define_tailwind_field!({
    name: FlexBasis,
    prefix: "flex",
    inherited: flex_basis,
    field_name: flex_basis,
    variants: []
});

define_tailwind_field!({
    name: FlexDirection,
    prefix: "flex",
    inherited: flex_direction,
    field_name: flex_direction,
    variants: []
});

define_tailwind_field!({
    name: FlexWrap,
    prefix: "flex",
    inherited: flex_wrap,
    field_name: flex_wrap,
    variants: []
});

define_tailwind_field!({
    name: Flex,
    prefix: "flex",
    inherited: flex,
    field_name: flex,
    variants: []
});

define_tailwind_field!({
    name: FlexGrow,
    prefix: "flex",
    inherited: flex_grow,
    field_name: flex_grow,
    variants: []
});

define_tailwind_field!({
    name: FlexShrink,
    prefix: "flex",
    inherited: flex_shrink,
    field_name: flex_shrink,
    variants: []
});

define_tailwind_field!({
    name: Grow,
    prefix: "grow",
    inherited: flex_grow,
    field_name: flex_grow,
    variants: []
});

define_tailwind_field!({
    name: Shrink,
    prefix: "shrink",
    inherited: flex_shrink,
    field_name: flex_shrink,
    variants: []
});

define_tailwind_field!({
    name: Order,
    prefix: "order",
    inherited: order,
    field_name: order,
    variants: []
});

define_tailwind_field!({
    name: GridTemplateColumns,
    prefix: "grid-cols",
    inherited: grid_template_columns,
    field_name: grid_template_columns,
    variants: []
});

define_tailwind_field!({
    name: GridColumn,
    prefix: "col",
    inherited: grid_column,
    field_name: grid_column,
    variants: []
});

define_tailwind_field!({
    name: GridColumnStart,
    prefix: "col-start",
    inherited: grid_column_start,
    field_name: grid_column_start,
    variants: []
});

define_tailwind_field!({
    name: GridColumnEnd,
    prefix: "col-end",
    inherited: grid_column_end,
    field_name: grid_column_end,
    variants: []
});

define_tailwind_field!({
    name: GridTemplateRows,
    prefix: "grid-rows",
    inherited: grid_template_rows,
    field_name: grid_template_rows,
    variants: []
});

define_tailwind_field!({
    name: GridRow,
    prefix: "row",
    inherited: grid_row,
    field_name: grid_row,
    variants: []
});

define_tailwind_field!({
    name: GridRowStart,
    prefix: "row-start",
    inherited: grid_row_start,
    field_name: grid_row_start,
    variants: []
});

define_tailwind_field!({
    name: GridRowEnd,
    prefix: "row-end",
    inherited: grid_row_end,
    field_name: grid_row_end,
    variants: []
});

define_tailwind_field!({
    name: GridAutoFlow,
    prefix: "grid-flow",
    inherited: grid_auto_flow,
    field_name: grid_auto_flow,
    variants: []
});

define_tailwind_field!({
    name: GridAutoColumns,
    prefix: "auto-cols",
    inherited: grid_auto_columns,
    field_name: grid_auto_columns,
    variants: []
});

define_tailwind_field!({
    name: GridAutoRows,
    prefix: "auto-rows",
    inherited: grid_auto_rows,
    field_name: grid_auto_rows,
    variants: []
});

define_tailwind_field!({
    name: Gap,
    prefix: "gap",
    inherited: gap,
    field_name: gap,
    variants: []
});

define_tailwind_field!({
    name: JustifyContent,
    prefix: "justify",
    inherited: justify_content,
    field_name: justify_content,
    variants: []
});

define_tailwind_field!({
    name: JustifyItems,
    prefix: "justify-items",
    inherited: justify_items,
    field_name: justify_items,
    variants: []
});

define_tailwind_field!({
    name: JustifySelf,
    prefix: "justify-self",
    inherited: justify_self,
    field_name: justify_self,
    variants: []
});

define_tailwind_field!({
    name: AlignContent,
    prefix: "align-content",
    inherited: align_content,
    field_name: align_content,
    variants: []
});

define_tailwind_field!({
    name: AlignItems,
    prefix: "align-items",
    inherited: align_items,
    field_name: align_items,
    variants: []
});

define_tailwind_field!({
    name: AlignSelf,
    prefix: "align-self",
    inherited: align_self,
    field_name: align_self,
    variants: []
});

define_tailwind_field!({
    name: PlaceContent,
    prefix: "place-content",
    inherited: place_content,
    field_name: place_content,
    variants: []
});

define_tailwind_field!({
    name: PlaceItems,
    prefix: "place-items",
    inherited: place_items,
    field_name: place_items,
    variants: []
});

define_tailwind_field!({
    name: PlaceSelf,
    prefix: "place-self",
    inherited: place_self,
    field_name: place_self,
    variants: []
});

// 3. Spacing
// Padding
// Margin
// Space Between
define_tailwind_field!({
    name: Padding,
    prefix: "p",
    inherited: spacing,
    field_name: padding,
    variants: ["x", "y", "t", "r", "b", "l"]
});

define_tailwind_field!({
    name: Margin,
    prefix: "m",
    inherited: spacing,
    field_name: margin,
    variants: ["x", "y", "t", "r", "b", "l"]
});

define_tailwind_field!({
    name: SpaceBetween,
    prefix: "space",
    inherited: spacing,
    field_name: space,
    variants: ["x", "y"]
});

// 4. Sizing
// Width
// Min-Width
// Max-Width
// Height
// Min-Height
// Max-Height
define_tailwind_field!({
    name: Width,
    prefix: "w",
    inherited: spacing,
    field_name: width,
    variants: []
});

define_tailwind_field!({
    name: MinWidth,
    prefix: "min-w",
    inherited: min_width,
    field_name: min_width,
    variants: []
});

define_tailwind_field!({
    name: MaxWidth,
    prefix: "max-w",
    inherited: max_width,
    field_name: max_width,
    variants: []
});

define_tailwind_field!({
    name: Height,
    prefix: "h",
    inherited: spacing,
    field_name: height,
    variants: []
});

define_tailwind_field!({
    name: MinHeight,
    prefix: "min-h",
    inherited: min_height,
    field_name: min_height,
    variants: []
});

define_tailwind_field!({
    name: MaxHeight,
    prefix: "max-h",
    inherited: spacing,
    field_name: max_height,
    variants: []
});

// 5. Typography
// Font Family
// Font Size
// Font Smoothing
// Font Style
// Font Weight
// Font Variant Numeric
// Letter Spacing
// Line Clamp
// Line Height
// List Style Image
// List Style Position
// List Style Type
// Text Align
// Text Color
// Text Decoration
// Text Decoration Color
// Text Decoration Style
// Text Decoration Thickness
// Text Underline Offset
// Text Transform
// Text Overflow
// Text Indent
// Vertical Align
// Whitespace
// Word Break
// Hyphens
// Content
define_tailwind_field!({
    name: FontFamily,
    prefix: "font",
    inherited: font_family,
    field_name: font_family,
    variants: []
});

define_tailwind_field!({
    name: FontSize,
    prefix: "text",
    inherited: font_size,
    field_name: font_size,
    variants: []
});

define_tailwind_field!({
    name: FontSmoothing,
    prefix: "",
    inherited: font_smoothing,
    field_name: font_smoothing,
    variants: []
});

define_tailwind_field!({
    name: FontStyle,
    prefix: "",
    inherited: font_style,
    field_name: font_style,
    variants: []
});

define_tailwind_field!({
    name: FontWeight,
    prefix: "font",
    inherited: font_weight,
    field_name: font_weight,
    variants: []
});

define_tailwind_field!({
    name: FontVariantNumeric,
    prefix: "",
    inherited: font_variant_numeric,
    field_name: font_variant_numeric,
    variants: []
});

define_tailwind_field!({
    name: LetterSpacing,
    prefix: "tracking",
    inherited: letter_spacing,
    field_name: letter_spacing,
    variants: []
});

define_tailwind_field!({
    name: LineClamp,
    prefix: "line-clamp",
    inherited: line_clamp,
    field_name: line_clamp,
    variants: []
});

define_tailwind_field!({
    name: LineHeight,
    prefix: "leading",
    inherited: line_height,
    field_name: line_height,
    variants: []
});

define_tailwind_field!({
    name: ListStyleImage,
    prefix: "list-image",
    inherited: list_style_image,
    field_name: list_style_image,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: ListStylePosition,
    prefix: "list",
    inherited: list_style_position,
    field_name: list_style_position,
    variants: []
});

define_tailwind_field!({
    name: ListStyleType,
    prefix: "list",
    inherited: list_style_type,
    field_name: list_style_type,
    variants: []
});

define_tailwind_field!({
    name: TextAlign,
    prefix: "text",
    inherited: text_align,
    field_name: text_align,
    variants: []
});

define_tailwind_color_field!({
    name: TextColor,
    prefix: "text",
    // inherited: text_color,
    field_name: text_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TextDecoration,
    prefix: "",
    inherited: text_decoration,
    field_name: text_decoration,
    variants: []
});

define_tailwind_color_field!({
    name: TextDecorationColor,
    prefix: "decoration",
    field_name: text_decoration_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TextDecorationStyle,
    prefix: "decoration",
    inherited: text_decoration_style,
    field_name: text_decoration_style,
    variants: []
});

define_tailwind_field!({
    name: TextDecorationThickness,
    prefix: "decoration",
    inherited: text_decoration_thickness,
    field_name: text_decoration_thickness,
    variants: []
});

define_tailwind_field!({
    name: TextUnderlineOffset,
    prefix: "underline-offset",
    inherited: text_underline_offset,
    field_name: text_underline_offset,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TextTransform,
    prefix: "",
    inherited: text_transform,
    field_name: text_transform,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TextOverflow,
    prefix: "",
    inherited: text_overflow,
    field_name: text_overflow,
    variants: []
});

define_tailwind_field!({
    name: TextIndent,
    prefix: "indent",
    inherited: spacing,
    field_name: text_indent,
    variants: []
});

define_tailwind_field!({
    name: VerticalAlign,
    prefix: "align",
    inherited: vertical_align,
    field_name: vertical_align,
    variants: []
});

define_tailwind_field!({
    name: Whitespace,
    prefix: "whitespace",
    inherited: whitespace,
    field_name: whitespace,
    variants: []
});

define_tailwind_field!({
    name: WordBreak,
    prefix: "break",
    inherited: word_break,
    field_name: word_break,
    variants: []
});

define_tailwind_field!({
    name: Hyphens,
    prefix: "hyphens",
    inherited: hyphens,
    field_name: hyphens,
    variants: []
});

define_tailwind_field!({
    name: Content,
    prefix: "content",
    inherited: content,
    field_name: content,
    variants: []
});

// 6. Backgrounds
// Background Attachment
// Background Clip
// Background Color
// Background Origin
// Background Position
// Background Repeat
// Background Size
// Background Image
// Background Color Stops
define_tailwind_field!({
    name: BackgroundAttachment,
    prefix: "bg",
    inherited: background_attachment,
    field_name: background_attachment,
    variants: []
});

define_tailwind_field!({
    name: BackgroundClip,
    prefix: "bg-clip",
    inherited: background_clip,
    field_name: background_clip,
    variants: []
});

define_tailwind_color_field!({
    name: BackgroundColor,
    prefix: "bg",
    field_name: background_color,
    variants: []
});

define_tailwind_field!({
    name: BackgroundOrigin,
    prefix: "bg-origin",
    inherited: background_origin,
    field_name: background_origin,
    variants: []
});

define_tailwind_field!({
    name: BackgroundPosition,
    prefix: "bg",
    inherited: background_position,
    field_name: background_position,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: BackgroundRepeat,
    prefix: "bg",
    inherited: background_repeat,
    field_name: background_repeat,
    variants: []
});

define_tailwind_field!({
    name: BackgroundSize,
    prefix: "bg",
    inherited: background_size,
    field_name: background_size,
    variants: []
});

define_tailwind_field!({
    name: BackgroundImage,
    prefix: "bg",
    inherited: background_image,
    field_name: background_image,
    variants: []
});

/*
* Alternatively, you can customize just your gradient colors by editing theme.gradientColorStops or theme.extend.gradientColorStops in your tailwind.config.js file.

In addition to the colors, you can also customize the gradient color stop positions by editing theme.gradientColorStopPositions or theme.extend.gradientColorStopPositions.
// https://tailwindcss.com/docs/gradient-color-stops
* */
define_tailwind_color_field!({
    name: GradientColorStopsFrom,
    prefix: "from",
    field_name: gradient_color_stops,
    variants: []
});

define_tailwind_color_field!({
    name: GradientColorStopsVia,
    prefix: "via",
    field_name: gradient_color_stops,
    variants: []
});

define_tailwind_color_field!({
    name: GradientColorStopsTo,
    prefix: "to",
    field_name: gradient_color_stops,
    variants: []
});

// 7. Borders
// Border Radius
// Border Width
// Border Color
// Border Style
// Divide Width
// Divide Color
// Divide Style
// Outline Width
// Outline Color
// Outline Style
// Outline Offset
// Ring Width
// Ring Color
// Ring Offset Width
// Ring Offset Color
define_tailwind_field!({
    name: BorderRadius,
    prefix: "rounded",
    inherited: border_radius,
    field_name: border_radius,
    variants: ["t", "r", "b", "l", "tl", "tr", "br", "bl", "s", "e", "ss", "se", "es", "ee"]
});

define_tailwind_field!({
    name: BorderWidth,
    prefix: "border",
    inherited: border_width,
    field_name: border_width,
    variants: ["x", "y", "t", "r", "b","l", "s", "e"]
});

define_tailwind_color_field!({
    name: BorderColor,
    prefix: "border",
    field_name: border_color,
    variants: ["x", "y", "t", "r", "b","l", "s", "e"]
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: BorderStyle,
    prefix: "border",
    inherited: border_style,
    field_name: border_style,
    variants: []
});

define_tailwind_field!({
    name: DivideWidth,
    prefix: "divide",
    // It inherits the border-width
    inherited: border_width,
    field_name: divide_width,
    variants: ["x", "y"]
});

define_tailwind_color_field!({
    name: DivideColor,
    prefix: "divide",
    field_name: divide_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: DivideStyle,
    prefix: "divide",
    inherited: divide_style,
    field_name: divide_style,
    variants: []
});

define_tailwind_field!({
    name: OutlineWidth,
    prefix: "outline",
    inherited: outline_width,
    field_name: outline_width,
    variants: []
});

define_tailwind_color_field!({
    name: OutlineColor,
    prefix: "outline",
    field_name: outline_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: OutlineStyle,
    prefix: "outline",
    inherited: outline_style,
    field_name: outline_style,
    variants: []
});

define_tailwind_field!({
    name: OutlineOffset,
    prefix: "outline-offset",
    inherited: outline_offset,
    field_name: outline_offset,
    variants: []
});

define_tailwind_field!({
    name: RingWidth,
    prefix: "ring",
    inherited: ring_width,
    field_name: ring_width,
    variants: []
});

define_tailwind_color_field!({
    name: RingColor,
    prefix: "ring",
    field_name: ring_color,
    variants: []
});

define_tailwind_field!({
    name: RingOffsetWidth,
    prefix: "ring-offset",
    inherited: ring_offset_width,
    field_name: ring_offset_width,
    variants: []
});

define_tailwind_color_field!({
    name: RingOffsetColor,
    prefix: "ring-offset",
    field_name: ring_offset_color,
    variants: []
});

// 8. Effects
// Box Shadow
// Box Shadow Color
// Opacity
// Mix Blend Mode
// Background Blend Mode
define_tailwind_field!({
    name: BoxShadow,
    prefix: "shadow",
    inherited: box_shadow,
    field_name: box_shadow,
    variants: []
});

define_tailwind_color_field!({
    name: BoxShadowColor,
    prefix: "shadow",
    field_name: box_shadow_color,
    variants: []
});

define_tailwind_field!({
    name: Opacity,
    prefix: "opacity",
    inherited: opacity,
    field_name: opacity,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: MixBlendMode,
    prefix: "mix-blend",
    inherited: mix_blend_mode,
    field_name: mix_blend_mode,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: BackgroundBlendMode,
    prefix: "bg-blend",
    inherited: background_blend_mode,
    field_name: background_blend_mode,
    variants: []
});

// 9. Filters
// Blur
// Brightness
// Contrast
// Drop Shadow
// Grayscale
// Hue Rotate
// Invert
// Saturate
// Sepia
// Backdrop Blur
// Backdrop Brightness
// Backdrop Contrast
// Backdrop Grayscale
// Backdrop Hue Rotate
// Backdrop Invert
// Backdrop Opacity
// Backdrop Saturate
// Backdrop Sepia
define_tailwind_field!({
    name: Blur,
    prefix: "blur",
    inherited: blur,
    field_name: blur,
    variants: []
});

define_tailwind_field!({
    name: Brightness,
    prefix: "brightness",
    inherited: brightness,
    field_name: brightness,
    variants: []
});

define_tailwind_field!({
    name: Contrast,
    prefix: "contrast",
    inherited: contrast,
    field_name: contrast,
    variants: []
});

define_tailwind_field!({
    name: DropShadow,
    prefix: "drop-shadow",
    inherited: drop_shadow,
    field_name: drop_shadow,
    variants: []
});

define_tailwind_field!({
    name: Grayscale,
    prefix: "grayscale",
    inherited: grayscale,
    field_name: grayscale,
    variants: []
});

define_tailwind_field!({
    name: HueRotate,
    prefix: "hue-rotate",
    inherited: hue_rotate,
    field_name: hue_rotate,
    variants: []
});

define_tailwind_field!({
    name: Invert,
    prefix: "invert",
    inherited: invert,
    field_name: invert,
    variants: []
});

define_tailwind_field!({
    name: Saturate,
    prefix: "saturate",
    inherited: saturate,
    field_name: saturate,
    variants: []
});

define_tailwind_field!({
    name: Sepia,
    prefix: "sepia",
    inherited: sepia,
    field_name: sepia,
    variants: []
});

define_tailwind_field!({
    name: BackdropBlur,
    prefix: "backdrop-blur",
    inherited: backdrop_blur,
    field_name: backdrop_blur,
    variants: []
});

define_tailwind_field!({
    name: BackdropBrightness,
    prefix: "backdrop-brightness",
    inherited: backdrop_brightness,
    field_name: backdrop_brightness,
    variants: []
});

define_tailwind_field!({
    name: BackdropContrast,
    prefix: "backdrop-contrast",
    inherited: backdrop_contrast,
    field_name: backdrop_contrast,
    variants: []
});

define_tailwind_field!({
    name: BackdropGrayscale,
    prefix: "backdrop-grayscale",
    inherited: backdrop_grayscale,
    field_name: backdrop_grayscale,
    variants: []
});

define_tailwind_field!({
    name: BackdropHueRotate,
    prefix: "backdrop-hue-rotate",
    inherited: backdrop_hue_rotate,
    field_name: backdrop_hue_rotate,
    variants: []
});

define_tailwind_field!({
    name: BackdropInvert,
    prefix: "backdrop-invert",
    inherited: backdrop_invert,
    field_name: backdrop_invert,
    variants: []
});

define_tailwind_field!({
    name: BackdropOpacity,
    prefix: "backdrop-opacity",
    inherited: backdrop_opacity,
    field_name: backdrop_opacity,
    variants: []
});

define_tailwind_field!({
    name: BackdropSaturate,
    prefix: "backdrop-saturate",
    inherited: backdrop_saturate,
    field_name: backdrop_saturate,
    variants: []
});

define_tailwind_field!({
    name: BackdropSepia,
    prefix: "backdrop-sepia",
    inherited: backdrop_sepia,
    field_name: backdrop_sepia,
    variants: []
});

// 10. Tables
// Border Collapse
// Border Spacing
// Table Layout
// Caption Side
// unchangeable and unconfigurable
define_tailwind_field!({
    name: BorderCollapse,
    prefix: "",
    inherited: border_collapse,
    field_name: border_collapse,
    variants: []
});

define_tailwind_field!({
    name: BorderSpacing,
    prefix: "border-spacing",
    inherited: spacing,
    field_name: border_spacing,
    variants: ["x", "y"]
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TableLayout,
    prefix: "table",
    inherited: table_layout,
    field_name: table_layout,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: CaptionSide,
    prefix: "caption",
    inherited: caption_side,
    field_name: caption_side,
    variants: []
});

// 11. Transitions and Animation
// Transition Property
// Transition Duration
// Transition Timing Function
// Transition delay
// Animation
define_tailwind_field!({
    name: TransitionProperty,
    prefix: "transition",
    inherited: transition_property,
    field_name: transition_property,
    variants: []
});

define_tailwind_field!({
    name: TransitionDuration,
    prefix: "duration",
    inherited: transition_duration,
    field_name: transition_duration,
    variants: []
});

define_tailwind_field!({
    name: TransitionTimingFunction,
    prefix: "ease",
    inherited: transition_timing_function,
    field_name: transition_timing_function,
    variants: []
});

define_tailwind_field!({
    name: TransitionDelay,
    prefix: "delay",
    inherited: transition_delay,
    field_name: transition_delay,
    variants: []
});

define_tailwind_field!({
    name: Animation,
    prefix: "animate",
    inherited: animation,
    field_name: animation,
    variants: []
});

// 12. Transforms
// Scale
// Rotate
// Translate
// Skew
// Transform Origin
define_tailwind_field!({
    name: Scale,
    prefix: "scale",
    inherited: scale,
    field_name: scale,
    variants: ["x", "y"]
});

define_tailwind_field!({
    name: Rotate,
    prefix: "rotate",
    inherited: rotate,
    field_name: rotate,
    variants: []
});

define_tailwind_field!({
    name: Translate,
    prefix: "translate",
    inherited: spacing,
    field_name: translate,
    variants: ["x", "y"]
});

define_tailwind_field!({
    name: Skew,
    prefix: "skew",
    inherited: skew,
    field_name: skew,
    variants: ["x", "y"]
});

define_tailwind_field!({
    name: TransformOrigin,
    prefix: "origin",
    inherited: transform_origin,
    field_name: transform_origin,
    variants: []
});

// 13. Interactivity
// Accent Color
// Appearance
// Cursor
// Caret Color
// Pointer Events
// Resize
// Scroll Behavior
// Scroll Margin
// Scroll Padding
// Scroll Snap Align
// Scroll Snap Stop
// Scroll Snap Type
// Touch Action
// User Select
// Will Change
define_tailwind_color_field!({
    name: AccentColor,
    prefix: "accent",
    field_name: accent_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: Appearance,
    prefix: "appearance",
    inherited: appearance,
    field_name: appearance,
    variants: []
});

define_tailwind_field!({
    name: Cursor,
    prefix: "cursor",
    inherited: cursor,
    field_name: cursor,
    variants: []
});

define_tailwind_color_field!({
    name: CaretColor,
    prefix: "caret",
    field_name: caret_color,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: PointerEvents,
    prefix: "pointer-events",
    inherited: pointer_events,
    field_name: pointer_events,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: Resize,
    prefix: "resize",
    inherited: resize,
    field_name: resize,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: ScrollBehavior,
    prefix: "scroll",
    inherited: scroll_behavior,
    field_name: scroll_behavior,
    variants: []
});

define_tailwind_field!({
    name: ScrollMargin,
    prefix: "scroll-m",
    inherited: spacing,
    field_name: scroll_margin,
    variants: ["x", "y", "s", "e", "t", "r", "b", "l"]
});

define_tailwind_field!({
    name: ScrollPadding,
    prefix: "scroll-p",
    inherited: spacing,
    field_name: scroll_padding,
    variants: ["x", "y", "s", "e", "t", "r", "b", "l"]
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: ScrollSnapAlign,
    prefix: "snap",
    inherited: scroll_snap_align,
    field_name: scroll_snap_align,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: ScrollSnapStop,
    prefix: "snap",
    inherited: scroll_snap_stop,
    field_name: scroll_snap_stop,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: ScrollSnapType,
    prefix: "snap",
    inherited: scroll_snap_type,
    field_name: scroll_snap_type,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: TouchAction,
    prefix: "touch",
    inherited: touch_action,
    field_name: touch_action,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: UserSelect,
    prefix: "select",
    inherited: user_select,
    field_name: user_select,
    variants: []
});

// unchangeable and unconfigurable
define_tailwind_field!({
    name: WillChange,
    prefix: "will-change",
    inherited: will_change,
    field_name: will_change,
    variants: []
});

// 14. SVG
// Fill
// Stroke
// Stroke Width
define_tailwind_color_field!({
    name: FillColor,
    prefix: "fill",
    field_name: fill,
    variants: []
});

define_tailwind_color_field!({
    name: StrokeColor,
    prefix: "stroke",
    field_name: stroke,
    variants: []
});

define_tailwind_field!({
    name: StrokeWidth,
    prefix: "stroke",
    inherited: stroke_width,
    field_name: stroke_width,
    variants: []
});

// 15. Accessibility
define_tailwind_field!({
    name: ScreenReaders,
    prefix: "",
    inherited: screen_readers,
    field_name: screen_readers,
    variants: []
});
