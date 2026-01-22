[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / StyleProperty

# Type Alias: StyleProperty

```ts
type StyleProperty =
  | "display"
  | "position"
  | "boxSizing"
  | "overflow"
  | "overflow.x"
  | "overflow.y"
  | "flexDirection"
  | "flexWrap"
  | "flexGrow"
  | "flexShrink"
  | "flexBasis"
  | "alignItems"
  | "alignSelf"
  | "alignContent"
  | "justifyContent"
  | "justifyItems"
  | "justifySelf"
  | "aspectRatio"
  | "size"
  | "size.width"
  | "size.height"
  | "minSize"
  | "minSize.width"
  | "minSize.height"
  | "maxSize"
  | "maxSize.width"
  | "maxSize.height"
  | "margin"
  | "margin.left"
  | "margin.right"
  | "margin.top"
  | "margin.bottom"
  | "padding"
  | "padding.left"
  | "padding.right"
  | "padding.top"
  | "padding.bottom"
  | "border"
  | "border.left"
  | "border.right"
  | "border.top"
  | "border.bottom"
  | "inset"
  | "inset.left"
  | "inset.right"
  | "inset.top"
  | "inset.bottom"
  | "gap"
  | "gap.width"
  | "gap.height"
  | "itemIsTable"
  | "itemIsReplaced"
  | "scrollbarWidth"
  | "textAlign"
  | "gridAutoFlow"
  | "gridRow"
  | "gridRow.start"
  | "gridRow.end"
  | "gridColumn"
  | "gridColumn.start"
  | "gridColumn.end"
  | "gridTemplateRows"
  | "gridTemplateColumns"
  | "gridAutoRows"
  | "gridAutoColumns"
  | "gridTemplateAreas"
  | "gridTemplateRowNames"
  | "gridTemplateColumnNames";
```

Valid property paths for Style.get() method.

Supports dot notation for nested properties.

## Example

```typescript
const style = new Style();
// Top-level properties
style.get("display", "flexGrow");

// Nested properties with dot notation
style.get("size.width", "margin.left");
```
