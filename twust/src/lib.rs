/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2025 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// #[cfg(feature = "daisyui")]
pub use twust_macro::{twust_one_class as tw1, twust_many_classes};
/// Typechecks tailwindcss classnames at compile time.
///
/// ## Features:
/// - Supports **single string** literals (`tw!("class1 class2")`)
/// - Supports **multiple string arguments** (`tw!["class1", "class2"]`)
/// - Supports **arrays of strings** (`tw!(["class1", "class2"])`)
///
///
/// ## Example Usage
///
/// ```rust, ignore
/// use twust::tw;
///
/// let single_class = tw!("scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// let multiple_classes = tw!["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"];
///
/// assert_eq!(single_class, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// assert_eq!(multiple_classes, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// ```
///
/// ## Notes
/// - The macro supports **both** `tw!(...)` and `tw![...]` syntax.
/// - It ensures at compile time that only string literals are used for class names.
#[macro_export]
macro_rules! tw {
    ($single:literal) => {
        $crate::twust_many_classes!($single)
    };

    ($first:literal $(, $rest:literal)*) => {
        concat!($crate::twust_many_classes!($first), $(" ", $crate::twust_many_classes!($rest)),*)
    };
}

// tws["scroll-m-14", "flex:]; // ["scroll-m-14", "flex"]
/// Typechecks tailwindcss classnames at compile time.
///
/// ## Features:
/// - Supports **multiple string arguments** (`tw!["class1", "class2"]`)
#[macro_export]
macro_rules! tws {
    ($($class:literal),*) => {
        [$($crate::twust_many_classes!($class)),*]
    };
}

#[macro_export]
macro_rules! tws1 {
    ($($class:literal),*) => {
        [$($crate::tw1!($class)),*]
    };
}
