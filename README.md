# `tw-macro`

A powerful Rust macro to validate TailwindCSS class names at compile-time.

## Table of Contents

- [Overview](#overview)
- [Statement of Problem](#statement-of-problem)
- [Solution](#solution)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [How does this compare with Other Rust
  Libraries](#how-does-this-compare-with-other-rust-libraries)
- [`tailwind.config.json` Overview](#tailwind.config.json-Overview)
- [Contribution](#contribution)
- [License](#license)

## Overview

`tw-macro` is a Rust procedural macro that provides compile-time validation for
TailwindCSS class names. Leveraging the power of Rust's macro system, `tw-macro`
ensures that you only use valid TailwindCSS class names, preventing runtime
errors and promoting a more robust development experience.

## Statement of Problem

TailwindCSS offers developers a flexible utility-first approach to styling web
applications. However, its flexibility can also lead to potential pitfalls:

- **Runtime Errors:** Invalid TailwindCSS class names can cause unexpected
  styling issues that are only caught during runtime.
- **Developer Experience:** Manually validating class names can be tedious and
  error-prone. Moreover, relying on runtime checks or external tools can disrupt
  the development workflow.

- **Increased Build Size:** Invalid class names that slip into the production
  code can increase the size of the final CSS bundle, affecting performance.

## Solution

`tw-macro` addresses these challenges by offering:

- **Compile-time Validation:** By checking the validity of TailwindCSS class
  names at compile time, `tw-macro` prevents invalid class names from making
  their way into the production code.

- **Seamless Integration:** As a Rust macro, `tw-macro` integrates seamlessly
  into your Rust workflow, offering immediate feedback without the need for
  external tools or manual validation.

- **Optimized Builds:** By ensuring only valid class names are used, `tw-macro`
  helps in reducing the unnecessary bloat in the final CSS bundle.

## Features

- **Comprehensive Coverage:** Supports all standard TailwindCSS class names,
  including responsive variants, pseudo-class variants, and more.
- **Custom Configurations:** Easily integrate with custom TailwindCSS
  configurations to support custom utility classes.

- **Performance:** Designed with performance in mind, ensuring minimal overhead
  during the compilation process.

## Installation

Add `tw-macro` to your `Cargo.toml`:

```toml
[dependencies]
tw-macro = "0.1.0"
```

## Usage

Simply prefix your TailwindCSS class strings with the `tw!` macro:

```rust
use tw_macro::tw;

let classes = tw!("bg-blue-500 hover:bg-blue-700");
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

If an invalid class name is used, the compiler will raise an error, preventing
it from being used in your application.

## How does this compare with Other Rust Libraries

### `tailwindcss-to-rust`

[`tailwindcss-to-rust`](https://crates.io/crates/tailwindcss-to-rust) is a CLI
tool that generates Rust code from compiled TailwindCSS. It allows developers to
refer to Tailwind classes from Rust, offering compile-time error checks for
nonexistent classes and code completion for available classes.

#### Shortcomings:

1. **Complex Setup:** The setup process for `tailwindcss-to-rust` requires
   several steps, including:

   - Installing multiple tools.
   - Customizing `tailwind.config.js` for Rust file checks.
   - Generating Rust code and ensuring the `tailwindcss` executable is in the
     `PATH`.
   - Modifying regular expressions to match a specific templating system.

   This setup can be daunting, especially for developers unfamiliar with
   TailwindCSS or Rust.

2. **Generated Code Maintenance:** `tailwindcss-to-rust` generates code based on
   the current state of the TailwindCSS configuration. Any changes to the
   configuration or updates to TailwindCSS itself may require regenerating the
   Rust code, making maintenance more challenging.

3. **Custom Class Grouping:** While the tool groups classes based on the
   Tailwind documentation, custom classes end up in an "unknown" group
   (`C::unk`). Although there's a plan to improve this, it's not yet
   implemented.

4. **Limited Modifiers:** Some parameterizable modifiers like `aria-*` and
   `data-*` are not included. Depending on the project, this can be a
   significant limitation.

5. **External Dependencies:** The tool requires the presence of the
   `tailwindcss` CLI tool, either through npm or as a standalone binary. This
   adds an external dependency, which might not be suitable for all projects,
   especially those that want to minimize their dependency tree.

### Approach with `tw-macro`

Our solution with `tw-macro` offers a more streamlined and integrated approach:

- **Simpler Setup:** Just add the macro to your project and start using it. No
  need for external tools or additional configuration steps.
- **Real-time Validation:** Instead of generating static Rust code from
  TailwindCSS, `tw-macro` validates class names in real-time during the
  compilation process.
- **No External Dependencies:** `tw-macro` is self-contained, meaning you don't
  need any external tools like the `tailwindcss` CLI.

- **Extensive Coverage:** We support all standard TailwindCSS class names,
  including responsive variants, pseudo-class variants, and more. With the
  macro's flexibility, supporting newer TailwindCSS features becomes easier.

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
    ...utilities...
    "extend": {
      ...extended utilities...
    }
  },
  "variants": {},
  "plugins": {}
}
```

### Key Sections:

1. **`corePlugins`**: Determines which utility plugins should be included in the
   generated CSS. Each key represents a utility, and its value (true/false)
   determines if it should be generated.

2. **`allowedLists`**:

   - `classes`: Array of allowed class names.
   - `modifiers`: Array of allowed modifiers like `hover`, `focus`, etc.

3. **`theme`**: Defines the default values and customizations for your design
   system.

   In the main section of `theme`, you have configurations for different
   utilities like `screens`, `colors`, `spacing`, etc. The `extend` section
   within `theme` allows you to add additional values to the default set.

## Contribution

Contributions are always welcome! If you have suggestions, bug reports, or want
to contribute to the code, please open an issue or pull request.

## License

`tw-macro` is licensed under the MIT license. See the `LICENSE` file for
details.
