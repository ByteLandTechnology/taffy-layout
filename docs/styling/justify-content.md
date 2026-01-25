---
title: Justify Content
sidebar_position: 5
---

# ↔️ Justify Content

**Align items along the main axis.**

The `justifyContent` property aligns items along the **main axis** (horizontal if `flexDirection` is `Row`; vertical if `Column`).

> [!TIP]
> 🔗 **MDN Documentation**: [justify-content](https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content)

## 🎛️ Values

| Value              | Description                                                                      |
| :----------------- | :------------------------------------------------------------------------------- |
| **`FlexStart`**    | **Default**. Items pack toward the start of the line.                            |
| **`FlexEnd`**      | Items pack toward the end of the line.                                           |
| **`Center`**       | Items are centered along the line.                                               |
| **`SpaceBetween`** | Items are evenly distributed. First item at start, last item at end.             |
| **`SpaceAround`**  | Items are evenly distributed with equal space around them.                       |
| **`SpaceEvenly`**  | Items are evenly distributed with equal space between any two items (and edges). |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const style = new Style({
  size: { width: 40, height: 40 },
});

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);
const child3 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  // CHANGE THIS to test different values
  justifyContent: JustifyContent.SpaceBetween,
  size: { width: 300, height: 80 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 80 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## API Reference

- [JustifyContent Enum](../../api/enumerations/JustifyContent.md)

## ⏭️ Next Steps

- **[Align Items](./align-items.md)** - Align items on the cross axis.
