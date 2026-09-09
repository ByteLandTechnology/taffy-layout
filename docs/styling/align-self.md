---
title: Align Self
sidebar_position: 15
---

# Align Self

**Override the parent's `alignItems` for a specific item.**

The `alignSelf` property overrides `alignItems` for an individual Flexbox or Grid item. Grid's `justifySelf` similarly overrides `justifyItems` and accepts the same `AlignSelf` values.

## Values

| Value                       | Description                                            |
| :-------------------------- | :----------------------------------------------------- |
| **`Auto`**                  | **Default**. Inherits the parent's `alignItems` value. |
| **`Stretch`**               | Item stretches to fill the container's cross size.     |
| **`Start` / `End`**         | Item aligns to the container's logical start/end edge. |
| **`SelfStart` / `SelfEnd`** | Item aligns using its own direction.                   |
| **`FlexStart`**             | Item aligns to the start edge.                         |
| **`FlexEnd`**               | Item aligns to the end edge.                           |
| **`Center`**                | Item aligns in the center.                             |
| **`Baseline`**              | Item aligns based on its baseline.                     |

The safe variants are `SafeStart`, `SafeEnd`, `SafeFlexStart`, `SafeFlexEnd`, `SafeCenter`, `SafeSelfStart`, and `SafeSelfEnd`. They fall back to start alignment when needed to prevent overflow before the start edge.

Set either self property to `AlignSelf.Auto` or `undefined` to restore the parent's alignment. For an unset self property, its direct getter returns `AlignSelf.Auto`, while `style.get("alignSelf")` or `style.get("justifySelf")` returns `undefined`.

## Example

```tsx live
const tree = new TaffyTree();

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  alignItems: AlignItems.FlexStart, // Default alignment is Top
  size: { width: 300, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
  gap: { width: 10, height: 0 },
});

const standardItem = new Style({ size: { width: 50, height: 40 } });

// This item overrides the parent's FlexStart alignment
const selfAlignedItem = new Style({
  size: { width: 50, height: 40 },
  alignSelf: AlignSelf.FlexEnd,
});

const child1 = tree.newLeaf(standardItem);
const child2 = tree.newLeaf(selfAlignedItem); // Will appear at the bottom
const child3 = tree.newLeaf(standardItem);

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Next Steps

- [Align Content](./align-content.md)
- [Align Items](./align-items.md)
