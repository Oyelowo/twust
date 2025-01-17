/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2025 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub use twust_macro::tw as twust;
// // use twust_macro::xtw;
//
// #[macro_export]
// macro_rules! tw {
//     // Case: Single string
//     ($single:literal) => {
//         $single
//     };
//     ($($class:literal),*) => {
//         concat!($($class, " "),*)
//     };
//
//     ([$($class:expr),*]) => {
//         concat!($($class, " "),*)
//     };
// }
//
// fn main() {
//     let _classnames_single = tw!("scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
//     let _classnames_array = tw!["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"];
//     let _classnames_array2 = tw!(["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"]);
//     // let _classnames_array2 = twx!(["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"]);
//     // let _classnames_array2 = twx!(["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"]);
// }
//

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
/// let array_classes = tw!(["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"]);
///
/// assert_eq!(single_class, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// assert_eq!(multiple_classes, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// assert_eq!(array_classes, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// ```
///
/// ## Notes
/// - The macro supports **both** `tw!(...)` and `tw![...]` syntax.
/// - It ensures at compile time that only string literals are used for class names.
#[macro_export]
macro_rules! tw {
    ($single:literal) => {
        $single
    };

    ($($class:literal),*) => {
        concat!($($class, " "),*)
    };

    ([$($class:expr),*]) => {
        concat!($($class, " "),*)
    };
}
