[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / AlignItems

# Enumeration: AlignItems

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

| Enumeration Member                 | Value | Description                                      |
| ---------------------------------- | ----- | ------------------------------------------------ |
| <a id="baseline"></a> `Baseline`   | `5`   | Items aligned to their text baselines            |
| <a id="center"></a> `Center`       | `4`   | Items centered along the cross axis              |
| <a id="end"></a> `End`             | `1`   | Items aligned to the end of the cross axis       |
| <a id="flexend"></a> `FlexEnd`     | `3`   | Items aligned to the end of the flex container   |
| <a id="flexstart"></a> `FlexStart` | `2`   | Items aligned to the start of the flex container |
| <a id="start"></a> `Start`         | `0`   | Items aligned to the start of the cross axis     |
| <a id="stretch"></a> `Stretch`     | `6`   | Items stretched to fill the container            |
