/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2025 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// #[cfg(feature = "daisyui")]
pub use twust_macro::{twust_many_classes, twust_one_class};
/// Typechecks Tailwind CSS class names at compile time.
///
/// ## Features:
/// - Supports **single string literals** (`tw!("class1 class2")`).
/// - Supports **multiple string arguments** (`tw!["class1", "class2"]`).
///
/// ## Example Usage
///
/// ```rust
/// use twust::tw;
///
/// // Single class string
/// let single_class = tw!("scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
///
/// // Multiple class strings
/// let multiple_classes = tw!["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"];
///
/// assert_eq!(single_class, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// assert_eq!(multiple_classes, "scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
/// ```
///
/// ## Notes:
/// - The macro supports **both** `tw!(...)` and `tw![...]` syntax.
/// - It ensures at compile time that only **valid Tailwind class names** are used.
/// - The macro automatically **removes trailing spaces** to ensure consistent output.
#[macro_export]
macro_rules! tw {
    ($single:literal) => {
        $crate::twust_many_classes!($single)
    };

    ($first:literal $(, $rest:literal)*) => {
        concat!($crate::twust_many_classes!($first), $(" ", $crate::twust_many_classes!($rest)),*)
    };
}

/// Typechecks a **single** Tailwind CSS class at compile time.
///
/// ## Features:
/// - **Accepts only one class at a time** (`tw1!("class-name")`).
/// - Ensures that the class is **valid at compile time**.
///
/// ## Example Usage
///
/// ```rust
/// use twust::tw1;
///
/// let class = tw1!("bg-red-500");
///
/// assert_eq!(class, "bg-red-500");
/// ```
///
/// ## Notes:
/// - Unlike `tw!`, `tw1!` **does not allow multiple classes**.
/// - Useful when enforcing single-class validation.
#[macro_export]
macro_rules! tw1 {
    ($class:literal) => {
        $crate::twust_one_class!($class)
    };
}

/// Typechecks Tailwind CSS class names at compile time and returns an **array** of class strings.
///
/// ## Features:
/// - Supports **multiple string arguments** (`tws!["class1", "class2"]`).
/// - Outputs a **Rust array of strings** (`["class1", "class2"]`).
///
/// ## Example Usage
///
/// ```rust
/// use twust::tws;
///
/// let class_list = tws!["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"];
///
/// assert_eq!(class_list, ["scroll-m-14", "flex", "supports-grid:grid", "supports-[display:grid]:grid"]);
/// ```
///
/// ## Notes:
/// - Unlike `tw!`, which returns a **single concatenated string**, `tws!` returns a **Rust array** of class names.
/// - Useful when working with frameworks or libraries that require separate class names instead of a single string.
#[macro_export]
macro_rules! tws {
    ($($class:literal),*) => {
        [$($crate::twust_many_classes!($class)),*]
    };
}

/// Typechecks multiple **single-class** Tailwind CSS names at compile time and returns an **array**.
///
/// ## Features:
/// - **Ensures each item is a single valid class**.
/// - Returns a **Rust array** (`["class1", "class2"]`).
///
/// ## Example Usage
///
/// ```rust
/// use twust::tws1;
///
/// let class_list = tws1!["bg-red-500", "text-lg", "p-4"];
///
/// assert_eq!(class_list, ["bg-red-500", "text-lg", "p-4"]);
/// ```
///
/// ## Notes:
/// - Unlike `tws!`, which allows compound classes like `"text-lg bg-red-500"`, `tws1!` **only allows one class per entry**.
/// - Useful for stricter **class validation** when working with UI frameworks.
#[macro_export]
macro_rules! tws1 {
    ($($class:literal),*) => {
        [$($crate::twust_one_class!($class)),*]
    };
}
