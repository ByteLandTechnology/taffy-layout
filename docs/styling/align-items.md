---
title: Align Items
sidebar_position: 14
---

# Align Items

**Control alignment of items along the cross axis.**

The `alignItems` property controls Flexbox items along the **cross axis** and Grid items along the block axis. Grid's `justifyItems` uses the same `AlignItems` values for the inline axis. Both properties are initially `undefined`, allowing the layout algorithm to choose its default alignment, commonly stretching auto-sized items.

## Values

| Value                       | Description                                                                        |
| :-------------------------- | :--------------------------------------------------------------------------------- |
| **`Stretch`**               | Items stretch to fill the container's cross size (respecting min/max constraints). |
| **`Start` / `End`**         | Align to the container's logical start/end edge.                                   |
| **`SelfStart` / `SelfEnd`** | Align using the item's own direction rather than the container's direction.        |
| **`FlexStart`**             | Items align to the start edge of the cross axis.                                   |
| **`FlexEnd`**               | Items align to the end edge of the cross axis.                                     |
| **`Center`**                | Items align in the center of the cross axis.                                       |
| **`Baseline`**              | Items align using the baselines calculated by Taffy.                               |

The measurement callback returns dimensions only; it does not accept a custom text baseline.

`SafeStart`, `SafeEnd`, `SafeFlexStart`, `SafeFlexEnd`, `SafeCenter`, `SafeSelfStart`, and `SafeSelfEnd` provide safe alignment. When the requested alignment would place overflowing content before the start edge, safe alignment falls back to start alignment. The corresponding values without `Safe` retain their requested alignment even when content overflows.

## Example

```tsx live
const tree = new TaffyTree();

const style = new Style({
  display: Display.Flex,
  size: { width: 50, height: 30 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const labelStyle = new Style({
  flexGrow: 1,
});

// Create children with different heights to demonstrate alignment
const child1 = tree.newLeaf(new Style({ size: { width: 50, height: 20 } }));
const child2 = tree.newLeaf(new Style({ size: { width: 50, height: 40 } }));
const child3 = tree.newLeaf(new Style({ size: { width: 50, height: 60 } }));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 300, height: 100 },
  gap: { width: 10, height: 0 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },

  // CHANGE THIS TO TEST DIFFERENT VALUES
  alignItems: AlignItems.Center,
  // Options: FlexStart, FlexEnd, Stretch, Baseline
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Align Self](./align-self.md)
- [Justify Content](./justify-content.md)
