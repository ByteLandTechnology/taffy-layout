---
title: Flex Basis
sidebar_position: 10
---

# Flex Basis

Define the default size of an element before the remaining space is distributed.

The **`flexBasis`** property specifies the initial main size of a flex item. This property determines the size of the content-box unless otherwise specified with `box-sizing`.

## Example

```tsx live
const tree = new TaffyTree();

const style1 = new Style();
style1.flexBasis = 100;
style1.size = { width: "auto", height: "100%" };
const child1 = tree.newLeaf(style1);

const style2 = new Style();
style2.flexBasis = 200;
style2.size = { width: "auto", height: "100%" };
const child2 = tree.newLeaf(style2);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 350, height: 60 };
rootStyle.gap = { width: 8, height: 0 };
rootStyle.padding = { left: 6, right: 6, top: 6, bottom: 6 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 350,
  height: 60,
});

console.log(`Child 1: flexBasis 100, Child 2: flexBasis 200`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `flexBasis` influences `auto` sizing.
- It is similar to `width` (or `height` depending on `flexDirection`) but specific to flex items.

## Next Steps

- [Flex Grow](./flex-grow.md)
- [Flex Shrink](./flex-shrink.md)
