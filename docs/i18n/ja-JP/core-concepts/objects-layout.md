---
title: Layout オブジェクト
sidebar_position: 3
---

# Layout オブジェクト

**`Layout`** オブジェクトには、レイアウトプロセスが終了した後のノードの最終的な計算結果が含まれます。

## 主なプロパティ

- **`location`**: 親ノードの左上隅を基準としたノードの `x` および `y` 座標。
- **`size`**: ノードの計算された幅（`width`）と高さ（`height`）（ピクセル単位）。
- **`margin` / `padding` / `border`**: 解決されたエッジのサイズ。

## 使い方

ツリーで `computeLayout` を呼び出した後、ツリー内の任意のノードのレイアウトを取得できます。

```typescript
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

const layout = tree.getLayout(node);

console.log(`位置: (${layout.position.x}, ${layout.position.y})`);
console.log(`サイズ: ${layout.size.width}x${layout.size.height}`);
```

## 次のステップ

- [サイズ、スペース、および単位](./size-and-space.md)
- [計測関数 (Measure Functions)](./measure-functions.md)
