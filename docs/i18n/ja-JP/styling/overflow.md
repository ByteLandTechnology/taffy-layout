---
title: オーバーフロー (Overflow)
sidebar_position: 24
---

# オーバーフロー (Overflow)

**コンテンツがコンテナのサイズを超えた場合の動作を制御します。**

`overflow` プロパティは、コンテンツがコンテナボックスより大きい場合に何が起こるかを指定します。

## 値

| 值            | 説明                                                                                                                   |
| :------------ | :--------------------------------------------------------------------------------------------------------------------- |
| **`Visible`** | **デフォルト**。コンテンツはコンテナの外へ流れ出ます。                                                                 |
| **`Hidden`**  | コンテンツはコンテナの端で切り取られます。                                                                             |
| **`Scroll`**  | Taffy はスクロールバー用のスペースを確保します（設定されている場合）。ただしスクロールバー自体はレンダリングしません。 |

## スクロールバーのサイズ

Taffy では、`Overflow.Scroll` は通常、ノードがスクロール*できる*ことを示すために使用されます。Taffy は `scrollbarSize` を計算し、レイアウト出力から読み取ることができます。

```ts
const tree = new TaffyTree();
const style = new Style({
  overflow: { x: Overflow.Scroll, y: Overflow.Scroll },
  scrollbarWidth: 15, // スクロールバーの推定サイズを設定するヘルパー
});

// レイアウト計算後：
const node = tree.newLeaf(style);
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(
  `Scrollbar Size: ${layout.scrollbarWidth} x ${layout.scrollbarHeight}`,
);
```

## 例

```tsx live
const tree = new TaffyTree();

const container = tree.newLeaf(
  new Style({
    size: { width: 100, height: 100 },
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
    // Hidden に変更してみてください
    overflow: { x: Overflow.Visible, y: Overflow.Visible },
  }),
);

const bigContent = tree.newLeaf(
  new Style({
    size: { width: 200, height: 200 },
  }),
);

tree.addChild(container, bigContent);

tree.computeLayout(container, { width: 100, height: 100 });

return <TaffyTreePreview tree={tree} root={container} />;
```

## 次のステップ

- [コアコンセプト](../core-concepts/index.md)
- [サイジング (Sizing)](./size.md)
