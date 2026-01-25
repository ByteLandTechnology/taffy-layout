---
title: 設定
sidebar_position: 5
---

# ⚙️ 設定

**特定のユースケースに向けて Taffy を最適化します。**

Taffy はすぐに使用できますが、パフォーマンス、精度、またはリソース制約に合わせて調整できます。

## 💾 容量の事前割り当て

必要なノード数がだいたいわかっている場合は、容量を指定してツリーを初期化すると、メモリの再割り当てを減らし、起動パフォーマンスを向上させることができます。

```tsx live
// 1,000 ノードを収容できるように初期化
const tree = TaffyTree.withCapacity(1000);
console.log(`Initial Node Capacity: ${tree.totalNodeCount()}`); // 実際のノードは 0

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

## 🎯 丸め設定

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
let layout1 = tree.getLayout(child1);
// layout1.width はアルゴリズムに応じて 51 または 50 に丸められる場合があります

// 2. 丸めを無効化
tree.disableRounding();
tree.computeLayout(root, { width: 150, height: 60 });
layout1 = tree.getLayout(child1);
// layout1.width は正確に 50.5 になります

console.log(`Precise Width: ${layout1.width}`);

return <TaffyTreePreview tree={tree} root={root} />;
```

## 🗑️ メモリ管理

Taffy は WebAssembly を使用しているため、JavaScript はガベージコレクションされますが、一部のバインディングでは基礎となる Rust 構造体が手動で管理されます。**ただし、この JS バインディングバージョンでは、`TaffyTree` が独自のライフサイクルを管理します。**

多くのツリーを作成する場合（ゲームループで 1 フームごとに 1 つなど）、WASM ヒープのメモリリークを避けるために、明示的にクリアまたは解放する必要があります。

```ts
const tree = new TaffyTree();

// ... ツリーを使用 ...

// オプション 1：ツリーを再利用（推奨）
// すべてのノードをクリアしますが、割り当てられたメモリは保持されます
tree.clear();

// オプション 2：完全に解放
tree.free();
```

## ⏭️ 次のステップ

- 🎨 **[スタイリングガイド](../styling/index.md)** - Flexbox と Grid について学びます。
- 🛠️ **[高度なトピック](../advanced/index.md)** - デバッグと内部構造。
