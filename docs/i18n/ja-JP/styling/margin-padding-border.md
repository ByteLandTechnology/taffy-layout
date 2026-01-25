---
title: Margin, Padding, and Border（マージン、パディング、ボーダー）
sidebar_position: 11
---

# 📦 Margin, Padding, and Border（マージン、パディング、ボーダー）

**ボックスモデルの間隔を制御します。**

Taffy は標準的な CSS ボックスモデルに従います。

> [!TIP]
> 🔗 **MDN ドキュメント**: [ボックスモデル](https://developer.mozilla.org/ja/docs/Web/CSS/CSS_Box_Model/Introduction_to_the_CSS_box_model)

## 🖼️ ボックスモデル

1.  **Content（コンテンツ）**：実際のアイテム（画像、テキスト、または子要素）。
2.  **Padding（パディング）**：コンテンツとボーダーの間のスペース。
3.  **Border（ボーダー）**：ボックスの境界。
4.  **Margin（マージン）**：ボーダーの外側のスペースで、他のアイテムを押しのけます。

## 🎛️ プロパティ

各プロパティは `left`、`right`、`top`、`bottom` を含む `Rect` です。

| プロパティ    | 説明                                                                                       |
| :------------ | :----------------------------------------------------------------------------------------- |
| **`margin`**  | 外側のスペース。`Auto` を指定してコンテンツを中央揃えできます（`margin: auto` と同様）。   |
| **`padding`** | 内側のスペース。コンテナのサイズに影響します。                                             |
| **`border`**  | ボーダー幅。Taffy はボーダーの*スペース*のみを計算します。描画は自分で行う必要があります。 |

## 💻 例

```tsx live
const tree = new TaffyTree();

// 内部コンテンツ
const contentNode = tree.newLeaf(
  new Style({
    flexGrow: 1, // 利用可能なスペースを埋める
  }),
);

// ボックスモデルをデモンストレーションするコンテナ
const boxNode = tree.newWithChildren(
  new Style({
    size: { width: 200, height: 120 },
    display: Display.Flex,

    // 1. Margin（外側）
    margin: { left: 20, top: 20 },

    // 2. Border（境界）- 論理幅のみ
    border: { left: 5, right: 5, top: 5, bottom: 5 },

    // 3. Padding（内側）
    padding: { left: 15, right: 15, top: 15, bottom: 15 },
  }),
  [contentNode],
);

// 例を保持するルート
const root = tree.newWithChildren(
  new Style({
    size: { width: 300, height: 200 },
  }),
  [boxNode],
);

tree.computeLayout(root, { width: 300, height: 200 });

// 階層構造を可視化
console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## ⏭️ 次のステップ

- **[Gap（ギャップ）](./gap.md)** - Flex/Grid アイテム間のスペース。
- **[Size（サイズ）](./size.md)** - 幅と高さの制御。
