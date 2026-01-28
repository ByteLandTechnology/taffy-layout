---
title: Border
sidebar_position: 6
---

# Border

Control the border width of an element.

**`border`** defines the thickness of the boundary of the box.

## Example

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.flexGrow = 1;
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 150, height: 100 };
boxStyle.display = Display.Flex;
boxStyle.border = { left: 10, top: 10, right: 10, bottom: 10 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `border` is a `Rect` containing `left`, `right`, `top`, `bottom`.
- Taffy only calculates the **space** for the border; rendering the actual border is up to your rendering engine.

## Next Steps

- [Gap](./gap.md)
- [Size](./size.md)
