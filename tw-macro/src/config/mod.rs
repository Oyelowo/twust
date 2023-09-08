mod classes;
mod macros;
mod spacings;
use crate::tailwind::tailwind_config::TailwindConfig;
use std::fs;

use self::classes::*;

trait TailwindField {
    fn get_prefix(&self) -> &'static str;
    fn get_variants(&self) -> Vec<&'static str>;
    fn get_default(&self) -> Vec<&str>;
    fn get_override(&self, config: &TailwindConfig) -> Vec<String>;
    fn get_extend(&self, config: &TailwindConfig) -> Vec<String>;
    fn handle_special_cases(&self, config: &TailwindConfig) -> Vec<String>;
}

// Put it all together
fn generate_classes_for_keys(field: &dyn TailwindField, keys: &Vec<String>) -> Vec<String> {
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

fn add_classes_for_field(
    field: &dyn TailwindField,
    config: &TailwindConfig,
    classes: &mut Vec<String>,
) {
    let overrides = field.get_override(&config);
    if !overrides.is_empty() {
        classes.extend(generate_classes_for_keys(field, &overrides));
    } else {
        let default = field.get_default();
        classes.extend(default.iter().map(|x| x.to_string()));
    }
    let extend = field.get_extend(&config);
    classes.extend(generate_classes_for_keys(field, &extend));
    classes.extend(field.handle_special_cases(&config));
}

fn read_tailwind_config() -> Result<TailwindConfig, std::io::Error> {
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

pub fn get_classes() -> Result<Vec<String>, std::io::Error> {
    let config = read_tailwind_config()?;
    let mut classes = Vec::new();
    let colors: [Box<dyn TailwindField>; 100] = [
        // Colors
        Box::new(AccentColor),
        Box::new(BackgroundColor),
        Box::new(BorderColor),
        Box::new(TextColor),
        Box::new(TextDecorationColor),
        Box::new(PlaceholderColor),
        Box::new(RingColor),
        Box::new(RingOffsetColor),
        Box::new(BoxShadowColor),
        Box::new(DivideColor),
        Box::new(OutlineColor),
        Box::new(FillColor),
        Box::new(StrokeColor),
        // Spacing
        Box::new(Padding),
        Box::new(Margin),
        Box::new(Width),
        Box::new(Height),
        Box::new(MaxHeight),
        Box::new(Gap),
        Box::new(Inset),
        Box::new(Translate),
        Box::new(TextIndent),
        Box::new(BorderSpacing),
        Box::new(ScrollMargin),
        Box::new(ScrollPadding),
        // Others
        Box::new(AspectRatio),
        Box::new(Columns),
        Box::new(ObjectPosition),
        Box::new(ZIndex),
        Box::new(Flex),
        Box::new(FlexGrow),
        Box::new(FlexShrink),
        Box::new(Order),
        Box::new(GridTemplateColumns),
        Box::new(GridColumn),
        Box::new(GridColumnStart),
        Box::new(GridColumnEnd),
        Box::new(GridTemplateRows),
        Box::new(GridRow),
        Box::new(GridRowStart),
        Box::new(GridRowEnd),
        Box::new(GridAutoColumns),
        Box::new(GridAutoRows),
        Box::new(MinWidth),
        Box::new(MaxWidth),
        Box::new(MinHeight),
        Box::new(PlaceholderOpacity),
        Box::new(FontFamily),
        Box::new(FontSize),
        Box::new(FontWeight),
        Box::new(LetterSpacing),
        Box::new(LineClamp),
        Box::new(LineHeight),
        Box::new(ListStyleImage),
        Box::new(ListStyleType),
        Box::new(TextDecorationThickness),
        Box::new(TextUnderlineOffset),
        Box::new(Content),
        Box::new(BackgroundPosition),
        Box::new(BackgroundSize),
        Box::new(BackgroundImage),
        Box::new(BorderRadius),
        Box::new(BorderWidth),
        Box::new(DivideWidth),
        Box::new(OutlineWidth),
        Box::new(OutlineOffset),
        Box::new(RingWidth),
        Box::new(RingOffsetWidth),
        Box::new(BoxShadow),
        Box::new(Opacity),
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
        Box::new(TransitionProperty),
        Box::new(TransitionDuration),
        Box::new(TransitionTimingFunction),
        Box::new(TransitionDelay),
        Box::new(Animation),
        Box::new(Scale),
        Box::new(Rotate),
        Box::new(Skew),
        Box::new(TransformOrigin),
        Box::new(Cursor),
        Box::new(WillChange),
        Box::new(StrokeWidth),
    ];

    for color in colors {
        add_classes_for_field(color.as_ref(), &config, &mut classes);
    }

    // add_classes_for_field(&AccentColor, &config, &mut classes);
    // add_classes_for_field(&BgColor, &config, &mut classes);
    // add_classes_for_field(&BorderColor, &config, &mut classes);
    // add_classes_for_field(&TextColor, &config, &mut classes);
    // add_classes_for_field(&TextDecorationColor, &config, &mut classes);
    // add_classes_for_field(&RingColor, &config, &mut classes);
    // add_classes_for_field(&RingOffsetColor, &config, &mut classes);
    // add_classes_for_field(&DivideColor, &config, &mut classes);
    // add_classes_for_field(&OutlineColor, &config, &mut classes);
    // add_classes_for_field(&FillColor, &config, &mut classes);
    // add_classes_for_field(&StrokeColor, &config, &mut classes);
    // add_classes_for_field(&PlaceholderColor, &config, &mut classes);

    Ok(classes)
}
