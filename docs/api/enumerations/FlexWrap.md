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

| Enumeration Member                     | Value | Description                                            |
| -------------------------------------- | ----- | ------------------------------------------------------ |
| <a id="nowrap"></a> `NoWrap`           | `0`   | All flex items are placed on a single line             |
| <a id="wrap"></a> `Wrap`               | `1`   | Flex items wrap onto multiple lines from top to bottom |
| <a id="wrapreverse"></a> `WrapReverse` | `2`   | Flex items wrap onto multiple lines from bottom to top |
