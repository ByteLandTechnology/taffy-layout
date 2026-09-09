# AlignItems

Cross-axis alignment enumeration for all children

Defines the default alignment for all flex/grid items along the cross axis.
This corresponds to the CSS `align-items` property.

## Example

```typescript
import { Style, AlignItems } from "taffy-layout";

const style = new Style();
style.alignItems = AlignItems.Center; // Center items on cross axis
style.alignItems = AlignItems.Stretch; // Stretch items to fill container
```

## Enumeration Members

| Enumeration Member                                            | Value | Description                                                            |
| ------------------------------------------------------------- | ----- | ---------------------------------------------------------------------- |
| <a id="enumeration-member-baseline"></a> `Baseline`           | `5`   | Items aligned to their text baselines                                  |
| <a id="enumeration-member-center"></a> `Center`               | `4`   | Items centered along the cross axis                                    |
| <a id="enumeration-member-end"></a> `End`                     | `1`   | Items aligned to the end of the cross axis                             |
| <a id="enumeration-member-flexend"></a> `FlexEnd`             | `3`   | Items aligned to the end of the flex container                         |
| <a id="enumeration-member-flexstart"></a> `FlexStart`         | `2`   | Items aligned to the start of the flex container                       |
| <a id="enumeration-member-safecenter"></a> `SafeCenter`       | `13`  | Safe center alignment that avoids start-edge overflow                  |
| <a id="enumeration-member-safeend"></a> `SafeEnd`             | `10`  | Safe end alignment that avoids start-edge overflow                     |
| <a id="enumeration-member-safeflexend"></a> `SafeFlexEnd`     | `12`  | Safe flex-end alignment that avoids start-edge overflow                |
| <a id="enumeration-member-safeflexstart"></a> `SafeFlexStart` | `11`  | Safe flex-start alignment that avoids start-edge overflow              |
| <a id="enumeration-member-safeselfend"></a> `SafeSelfEnd`     | `15`  | Safe self-end alignment that avoids start-edge overflow                |
| <a id="enumeration-member-safeselfstart"></a> `SafeSelfStart` | `14`  | Safe self-start alignment that avoids start-edge overflow              |
| <a id="enumeration-member-safestart"></a> `SafeStart`         | `9`   | Safe start alignment that avoids start-edge overflow                   |
| <a id="enumeration-member-selfend"></a> `SelfEnd`             | `8`   | Items aligned to the end edge determined by the item's own direction   |
| <a id="enumeration-member-selfstart"></a> `SelfStart`         | `7`   | Items aligned to the start edge determined by the item's own direction |
| <a id="enumeration-member-start"></a> `Start`                 | `0`   | Items aligned to the start of the cross axis                           |
| <a id="enumeration-member-stretch"></a> `Stretch`             | `6`   | Items stretched to fill the container                                  |
