# LengthPercentage

```ts
type LengthPercentage = number | `${number}%`;
```

Length or percentage value (no auto support).

Used for properties that require explicit values, such as `padding`, `border`, and `gap`.

## Remarks

- `number`: Fixed size in pixels
- `"{number}%"`: Percentage of parent's size (0-100)

## Example

```typescript
import {
  Style,
  type LengthPercentage,
  type Rect,
  type Size,
} from "taffy-layout";

const style = new Style();

const padding: Rect<LengthPercentage> = {
  left: 10,
  right: 10,
  top: 5,
  bottom: 5,
};

const gap: Size<LengthPercentage> = {
  width: "5%",
  height: "5%",
};

style.padding = padding;
style.gap = gap;
```
