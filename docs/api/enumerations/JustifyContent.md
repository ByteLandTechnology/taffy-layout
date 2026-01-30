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

| Enumeration Member                       | Value | Description                                            |
| ---------------------------------------- | ----- | ------------------------------------------------------ |
| <a id="center"></a> `Center`             | `4`   | Items centered along the main axis                     |
| <a id="end"></a> `End`                   | `1`   | Items packed toward the end of the main axis           |
| <a id="flexend"></a> `FlexEnd`           | `3`   | Items packed toward the end of the flex container      |
| <a id="flexstart"></a> `FlexStart`       | `2`   | Items packed toward the start of the flex container    |
| <a id="spacearound"></a> `SpaceAround`   | `7`   | Items evenly distributed with equal space around each  |
| <a id="spacebetween"></a> `SpaceBetween` | `6`   | Items evenly distributed with first/last at edges      |
| <a id="spaceevenly"></a> `SpaceEvenly`   | `8`   | Items evenly distributed with equal space between each |
| <a id="start"></a> `Start`               | `0`   | Items packed toward the start of the main axis         |
| <a id="stretch"></a> `Stretch`           | `5`   | Items stretched along the main axis                    |
