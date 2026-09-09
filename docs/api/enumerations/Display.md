# Display

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

| Enumeration Member                                  | Value | Description                                                                  |
| --------------------------------------------------- | ----- | ---------------------------------------------------------------------------- |
| <a id="enumeration-member-block"></a> `Block`       | `0`   | Block layout for the element's children, including flow, margins, and floats |
| <a id="enumeration-member-flex"></a> `Flex`         | `1`   | Flexbox layout for one-dimensional item arrangement                          |
| <a id="enumeration-member-flowroot"></a> `FlowRoot` | `4`   | Block layout that always establishes a new block formatting context          |
| <a id="enumeration-member-grid"></a> `Grid`         | `2`   | CSS Grid layout for two-dimensional item arrangement                         |
| <a id="enumeration-member-none"></a> `None`         | `3`   | Element is removed from layout calculation entirely                          |
