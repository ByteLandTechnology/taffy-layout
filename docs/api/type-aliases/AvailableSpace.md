# AvailableSpace

```ts
type AvailableSpace = number | "min-content" | "max-content";
```

Available space constraint for layout computation.

Specifies how much space is available for a node during layout calculation.
This is passed to `computeLayout()` to define the container constraints.

## Remarks

- Use `number` to offer a definite amount of space
- Use `"min-content"` for an intrinsic minimum-content constraint
- Use `"max-content"` for an intrinsic maximum-content constraint
  Available space does not override explicit sizes, min/max constraints, or
  Flexbox wrapping rules. It is not a guaranteed final size.

## Example

```typescript
import init, {
  TaffyTree,
  Style,
  type AvailableSpace,
  type Size,
} from "taffy-layout";

await init();
const tree = new TaffyTree();
const root: bigint = tree.newLeaf(new Style());

// Fixed size container with type annotation
const fixedSpace: Size<AvailableSpace> = {
  width: 800,
  height: 600,
};
tree.computeLayout(root, fixedSpace);

// Flexible width, fixed height
const flexibleSpace: Size<AvailableSpace> = {
  width: "max-content",
  height: 400,
};
tree.computeLayout(root, flexibleSpace);
```
