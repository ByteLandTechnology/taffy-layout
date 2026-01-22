[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / LayoutPropertyValues

# Type Alias: LayoutPropertyValues

```ts
type LayoutPropertyValues = { [K in LayoutProperty]: K extends "order" ? number : K extends "position" ? Point<number> : K extends "x" | "y" ? number : K extends "size" ? Size<number> : K extends "width" | "height" ? number : K extends "contentSize" ? Size<number> : K extends "contentWidth" | "contentHeight" ? number : K extends "scrollbarSize" ? Size<number> : K extends (...) | (...) ? number : (...) extends (...) ? (...) : (...) };
```

Type-safe property values for Layout.get().

Maps property paths to their expected value types.
