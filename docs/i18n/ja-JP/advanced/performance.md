---
title: パフォーマンス
sidebar_position: 2
---

# パフォーマンス

**Taffy レイアウトを高速に保つためのヒント。**

Taffy は効率的で高性能に設計されていますが、特定のパターンはパフォーマンスに影響を与える可能性があります。

## 1. 容量の事前割り当て

ノード数を把握している場合は、`withCapacity` を使用して再割り当てを防ぎます。

```tsx live
const tree = TaffyTree.withCapacity(2000);
console.log(tree.totalNodeCount());

const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Row;
containerStyle.size = { width: 220, height: 70 };
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const itemStyle = new Style();
itemStyle.flexGrow = 1;

const first = tree.newLeaf(itemStyle);
const second = tree.newLeaf(itemStyle);
const root = tree.newWithChildren(containerStyle, [first, second]);

tree.computeLayout(root, { width: 220, height: 70 });

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div
      style={{
        padding: "8px 12px",
        borderRadius: 8,
        background: "rgba(0, 122, 255, 0.12)",
        color: "#0f172a",
        fontSize: 12,
        fontWeight: 600,
        display: "inline-flex",
        alignItems: "center",
      }}
    >
      Capacity: {tree.totalNodeCount()}
    </div>
  </div>
);
```

## 2. 増分レイアウト

変更されたノードのみが再計算されます。Taffy は**遅延評価的に**動作し、変更の影響を受けたブランチのみを再計算します。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: 120, height: 60 };
const child = tree.newLeaf(childStyle);
const root = tree.newWithChildren(
  new Style({
    display: Display.Flex,
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  }),
  [child],
);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 一般的な落とし穴

### 1. 過度なネスト

深さのレベルごとに再帰アルゴリズムの複雑さが増します。

- **悪い**：パディングのためだけに `View -> View -> View -> Button`。
- **良い**：ラッパーノードの代わりに親ノードで `padding` を使用します。

### 2. 深い測定関数

カスタム測定関数（テキスト/画像用）は頻繁に呼び出されます。

- **最適化**：測定コールバックが高速であることを確認してください。測定内での DOM リフローや重い計算を避けてください。

## 最適化パターン

### スタイルの再利用

タイトループで `Style` オブジェクトを作成する（例：ゲームレンダリング）ことは、JS では高コストになる可能性があります。可能な限り定義オブジェクトを再利用してください。

```ts
// ✅ 良い
const tree = new TaffyTree();
const ITEM_STYLE = new Style({ flexGrow: 1 });
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(ITEM_STYLE);
}
```

```ts
// ❌ アイテムが同じ場合は避ける
const tree = new TaffyTree();
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(new Style({ flexGrow: 1 }));
}
```

### バッチプロパティアクセス

バッチ getter と setter を使用して、WASM ブリッジのオーバーヘッドを最小限に抑えます。

**スタイルバッチ更新：**
プロパティを 1 つずつ設定する代わりに：

```ts
const style = new Style();
// ❌ 複数の呼び出し = 高オーバーヘッド
style.display = Display.Flex;
style.width = 100;
style.height = 100;
```

`set()` を使用：

```ts
const style = new Style();
// ✅ 単一の呼び出し = 低オーバーヘッド
style.set({
  display: Display.Flex,
  width: 100,
  height: 100,
});
```

**レイアウトバッチ読み取り：**
レイアウトプロパティを個別に読み取る代わりに：

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ❌ 複数の呼び出し
const layout = tree.getLayout(node);
const x = layout.x;
const y = layout.y;
const w = layout.width;
const h = layout.height;
```

`get()` を使用：

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ✅ 単一の呼び出しが値の配列を返す
const layout = tree.getLayout(node);
const [x, y, w, h] = layout.get("x", "y", "width", "height");
```

## ベンチマーク

`performance.now()` を使用してレイアウトパスを測定します。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());

const start = performance.now();
tree.computeLayout(root, { width: 1000, height: 1000 });
const end = performance.now();
console.log(`Layout took ${end - start}ms`);
```

## 次のステップ

- [測定関数](../core-concepts/measure-functions.md)
- [レイアウトのデバッグ](./debugging.md)
