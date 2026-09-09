---
title: 設定
sidebar_position: 5
---

# 設定

**特定のユースケースに向けて Taffy を最適化します。**

Taffy はすぐに使用できますが、パフォーマンス、精度、またはリソース制約に合わせて調整できます。

## 容量の事前割り当て

必要なノード数がだいたいわかっている場合は、容量を指定してツリーを初期化すると、メモリの再割り当てを減らし、起動パフォーマンスを向上させることができます。

```tsx live
// 1,000 ノードを収容できるように初期化
const tree = TaffyTree.withCapacity(1000);
console.log(`Initial Node Count: ${tree.totalNodeCount()}`); // 実際のノードは 0

const style = new Style({
  display: Display.Flex,
  size: { width: 200, height: 40 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const root = tree.newLeaf(style);
tree.computeLayout(root, { width: 200, height: 40 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 丸め設定

レイアウト値を整数（ピクセル）に合わせるか、正確な浮動小数点数のままにするかを制御します。

| 設定           | 関数                     | 説明                                                                               |
| :------------- | :----------------------- | :--------------------------------------------------------------------------------- |
| **丸め有効化** | `tree.enableRounding()`  | **デフォルト**。値を最も近いピクセルに丸めます。UI でのぼやけた境界線を防ぎます。  |
| **丸め無効化** | `tree.disableRounding()` | 高精度浮動小数点数を使用します。ベクターグラフィックやズーム可能な UI に最適です。 |

```tsx live
const tree = new TaffyTree();

// 合計 101px になる 2 つのアイテムを作成
// 50.5 + 50.5 = 101
const style = new Style({
  size: { width: 50.5, height: 50 },
  display: Display.Flex,
  justifyContent: JustifyContent.Center,
  alignItems: AlignItems.Center,
});
const child1 = tree.newLeaf(style);
const child2 = tree.newLeaf(style);

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 150, height: 60 },
  alignItems: AlignItems.Center,
});
const root = tree.newWithChildren(rootStyle, [child1, child2]);

// 1. デフォルト（丸め有効）
tree.computeLayout(root, { width: 150, height: 60 });
const roundedLayout = tree.getLayout(child1);
// roundedLayout.width は整数になります。隣接する境界から幅を計算します。
roundedLayout.free();

// 2. 丸めを無効化
tree.disableRounding();
tree.computeLayout(root, { width: 150, height: 60 });
const layout1 = tree.getLayout(child1);
// layout1.width は正確に 50.5 になります

console.log(`Precise Width: ${layout1.width}`);
layout1.free();

return <TaffyTreePreview tree={tree} root={root} />;
```

## メモリ管理

`FinalizationRegistry` が利用可能な環境では、JavaScript バインディングは不要になった WASM オブジェクトの自動解放を登録します。ただし解放時期は保証されず、未対応の環境では自動解放されません。不要になった `TaffyTree`、`Style`、`Layout`、捕捉した `TaffyError` は `.free()` で明示的に解放できます。

`newLeaf()`、`newWithChildren()`、`setStyle()` はスタイルをコピーするため、渡した `Style` はその後解放できます。`getStyle()`、`getLayout()`、測定コールバックの第 5 引数も独立した所有オブジェクトで、ツリーの解放とは別に解放します。`Layout.get()` などが返す通常の JavaScript オブジェクトや配列、ノード ID 自体には `.free()` は不要です。

解放したオブジェクトは再使用しないでください。ライブ例の `TaffyTreePreview` には有効なツリーが必要なので、描画での使用が終わってからツリーを解放します。短い例で後片付けを省略している場合も、実際のアプリケーションではこの寿命管理が必要です。

WASM ヒープでのメモリ消費の急増やリークを防ぐため、明示的にメモリを管理することを推奨します：

- **再利用（推奨）：** `.clear()` を使用して、ノード格納領域の容量を保持したままツリーをリセットします。子リストなどは作り直すため、すべての再割り当てを回避できるわけではありません。
- **破棄：** ツリーの使用が完全に終了し、すぐにメモリを解放したい場合は、`.free()` を使用します。

```ts
const tree = new TaffyTree();

// ... ツリーを使用 ...

// オプション 1：ツリーを再利用（推奨）
// すべてのノードをクリアしますが、割り当てられたメモリは保持されます
tree.clear();

// オプション 2：完全に解放
tree.free();
```

## 次のステップ

- **[スタイリングガイド](../styling/index.md)** - Flexbox と Grid について学びます。
- **[高度なトピック](../advanced/index.md)** - デバッグと内部構造。
