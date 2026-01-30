# TextAlign

Text alignment enumeration (for block layout)

Used by block layout to implement the legacy behaviour of `<center>` and
`<div align="left | right | center">`.

## Example

```typescript
import { Style, TextAlign } from "taffy-layout";

const style = new Style();
style.textAlign = TextAlign.LegacyCenter; // Center block children
```

## Enumeration Members

| Enumeration Member                       | Value | Description                                                  |
| ---------------------------------------- | ----- | ------------------------------------------------------------ |
| <a id="auto"></a> `Auto`                 | `0`   | No special legacy text align behaviour                       |
| <a id="legacycenter"></a> `LegacyCenter` | `3`   | Corresponds to `-webkit-center` or `-moz-center` in browsers |
| <a id="legacyleft"></a> `LegacyLeft`     | `1`   | Corresponds to `-webkit-left` or `-moz-left` in browsers     |
| <a id="legacyright"></a> `LegacyRight`   | `2`   | Corresponds to `-webkit-right` or `-moz-right` in browsers   |
