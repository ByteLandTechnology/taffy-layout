[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Rect

# Interface: Rect\<T\>

Rectangle with left, right, top, and bottom values.

Used for box model properties like `margin`, `padding`, `border`, and `inset`.

## Example

```typescript
import {
  Style,
  type Rect,
  type LengthPercentage,
  type LengthPercentageAuto,
} from "taffy-js";

const style = new Style();

// Typed padding
const padding: Rect<LengthPercentage> = {
  left: 10,
  right: 10,
  top: 10,
  bottom: 10,
};

// Typed margin with auto
const margin: Rect<LengthPercentageAuto> = {
  left: "auto",
  right: "auto",
  top: 10,
  bottom: 30,
};

style.padding = padding;
style.margin = margin;
```

## Type Parameters

| Type Parameter | Description                 |
| -------------- | --------------------------- |
| `T`            | The type of each side value |

## Properties

| Property                     | Type | Description           |
| ---------------------------- | ---- | --------------------- |
| <a id="bottom"></a> `bottom` | `T`  | The bottom side value |
| <a id="left"></a> `left`     | `T`  | The left side value   |
| <a id="right"></a> `right`   | `T`  | The right side value  |
| <a id="top"></a> `top`       | `T`  | The top side value    |
