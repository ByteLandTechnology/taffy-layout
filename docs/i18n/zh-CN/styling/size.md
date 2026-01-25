---
title: 尺寸
---

# 尺寸

使用 `size`、`minSize` 和 `maxSize` 控制元素的尺寸。

## 常用属性

- `size`：主要尺寸
- `minSize`：最小尺寸
- `maxSize`：最大尺寸

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

## 📏 宽度和高度

**精确控制元素的尺寸。**

使用 `size`、`minSize` 和 `maxSize` 为元素的尺寸设置边界。

> [!TIP]
> 🔗 **MDN 文档**：[width](https://developer.mozilla.org/zh-CN/docs/Web/CSS/width)、[height](https://developer.mozilla.org/zh-CN/docs/Web/CSS/height)、[min-width](https://developer.mozilla.org/zh-CN/docs/Web/CSS/min-width)、[max-width](https://developer.mozilla.org/zh-CN/docs/Web/CSS/max-width)

## 🎛️ 属性

这些属性接受包含 `width` 和 `height` 的 `Size` 对象。

| 属性          | 描述                                                     |
| :------------ | :------------------------------------------------------- |
| **`size`**    | 理想尺寸。如果为 `auto`，则由内容或 flex/grid 规则决定。 |
| **`minSize`** | 最小尺寸。防止元素收缩到小于此值。                       |
| **`maxSize`** | 最大尺寸。防止元素增长超过此值。                         |

## 📐 尺寸值

`width` 和 `height` 属性接受以下值类型：

| 值          | 描述                                           | 示例（JS）                                            |
| :---------- | :--------------------------------------------- | :---------------------------------------------------- |
| **Auto**    | 根据内容调整大小（或在某些 flex 情况下拉伸）。 | `"auto"`                                              |
| **Points**  | 精确的像素值。                                 | `150`                                                 |
| **Percent** | 父元素尺寸的百分比。                           | `"50%"` 或 `0.5`（JS 绑定中通常使用字符串 `"50%"`）。 |

## 💻 示例

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  // 固定尺寸
  size: { width: 200, height: 200 },
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  gap: { width: 0, height: 10 },
});

const percentageChild = tree.newLeaf(
  new Style({
    // 父元素宽度的 80%，高度固定 30px
    size: { width: "80%", height: 30 },
  }),
);

const minMaxChild = tree.newLeaf(
  new Style({
    // 希望是 10px，但被强制至少 50px
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

## ⏭️ 后续步骤

- **[宽高比](./aspect-ratio.md)** - 保持宽高比
- **[外边距、内边距、边框](./margin-padding-border.md)** - 添加尺寸周围的间距
