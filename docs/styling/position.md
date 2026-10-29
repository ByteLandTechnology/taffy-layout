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
| **`Absolute`** | The element is **removed from the flow** and positioned in its containing block: the nearest ancestor that is not `Static`, or the initial containing block. |
| **`Fixed`**    | The element is **removed from the flow** and positioned in the initial containing block, regardless of positioned ancestors.                                 |
| **`Sticky`**   | Laid out like `Static`, but acts as a containing block for absolutely positioned descendants. Sticky offsets depend on scrolling and are left to the caller. |

`Relative`, `Absolute`, `Fixed` and `Sticky` elements establish containing blocks for absolutely positioned descendants. Insets and percentage sizes normally use that ancestor's padding box; Grid placement can further restrict the positioning area.

The **initial containing block** is anchored at the canvas origin. In each axis, a definite value passed to `computeLayout(root, availableSpace)` supplies its size, excluding any reserved root scrollbar space. If that axis is `"min-content"` or `"max-content"`, positioning falls back to the root's padding box in that axis, excluding reserved scrollbar space. It is therefore not necessarily the root's size or position. For example, with a static 100×100 root and available space of 300×200, a 10×10 `Fixed` child or `Absolute` child without a positioned ancestor at `right: 0, bottom: 0` is placed at canvas coordinates `(290, 190)`.

Returned coordinates use a separate reference: in-flow nodes have `Layout.x` / `Layout.y` relative to their parent's border box; absolute descendants with a positioned ancestor use that ancestor's border box. Fixed descendants and absolute descendants without a positioned ancestor are reported relative to the **root's border box**, even when their positioning area is the initial containing block. Add that reference node's canvas position to draw them. The root's own coordinates are canvas coordinates and include margins, direction and applicable insets. See [Visual Debugging](../advanced/debugging.md) for a traversal that handles these cases.

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
