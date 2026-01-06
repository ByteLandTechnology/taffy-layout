[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Point

# Interface: Point\<T\>

Point with x and y coordinates/values.

Used for properties that have separate horizontal (x) and vertical (y) values,
such as `overflow`.

## Type Parameters

| Type Parameter | Description                 |
| -------------- | --------------------------- |
| `T`            | The type of each coordinate |

## Properties

| Property           | Type | Description                                                                                                                                                                                                                                                                                                                |
| ------------------ | ---- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="x"></a> `x` | `T`  | The horizontal value                                                                                                                                                                                                                                                                                                       |
| <a id="y"></a> `y` | `T`  | The vertical value <details> <summary><strong>TypeScript Example</strong></summary> `import { Style, Overflow, type Point } from 'taffy-js'; const style = new Style(); const overflow: Point<typeof Overflow[keyof typeof Overflow]> = { x: Overflow.Hidden, y: Overflow.Scroll }; style.overflow = overflow;` </details> |
