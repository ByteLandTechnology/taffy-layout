---
title: コアコンセプト概要
sidebar_position: 1
---

# コアコンセプト概要

Taffy は小さな API サーフェスを持ちますが、一貫したレイアウトモデルを持っています。このセクションでは、あらゆるレイアウトを理解するための概念を提供します。

## 3 つのコアオブジェクト

- **Style**: ノードのレイアウトルール
- **TaffyTree**: レイアウトツリー + 計算エントリーポイント
- **Layout**: 計算結果（位置、サイズ、マージン）

```
Style + Tree  -> computeLayout -> Layout
```

## ツリーモデル

すべてのノードは子を持つことができ、親は子がメイン軸とクロス軸に沿ってどのように配置されるかを制御します。

```
Root (flex)
├── Sidebar
└── Content
    ├── Header
    └── Body
```

### ノード作成 API

- `newLeaf(style)` リーフノードを作成
- `newWithChildren(style, children)` コンテナを作成
- `addChild` / `insertChildAtIndex` ツリーを変更

## レイアウトフロー

1. ルートと子のスタイルを設定
2. `computeLayout(root, availableSpace)` を呼び出す
3. `getLayout(node)` の結果を読み取る

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.flexGrow = 1;

const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 400, height: 200 };

const root = tree.newWithChildren(rootStyle, [child1, child2]);

tree.computeLayout(root, {
  width: 400,
  height: 200,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## レイアウトに含まれるもの

- `x`, `y`: 親に対する相対位置
- `width`, `height`: 計算されたサイズ
- `margin`, `padding`, `border`: エッジサイズ

## 次のステップ

- [サイズ、スペース、単位](./size-and-space.md)
- [測定関数](./measure-functions.md)
- [レイアウトクックブック](../cookbook/)
