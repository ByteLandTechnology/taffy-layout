[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / Display

# Enumeration: Display

Display mode enumeration

Controls the layout algorithm type for an element. This corresponds to the CSS `display` property
and determines how an element and its children are laid out.

## Example

```typescript
import { Style, Display } from "taffy-layout";

const style = new Style();
style.display = Display.Flex; // Enable flexbox layout
style.display = Display.Grid; // Enable grid layout
style.display = Display.None; // Hide element from layout
```

## Enumeration Members

| Enumeration Member         | Value | Description                                                     |
| -------------------------- | ----- | --------------------------------------------------------------- |
| <a id="block"></a> `Block` | `0`   | Block-level layout where element takes the full available width |
| <a id="flex"></a> `Flex`   | `1`   | Flexbox layout for one-dimensional item arrangement             |
| <a id="grid"></a> `Grid`   | `2`   | CSS Grid layout for two-dimensional item arrangement            |
| <a id="none"></a> `None`   | `3`   | Element is removed from layout calculation entirely             |
