---
title: Styling
sidebar_position: 4
---

# Styling

**Comprehensive guide to Taffy's styling properties.**

Taffy's styling API is modeled after CSS. Below is a categorized list of all supported properties.

## Layout Mode

Core properties that define how a node behaves.

| Property                        | Description                                       |
| :------------------------------ | :------------------------------------------------ |
| **[`display`](./display.md)**   | `Flex` (default), `Grid`, or `None`.              |
| **[`position`](./position.md)** | `Relative` (flow) or `Absolute` (overlay).        |
| **[`overflow`](./overflow.md)** | `Visible`, `Hidden`, or `Scroll`.                 |
| **[`inset`](./inset.md)**       | `top`, `bottom`, `left`, `right` for positioning. |

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

| Property                                     | Description                   |
| :------------------------------------------- | :---------------------------- |
| **[`flexDirection`](./flex-direction.md)**   | `Row` or `Column` direction.  |
| **[`flexWrap`](./flex-wrap.md)**             | `Wrap` or `NoWrap`.           |
| **[`flexBasis`](./flex-basis.md)**           | Initial main size.            |
| **[`flexGrow`](./flex-grow.md)**             | Growth factor.                |
| **[`flexShrink`](./flex-shrink.md)**         | Shrink factor.                |
| **[`justifyContent`](./justify-content.md)** | Main-axis alignment.          |
| **[`alignItems`](./align-items.md)**         | Default cross-axis alignment. |
| **[`alignSelf`](./align-self.md)**           | Override alignment for item.  |
| **[`alignContent`](./align-content.md)**     | Align wrapping lines.         |

## Grid Layout

Properties for 2D layouts.

| Property                                  | Description               |
| :---------------------------------------- | :------------------------ |
| **[`gridTemplate`](./grid-templates.md)** | Define columns and rows.  |
| **[`gridColumn`](./grid-column.md)**      | Place items in columns.   |
| **[`gridRow`](./grid-row.md)**            | Place items in rows.      |
| **[`gridAutoFlow`](./grid-auto-flow.md)** | Auto-placement algorithm. |
