---
title: Overflow
sidebar_position: 24
---

# Overflow

**Control behavior when content exceeds the container size.**

The `overflow` property configures layout behavior when content is larger than the container's box. Set both axes with `{ x, y }`, or use `overflowX` and `overflowY`. Your renderer is responsible for clipping, scrolling, and drawing scrollbars.

## Values

| Value         | Description                                                                                                      |
| :------------ | :--------------------------------------------------------------------------------------------------------------- |
| **`Visible`** | **Default**. Uses content-based automatic minimum sizing; overflow can contribute to the parent's scroll region. |
| **`Clip`**    | Uses content-based automatic minimum sizing; overflowing content does not enlarge the parent's scroll region.    |
| **`Hidden`**  | Uses a zero automatic minimum for Flex/Grid items; reserves no scrollbar space.                                  |
| **`Scroll`**  | Uses a zero automatic minimum for Flex/Grid items and reserves configured scrollbar space.                       |

## Scrollbar Sizing

`Overflow.Scroll` reserves `scrollbarWidth` space even when content fits; the default width is `0`. Taffy computes `scrollbarSize`, which you can read from the layout output. Horizontal overflow reserves height, and vertical overflow reserves width.

`contentSize`, `contentWidth`, and `contentHeight` expose reachable content extents measured from the scroll origin. See [The Layout Object](../core-concepts/objects-layout.md) for their coordinate semantics, including RTL.

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
