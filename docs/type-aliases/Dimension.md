[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Dimension

# Type Alias: Dimension

```ts
type Dimension =
  | {
      Length: number;
    }
  | {
      Percent: number;
    }
  | "Auto";
```

Dimension type supporting length, percentage, or auto values.

Used for sizing properties like `width`, `height`, `flexBasis`, etc.

## Remarks

- `{ Length: number }`: Fixed size in pixels
- `{ Percent: number }`: Percentage of parent's size (0-100)
- `"Auto"`: Size determined by content or layout algorithm

<details>
<summary><strong>TypeScript Example</strong></summary>

```typescript
import { Style, type Dimension, type Size } from "taffy-js";

const style = new Style();

// With explicit type annotations
const fixedSize: Size<Dimension> = {
  width: { Length: 200 },
  height: { Length: 100 },
};

const percentSize: Size<Dimension> = {
  width: { Percent: 50 },
  height: { Percent: 100 },
};

const autoSize: Size<Dimension> = {
  width: "Auto",
  height: "Auto",
};

style.size = fixedSize;
```

</details>
