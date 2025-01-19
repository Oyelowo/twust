# Macros

Twust provides the following macros:

### `tw!` - Type-Checked Tailwind Classes

```rust

use twust::tw;

let class = tw!("hover:bg-green-500 text-white font-bold");
assert_eq!(class, "hover:bg-green-500 text-white font-bold");

```

### `tws!` - Compile-time Checked Array of Classes

```rust

use twust::tws;

let class_list = tws!["border", "rounded-md", "shadow"];
assert_eq!(class_list, ["border", "rounded-md", "shadow"]);

```

### `tw1!` - Single Tailwind Class Only

```rust

use twust::tw1;

let single_class = tw1!("flex");
assert_eq!(single_class, "flex");

```

### `tws1!` - Array of Single-Class Items Only

```rust

use twust::tws1;

let class_list = tws1!["text-xl", "border", "m-4"];
assert_eq!(class_list, ["text-xl", "border", "m-4"]);

```
