---
title: Performance
sidebar_position: 2
---

# Performance

**Tips for keeping Taffy layouts blazing fast.**

Taffy is designed to be efficient and high-performance, but specific patterns can affect performance.

## 1. Pre-Allocate Capacity

If you know the node count, use `withCapacity` to avoid re-allocations.

```tsx live
const tree = TaffyTree.withCapacity(2000);
console.log(tree.totalNodeCount());

const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Row;
containerStyle.size = { width: 220, height: 70 };
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const itemStyle = new Style();
itemStyle.flexGrow = 1;

const first = tree.newLeaf(itemStyle);
const second = tree.newLeaf(itemStyle);
const root = tree.newWithChildren(containerStyle, [first, second]);

tree.computeLayout(root, { width: 220, height: 70 });

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div
      style={{
        padding: "8px 12px",
        borderRadius: 8,
        background: "rgba(0, 122, 255, 0.12)",
        color: "#0f172a",
        fontSize: 12,
        fontWeight: 600,
        display: "inline-flex",
        alignItems: "center",
      }}
    >
      Capacity: {tree.totalNodeCount()}
    </div>
  </div>
);
```

## 2. Incremental Layouts

Only changed nodes are recalculated. Taffy acts **lazily** and only recomputes the branch affected by the change.

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: 120, height: 60 };
const child = tree.newLeaf(childStyle);
const root = tree.newWithChildren(
  new Style({
    display: Display.Flex,
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  }),
  [child],
);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Common Pitfalls

### 1. Excessive Nesting

Every level of depth adds complexity to the recursive algorithm.

- **Bad**: `View -> View -> View -> Button` just for padding.
- **Good**: Use `padding` on the parent instead of wrapper nodes.

### 2. Deep Measurement Functions

Custom measure functions (for text/images) are called frequently.

- **Optimize**: Ensure your measurement callback is fast. Avoid DOM reflows or heavy calculations inside measurements.

## Optimization Patterns

### Reuse Styles

Creating `Style` objects in tight loops (e.g., game rendering) can be expensive in JS.
Reuse definition objects where possible.

```ts
// ✅ Good
const tree = new TaffyTree();
const ITEM_STYLE = new Style({ flexGrow: 1 });
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(ITEM_STYLE);
}
```

```ts
// ❌ Avoid if items are identical
const tree = new TaffyTree();
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(new Style({ flexGrow: 1 }));
}
```

### Batch Property Access

Use batch getters and setters to minimize WASM bridge overhead.

**Style Batch Update:**
Instead of setting properties one by one:

```ts
const style = new Style();
// ❌ Multiple calls = High overhead
style.display = Display.Flex;
style.width = 100;
style.height = 100;
```

Use `set()`:

```ts
const style = new Style();
// ✅ Single call = Low overhead
style.set({
  display: Display.Flex,
  width: 100,
  height: 100,
});
```

**Layout Batch Read:**
Instead of reading layout properties individually:

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ❌ Multiple calls
const layout = tree.getLayout(node);
const x = layout.x;
const y = layout.y;
const w = layout.width;
const h = layout.height;
```

Use `get()`:

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ✅ Single call returns array of values
const layout = tree.getLayout(node);
const [x, y, w, h] = layout.get("x", "y", "width", "height");
```

## Benchmarking

Use `performance.now()` to measure your layout pass.

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());

const start = performance.now();
tree.computeLayout(root, { width: 1000, height: 1000 });
const end = performance.now();
console.log(`Layout took ${end - start}ms`);
```

## Next Steps

- [Measure Functions](../core-concepts/measure-functions.md)
- [Debugging Layouts](./debugging.md)
