# AlignContent

Multi-line content alignment enumeration

Controls the distribution of space between and around content items along the cross axis
in a multi-line flex container. This corresponds to the CSS `align-content` property.

In Flexbox, this property distributes wrapped lines. In Grid it aligns row
tracks; in block layout it aligns the block content vertically.

## Example

```typescript
import { Style, AlignContent, FlexWrap } from "taffy-layout";

const style = new Style();
style.flexWrap = FlexWrap.Wrap;
style.alignContent = AlignContent.SpaceBetween; // Distribute lines evenly
```

## Enumeration Members

| Enumeration Member                                            | Value | Description                                               |
| ------------------------------------------------------------- | ----- | --------------------------------------------------------- |
| <a id="enumeration-member-center"></a> `Center`               | `4`   | Lines centered within the container                       |
| <a id="enumeration-member-end"></a> `End`                     | `1`   | Lines packed toward the end of the cross axis             |
| <a id="enumeration-member-flexend"></a> `FlexEnd`             | `3`   | Lines packed toward the end of the flex container         |
| <a id="enumeration-member-flexstart"></a> `FlexStart`         | `2`   | Lines packed toward the start of the flex container       |
| <a id="enumeration-member-safecenter"></a> `SafeCenter`       | `13`  | Safe center alignment that avoids start-edge overflow     |
| <a id="enumeration-member-safeend"></a> `SafeEnd`             | `10`  | Safe end alignment that avoids start-edge overflow        |
| <a id="enumeration-member-safeflexend"></a> `SafeFlexEnd`     | `12`  | Safe flex-end alignment that avoids start-edge overflow   |
| <a id="enumeration-member-safeflexstart"></a> `SafeFlexStart` | `11`  | Safe flex-start alignment that avoids start-edge overflow |
| <a id="enumeration-member-safestart"></a> `SafeStart`         | `9`   | Safe start alignment that avoids start-edge overflow      |
| <a id="enumeration-member-spacearound"></a> `SpaceAround`     | `7`   | Lines evenly distributed with equal space around each     |
| <a id="enumeration-member-spacebetween"></a> `SpaceBetween`   | `6`   | Lines evenly distributed with first/last at edges         |
| <a id="enumeration-member-spaceevenly"></a> `SpaceEvenly`     | `8`   | Lines evenly distributed with equal space between each    |
| <a id="enumeration-member-start"></a> `Start`                 | `0`   | Lines packed toward the start of the cross axis           |
| <a id="enumeration-member-stretch"></a> `Stretch`             | `5`   | Lines stretched to fill the container                     |
