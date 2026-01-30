# LayoutProperty

```ts
type LayoutProperty =
  | "order"
  | "position"
  | "x"
  | "y"
  | "size"
  | "width"
  | "height"
  | "contentSize"
  | "contentWidth"
  | "contentHeight"
  | "scrollbarSize"
  | "scrollbarWidth"
  | "scrollbarHeight"
  | "border"
  | "borderLeft"
  | "borderRight"
  | "borderTop"
  | "borderBottom"
  | "padding"
  | "paddingLeft"
  | "paddingRight"
  | "paddingTop"
  | "paddingBottom"
  | "margin"
  | "marginLeft"
  | "marginRight"
  | "marginTop"
  | "marginBottom";
```

Valid property keys for Layout.get() method.

Supports both object properties and individual flat properties.

## Example

```typescript
import { TaffyTree, Style } from "taffy-layout";

const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });
const layout = tree.getLayout(root);
// Object properties
layout.get("position", "size");

// Individual flat properties
layout.get("width", "height", "marginLeft");

// Mixed
layout.get("position", "width", "paddingTop");

tree.free();
```
