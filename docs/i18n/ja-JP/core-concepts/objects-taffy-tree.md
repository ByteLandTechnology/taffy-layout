---
title: TaffyTree オブジェクト
sidebar_position: 2
---

# TaffyTree オブジェクト

**`TaffyTree`** オブジェクトは、レイアウトツリー全体を表し、レイアウト計算のエントリポイントとして機能します。

## 主な役割

- **ノード管理**: ノードの作成、追加、削除、および挿入。
- **ツリーの階層構造**: ノード間の親子関係の維持。
- **計算**: `computeLayout` または `computeLayoutWithMeasure` を介したレイアウトアルゴリズムの呼び出し。
- **結果の取得**: 各ノードの計算結果を保存し、アクセスを提供します。

ノード ID は `bigint` で、作成元のツリーの有効なノードにのみ使えます。ノードの削除や `clear()` の後は対応する ID を破棄してください。古い ID へのアクセスでノードの存在を調べないでください。

`remove(node)` が削除するのは指定したノードだけで、その子は親から切り離されて有効なまま残ります。`clear()` は全ノードを削除してツリーを再利用できる状態にします。ツリー自体の使用が終わったら `.free()` で解放します。

## 使い方

```typescript
const tree = new TaffyTree();

// ノードの作成
const child = tree.newLeaf(new Style());
const root = tree.newWithChildren(new Style(), [child]);

// レイアウトの計算
tree.computeLayout(root, { width: 500, height: 500 });

// 結果へのアクセス
const layout = tree.getLayout(child);
```

## 次のステップ

- [Layout オブジェクト](./objects-layout.md)
- [サイズ、スペース、および単位](./size-and-space.md)
