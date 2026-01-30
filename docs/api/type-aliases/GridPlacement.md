# GridPlacement

```ts
type GridPlacement =
  | "auto"
  | number
  | {
      ident: string;
      line: number;
    }
  | {
      ident?: string;
      span: number;
    };
```

Grid placement type for positioning grid items.

Specifies how an item is placed on a grid track (row or column).
Follows CSS `grid-row-start` / `grid-column-start` specification.

## Remarks

- `"auto"`: Auto-placement using the grid's flow algorithm
- `number`: Place at a specific line index (1-indexed, can be negative)
- `{ span: number }`: Span a specified number of tracks

## Example

```typescript
import type { GridPlacement, Line } from "taffy-layout";

// Line index (CSS: grid-row-start: 2)
const lineIndex: GridPlacement = 2;

// Auto placement (CSS: grid-row-start: auto)
const auto: GridPlacement = "auto";

// Span (CSS: grid-row-start: span 3)
const span: GridPlacement = { span: 3 };

// Named line (CSS: grid-row-start: header 2)
const named: GridPlacement = { line: 2, ident: "header" };

// Named span (CSS: grid-row-start: span 2 header)
const namedSpan: GridPlacement = { span: 2, ident: "header" };
```
