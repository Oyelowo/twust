# Twust - Static TailwindCSS Validation in Rust

Twust is a **compile-time TailwindCSS static checker** for Rust. It validates class names before runtime, preventing typos, incorrect class names, and misconfigurations.

## Why Twust?
- **Compile-time validation**: Catch invalid TailwindCSS classes before your Rust app runs.
- **Zero runtime overhead**: Errors are flagged at compile time.
- **Supports Tailwind plugins** like DaisyUI (via feature flags).
- **Works with Rust UI frameworks** (Leptos, Dioxus, Yew, Sycamore).

```rust
use twust::tw;

let class = tw!("bg-blue-500 text-lg hover:bg-blue-600");
assert_eq!(class, "bg-blue-500 text-lg hover:bg-blue-600");
