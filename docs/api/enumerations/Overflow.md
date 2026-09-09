# Overflow

Overflow handling enumeration

Controls overflow sizing, automatic minimum sizes, and scrollbar space.
This corresponds to the CSS `overflow` property.
Taffy computes layout only; clipping, drawing, and scrolling are implemented
by the renderer consuming the layout.

## Example

```typescript
import { Style, Overflow } from "taffy-layout";

const style = new Style();
style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
```

## Enumeration Members

| Enumeration Member                                | Value | Description                                                                              |
| ------------------------------------------------- | ----- | ---------------------------------------------------------------------------------------- |
| <a id="enumeration-member-clip"></a> `Clip`       | `1`   | Clipped, non-scrollable overflow semantics; retain content-based automatic minimum sizes |
| <a id="enumeration-member-hidden"></a> `Hidden`   | `2`   | Hidden overflow semantics; allow the automatic minimum size to shrink to zero            |
| <a id="enumeration-member-scroll"></a> `Scroll`   | `3`   | Reserve scrollbar space using scrollbarWidth and allow zero automatic minimum sizes      |
| <a id="enumeration-member-visible"></a> `Visible` | `0`   | Visible overflow semantics; retain content-based automatic minimum sizes                 |
