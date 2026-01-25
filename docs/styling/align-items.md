---
title: Align Items
sidebar_position: 6
---

# 🎯 Align Items

**Control alignment of items along the cross axis.**

The `alignItems` property defines the default behavior for how flexible items are laid out along the **cross axis** on the current line. Think of it as the `justifyContent` for the cross axis (perpendicular to the main axis).

> [!TIP]
> 🔗 **MDN Documentation**: [align-items](https://developer.mozilla.org/en-US/docs/Web/CSS/align-items)

## 🎛️ Values

| Value           | Description                                                                                     |
| :-------------- | :---------------------------------------------------------------------------------------------- |
| **`Stretch`**   | **Default**. Items stretch to fill the container's cross size (respecting min/max constraints). |
| **`FlexStart`** | Items align to the start edge of the cross axis.                                                |
| **`FlexEnd`**   | Items align to the end edge of the cross axis.                                                  |
| **`Center`**    | Items align in the center of the cross axis.                                                    |
| **`Baseline`**  | Items align based on their text baseline.                                                       |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const style = new Style({
  display: Display.Flex,
  size: { width: 50, height: 30 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const labelStyle = new Style({
  flexGrow: 1,
});

// Create children with different heights to demonstrate alignment
const child1 = tree.newLeaf(new Style({ size: { width: 40, height: 20 } }));
const child2 = tree.newLeaf(new Style({ size: { width: 40, height: 40 } }));
const child3 = tree.newLeaf(new Style({ size: { width: 40, height: 60 } }));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 300, height: 100 },
  gap: { width: 10, height: 0 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },

  // CHANGE THIS TO TEST DIFFERENT VALUES
  alignItems: AlignItems.Center,
  // Options: FlexStart, FlexEnd, Stretch, Baseline
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Align Self](./align-self.md)** - Override this property for individual items.
- **[Align Content](./align-content.md)** - Align standard lines in multi-line containers.
