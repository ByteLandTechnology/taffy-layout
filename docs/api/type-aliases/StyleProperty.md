# StyleProperty

```ts
type StyleProperty =
  | "display"
  | "position"
  | "boxSizing"
  | "overflow"
  | "overflowX"
  | "overflowY"
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
  | "width"
  | "height"
  | "minSize"
  | "minWidth"
  | "minHeight"
  | "maxSize"
  | "maxWidth"
  | "maxHeight"
  | "margin"
  | "marginLeft"
  | "marginRight"
  | "marginTop"
  | "marginBottom"
  | "padding"
  | "paddingLeft"
  | "paddingRight"
  | "paddingTop"
  | "paddingBottom"
  | "border"
  | "borderLeft"
  | "borderRight"
  | "borderTop"
  | "borderBottom"
  | "inset"
  | "left"
  | "right"
  | "top"
  | "bottom"
  | "gap"
  | "columnGap"
  | "rowGap"
  | "itemIsTable"
  | "itemIsReplaced"
  | "scrollbarWidth"
  | "textAlign"
  | "gridAutoFlow"
  | "gridRow"
  | "gridRowStart"
  | "gridRowEnd"
  | "gridColumn"
  | "gridColumnStart"
  | "gridColumnEnd"
  | "gridTemplateRows"
  | "gridTemplateColumns"
  | "gridAutoRows"
  | "gridAutoColumns"
  | "gridTemplateAreas"
  | "gridTemplateRowNames"
  | "gridTemplateColumnNames";
```

Valid property keys for Style.get() method.

Supports both object properties and individual flat properties.

## Example

```typescript
const style = new Style();
// Top-level properties
style.get("display", "flexGrow");

// Individual flat properties
style.get("width", "marginLeft", "paddingTop");

// Object properties
style.get("size", "margin");
```
