[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / StylePropertyValues

# Type Alias: StylePropertyValues

```ts
type StylePropertyValues = { [K in StyleProperty]?: K extends "display" ? Display : K extends "position" ? Position : K extends "boxSizing" ? BoxSizing : K extends "overflow" ? Point<Overflow> : K extends "overflowX" | "overflowY" ? Overflow : K extends "flexDirection" ? FlexDirection : K extends "flexWrap" ? FlexWrap : K extends "flexGrow" | "flexShrink" | "scrollbarWidth" ? number : K extends "flexBasis" ? Dimension : (...) extends (...) ? (...) : (...) };
```

Type-safe property values for batch setting.

Maps property paths to their expected value types.
