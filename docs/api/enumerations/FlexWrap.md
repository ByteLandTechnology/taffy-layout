# FlexWrap

Flex wrap mode enumeration

Controls whether flex items wrap onto multiple lines when they overflow the container.
This corresponds to the CSS `flex-wrap` property.

## Example

```typescript
import { Style, FlexWrap } from "taffy-layout";

const style = new Style();
style.flexWrap = FlexWrap.NoWrap; // All items on single line
style.flexWrap = FlexWrap.Wrap; // Items wrap to new lines
```

## Enumeration Members

| Enumeration Member                                        | Value | Description                                                                          |
| --------------------------------------------------------- | ----- | ------------------------------------------------------------------------------------ |
| <a id="enumeration-member-nowrap"></a> `NoWrap`           | `0`   | All flex items are placed on a single line                                           |
| <a id="enumeration-member-wrap"></a> `Wrap`               | `1`   | Flex items wrap into lines along the cross axis, following the container's direction |
| <a id="enumeration-member-wrapreverse"></a> `WrapReverse` | `2`   | Flex items wrap into lines in the reverse cross-axis direction                       |
