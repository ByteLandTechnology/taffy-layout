# StylePropertyValues

```ts
type StylePropertyValues = { [K in StyleProperty]?: K extends "display" ? Display : K extends "position" ? Position : K extends "direction" ? Direction : K extends "float" ? Float : K extends "clear" ? Clear : K extends "boxSizing" ? BoxSizing : K extends "overflow" ? Point<Overflow> : K extends "overflowX" | "overflowY" ? Overflow : K extends "flexDirection" ? FlexDirection : (...) extends (...) ? (...) : (...) };
```

Type-safe property values for batch setting.

Maps property paths to their expected value types.
