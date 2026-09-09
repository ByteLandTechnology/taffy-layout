# JustifyContent

Main axis alignment enumeration

Defines how flex items are aligned and spaced along the main axis.
This corresponds to the CSS `justify-content` property.

## Example

```typescript
import { Style, JustifyContent } from "taffy-layout";

const style = new Style();
style.justifyContent = JustifyContent.Center; // Center items
style.justifyContent = JustifyContent.SpaceBetween; // Distribute evenly
```

## Enumeration Members

| Enumeration Member                                            | Value | Description                                               |
| ------------------------------------------------------------- | ----- | --------------------------------------------------------- |
| <a id="enumeration-member-center"></a> `Center`               | `4`   | Items centered along the main axis                        |
| <a id="enumeration-member-end"></a> `End`                     | `1`   | Items packed toward the end of the main axis              |
| <a id="enumeration-member-flexend"></a> `FlexEnd`             | `3`   | Items packed toward the end of the flex container         |
| <a id="enumeration-member-flexstart"></a> `FlexStart`         | `2`   | Items packed toward the start of the flex container       |
| <a id="enumeration-member-safecenter"></a> `SafeCenter`       | `13`  | Safe center alignment that avoids start-edge overflow     |
| <a id="enumeration-member-safeend"></a> `SafeEnd`             | `10`  | Safe end alignment that avoids start-edge overflow        |
| <a id="enumeration-member-safeflexend"></a> `SafeFlexEnd`     | `12`  | Safe flex-end alignment that avoids start-edge overflow   |
| <a id="enumeration-member-safeflexstart"></a> `SafeFlexStart` | `11`  | Safe flex-start alignment that avoids start-edge overflow |
| <a id="enumeration-member-safestart"></a> `SafeStart`         | `9`   | Safe start alignment that avoids start-edge overflow      |
| <a id="enumeration-member-spacearound"></a> `SpaceAround`     | `7`   | Items evenly distributed with equal space around each     |
| <a id="enumeration-member-spacebetween"></a> `SpaceBetween`   | `6`   | Items evenly distributed with first/last at edges         |
| <a id="enumeration-member-spaceevenly"></a> `SpaceEvenly`     | `8`   | Items evenly distributed with equal space between each    |
| <a id="enumeration-member-start"></a> `Start`                 | `0`   | Items packed toward the start of the main axis            |
| <a id="enumeration-member-stretch"></a> `Stretch`             | `5`   | Items stretched along the main axis                       |
