---
title: Grid レイアウト
---

# Grid レイアウト

Taffy の Grid API は CSS Grid を反映しており、二次元レイアウトに最適です。行と列のトラックを定義し、ラインまたはエリアでアイテムを配置します。

## コアコンセプト

- **Track**：行または列のサイズ定義
- **Line**：配置に使用されるグリッドライン
- **Area**：名前付きリージョン（使用する場合）

```
Columns:  1fr 2fr
Rows:     auto 1fr

┌───────────────┐
│ Header        │
├───────┬───────┤
│ Nav   │ Main  │
# 🎨 スタイリング (Styling)

**Taffy のスタイリングプロパティに関する包括的なガイド。**

Taffy のスタイリング API は CSS をモデルにしています。以下は、サポートされているすべてのプロパティの分類リストです。

## 📐 レイアウトモード

ノードの振る舞いを定義するコアプロパティ。

| プロパティ | 説明 |
| :--- | :--- |
| **[`display`](./display.md)** | `Flex` (デフォルト), `Grid`, または `None`。 |
| **[`position`](./position.md)** | `Relative` (フロー) または `Absolute` (絶対配置)。 |
| **[`overflow`](./overflow.md)** | `Visible`, `Hidden`, または `Scroll`。 |
| **[`inset`](./inset.md)** | 配置用の `top`, `bottom`, `left`, `right`。 |

## 📦 サイジングとスペーシング

寸法と間隔を制御します。

| プロパティ | 説明 |
| :--- | :--- |
| **[`size` / `minSize` / `maxSize`](./size.md)** | 幅と高さの制御。 |
| **[`aspectRatio`](./aspect-ratio.md)** | アスペクト比。 |
| **[`margin`, `padding`, `border`](./margin-padding-border.md)** | ボックスモデルの間隔。 |
| **[`gap`](./gap.md)** | Flex/Grid アイテム間のギャップ。 |

## 🔗 Flexbox

1次元レイアウト用のプロパティ。

| プロパティ | 説明 |
| :--- | :--- |
| **[`flexDirection`](./flex-direction.md)** | `Row` または `Column` 方向。 |
| **[`flexWrap`](./flex-wrap.md)** | `Wrap` または `NoWrap`. |
| **[`flexBasis`, `flexGrow`, `flexShrink`](./flex-basis-grow-shrink.md)** | アイテムのサイズ変更ロジック。 |
| **[`justifyContent`](./justify-content.md)** | 主軸の配置。 |
| **[`alignItems`](./align-items.md)** | デフォルトのクロス軸配置。 |
| **[`alignSelf`](./align-self.md)** | 個別アイテムの配置上書き。 |
| **[`alignContent`](./align-content.md)** | 多段ラインの配置。 |

## ▦ Grid レイアウト

2次元レイアウト用のプロパティ。

| プロパティ | 説明 |
| :--- | :--- |
| **[`gridTemplate*`](./grid-templates.md)** | 列と行を定義します。 |
| **[`gridColumn` / `gridRow`](./grid-placement.md)** | グリッド内のアイテム配置。 |
| **[`gridAutoFlow`](./grid-auto-flow.md)** | 自動配置アルゴリズム。 |
```
