---
title: Margin
sidebar_position: 4
---

# Margin

Control the outer spacing of an element.

**`margin`** creates space around an element, outside of any defined borders.

## Example

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: "100%", height: "100%" };
const innerNode = tree.newLeaf(childStyle);

const boxStyle = new Style();
boxStyle.size = { width: 100, height: 100 };
boxStyle.margin = { left: 20, top: 20, right: 20, bottom: 20 };
const boxNode = tree.newWithChildren(boxStyle, [innerNode]);

const rootStyle = new Style();
rootStyle.size = { width: 200, height: 200 };
rootStyle.display = Display.Flex;

const root = tree.newWithChildren(rootStyle, [boxNode]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Quick Notes

- `margin` is a `Rect` containing `left`, `right`, `top`, `bottom`.
- It accepts `Auto` to center content (like `margin: auto`).

## Next Steps

- [Padding](./padding.md)
- [Border](./border.md)
