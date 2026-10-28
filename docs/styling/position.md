---
title: Positioning
sidebar_position: 22
---

# Position

**Control how an element is placed in the document.**

The `position` property determines if an element participates in the normal layout flow or if it is removed and positioned manually.

## Values

| Value          | Description                                                                                                                                                  |
| :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`Static`**   | **Default**. The element remains in the document flow and ignores `inset`. It is not a containing block for absolutely positioned descendants.               |
| **`Relative`** | The element remains in the document flow. `inset` offsets move it visually, but it still takes up space in its original location.                            |
| **`Absolute`** | The element is **removed from the flow** and positioned in its containing block: the nearest ancestor that is not `Static`, or the root.                     |
| **`Fixed`**    | The element is **removed from the flow** and positioned relative to the root.                                                                                |
| **`Sticky`**   | Laid out like `Static`, but acts as a containing block for absolutely positioned descendants. Sticky offsets depend on scrolling and are left to the caller. |

`Relative`, `Absolute`, `Fixed` and `Sticky` elements are containing blocks for absolutely positioned descendants. The `Layout.x` / `Layout.y` of an `Absolute` or `Fixed` element are relative to the border box of its containing block, which is not necessarily its parent.

## Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  position: Position.Relative, // containing block of the absolute child
  size: { width: 300, height: 120 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({
  size: { width: 60, height: 60 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});

const absoluteItem = new Style({
  position: Position.Absolute,
  size: { width: 50, height: 50 },
  inset: { top: 0, right: 0, left: "auto", bottom: "auto" },
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(standardItem);

// This child floats over the others
const childAbs = tree.newLeaf(absoluteItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, childAbs]);

tree.computeLayout(root, { width: 300, height: 120 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Inset](./inset.md)
- [Display](./display.md)
