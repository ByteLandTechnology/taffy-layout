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

Taffy は制約が一致する計算結果をキャッシュから再利用します。スタイルやツリー構造の変更は対象ノードと祖先のキャッシュを無効化します。配置や利用可能なスペースが変わると、直接変更していない兄弟や子孫も再計算されることがあります。

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
//    Taffy は条件が一致するキャッシュを再利用します。
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

const style = new Style({ width: 100 / 3, height: 20 });
const node = tree.newLeaf(style);
style.free();
tree.computeLayout(node, { width: 100, height: 100 });
const layout = tree.getLayout(node);
console.log(layout.width); // 約 33.333332（内部は32ビット浮動小数点数）
layout.free();
tree.free();
```

## デバッグのヒント

- **`printTree(root)`**: ツリー階層、レイアウトモード、計算結果を文字列として返します。`console.log(tree.printTree(root))` で表示できます。
- **分離**: 複雑なツリーの動作がおかしい場合は、問題のあるノードのみを含む小さな再現を作成して問題を分離します。

## 次のステップ

- ️**[設定](./configuration.md)** - エンジン設定を調整します。
- **[レイアウトのデバッグ](../advanced/debugging.md)** - トラブルシューティング方法を学びます。
