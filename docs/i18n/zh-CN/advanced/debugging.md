---
title: 调试
sidebar_position: 1
---

# 调试

**用于检查和排查布局的工具。**

## 打印树

您手头最有力的工具是 `tree.printTree(node)`。它生成树结构、样式配置和计算布局的文本表示。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
```

**示例输出：**

```text
DIV [x: 0    y: 0    w: 100  h: 100  content_w: 100  content_h: 100  border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (1)
└── LEAF [x: 0    y: 0    w: 50   h: 50   content_w: 50   content_h: 50   border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (2)
```

> **注意**：实际输出格式可能因版本而略有不同，但始终显示层次结构和关键约束。

## 可视化调试

如果您渲染到画布或屏幕：

1. **绘制边框**：在每个计算出的布局矩形周围绘制一条彩色 1px 边框。
2. **颜色编码**：为不同的 `display` 类型使用不同的颜色（例如，Flex 用蓝色，Grid 用红色）。

```ts
// 模拟渲染器
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// 可视化调试器函数
function debugDraw(node: any) {
  const layout = tree.getLayout(node);
  renderer.strokeRect(layout.x, layout.y, layout.width, layout.height, "red");

  for (const child of tree.children(node)) {
    debugDraw(child);
  }
}
debugDraw(root);
```

## 隔离

如果特定的子树表现异常：

1. 创建一个新的 `TaffyTree`。
2. 仅复制该子树结构。
3. 硬编码输入约束（提供给子树的宽度/高度）。
4. 运行 `computeLayout` 并检查。

这会将问题与外部父约束隔离开来。
