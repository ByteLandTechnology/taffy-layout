---
title: Configuration
sidebar_position: 5
---

# Configuration

**Optimize Taffy for your specific use case.**

Taffy works out-of-the-box, but you can tune it for performance, precision, or resource constraints.

## Capacity Pre-allocation

If you know roughly how many nodes you'll need, initializing the tree with a capacity can reduce memory re-allocations and improve startup performance.

```tsx live
// Initialize with capacity for 1,000 nodes
const tree = TaffyTree.withCapacity(1000);
console.log(`Initial Node Capacity: ${tree.totalNodeCount()}`); // 0 actual nodes

const style = new Style({
  display: Display.Flex,
  size: { width: 200, height: 40 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const root = tree.newLeaf(style);
tree.computeLayout(root, { width: 200, height: 40 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Rounding Settings

Control whether layout values are snapped to whole integers (pixels) or kept as precise floats.

| Setting              | Function                 | Description                                                                 |
| :------------------- | :----------------------- | :-------------------------------------------------------------------------- |
| **Enable Rounding**  | `tree.enableRounding()`  | **Default**. Rounds values to nearest pixel. Prevents blurry borders in UI. |
| **Disable Rounding** | `tree.disableRounding()` | Uses high-precision floats. Best for vector graphics or zoomable UIs.       |

```tsx live
const tree = new TaffyTree();

// Create two items that would sum to 101px
// 50.5 + 50.5 = 101
const style = new Style({
  size: { width: 50.5, height: 50 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});
const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 150, height: 60 },
  alignItems: AlignItems.Center,
});
const root = tree.newWithChildren(rootStyle, [child1, child2]);

// 1. Default (Rounding Enabled)
tree.computeLayout(root, { width: 150, height: 60 });
let layout1 = tree.getLayout(child1);
// layout1.width might be rounded to 51 or 50 depending on algorithm

// 2. Disable Rounding
tree.disableRounding();
tree.computeLayout(root, { width: 150, height: 60 });
layout1 = tree.getLayout(child1);
// layout1.width will be exactly 50.5

console.log(`Precise Width: ${layout1.width}`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Memory Management

Although Taffy's JavaScript bindings use `FinalizationRegistry` to automatically clean up WASM memory when `TaffyTree` objects are garbage collected, relying solely on the GC can be insufficient for high-performance applications (like game loops) where trees are created frequently.

To prevent memory spikes or leaks in the WASM heap, you should explicitly manage memory:

- **Reuse (Recommended):** Use `.clear()` to reset a tree without deallocating its memory. This is ideal for game loops or recursive layouts, as it avoids constant allocation overhead.
- **Dispose:** Use `.free()` if you are completely done with a tree and want to release its memory immediately.

```ts
const tree = new TaffyTree();

// ... use tree ...

// Option 1: Reuse the tree (Recommended)
// Clears all nodes but keeps memory allocated
tree.clear();

// Option 2: Free completely
tree.free();
```

## Next Steps

- 🎨 **[Styling Guide](../styling/index.md)** - Learn about Flexbox and Grid.
- 🛠️ **[Advanced Topics](../advanced/index.md)** - Debugging and Internals.
