---
title: Sizing
---

# Sizing

Use `size`, `minSize`, and `maxSize` to control dimensions.

## Common Properties

- `size`: main size
- `minSize`: minimum size
- `maxSize`: maximum size

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: 200, height: 100 };
style.minSize = { width: 120, height: 60 };
style.maxSize = { width: 300, height: 160 };

const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 240, height: 140 };
rootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 240,
  height: 140,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## 📏 Width and Height

**Control the precise dimensions of an element.**

Use `size`, `minSize`, and `maxSize` to set boundaries for an element's dimensions.

> [!TIP]
> 🔗 **MDN Documentation**: [width](https://developer.mozilla.org/en-US/docs/Web/CSS/width), [height](https://developer.mozilla.org/en-US/docs/Web/CSS/height), [min-width](https://developer.mozilla.org/en-US/docs/Web/CSS/min-width), [max-width](https://developer.mozilla.org/en-US/docs/Web/CSS/max-width)

## 🎛️ Properties

These properties take a `Size` object containing `width` and `height`.

| Property      | Description                                                              |
| :------------ | :----------------------------------------------------------------------- |
| **`size`**    | Ideal size. If `auto`, size is determined by content or flex/grid rules. |
| **`minSize`** | Minimum size. Prevents the item from shrinking below this value.         |
| **`maxSize`** | Maximum size. Prevents the item from growing above this value.           |

## 📐 Dimension Values

The `width` and `height` properties accept specific value types:

| Value       | Description                                      | Example (JS)                                                                    |
| :---------- | :----------------------------------------------- | :------------------------------------------------------------------------------ |
| **Auto**    | Size to content (or stretch in some flex cases). | `"auto"`                                                                        |
| **Points**  | Exact pixel value.                               | `150`                                                                           |
| **Percent** | Percentage of parent's size.                     | `"50%"` or `0.5` (if using float helpers) usually string `"50%"` in JS binding. |

## 💻 Example

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  // Fixed size
  size: { width: 200, height: 200 },
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  gap: { width: 0, height: 10 },
});

const percentageChild = tree.newLeaf(
  new Style({
    // 80% of parent width, fixed 30px height
    size: { width: "80%", height: 30 },
  }),
);

const minMaxChild = tree.newLeaf(
  new Style({
    // Wants to be 10px, but forced to at least 50px
    size: { width: 10, height: 30 },
    minSize: { width: 50, height: "auto" },
  }),
);

const root = tree.newWithChildren(containerStyle, [
  percentageChild,
  minMaxChild,
]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ Next Steps

- **[Aspect Ratio](./aspect-ratio.md)** - Maintain width/height ratio.
- **[Margin, Padding, Border](./margin-padding-border.md)** - Add spacing around the size.
