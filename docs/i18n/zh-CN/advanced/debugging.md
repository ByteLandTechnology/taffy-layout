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
import { Position, Style, TaffyTree } from "taffy-layout";

// 模拟渲染器
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// 可视化调试器函数
function debugDraw(
  node: bigint,
  parentOrigin = { x: 0, y: 0 },
  absoluteOrigin = parentOrigin,
  rootOrigin = parentOrigin,
) {
  const style = tree.getStyle(node);
  const position = style.position;
  style.free();

  const origin =
    position === Position.Fixed
      ? rootOrigin
      : position === Position.Absolute
        ? absoluteOrigin
        : parentOrigin;
  const layout = tree.getLayout(node);
  const point = { x: origin.x + layout.x, y: origin.y + layout.y };
  renderer.strokeRect(point.x, point.y, layout.width, layout.height, "red");
  layout.free();

  const childRootOrigin = node === root ? point : rootOrigin;
  const childAbsoluteOrigin =
    node === root || position !== Position.Static ? point : absoluteOrigin;
  for (const child of tree.children(node)) {
    debugDraw(child, point, childAbsoluteOrigin, childRootOrigin);
  }
}
debugDraw(root);
```

调用 `debugDraw(root)` 时，使用传给 `computeLayout` 的同一个根节点。遍历分别记录直接父节点、最近的非静态祖先及根节点的画布坐标。流内节点使用父节点；绝对定位节点使用定位祖先，没有此类祖先时回退到根节点；固定定位节点始终使用根节点。这是因为 Taffy 将初始包含块中后代的坐标转换为相对于根节点 border box 的坐标，尽管其定位区域由 `availableSpace` 决定。详见[定位](../styling/position.md)。此例只绘制布局矩形；裁剪、滚动和 sticky 偏移由渲染器处理。

## 隔离

如果特定的子树表现异常：

1. 创建一个新的 `TaffyTree`。
2. 仅复制该子树结构。
3. 硬编码输入约束（提供给子树的宽度/高度）。
4. 运行 `computeLayout` 并检查。

这会将问题与外部父约束隔离开来。
