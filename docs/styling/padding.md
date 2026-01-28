---
title: Padding
sidebar_position: 5
---

# Padding

Control the inner spacing of an element.

**`padding`** creates space between an element's content and its border.

## Example

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.padding = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `padding` is a `Rect` containing `left`, `right`, `top`, `bottom`.
- It affects the inner size available for children.

## Next Steps

- [Border](./border.md)
- [Gap](./gap.md)
