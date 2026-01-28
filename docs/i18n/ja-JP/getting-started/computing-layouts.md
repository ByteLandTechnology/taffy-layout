---
title: レイアウトの計算
sidebar_position: 4
---

# レイアウトの計算

**スタイルとツリー構造を具体的なピクセル座標に変換します。**

ツリーの構築が完了したら、`computeLayout` を呼び出して各ノードの最終的な位置とサイズを計算します。

## 標準レイアウト計算

レイアウトを計算する際、**利用可能なスペース**（ルートノードの制約）を提供する必要があります。

```tsx live
const tree = new TaffyTree();
const rootStyle = new Style({
  display: Display.Flex,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  size: { width: 400, height: 100 },
});

const child = tree.newLeaf(
  new Style({
    size: { width: 50, height: 50 },
  }),
);
const root = tree.newWithChildren(rootStyle, [child]);

// 1. レイアウトを計算
//    制約を渡します：width: 400, height: 100
tree.computeLayout(root, { width: 400, height: 100 });

// 2. 結果を読み取り
//    エンジンがすべてのノードのレイアウトデータを入力しました。
const rootLayout = tree.getLayout(root);
const childLayout = tree.getLayout(child);

console.log(`Root Size: ${rootLayout.width}x${rootLayout.height}`);
console.log(`Child Pos: ${childLayout.x}, ${childLayout.y}`);

return (
  <div
    style={{
      width: rootLayout.width,
      height: rootLayout.height,
      background: "#f0f0f0",
      position: "relative",
    }}
  >
    <div
      style={{
        width: childLayout.width,
        height: childLayout.height,
        left: childLayout.x,
        top: childLayout.y,
        position: "absolute",
        background: "#007aff",
      }}
    />
    <div
      style={{
        position: "absolute",
        bottom: 5,
        right: 5,
        fontSize: 10,
        color: "#666",
      }}
    >
      Child at ({childLayout.x}, {childLayout.y})
    </div>
  </div>
);
```

## 増分レイアウト

Taffy はインテリジェントなキャッシュを採用しています。特定のノードのスタイルやコンテンツを変更すると、次の計算では影響を受けるツリーの部分のみが再計算されます。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
const childNode = tree.newLeaf(new Style());
tree.addChild(root, childNode);

// 1. 最初のレイアウト
tree.computeLayout(root, { width: 800, height: 600 });

// 2. リーフノードを変更
const newStyle = new Style({ width: 250 });
tree.setStyle(childNode, newStyle);

// 3. 再計算
//    Taffy は影響を受けないブランチの再計算をスキップします。
tree.computeLayout(root, { width: 800, height: 600 });
```

## 丸めと精度

デフォルトでは、Taffy はすべての出力座標を最も近いピクセル（整数）に丸め、標準ディスプレイグリッドに合わせます。

### 丸めの無効化

高 DPI レンダリングやベクターグラフィックなど、サブピクセル精度が必要なシナリオでは、丸めを無効にできます。

```ts
const tree = new TaffyTree();

// サブピクセル精度を有効化
tree.disableRounding();

// ... レイアウトを計算 ...
const node = tree.newLeaf(new Style());
const layout = tree.getLayout(node);
console.log(layout.width); // 33 ではなく 33.33333... になる可能性があります
```

## デバッグのヒント

- **`printTree(root)`**: ツリー全体の深さ、スタイル、計算されたレイアウトのテキスト表現を出力します。デバッグに不可欠です。
- **分離**: 複雑なツリーの動作がおかしい場合は、問題のあるノードのみを含む小さな再現を作成して問題を分離します。

## 次のステップ

- ️**[設定](./configuration.md)** - エンジン設定を調整します。
- **[レイアウトのデバッグ](../advanced/debugging.md)** - トラブルシューティング方法を学びます。
