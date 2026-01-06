[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / LengthPercentageAuto

# Type Alias: LengthPercentageAuto

```ts
type LengthPercentageAuto =
  | {
      Length: number;
    }
  | {
      Percent: number;
    }
  | "Auto";
```

Length, percentage, or auto value.

Used for properties that support auto values, such as `margin` and `inset`.

## Remarks

- `{ Length: number }`: Fixed size in pixels
- `{ Percent: number }`: Percentage of parent's size (0-100)
- `"Auto"`: Automatic value (behavior depends on property)

<details>
<summary><strong>TypeScript Example</strong></summary>

```typescript
import { Style, type LengthPercentageAuto, type Rect } from "taffy-js";

const style = new Style();

// Auto margins for horizontal centering
const centerMargin: Rect<LengthPercentageAuto> = {
  left: "Auto",
  right: "Auto",
  top: { Length: 0 },
  bottom: { Length: 0 },
};

style.margin = centerMargin;
```

</details>
