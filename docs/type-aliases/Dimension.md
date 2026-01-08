[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Dimension

# Type Alias: Dimension

```ts
type Dimension = number | `${number}%` | "auto";
```

Dimension type supporting length, percentage, or auto values.

Used for sizing properties like `width`, `height`, `flexBasis`, etc.

## Remarks

- `number`: Fixed size in pixels
- `"{number}%"`: Percentage of parent's size (0-100)
- `"auto"`: Size determined by content or layout algorithm

## Example

```typescript
import { Style, type Dimension, type Size } from "taffy-layout";

const style = new Style();

// With explicit type annotations
const fixedSize: Size<Dimension> = {
  width: 200,
  height: 100,
};

const percentSize: Size<Dimension> = {
  width: "50%",
  height: "100%",
};

const autoSize: Size<Dimension> = {
  width: "auto",
  height: "auto",
};

style.size = fixedSize;
```
