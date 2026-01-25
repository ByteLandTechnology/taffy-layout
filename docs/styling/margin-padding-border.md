---
title: Margin, Padding, and Border
sidebar_position: 11
---

# 📦 Margin, Padding, and Border

**Control the Box Model spacing.**

Taffy follows the standard CSS Box Model.

> [!TIP]
> 🔗 **MDN Documentation**: [The Box Model](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model)

## 🖼️ The Box Model

1.  **Content**: The actual item (image, text, or children).
2.  **Padding**: Space between content and the border.
3.  **Border**: The boundary of the box.
4.  **Margin**: Space outside the border, pushing other items away.

## 🎛️ Properties

Each property is a `Rect` containing `left`, `right`, `top`, `bottom`.

| Property      | Description                                                                             |
| :------------ | :-------------------------------------------------------------------------------------- |
| **`margin`**  | Outer spacing. Accepts `Auto` to center content (like `margin: auto`).                  |
| **`padding`** | Inner spacing. Affects the size of the container.                                       |
| **`border`**  | Border width. Taffy only calculates the _space_ for the border; rendering is up to you. |

## 💻 Example

```tsx live
const tree = new TaffyTree();

// Inner content
const contentNode = tree.newLeaf(
  new Style({
    flexGrow: 1, // Fill available space
  }),
);

// Container demonstrating box model
const boxNode = tree.newWithChildren(
  new Style({
    size: { width: 200, height: 120 },
    display: Display.Flex,

    // 1. Margin (Outside)
    margin: { left: 20, top: 20 },

    // 2. Border (Boundary) - Logical width only
    border: { left: 5, right: 5, top: 5, bottom: 5 },

    // 3. Padding (Inside)
    padding: { left: 15, right: 15, top: 15, bottom: 15 },
  }),
  [contentNode],
);

// Root to hold the example
const root = tree.newWithChildren(
  new Style({
    size: { width: 300, height: 200 },
  }),
  [boxNode],
);

tree.computeLayout(root, { width: 300, height: 200 });

// Visualize the hierarchy
console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Gap](./gap.md)** - Spacing between flex/grid items.
- **[Size](./size.md)** - Controlling width and height.
