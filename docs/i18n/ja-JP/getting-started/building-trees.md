---
title: ノードツリーの構築
sidebar_position: 3
---

# 🏗️ ノードツリーの構築

Taffy のレイアウトはノードツリーとして表現されます。各ノードには `Style` があり、親ノードは子ノードの配置方法を制御します。

## 🔑 主要な操作

| 操作                 | メソッド                                        | 説明                                                         |
| :------------------- | :---------------------------------------------- | :----------------------------------------------------------- |
| **リーフノード作成** | `tree.newLeaf(style)`                           | 子ノードを持たないノードを作成します（テキスト、画像など）。 |
| **親ノード作成**     | `tree.newWithChildren(style, children[])`       | 初期子ノードを持つノードを作成します。                       |
| **子ノード追加**     | `tree.addChild(parent, child)`                  | 親ノードに子ノードを追加します。                             |
| **子ノード挿入**     | `tree.insertChildAtIndex(parent, index, child)` | 特定の位置に子ノードを挿入します。                           |
| **子ノード削除**     | `tree.removeChild(parent, child)`               | 特定の子ノードを削除します。                                 |
| **スタイル取得**     | `tree.getStyle(node)`                           | ノードのスタイルオブジェクトを取得します。                   |
| **スタイル設定**     | `tree.setStyle(node, style)`                    | ノードのスタイルを更新します。                               |

## 🍃 ノードの作成

### リーフノード

リーフノードはレイアウトツリーの末端であり、通常はテキスト、画像、ボタンなどのコンテンツを表します。

```ts
const tree = new TaffyTree();

// 最初にスタイルを作成
const style = new Style({
  display: Display.Flex,
  size: { width: 100, height: 50 },
});

// ノードを作成
const leafNode = tree.newLeaf(style);
```

### コンテナノード

コンテナノードは他のノードをグループ化します。便宜上、既存の子ノードを含めて作成できます。

```ts
const tree = new TaffyTree();
const containerStyle = new Style({ display: Display.Flex });

const child1 = tree.newLeaf(new Style());
const child2 = tree.newLeaf(new Style());

// 子ノードをすぐに含めてコンテナを作成
const containerNode = tree.newWithChildren(containerStyle, [child1, child2]);
```

## 🌲 子ノードの管理

これらのメソッドを使用してツリー構造を動的に操作します。

```ts
const tree = new TaffyTree();
const parent = tree.newLeaf(new Style());
const child = tree.newLeaf(new Style());

// リストの末尾に子ノードを追加
tree.addChild(parent, child); // インデックス: 0

// 先頭に新しい子ノードを挿入
const firstChild = tree.newLeaf(new Style());
tree.insertChildAtIndex(parent, 0, firstChild); // インデックス: 0、以前の子ノードはインデックス 1 に

// 子ノードを置換
const newChild = tree.newLeaf(new Style());
tree.replaceChildAtIndex(parent, 1, newChild);

// 子ノードを削除
tree.removeChild(parent, firstChild);
```

## 🎨 スタイルの更新

スタイルはいつでも更新できます。これによりノードは「ダーティ」とマークされ、レイアウトの再計算が必要になります。

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style({ flexGrow: 1 }));

// 後でスタイルを更新
const newStyle = new Style({ flexGrow: 2, width: 100 });
tree.setStyle(node, newStyle);
```

> [!NOTE]
> **パフォーマンスのヒント**: 静的コンテンツの初期化には `Style` オブジェクトを再利用しようとしますが、`tree.setStyle` はスタイルデータを内部エンジンにコピーするため、ツリーに渡した**後**に JS `Style` オブジェクトを変更しても、次に `setStyle` を呼び出すまでツリーに影響しません。

## ⏭️ 次のステップ

- 📐 **[レイアウトの計算](./computing-layouts.md)** - 位置とサイズを計算します。
- ⚙️ **[設定](./configuration.md)** - グローバル設定。
