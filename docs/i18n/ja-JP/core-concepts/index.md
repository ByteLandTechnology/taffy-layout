---
title: コアコンセプト
sidebar_position: 3
---

# コアコンセプト

Taffy の API サーフェスは小さいですが、一貫したレイアウトモデルを持っています。このセクションでは、あらゆるレイアウトを理解するためのコンセプトを説明します。

## 3 つのコアオブジェクト

Taffy のレイアウトは、主に 3 つのオブジェクトによって駆動されます。

1.  **[Style](./objects-style.md)**: ノードのレイアウト規則（例：「flex コンテナにする」、「10px のパディングを持つ」）を定義します。
2.  **[TaffyTree](./objects-taffy-tree.md)**: ノードの階層構造を管理し、計算のエントリポイントとなります。
3.  **[Layout](./objects-layout.md)**: 計算の出力であり、最終的な位置とサイズを含みます。

```
Style + Tree  -> computeLayout -> Layout
```

## 基本原則

コアオブジェクト以外にも、理解しておくべき重要な原則がいくつかあります。

- **[サイズ、スペース、および単位](./size-and-space.md)**: Taffy が固定サイズ、パーセンテージ、およびコンテンツベースのサイジングをどのように処理するか。
- **[計測関数 (Measure Functions)](./measure-functions.md)**: テキストやプラットフォーム固有のウィジェット用にカスタムサイジングロジックを統合します。
- **ツリーモデル**: すべてのノードは子を持つことができ、親は軸に沿ってそれらがどのように配置されるかを制御します。

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

- [Style オブジェクト](./objects-style.md)
- [サイズ、スペース、および単位](./size-and-space.md)
- [レイアウトクックブック (Cookbook)](../cookbook/)
