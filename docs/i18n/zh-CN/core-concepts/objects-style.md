---
title: Style 对象
sidebar_position: 1
---

# Style 对象

**`Style`** 对象定义了单个节点的布局规则。它包含决定节点如何调整大小、定位以及如何排列其子节点的属性。

## 关键职责

- **布局模式**：决定节点使用 Flexbox、Grid 还是绝对定位。
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

## 下一步

- [TaffyTree 对象](./objects-taffy-tree.md)
- [Layout 对象](./objects-layout.md)
