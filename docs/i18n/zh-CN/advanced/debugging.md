---
title: 调试
sidebar_position: 1
---

# 调试

**用于检查和排查布局的工具。**

## 打印树

`tree.printTree(node)` 返回包含节点层次、布局类型、位置、尺寸及边缘尺寸的字符串。使用 `console.log()` 才会打印它；它不会列出完整的样式配置。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
```

**示例输出：**

```text
└──  LEAF [x: 0    y: 0    w: 0    h: 0    content_w: 0    content_h: 0    border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (4294967297)
```

这个空叶子没有固定尺寸或内容测量，因此尺寸为零；传入的可用空间不会强制它填满容器。括号中的节点 ID 应视为不透明值。

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
function debugDraw(node: bigint, parentX = 0, parentY = 0) {
  const layout = tree.getLayout(node);
  const x = parentX + layout.x;
  const y = parentY + layout.y;
  renderer.strokeRect(x, y, layout.width, layout.height, "red");
  layout.free();

  for (const child of tree.children(node)) {
    debugDraw(child, x, y);
  }
}
debugDraw(root);
```

布局坐标相对于直接父节点；绘制嵌套树时需要像上例一样累加祖先偏移。

## 隔离

如果特定的子树表现异常：

1. 创建一个新的 `TaffyTree`。
2. 仅复制该子树结构。
3. 硬编码输入约束（提供给子树的宽度/高度）。
4. 运行 `computeLayout` 并检查。

这会将问题与外部父约束隔离开来。
