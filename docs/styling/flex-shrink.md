---
title: Flex Shrink
sidebar_position: 12
---

# Flex Shrink

Control how flex items shrink to fit within the available space.

The **`flexShrink`** property defines the ability for a flex item to shrink if necessary. It specifies the flex shrink factor, which determines how much the flex item will shrink relative to the rest of the flex items in the flex container when negative free space is distributed.

## Example

```tsx live
const tree = new TaffyTree();

const noShrinkStyle = new Style();
noShrinkStyle.flexShrink = 0;
noShrinkStyle.size = { width: 200, height: "100%" };
const child1 = tree.newLeaf(noShrinkStyle);

const shrinkStyle = new Style();
shrinkStyle.flexShrink = 1;
shrinkStyle.size = { width: 200, height: "100%" };
const child2 = tree.newLeaf(shrinkStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 }; // Parent is smaller than total children width
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

console.log(
  `Child 1 (no shrink): 200px, Child 2 (shrink: 1): 200px (will shrink)`,
);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- Default value is `1`, meaning items will shrink to prevent overflow by default.
- Set to `0` to prevent an item from shrinking.

## Next Steps

- [Flex Basis](./flex-basis.md)
- [Justify Content](./justify-content.md)
