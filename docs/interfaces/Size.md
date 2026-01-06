[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Size

# Interface: Size\<T\>

Generic size type with width and height.

A two-dimensional container for width and height values. The type parameter `T`
determines what kind of values are stored.

## Example

```typescript
import type { Size, Dimension, AvailableSpace } from "taffy-js";

// Size with explicit type parameters
const pixelSize: Size<number> = { width: 200, height: 100 };

const dimensionSize: Size<Dimension> = {
  width: { Length: 200 },
  height: { Percent: 50 },
};

const availableSize: Size<AvailableSpace> = {
  width: 800,
  height: "MaxContent",
};
```

## Type Parameters

| Type Parameter | Description                                                                |
| -------------- | -------------------------------------------------------------------------- |
| `T`            | The type of each dimension (e.g., `number`, `Dimension`, `AvailableSpace`) |

## Properties

| Property                     | Type | Description                    |
| ---------------------------- | ---- | ------------------------------ |
| <a id="height"></a> `height` | `T`  | The vertical dimension value   |
| <a id="width"></a> `width`   | `T`  | The horizontal dimension value |
