# AlignSelf

Cross-axis alignment enumeration for a single element

Overrides the parent's `align-items` value for a specific child element.
This corresponds to the CSS `align-self` property.

## Example

```typescript
import { Style, AlignSelf } from "taffy-layout";

const style = new Style();
style.alignSelf = AlignSelf.Auto; // Use parent's align-items
style.alignSelf = AlignSelf.Center; // Override to center this item
```

## Enumeration Members

| Enumeration Member                                            | Value | Description                                                    |
| ------------------------------------------------------------- | ----- | -------------------------------------------------------------- |
| <a id="enumeration-member-auto"></a> `Auto`                   | `0`   | Inherits the parent container's `align-items` value            |
| <a id="enumeration-member-baseline"></a> `Baseline`           | `6`   | Item aligned to its text baseline                              |
| <a id="enumeration-member-center"></a> `Center`               | `5`   | Item centered along the cross axis                             |
| <a id="enumeration-member-end"></a> `End`                     | `2`   | Item aligned to the end of the cross axis                      |
| <a id="enumeration-member-flexend"></a> `FlexEnd`             | `4`   | Item aligned to the end of the flex container                  |
| <a id="enumeration-member-flexstart"></a> `FlexStart`         | `3`   | Item aligned to the start of the flex container                |
| <a id="enumeration-member-safecenter"></a> `SafeCenter`       | `14`  | Safe center alignment that avoids start-edge overflow          |
| <a id="enumeration-member-safeend"></a> `SafeEnd`             | `11`  | Safe end alignment that avoids start-edge overflow             |
| <a id="enumeration-member-safeflexend"></a> `SafeFlexEnd`     | `13`  | Safe flex-end alignment that avoids start-edge overflow        |
| <a id="enumeration-member-safeflexstart"></a> `SafeFlexStart` | `12`  | Safe flex-start alignment that avoids start-edge overflow      |
| <a id="enumeration-member-safeselfend"></a> `SafeSelfEnd`     | `16`  | Safe self-end alignment that avoids start-edge overflow        |
| <a id="enumeration-member-safeselfstart"></a> `SafeSelfStart` | `15`  | Safe self-start alignment that avoids start-edge overflow      |
| <a id="enumeration-member-safestart"></a> `SafeStart`         | `10`  | Safe start alignment that avoids start-edge overflow           |
| <a id="enumeration-member-selfend"></a> `SelfEnd`             | `9`   | Item aligned to the end edge determined by its own direction   |
| <a id="enumeration-member-selfstart"></a> `SelfStart`         | `8`   | Item aligned to the start edge determined by its own direction |
| <a id="enumeration-member-start"></a> `Start`                 | `1`   | Item aligned to the start of the cross axis                    |
| <a id="enumeration-member-stretch"></a> `Stretch`             | `7`   | Item stretched to fill the container                           |
