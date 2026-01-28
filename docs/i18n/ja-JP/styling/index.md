---
title: スタイリング
sidebar_position: 4
---

# スタイリング

**Taffy のスタイリングプロパティに関する包括的なガイド。**

Taffy のスタイリング API は CSS をモデルにしています。以下は、サポートされているすべてのプロパティの分類リストです。

## レイアウトモード

ノードの動作を定義するコアプロパティ。

| プロパティ                      | 説明                                                    |
| :------------------------------ | :------------------------------------------------------ |
| **[`display`](./display.md)**   | `Flex`（デフォルト）、`Grid`、または `None`。           |
| **[`position`](./position.md)** | `Relative`（フロー）または `Absolute`（オーバーレイ）。 |
| **[`overflow`](./overflow.md)** | `Visible`、`Hidden`、または `Scroll`。                  |
| **[`inset`](./inset.md)**       | ポジショニング用の `top`、`bottom`、`left`、`right`。   |

## サイズと間隔

寸法と間隔を制御します。

| プロパティ                                      | 説明                         |
| :---------------------------------------------- | :--------------------------- |
| **[`size` / `minSize` / `maxSize`](./size.md)** | 幅と高さの制御。             |
| **[`aspectRatio`](./aspect-ratio.md)**          | 幅と高さの比率。             |
| **[`margin`](./margin.md)**                     | 外側の余白。                 |
| **[`padding`](./padding.md)**                   | 内側の余白。                 |
| **[`border`](./border.md)**                     | 枠線の幅（スペースのみ）。   |
| **[`gap`](./gap.md)**                           | Flex/Grid アイテム間の間隔。 |

## Flexbox レイアウト

1D レイアウトのプロパティ。

| プロパティ                                   | 説明                           |
| :------------------------------------------- | :----------------------------- |
| **[`flexDirection`](./flex-direction.md)**   | `Row` または `Column` の方向。 |
| **[`flexWrap`](./flex-wrap.md)**             | `Wrap` または `NoWrap`。       |
| **[`flexBasis`](./flex-basis.md)**           | 初期メインサイズ。             |
| **[`flexGrow`](./flex-grow.md)**             | 伸長係数。                     |
| **[`flexShrink`](./flex-shrink.md)**         | 収縮係数。                     |
| **[`justifyContent`](./justify-content.md)** | メイン軸の整列。               |
| **[`alignItems`](./align-items.md)**         | デフォルトのクロス軸の整列。   |
| **[`alignSelf`](./align-self.md)**           | アイテムの整列オーバーライド。 |
| **[`alignContent`](./align-content.md)**     | 折り返し行の整列。             |

## Grid レイアウト

2D レイアウトのプロパティ。

| プロパティ                                | 説明                       |
| :---------------------------------------- | :------------------------- |
| **[`gridTemplate`](./grid-templates.md)** | 列と行を定義します。       |
| **[`gridColumn`](./grid-column.md)**      | 列方向へのアイテムの配置。 |
| **[`gridRow`](./grid-row.md)**            | 行方向へのアイテムの配置。 |
| **[`gridAutoFlow`](./grid-auto-flow.md)** | 自動配置アルゴリズム。     |
