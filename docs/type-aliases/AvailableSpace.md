[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / AvailableSpace

# Type Alias: AvailableSpace

```ts
type AvailableSpace =
  | {
      Definite: number;
    }
  | "MinContent"
  | "MaxContent";
```

Available space constraint for layout computation.

Specifies how much space is available for a node during layout calculation.
This is passed to `computeLayout()` to define the container constraints.

## Remarks

- Use `{ Definite: number }` when you have a fixed container size
- Use `"MinContent"` to shrink-wrap to the minimum content size
- Use `"MaxContent"` to expand to fit all content without wrapping

<details>
<summary><strong>TypeScript Example</strong></summary>

```typescript
import init, {
  TaffyTree,
  Style,
  type AvailableSpace,
  type Size,
} from "taffy-js";

await init();
const tree = new TaffyTree();
const root: bigint = tree.newLeaf(new Style());

// Fixed size container with type annotation
const fixedSpace: Size<AvailableSpace> = {
  width: { Definite: 800 },
  height: { Definite: 600 },
};
tree.computeLayout(root, fixedSpace);

// Flexible width, fixed height
const flexibleSpace: Size<AvailableSpace> = {
  width: "MaxContent",
  height: { Definite: 400 },
};
tree.computeLayout(root, flexibleSpace);
```

</details>
