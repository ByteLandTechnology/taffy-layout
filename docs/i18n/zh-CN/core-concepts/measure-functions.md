---
title: 测量函数
sidebar_position: 5
---

# 测量函数

当叶子节点的尺寸依赖于其内容（例如文本、图像或平台特定的小部件）时，Taffy 无法仅从样式属性计算尺寸。在这种情况下，您必须提供**测量函数**。

## 使用时机

当您的树包含需要基于内容尺寸的节点（例如尺寸为 `"auto"` 的文本节点）时，使用 `computeLayoutWithMeasure()` 代替标准的 `computeLayout()`。Taffy 将为需要内容尺寸的叶子节点调用您的回调。

## 工作原理

测量函数是 Taffy 在布局过程中调用的回调。它问您："给定这些约束，这个内容有多大？"

### 参数

1. **`knownDimensions`**：布局引擎在当前阶段已确定的尺寸提示，不是样式尺寸的拷贝，也不保证已减去 padding 和 border。没有提示的轴为 `undefined`；即使样式设置了固定尺寸，某次回调中该轴也可能为 `undefined`。
2. **`availableSpace`**：每个轴为像素数、`"min-content"` 或 `"max-content"`，表示本次内容测量的约束；数值约束已经扣除 padding、border 和预留的滚动条。
3. **`node`**：被测量节点的 ID（`bigint`）。
4. **`context`**：通过 `newLeafWithContext()` 或 `setNodeContext()` 关联的可选内容数据；未关联时为 `undefined`。
5. **`style`**：节点当前样式的独立副本，包含显式的网格区域行列数。用完后调用 `style.free()`。

普通 `newLeaf()` 创建的叶子也可触发测量回调，context 只用于携带可选的内容数据。修改回调中的样式副本不会更新树。已知尺寸、隐藏节点或缓存命中可能使某次布局无需调用回调。

### 返回值

函数**必须同步**返回一个 `Size` 对象，包含内容的 `width` 和 `height`（以像素为单位），不要再次加上 padding 或 border。Taffy 会结合盒模型、已知尺寸和 min/max 约束计算最终的边框框尺寸。

当前绑定会将回调抛出的异常或无法解析的返回值作为零内容尺寸处理，不会将这些错误重新抛给 `computeLayoutWithMeasure()` 的调用方。需要诊断时，应在回调内记录错误并在布局调用结束后处理；不要依赖回调内的断言向外抛出。

## 示例

```tsx live
const tree = new TaffyTree();

const style = new Style();
// 此节点没有固定尺寸，因此 Taffy 将询问测量函数
style.size = { width: "auto", height: "auto" };

const measuredNode = tree.newLeafWithContext(style, { width: 150, height: 50 });

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
  (knownDims, availableSpace, node, context, measuredStyle) => {
    measuredStyle.free(); // 本例只使用上下文，不需要样式副本
    // 1. 使用本次测量提供的尺寸提示（本例没有 padding 或 border）
    // 2. 否则，基于可用空间或内容固有尺寸计算
    const contentWidth = context?.width ?? 150;
    const contentHeight = context?.height ?? 50;
    const width =
      knownDims.width ??
      (typeof availableSpace.width === "number"
        ? Math.min(availableSpace.width, contentWidth)
        : contentWidth);

    const height = knownDims.height ?? contentHeight;

    return { width, height };
  },
);

const measuredLayout = tree.getLayout(measuredNode);
const measuredSize = measuredLayout.size;
measuredLayout.free();

return (
  <div style={{ display: "flex", gap: 10 }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
      <strong>Measured Size:</strong>
      <br />
      {measuredSize.width} x {measuredSize.height}
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

## 内容更新与缓存

context 中的 JavaScript 对象按引用传递；`getNodeContext()`、`getNodeContextMut()` 和回调收到的是同一个对象。直接修改其属性不会使布局缓存失效。修改内容、字体或其他影响测量的外部数据后，对受影响的叶子调用 `tree.markDirty(node)`，再计算布局。仅替换回调也不会使旧缓存失效。

`tree.setNodeContext(node, value)` 会标记节点及祖先需要重新计算，即使传回同一个对象。测量回调可能在一次计算中调用多次，也可能因缓存或已知尺寸而完全不调用，因此不要依赖调用次数。

## 下一步

- [样式指南 (Styling Guide)](../styling/index.md)
- [布局参考 (Cookbook)](../cookbook/)
