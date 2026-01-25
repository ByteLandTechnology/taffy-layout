---
title: Overflow（溢出）
sidebar_position: 12
---

# 🌊 Overflow（溢出）

**控制内容超出容器尺寸时的行为。**

`overflow` 属性指定当内容大于容器盒子时发生的情况。

> [!TIP]
> 🔗 **MDN 文档**: [overflow](https://developer.mozilla.org/zh-CN/docs/Web/CSS/overflow)

## 🎛️ 取值

| 值            | 描述                                                       |
| :------------ | :--------------------------------------------------------- |
| **`Visible`** | **默认值**。内容流出容器外。                               |
| **`Hidden`**  | 内容在容器边缘被裁剪。                                     |
| **`Scroll`**  | Taffy 为滚动条预留空间（如果已配置），但它不会渲染滚动条。 |

## 📜 滚动条尺寸

在 Taffy 中，`Overflow.Scroll` 通常用于表示节点*可以*滚动。Taffy 会计算 `scrollbarSize`，你可以从布局输出中读取它。

```ts
const tree = new TaffyTree();
const style = new Style({
  overflow: { x: Overflow.Scroll, y: Overflow.Scroll },
  scrollbarWidth: 15, // 用于设置预估滚动条尺寸的辅助选项
});

// 布局计算后：
const node = tree.newLeaf(style);
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(
  `Scrollbar Size: ${layout.scrollbarWidth} x ${layout.scrollbarHeight}`,
);
```

## 💻 示例

```tsx live
const tree = new TaffyTree();

const container = tree.newLeaf(
  new Style({
    size: { width: 100, height: 100 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    // 尝试将其改为 Hidden
    overflow: { x: Overflow.Visible, y: Overflow.Visible },
  }),
);

const bigContent = tree.newLeaf(
  new Style({
    size: { width: 200, height: 200 },
  }),
);

tree.addChild(container, bigContent);

tree.computeLayout(container, { width: 100, height: 100 });

return <TaffyTreePreview tree={tree} root={container} />;
```

## ⏭️ 后续步骤

- **[Size（尺寸）](./size.md)** - 限制容器尺寸。
