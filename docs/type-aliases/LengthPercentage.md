[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / LengthPercentage

# Type Alias: LengthPercentage

```ts
type LengthPercentage =
  | {
      Length: number;
    }
  | {
      Percent: number;
    };
```

Length or percentage value (no auto support).

Used for properties that require explicit values, such as `padding`, `border`, and `gap`.

## Remarks

- `{ Length: number }`: Fixed size in pixels
- `{ Percent: number }`: Percentage of parent's size (0-100)

<details>
<summary><strong>TypeScript Example</strong></summary>

```typescript
import { Style, type LengthPercentage, type Rect, type Size } from "taffy-js";

const style = new Style();

const padding: Rect<LengthPercentage> = {
  left: { Length: 10 },
  right: { Length: 10 },
  top: { Length: 5 },
  bottom: { Length: 5 },
};

const gap: Size<LengthPercentage> = {
  width: { Percent: 5 },
  height: { Percent: 5 },
};

style.padding = padding;
style.gap = gap;
```

</details>
