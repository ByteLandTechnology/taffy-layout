---
title: Style 对象
sidebar_position: 1
---

# Style 对象

**`Style`** 对象定义了单个节点的布局规则。它包含决定节点如何调整大小、定位以及如何排列其子节点的属性。

## 关键职责

- **布局模式与定位**：`display` 选择 Flexbox、Grid、Block 等布局算法；`position` 独立控制相对或绝对定位。
- **尺寸**：定义宽度、高度、宽高比以及最小/最大限制。
- **间距**：控制外边距、内边距、边框和间距 (gap)。
- **对齐**：指定子节点如何沿主轴和交叉轴对齐。

## 用法

样式通常在创建节点时创建并传递给节点。

```typescript
const style = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 100, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
```

`new Style()` 的 `display` 默认为 `Display.Flex`。创建节点和调用 `setStyle()` 时会复制样式；修改原对象或 `getStyle()` 返回的副本后，需要再次调用 `tree.setStyle(node, style)` 才会更新节点。

设置非 `Auto` 对齐值后，`get()` 与直接属性访问返回相同的公开枚举值。未设置或设为 `Auto` 的 `alignSelf` / `justifySelf` 保留例外：直接属性返回 `AlignSelf.Auto`，`get()` 返回 `undefined`。直接使用导出的枚举进行比较和赋值，无需对结果加减偏移或交换枚举值。

## 下一步

- [TaffyTree 对象](./objects-taffy-tree.md)
- [Layout 对象](./objects-layout.md)
