---
title: Styling
sidebar_position: 4
---

# Styling

**Comprehensive guide to Taffy's styling properties.**

Taffy's styling API is modeled after CSS. Below are the main property groups; the [Style API reference](../api/classes/Style.md) lists every exposed property and its accepted values.

## Layout Mode

Core properties that define how a node behaves.

| Property                                                   | Description                                               |
| :--------------------------------------------------------- | :-------------------------------------------------------- |
| **[`display`](./display.md)**                              | `Flex` (default), `Grid`, `Block`, `FlowRoot`, or `None`. |
| **[`direction`](./display.md#writing-direction)**          | `Ltr` (default) or `Rtl` logical layout direction.        |
| **[`float` / `clear`](./display.md#floats-and-clearance)** | Float placement and clearance in block layout.            |
| **[`position`](./position.md)**                            | `Relative` (flow) or `Absolute` (overlay).                |
| **[`overflow`](./overflow.md)**                            | `Visible`, `Clip`, `Hidden`, or `Scroll`.                 |
| **[`inset`](./inset.md)**                                  | `top`, `bottom`, `left`, `right` for positioning.         |

## Sizing & Spacing

Control dimensions and spacing.

| Property                                        | Description                      |
| :---------------------------------------------- | :------------------------------- |
| **[`size` / `minSize` / `maxSize`](./size.md)** | Width and height control.        |
| **[`aspectRatio`](./aspect-ratio.md)**          | Ratio between width and height.  |
| **[`margin`](./margin.md)**                     | Outer spacing.                   |
| **[`padding`](./padding.md)**                   | Inner spacing.                   |
| **[`border`](./border.md)**                     | Border width (space only).       |
| **[`gap`](./gap.md)**                           | Spacing between Flex/Grid items. |

## Flexbox Layout

Properties for 1D layouts.

| Property                                     | Description                                        |
| :------------------------------------------- | :------------------------------------------------- |
| **[`flexDirection`](./flex-direction.md)**   | `Row`, `Column`, `RowReverse`, or `ColumnReverse`. |
| **[`flexWrap`](./flex-wrap.md)**             | `Wrap`, `WrapReverse`, or `NoWrap`.                |
| **[`flexBasis`](./flex-basis.md)**           | Initial main size.                                 |
| **[`flexGrow`](./flex-grow.md)**             | Growth factor.                                     |
| **[`flexShrink`](./flex-shrink.md)**         | Shrink factor.                                     |
| **[`justifyContent`](./justify-content.md)** | Main-axis alignment.                               |
| **[`alignItems`](./align-items.md)**         | Default cross-axis alignment.                      |
| **[`alignSelf`](./align-self.md)**           | Override alignment for item.                       |
| **[`alignContent`](./align-content.md)**     | Align wrapping lines.                              |

## Grid Layout

Properties for 2D layouts.

| Property                                                                                           | Description                            |
| :------------------------------------------------------------------------------------------------- | :------------------------------------- |
| **[`gridTemplateRows` / `gridTemplateColumns`](./grid-templates.md)**                              | Define rows and columns.               |
| **[`gridTemplateAreas` and area counts](./grid-templates.md#named-areas-and-template-dimensions)** | Named areas and template dimensions.   |
| **[`justifyItems` / `justifySelf`](./align-items.md)**                                             | Default and per-item inline alignment. |
| **[`gridColumn`](./grid-column.md)**                                                               | Place items in columns.                |
| **[`gridRow`](./grid-row.md)**                                                                     | Place items in rows.                   |
| **[`gridAutoFlow`](./grid-auto-flow.md)**                                                          | Auto-placement algorithm.              |
