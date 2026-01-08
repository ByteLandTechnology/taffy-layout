[**Taffy Layout API Documentation**](../index.md)

---

[Taffy Layout API Documentation](../index.md) / AvailableSpace

# Type Alias: AvailableSpace

```ts
type AvailableSpace = number | "min-content" | "max-content";
```

Available space constraint for layout computation.

Specifies how much space is available for a node during layout calculation.
This is passed to `computeLayout()` to define the container constraints.

## Remarks

- Use `number` when you have a fixed container size
- Use `"min-content"` to shrink-wrap to the minimum content size
- Use `"max-content"` to expand to fit all content without wrapping

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
