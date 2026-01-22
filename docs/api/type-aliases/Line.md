[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Line

# Type Alias: Line\<T\>

```ts
type Line<T> = {
  end: T;
  start: T;
};
```

Line type representing start and end positions.

A container for start and end values, used for CSS grid-row and grid-column
shorthand properties.

## Example

```typescript
import { Style, Display, type Line, type GridPlacement } from "taffy-layout";

const style = new Style();
style.display = Display.Grid;

// CSS: grid-row: 1 / 3
style.gridRow = { start: 1, end: 3 };

// CSS: grid-column: 1 / span 2
style.gridColumn = { start: 1, end: { span: 2 } };

// CSS: grid-row: auto / auto
style.gridRow = { start: "auto", end: "auto" };
```

## Type Parameters

| Type Parameter | Description                      |
| -------------- | -------------------------------- |
| `T`            | The type of start and end values |

## Properties

| Property                   | Type | Description                |
| -------------------------- | ---- | -------------------------- |
| <a id="end"></a> `end`     | `T`  | The ending line/position   |
| <a id="start"></a> `start` | `T`  | The starting line/position |
