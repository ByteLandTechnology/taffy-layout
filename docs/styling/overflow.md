---
title: Overflow
sidebar_position: 24
---

# Overflow

**Control behavior when content exceeds the container size.**

The `overflow` property specifies what happens if content is larger than the container's box.

## Values

| Value         | Description                                                                          |
| :------------ | :----------------------------------------------------------------------------------- |
| **`Visible`** | **Default**. Content flows outside the container.                                    |
| **`Hidden`**  | Content is clipped at the container edge.                                            |
| **`Scroll`**  | Taffy reserves space for scrollbars (if configured), though it does not render them. |

## Scrollbar Sizing

In Taffy, `Overflow.Scroll` is often used to signal that a node _can_ scroll. Taffy computes `scrollbarSize` which you can read from the layout output.

```ts
const tree = new TaffyTree();
const style = new Style({
  overflow: { x: Overflow.Scroll, y: Overflow.Scroll },
  scrollbarWidth: 15, // Helper to set estimated scrollbar size
});

// After layout computation:
const node = tree.newLeaf(style);
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(
  `Scrollbar Size: ${layout.scrollbarWidth} x ${layout.scrollbarHeight}`,
);
```

## Example

```tsx live
const tree = new TaffyTree();

const container = tree.newLeaf(
  new Style({
    size: { width: 100, height: 100 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    // Try changing this to Hidden
    overflow: { x: Overflow.Visible, y: Overflow.Visible },
  }),
);

const bigContent = tree.newLeaf(
  new Style({
    size: { width: 200, height: 200 },
  }),
);

tree.addChild(container, bigContent);

tree.computeLayout(container, { width: 100, height: 100 });

return <TaffyTreePreview tree={tree} root={container} />;
```

## Next Steps

- [Core Concepts](../core-concepts/index.md)
- [Size](./size.md)
