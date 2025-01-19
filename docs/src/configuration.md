# Configuration

# Tailwind Configuration

Twust reads custom Tailwind settings from `tailwind.config.json`.

## Example Configuration

```json
{
  "theme": {
    "extend": {
      "colors": {
        "primary": "#ff6347",
        "secondary": "#4a90e2"
      }
    }
  },
  "corePlugins": {
    "preflight": false
  },
  "variants": {
    "backgroundColor": ["responsive", "hover", "focus"]
  }
}
```



## Using Custom Classes

```rust
tw!("bg-primary text-secondary hover:bg-secondary");
```
