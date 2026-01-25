# 🐞 デバッグ

**レイアウトの検査とトラブルシューティングのためのツール。**

## 🖨️ ツリーを印刷

最も強力なツールは `tree.printTree(node)` です。ツリー構造、スタイル設定、計算されたレイアウトのテキスト表現を生成します。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
```

**出力例：**

```text
DIV [x: 0    y: 0    w: 100  h: 100  content_w: 100  content_h: 100  border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (1)
└── LEAF [x: 0    y: 0    w: 50   h: 50   content_w: 50   content_h: 50   border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (2)
```

> **注意**：実際の出力形式はバージョンによって多少異なる場合がありますが、常に階層と主要な制約を表示します。

## 📏 可視化デバッグ

キャンバスや画面にレンダリングする場合：

1. **境界線を描画**：計算された各レイアウト矩形の周りに色付きの 1px 境界線を描画します。
2. **色分け**：異なる `display` タイプに異なる色を使用します（例：Flex は青、Grid は赤）。

```ts
// モックレンダラー
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());
tree.computeLayout(root, { width: 100, height: 100 });

// 可視化デバッガー関数
function debugDraw(node: any) {
  const layout = tree.getLayout(node);
  renderer.strokeRect(layout.x, layout.y, layout.width, layout.height, "red");

  for (const child of tree.children(node)) {
    debugDraw(child);
  }
}
debugDraw(root);
```

## 🔍 分離

特定のサブツリーの動作がおかしい場合：

1. 新しい `TaffyTree` を作成します。
2. そのサブツリー構造のみをレプリケートします。
3. 入力制約（サブツリーに提供される幅/高さ）をハードコードします。
4. `computeLayout` を実行して検査します。

これにより、外部の親制約から問題を分離できます。
