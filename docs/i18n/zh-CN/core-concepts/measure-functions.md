---
title: 测量函数
sidebar_position: 5
---

# 测量函数

当叶子节点的尺寸依赖于其内容（例如文本、图像或平台特定的小部件）时，Taffy 无法仅从样式属性计算尺寸。在这种情况下，您必须提供**测量函数**。

## 使用时机

当您的树包含需要基于内容尺寸的节点（例如 `width: auto` 或 `measure` 模式）时，使用 `computeLayoutWithMeasure()` 代替标准的 `computeLayout()`。Taffy 将为需要内容尺寸的叶子节点调用您的回调。

## 工作原理

测量函数是 Taffy 在布局过程中调用的回调。它问您："给定这些约束，这个内容有多大？"

### 参数

1. **`knownDimensions`**: 在节点样式中明确定义的尺寸（例如如果设置了 `width: 100`，`knownDimensions.width` 将是 `100`）。
2. **`availableSpace`**: 父节点提供的空间。这约束了内容可以有多大。

### 返回值

函数**必须**返回一个 `Size` 对象，包含测量的 `width` 和 `height`（以像素为单位）。

## 示例

```tsx live
const tree = new TaffyTree();

const style = new Style();
// 此节点没有固定尺寸，因此 Taffy 将询问测量函数
style.size = { width: "auto", height: "auto" };

const measuredNode = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.size = { width: 300, height: 100 };
rootStyle.alignItems = AlignItems.Center;
rootStyle.justifyContent = JustifyContent.Center;

const root = tree.newWithChildren(rootStyle, [measuredNode]);

// 我们使用 computeLayoutWithMeasure 而不是 computeLayout
tree.computeLayoutWithMeasure(
  root,
  { width: 300, height: 100 },
  (knownDims, availableSpace) => {
    // 1. 检查是否有已知尺寸（样式覆盖）
    // 2. 否则，基于可用空间或内容固有尺寸计算
    const width =
      knownDims.width ??
      (typeof availableSpace.width === "number"
        ? Math.min(availableSpace.width, 150)
        : 150);

    const height = knownDims.height ?? 50;

    return { width, height };
  },
);

return (
  <div style={{ display: "flex", gap: 10 }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
      <strong>Measured Size:</strong>
      <br />
      {tree.getLayout(measuredNode).width} x{" "}
      {tree.getLayout(measuredNode).height}
    </div>
  </div>
);
```

## 典型用例

- **文本布局**: 基于字体大小、文本内容和换行宽度计算宽度/高度。
- **图像**: 返回图像的固有尺寸。
- **原生 UI 小部件**: 包装具有自己尺寸逻辑的平台特定控件。

## 性能提示

- **缓存结果**: 测量可能很昂贵。基于输入（`knownDimensions`、`availableSpace`、内容字符串等）缓存结果以避免重新计算相同的测量。
- **避免副作用**: 测量函数应该是纯函数。不要在其中修改 DOM 或外部状态。

## 下一步

- [样式指南 (Styling Guide)](../styling/index.md)
- [布局参考 (Cookbook)](../cookbook/)
