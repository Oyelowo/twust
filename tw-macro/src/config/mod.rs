/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
mod classes;
mod macros;
pub mod modifiers;
pub mod noconfig;
// use crate::plugins::daisyui::DAISY_UI_CLASSES;
// use crate::plugins::DAISY_UI_CLASSES;
use crate::plugins::daisyui;
use crate::tailwind::tailwind_config::TailwindConfig;
use std::fs;

use self::classes::*;

pub trait TailwindField {
    fn get_prefix(&self) -> &'static str;
    fn get_variants(&self) -> Vec<&'static str>;
    fn get_default(&self, config: &TailwindConfig) -> Vec<&str>;
    fn get_override(&self, config: &TailwindConfig) -> Vec<String>;
    fn get_extend(&self, config: &TailwindConfig) -> Vec<String>;
    fn handle_special_cases(&self, config: &TailwindConfig) -> Vec<String>;
}

// Put it all together
fn generate_classes_for_keys(field: &dyn TailwindField, keys: &[String]) -> Vec<String> {
    let mut classes = Vec::new();
    let variants = field.get_variants();
    let prefix = field.get_prefix();

    for key in keys.iter() {
        // e.g bg-red
        classes.push(format!("{prefix}-{key}"));
        for variant in variants.iter() {
            // e.g border-t-red, border-r-red-500,
            classes.push(format!("{prefix}{variant}-{key}"));
        }
    }

    classes
}

pub fn add_classes_for_field(
    field: &dyn TailwindField,
    config: &TailwindConfig,
    classes: &mut Vec<String>,
) {
    let overrides = field.get_override(config);
    if !overrides.is_empty() {
        classes.extend(generate_classes_for_keys(field, &overrides));
    } else {
        let default = field.get_default(config);
        classes.extend(default.iter().map(|x| x.to_string()));
    }
    let extend = field.get_extend(config);
    classes.extend(generate_classes_for_keys(field, &extend));
    classes.extend(field.handle_special_cases(config));
}

pub(crate) fn read_tailwind_config() -> Result<TailwindConfig, std::io::Error> {
    let current_dir = std::env::current_dir()?;

    // Construct the path to tailwind.config.json relative to the current directory
    // typically, top-level directory.
    let config_path = current_dir.join("tailwind.config.json");

    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "tailwind.config.json was not found in the top-level directory - \n{config_path:?}. Ensure it exists."
            ),
        ));
    }

    let content = fs::read_to_string(config_path)?;
    let config: TailwindConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn get_classes(config: &TailwindConfig) -> Vec<String> {
    // Check that config overrides keys are not one of this list, as those cannot be set
    // by the user.

    let mut classes = Vec::new();
    let utilities: [Box<dyn TailwindField>; 175] = [
        Box::new(AspectRatio),
        Box::new(Container),
        Box::new(Columns),
        Box::new(BreakAfter),
        Box::new(BreakBefore),
        Box::new(BreakInside),
        Box::new(BoxDecorationBreak),
        Box::new(BoxSizing),
        Box::new(Display),
        Box::new(Float),
        Box::new(Clear),
        Box::new(Isolation),
        Box::new(ObjectFit),
        Box::new(ObjectPosition),
        Box::new(Overflow),
        Box::new(OverscrollBehavior),
        Box::new(Position),
        Box::new(Top),
        Box::new(Right),
        Box::new(Bottom),
        Box::new(Left),
        Box::new(Start),
        Box::new(End),
        Box::new(Inset),
        Box::new(Visibility),
        Box::new(ZIndex),
        Box::new(FlexBasis),
        Box::new(FlexDirection),
        Box::new(FlexWrap),
        Box::new(Flex),
        Box::new(FlexGrow),
        Box::new(FlexShrink),
        Box::new(Grow),
        Box::new(Shrink),
        Box::new(Order),
        Box::new(GridTemplateColumns),
        Box::new(GridColumn),
        Box::new(GridColumnStart),
        Box::new(GridColumnEnd),
        Box::new(GridTemplateRows),
        Box::new(GridRow),
        Box::new(GridRowStart),
        Box::new(GridRowEnd),
        Box::new(GridAutoFlow),
        Box::new(GridAutoColumns),
        Box::new(GridAutoRows),
        Box::new(Gap),
        Box::new(JustifyContent),
        Box::new(JustifyItems),
        Box::new(JustifySelf),
        Box::new(AlignContent),
        Box::new(AlignItems),
        Box::new(AlignSelf),
        Box::new(PlaceContent),
        Box::new(PlaceItems),
        Box::new(PlaceSelf),
        Box::new(Padding),
        Box::new(Margin),
        Box::new(SpaceBetween),
        Box::new(Width),
        Box::new(MinWidth),
        Box::new(MaxWidth),
        Box::new(Height),
        Box::new(MinHeight),
        Box::new(MaxHeight),
        Box::new(FontFamily),
        Box::new(FontSize),
        Box::new(FontSmoothing),
        Box::new(FontStyle),
        Box::new(FontWeight),
        Box::new(FontVariantNumeric),
        Box::new(LetterSpacing),
        Box::new(LineClamp),
        Box::new(LineHeight),
        Box::new(ListStyleImage),
        Box::new(ListStylePosition),
        Box::new(ListStyleType),
        Box::new(TextAlign),
        Box::new(TextColor),
        Box::new(TextDecoration),
        Box::new(TextDecorationColor),
        Box::new(TextDecorationStyle),
        Box::new(TextDecorationThickness),
        Box::new(TextUnderlineOffset),
        Box::new(TextTransform),
        Box::new(TextOverflow),
        Box::new(TextIndent),
        Box::new(VerticalAlign),
        Box::new(Whitespace),
        Box::new(WordBreak),
        Box::new(Hyphens),
        Box::new(Content),
        Box::new(BackgroundAttachment),
        Box::new(BackgroundClip),
        Box::new(BackgroundColor),
        Box::new(BackgroundOrigin),
        Box::new(BackgroundPosition),
        Box::new(BackgroundRepeat),
        Box::new(BackgroundSize),
        Box::new(BackgroundImage),
        Box::new(GradientColorStopsFrom),
        Box::new(GradientColorStopsVia),
        Box::new(GradientColorStopsTo),
        Box::new(BorderRadius),
        Box::new(BorderWidth),
        Box::new(BorderColor),
        Box::new(BorderStyle),
        Box::new(BorderOpacity),
        Box::new(DivideWidth),
        Box::new(DivideColor),
        Box::new(DivideStyle),
        Box::new(OutlineWidth),
        Box::new(OutlineColor),
        Box::new(OutlineStyle),
        Box::new(OutlineOffset),
        Box::new(RingWidth),
        Box::new(RingColor),
        Box::new(RingOffsetWidth),
        Box::new(RingOffsetColor),
        Box::new(BoxShadow),
        Box::new(BoxShadowColor),
        Box::new(Opacity),
        Box::new(MixBlendMode),
        Box::new(BackgroundBlendMode),
        Box::new(Blur),
        Box::new(Brightness),
        Box::new(Contrast),
        Box::new(DropShadow),
        Box::new(Grayscale),
        Box::new(HueRotate),
        Box::new(Invert),
        Box::new(Saturate),
        Box::new(Sepia),
        Box::new(BackdropBlur),
        Box::new(BackdropBrightness),
        Box::new(BackdropContrast),
        Box::new(BackdropGrayscale),
        Box::new(BackdropHueRotate),
        Box::new(BackdropInvert),
        Box::new(BackdropOpacity),
        Box::new(BackdropSaturate),
        Box::new(BackdropSepia),
        Box::new(BorderCollapse),
        Box::new(BorderSpacing),
        Box::new(TableLayout),
        Box::new(CaptionSide),
        Box::new(TransitionProperty),
        Box::new(TransitionDuration),
        Box::new(TransitionTimingFunction),
        Box::new(TransitionDelay),
        Box::new(Animation),
        Box::new(Scale),
        Box::new(Rotate),
        Box::new(Translate),
        Box::new(Skew),
        Box::new(TransformOrigin),
        Box::new(AccentColor),
        Box::new(Appearance),
        Box::new(Cursor),
        Box::new(CaretColor),
        Box::new(PointerEvents),
        Box::new(Resize),
        Box::new(ScrollBehavior),
        Box::new(ScrollMargin),
        Box::new(ScrollPadding),
        Box::new(ScrollSnapAlign),
        Box::new(ScrollSnapStop),
        Box::new(ScrollSnapType),
        Box::new(TouchAction),
        Box::new(UserSelect),
        Box::new(WillChange),
        Box::new(FillColor),
        Box::new(StrokeColor),
        Box::new(StrokeWidth),
        Box::new(ScreenReaders),
    ];

    for utility in utilities {
        add_classes_for_field(utility.as_ref(), config, &mut classes);
    }

    let allowed_extra_classes = config
        .allowed_lists
        .as_ref()
        .and_then(|x| x.classes.to_owned())
        .unwrap_or_default();

    classes.extend(allowed_extra_classes);
    classes.push("group".to_string());

    classes.extend(daisyui::get_it().iter().map(ToString::to_string));

    classes
}
