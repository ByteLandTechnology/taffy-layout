[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / AlignContent

# Enumeration: AlignContent

Multi-line content alignment enumeration

Controls the distribution of space between and around content items along the cross axis
in a multi-line flex container. This corresponds to the CSS `align-content` property.

**Note**: This property only has effect when `flex-wrap` is set to `Wrap` or `WrapReverse`.

## Example

```typescript
import { Style, AlignContent, FlexWrap } from "taffy-layout";

const style = new Style();
style.flexWrap = FlexWrap.Wrap;
style.alignContent = AlignContent.SpaceBetween; // Distribute lines evenly
```

## Enumeration Members

| Enumeration Member                       | Value | Description                                            |
| ---------------------------------------- | ----- | ------------------------------------------------------ |
| <a id="center"></a> `Center`             | `4`   | Lines centered within the container                    |
| <a id="end"></a> `End`                   | `1`   | Lines packed toward the end of the cross axis          |
| <a id="flexend"></a> `FlexEnd`           | `3`   | Lines packed toward the end of the flex container      |
| <a id="flexstart"></a> `FlexStart`       | `2`   | Lines packed toward the start of the flex container    |
| <a id="spacearound"></a> `SpaceAround`   | `7`   | Lines evenly distributed with equal space around each  |
| <a id="spacebetween"></a> `SpaceBetween` | `6`   | Lines evenly distributed with first/last at edges      |
| <a id="spaceevenly"></a> `SpaceEvenly`   | `8`   | Lines evenly distributed with equal space between each |
| <a id="start"></a> `Start`               | `0`   | Lines packed toward the start of the cross axis        |
| <a id="stretch"></a> `Stretch`           | `5`   | Lines stretched to fill the container                  |
