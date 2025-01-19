# `twust`
[Book](https://oyelowo.github.io/twust/configuration.html) | [doc.rs](https://docs.rs/twust/latest/twust/) | [Discord](https://discord.gg/xuA39Zpb)

Twust is a powerful static checker in Rust for TailwindCSS class names at compile-time. It ensures correctness by validating class names before runtime, preventing errors and improving developer experience.

<img width="1314" alt="Screenshot 2023-10-11 at 1 40 26" src="https://github.com/Oyelowo/twust/assets/31687368/881a37be-07be-462c-9952-f4897546dbf4">

<!--toc:start-->
- [`twust`](#twust)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [`tw!` - Compile-time Type-Checked Tailwind Classes](#tw-compile-time-type-checked-tailwind-classes)
    - [`tws!` - Compile-time Checked Array of Classes](#tws-compile-time-checked-array-of-classes)
    - [`tw1!` - Single Tailwind Class Only](#tw1-single-tailwind-class-only)
    - [`tws1!` - Array of Single-Class Items Only](#tws1-array-of-single-class-items-only)
  - [`tailwind.config.json` Overview](#tailwindconfigjson-overview)
    - [Basic Structure:](#basic-structure)
  - [Statement of Problem](#statement-of-problem)
  - [Solution](#solution)
  - [How does this compare with Other Rust Libraries?](#how-does-this-compare-with-other-rust-libraries)
    - [`tailwindcss-to-rust`](#tailwindcss-to-rust)
    - [`twust`](#twust)
  - [License](#license)
  - [⭐ Show Some Love!](#show-some-love)
<!--toc:end-->

---

## Features

- **Compile-time validation** of Tailwind CSS class names. Prevents typos and invalid Tailwind class names.
- **No runtime overhead** – errors are caught at compile time.
- **Supports single and multiple class formats.**
<!-- - **Custom error messages** for invalid Tailwind classes. -->
- **Supports DaisyU under a feature flag**.
- **Configurable: Supports overriding, extending, custom classes, custom modifiers, Plugins and many more**.
- **Flexible macro-based API**.
- **Returns class names as a string or an array** for easy use in UI frameworks.
- **Works seamlessly in Rust UI frameworks like Leptos, Dioxus, Yew, and Sycamore**.
- **Lightweight and blazing fast!**

---

## Installation

```toml
[dependencies]
twust = "1.0.7"
```

Enable optional features (e.g., `daisyui` support):

```toml
[dependencies]
twust = { version = "1.0.7", features = ["daisyui"] }
```

---

## Usage

### `tw!` - Compile-time Type-Checked Tailwind Classes

```rust
use twust::tw;

let class = tw!("bg-red-500 text-lg");
assert_eq!(class, "bg-red-500 text-lg");

// You can also separate classnames by space, these will be merged
let classes_list = tw!["bg-blue-500 hover:bg-blue-700", "bg-purple", "py-sm md:py-md tablet:py-sm lg:py-lg xl:py-xl"];
assert_eq!(classes_list, "bg-blue-500 hover:bg-blue-700 bg-purple py-sm md:py-md tablet:py-sm lg:py-lg xl:py-xl")

    // You can override/extend color/background color in tailwind.config.json
tw!("bg-taxvhiti bg-tahiti-500 bg-tahiti bg-midnight bg-purple bg-red-50 bg-tahiti-800 border-s-tahiti-800");
tw!("md:text-red-50 text-slate-50 text-purple text-tahiti-500");
tw!("py-sm md:py-md tablet:py-sm lg:py-lg xl:py-xl");
tw!("group");
tw!("hover:-translate-y-0.5 transition motion-reduce:hover:translate-y-0 motion-reduce:transition-none");
tw!("group/edit block invisible md:hover:bg-slate-200 group-hover/item:visible");
tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block");
tw!("scroll-m-15 group-aria-[sort=ascending]:rotate-0");

// Even scroll margin can also be configured, here we add, sm and md under the Spacing/scrollMargin field in the config file
tw!("scroll-mx-sm scroll-mx-md");
tw!("px-[-45px] px-[-45cm] px-[-45rem] px-[-45em] px-[-45%] px-[-45vh]");
tw!("m-4 last:first:invalid:last:first:p-4 last:m-4 pb-[calc(100%-34px)] pb-[23px] [mask-type:luminance]
    [mask-type:luminance] hover:[mask-type:alpha] lg:[--scroll-offset:44px] oyelowo oyedayo break-after-avoid"
);
tw!("h-full border-2 border-opacity-60 rounded-lg overflow-hidden");
```

### `tws!` - Compile-time Checked Array of Classes

```rust
use twust::tws;

let class_list = tws!["bg-red-500", "text-lg", "p-4"];
assert_eq!(class_list, ["bg-red-500", "text-lg", "p-4"]);
```

### `tw1!` - Single Tailwind Class Only

```rust
use twust::tw1;

let class = tw1!("bg-red-500");
assert_eq!(class, "bg-red-500");
```

### `tws1!` - Array of Single-Class Items Only

```rust
use twust::tws1;

let class_list = tws1!["text-xl", "border", "m-4"];
assert_eq!(class_list, ["text-xl", "border", "m-4"]);
```

---

## `tailwind.config.json` Overview

### Basic Structure:

```json
{
  "corePlugins": {},
  "allowedLists": {
    "classes": [],
    "modifiers": []
  },
  "theme": {
    "extend": {}
  },
  "variants": {},
  "plugins": {}
}
```

---

## Statement of Problem

TailwindCSS offers developers a flexible utility-first approach to styling web applications. However, its flexibility can also lead to potential pitfalls:

- **Runtime Errors:** Invalid TailwindCSS class names can cause unexpected styling issues that are only caught during runtime.
- **Developer Experience:** Manually validating class names can be tedious and error-prone.
- **Plugin Compatibility:** Some TailwindCSS utilities extend their functionality with plugins like DaisyUI, which traditional methods might not support.
- **Increased Build Size:** Invalid class names that slip into the production code can increase the final CSS bundle size.

---

## Solution

Twust addresses these challenges by offering:

- **Compile-time Validation:** Ensures that only valid TailwindCSS class names are used, preventing errors in production.
- **Seamless Integration:** Works within Rust macros for an improved developer experience.
- **Plugin Support:** Easily integrate popular plugins like DaisyUI with feature flags.
- **Optimized Builds:** Reduces unnecessary CSS bloat.

---

## How does this compare with Other Rust Libraries?

### `tailwindcss-to-rust`

- Requires complex setup and external dependencies.
- Generates Rust code that must be maintained manually.
- Lacks full support for all Tailwind utilities.

### `twust`

- No setup required – just use the macros.
- Works in real-time at compile-time.
- Self-contained with no external dependencies.
- Supports all standard TailwindCSS class names, including responsive variants.

---

## License

Twust is licensed under MIT/Apache-2.0. See LICENSE for details.

© [Oyelowo Oyedayo](https://github.com/Oyelowo)

**Email: oyelowo.oss@gmail.com**

---

## ⭐ Show Some Love!

If you find Twust useful, give it a ⭐ on [GitHub](https://github.com/Oyelowo/twust)!
