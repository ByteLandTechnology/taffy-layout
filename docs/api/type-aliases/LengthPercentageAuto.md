# LengthPercentageAuto

```ts
type LengthPercentageAuto = number | `${number}%` | "auto";
```

Length, percentage, or auto value.

Used for properties that support auto values, such as `margin` and `inset`.

## Remarks

- `number`: Fixed size in pixels
- `"{number}%"`: Percentage whose reference size depends on the property; margin percentages use the containing width, while inset percentages use the corresponding axis
- `"auto"`: Automatic value (behavior depends on property)

## Example

```typescript
import { Style, type LengthPercentageAuto, type Rect } from "taffy-layout";

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
