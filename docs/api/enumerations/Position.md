# Position

Position mode enumeration

Controls how an element is positioned within its parent container.
This corresponds to the CSS `position` property.

## Example

```typescript
import { Style, Position } from "taffy-layout";

const style = new Style();
style.position = Position.Relative; // Normal document flow
style.position = Position.Absolute; // Removed from flow, uses inset values
```

## Enumeration Members

| Enumeration Member                                  | Value | Description                                                                    |
| --------------------------------------------------- | ----- | ------------------------------------------------------------------------------ |
| <a id="enumeration-member-absolute"></a> `Absolute` | `1`   | Element is removed from flow and positioned within its parent layout container |
| <a id="enumeration-member-relative"></a> `Relative` | `0`   | Element participates in normal document flow                                   |
