# FAQ

### Why use Twust instead of tailwindcss-to-rust?
Twust is simpler, requires no setup, and validates classes at **compile-time** instead of generating code.

### Can I use this with DaisyUI?
Yes! Enable the feature flag:

```toml
twust = { version = "1.0.7", features = ["daisyui"] }
```
