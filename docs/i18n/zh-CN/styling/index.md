---
title: 样式
sidebar_position: 4
---

# 样式

**Taffy 样式属性综合指南。**

Taffy 的样式 API 模仿 CSS 设计。以下是所有支持属性的分类列表。

## 布局模式

定义节点行为的核心属性。

| 属性                            | 描述                                        |
| :------------------------------ | :------------------------------------------ |
| **[`display`](./display.md)**   | `Flex`（默认）、`Grid` 或 `None`。          |
| **[`position`](./position.md)** | `Relative`（流式）或 `Absolute`（覆盖）。   |
| **[`overflow`](./overflow.md)** | `Visible`、`Hidden` 或 `Scroll`。           |
| **[`inset`](./inset.md)**       | 定位用的 `top`、`bottom`、`left`、`right`。 |

## 尺寸与间距

控制尺寸和间距。

| 属性                                            | 描述                       |
| :---------------------------------------------- | :------------------------- |
| **[`size` / `minSize` / `maxSize`](./size.md)** | 宽度和高度控制。           |
| **[`aspectRatio`](./aspect-ratio.md)**          | 宽高比。                   |
| **[`margin`](./margin.md)**                     | 外边距。                   |
| **[`padding`](./padding.md)**                   | 内边距。                   |
| **[`border`](./border.md)**                     | 边框宽度 (仅空间)。        |
| **[`gap`](./gap.md)**                           | Flex/Grid 项目之间的间距。 |

## Flexbox 布局

一维布局的属性。

| 属性                                         | 描述                     |
| :------------------------------------------- | :----------------------- |
| **[`flexDirection`](./flex-direction.md)**   | `Row` 或 `Column` 方向。 |
| **[`flexWrap`](./flex-wrap.md)**             | `Wrap` 或 `NoWrap`。     |
| **[`flexBasis`](./flex-basis.md)**           | 初始主轴大小。           |
| **[`flexGrow`](./flex-grow.md)**             | 增长因子。               |
| **[`flexShrink`](./flex-shrink.md)**         | 收缩因子。               |
| **[`justifyContent`](./justify-content.md)** | 主轴对齐。               |
| **[`alignItems`](./align-items.md)**         | 默认交叉轴对齐。         |
| **[`alignSelf`](./align-self.md)**           | 项目的覆盖对齐。         |
| **[`alignContent`](./align-content.md)**     | 对齐换行线。             |

## Grid 布局

二维布局的属性。

| 属性                                      | 描述             |
| :---------------------------------------- | :--------------- |
| **[`gridTemplate`](./grid-templates.md)** | 定义列和行。     |
| **[`gridColumn`](./grid-column.md)**      | 在网格中放置列。 |
| **[`gridRow`](./grid-row.md)**            | 在网格中放置行。 |
| **[`gridAutoFlow`](./grid-auto-flow.md)** | 自动放置算法。   |
