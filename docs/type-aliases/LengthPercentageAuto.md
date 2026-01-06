[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / LengthPercentageAuto

# Type Alias: LengthPercentageAuto

```ts
type LengthPercentageAuto = number | `${number}%` | "auto";
```

Length, percentage, or auto value.

Used for properties that support auto values, such as `margin` and `inset`.

## Remarks

- `number`: Fixed size in pixels
- `"{number}%"`: Percentage of parent's size (0-100)
- `"auto"`: Automatic value (behavior depends on property)

## Example

```typescript
import { Style, type LengthPercentageAuto, type Rect } from "taffy-js";

const style = new Style();

// Auto margins for horizontal centering
const centerMargin: Rect<LengthPercentageAuto> = {
  left: "auto",
  right: "auto",
  top: 0,
  bottom: 0,
};

style.margin = centerMargin;
```
