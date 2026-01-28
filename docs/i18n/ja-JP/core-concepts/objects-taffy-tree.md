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
