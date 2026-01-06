[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / Rect

# Interface: Rect\<T\>

Rectangle with left, right, top, and bottom values.

Used for box model properties like `margin`, `padding`, `border`, and `inset`.

## Type Parameters

| Type Parameter | Description                 |
| -------------- | --------------------------- |
| `T`            | The type of each side value |

## Properties

| Property                     | Type | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------------------------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <a id="bottom"></a> `bottom` | `T`  | The bottom side value <details> <summary><strong>TypeScript Example</strong></summary> `import { Style, type Rect, type LengthPercentage, type LengthPercentageAuto } from 'taffy-js'; const style = new Style(); // Typed padding const padding: Rect<LengthPercentage> = { left: { Length: 10 }, right: { Length: 10 }, top: { Length: 10 }, bottom: { Length: 10 } }; // Typed margin with auto const margin: Rect<LengthPercentageAuto> = { left: "Auto", right: "Auto", top: { Length: 10 }, bottom: { Length: 30 } }; style.padding = padding; style.margin = margin;` </details> |
| <a id="left"></a> `left`     | `T`  | The left side value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| <a id="right"></a> `right`   | `T`  | The right side value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| <a id="top"></a> `top`       | `T`  | The top side value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
