---
title: クイックスタート
sidebar_position: 2
---

# ⚡ クイックスタート

**数分で動作する Taffy レイアウトを作成しましょう。**

## 1. 最小限の例

この例では、コンテナと 2 つの子ノードを含むシンプルな Flexbox レイアウトを作成します。

```ts
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// 1. ライブラリを初期化
await loadTaffy();
const tree = new TaffyTree();

// 2. スタイルを作成
const containerStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  size: { width: 300, height: 200 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});

const childStyle = new Style({
  flexGrow: 1,
  size: { width: "100%", height: "auto" },
});

// 3. ノードツリーを作成
//    （newWithChildren を使用する場合、リーフノードを先に作成する必要があります）
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// 4. レイアウトを計算
//    ルートノードと利用可能なスペースを渡します
tree.computeLayout(container, { width: 300, height: 200 });

// 5. 計算結果を読み取り
const containerLayout = tree.getLayout(container);
const child1Layout = tree.getLayout(child1);

console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
// 出力: Container: 300x200

// 6. デバッグ：ツリー構造全体を出力
console.log(tree.printTree(container));
```

## ⏭️ 次のステップ

- 🏗️ **[ノードツリーの構築](./building-trees.md)** - ノードツリーの操作方法を学びます。
- 📐 **[レイアウトの計算](./computing-layouts.md)** - レイアウト計算のプロセスを理解します。
- 🧠 **[コアコンセプト](../core-concepts/overview.md)** - Taffy のモデルを深く理解します。
