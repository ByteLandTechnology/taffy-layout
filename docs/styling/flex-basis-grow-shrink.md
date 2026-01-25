---
title: Flex Grow, Shrink, and Basis
sidebar_position: 4
---

# Flex Grow, Shrink, and Basis

Control how flex items grow and shrink to fit the available space.

- **`flexGrow`**: Defines the ability for a flex item to grow if necessary.
- **`flexShrink`**: Defines the ability for a flex item to shrink if necessary.
- **`flexBasis`**: Defines the default size of an element before the remaining space is distributed.

## Example

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 240, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 240,
  height: 60,
});

console.log(`Fixed width: 120, grow: 1`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- Without `flexGrow`, items size to content
- `flexBasis` influences `auto` sizing

## Next Steps

- [Justify Content](./justify-content.md)
- [Align Items](./align-items.md) & [Align Content](./align-content.md)
