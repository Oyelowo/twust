# Usage

Twust provides macros to validate TailwindCSS classes at compile time.

## Basic Example

```rust

use twust::tw;

let class = tw!("bg-red-500 text-lg");
assert_eq!(class, "bg-red-500 text-lg");

```

## Restricting to One Class

```rust

use twust::tw1;

let single_class = tw1!("bg-blue-500");
assert_eq!(single_class, "bg-blue-500");

```

## Using Multiple Classes As Array

```rust

use twust::tws;

let class_list = tws!["bg-red-500 text-lg", "p-4 bg-blue-500"];
assert_eq!(class_list, ["bg-red-500", "text-lg", "p-4"]);

```


## Using Multiple Classes As Array: One class per item

```rust

use twust::tws1;

let class_list = tws1!["bg-red-500", "text-lg", "p-4"];
assert_eq!(class_list, ["bg-red-500", "text-lg", "p-4"]);

```

