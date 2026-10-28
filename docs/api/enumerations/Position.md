# Position

Position mode enumeration

Controls how an element is positioned within its parent container.
This corresponds to the CSS `position` property.

## Example

```typescript
import { Style, Position } from "taffy-layout";

const style = new Style();
style.position = Position.Static; // Normal document flow, insets ignored (default)
style.position = Position.Relative; // Normal document flow, offset by insets
style.position = Position.Absolute; // Removed from flow, uses inset values
```

## Enumeration Members

| Enumeration Member                                  | Value | Description                                                                                                                                            |
| --------------------------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| <a id="enumeration-member-absolute"></a> `Absolute` | `1`   | Element is removed from flow and positioned relative to its containing block: the nearest ancestor that is not statically positioned, or the root      |
| <a id="enumeration-member-fixed"></a> `Fixed`       | `3`   | Element is removed from flow and positioned relative to the root                                                                                       |
| <a id="enumeration-member-relative"></a> `Relative` | `0`   | Element participates in normal document flow and is offset by its insets. It is a containing block for absolutely positioned descendants.              |
| <a id="enumeration-member-static"></a> `Static`     | `2`   | Element participates in normal document flow and ignores its insets (CSS default). It is not a containing block for absolutely positioned descendants. |
| <a id="enumeration-member-sticky"></a> `Sticky`     | `4`   | Laid out like `Static`, but is a containing block for absolutely positioned descendants. Applying sticky offsets is left to the caller.                |
