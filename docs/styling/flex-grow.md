---
title: Flex Grow
sidebar_position: 11
---

# Flex Grow

Control how flex items grow to fill the available space.

The **`flexGrow`** property defines the ability for a flex item to grow if necessary. It accepts a unitless value that serves as a proportion. It dictates what amount of the available space inside the flex container the item should take up.

## Example

```tsx live
const tree = new TaffyTree();

const fixedStyle = new Style();
fixedStyle.size = { width: 100, height: "100%" };
const child1 = tree.newLeaf(fixedStyle);

const growStyle = new Style();
growStyle.flexGrow = 1;
growStyle.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(growStyle);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 300, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 300,
  height: 60,
});

console.log(`Child 1: 100px, Child 2: flexible (grow: 1)`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- Without `flexGrow`, items size to their content or defined `size`.
- If all items have `flexGrow: 1`, the remaining space in the container will be distributed equally to all children.

## Next Steps

- [Flex Shrink](./flex-shrink.md)
- [Flex Basis](./flex-basis.md)
