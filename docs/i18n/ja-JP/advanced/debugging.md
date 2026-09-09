---
title: デバッグ
sidebar_position: 1
---

# デバッグ

**レイアウトの検査とトラブルシューティングのためのツール。**

## ツリーを印刷

`tree.printTree(node)` は、ツリー構造、レイアウトモード、計算された寸法と位置を文字列として返します。表示するには `console.log()` などに渡します。

```ts
const tree = new TaffyTree();
const childStyle = new Style({ width: 50, height: 50 });
const child = tree.newLeaf(childStyle);
childStyle.free();
const rootStyle = new Style({ width: 100, height: 100 });
const root = tree.newWithChildren(rootStyle, [child]);
rootStyle.free();
tree.computeLayout(root, { width: 100, height: 100 });

console.log(tree.printTree(root));
tree.free();
```

**出力例：**

```text
└──  FLEX ROW [x: 0    y: 0    w: 100  h: 100  content_w: 50   content_h: 50   border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (4294967298)
    └──  LEAF [x: 0    y: 0    w: 50   h: 50   content_w: 0    content_h: 0    border: l:0 r:0 t:0 b:0, padding: l:0 r:0 t:0 b:0] (4294967297)
```

末尾の値はノード ID です。`content_w` / `content_h` は測定済みコンテンツや子の到達可能な範囲なので、空のリーフの幅・高さとは一致しません。

## 可視化デバッグ

キャンバスや画面にレンダリングする場合：

1. **境界線を描画**：計算された各レイアウト矩形の周りに色付きの 1px 境界線を描画します。
2. **色分け**：異なる `display` タイプに異なる色を使用します（例：Flex は青、Grid は赤）。

```ts
// モックレンダラー
const renderer = {
  strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
};
const tree = new TaffyTree();
const childStyle = new Style({ width: 20, height: 20 });
const child = tree.newLeaf(childStyle);
childStyle.free();
const parentStyle = new Style({
  width: 60,
  height: 60,
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
const parent = tree.newWithChildren(parentStyle, [child]);
parentStyle.free();
const rootStyle = new Style({
  width: 100,
  height: 100,
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
const root = tree.newWithChildren(rootStyle, [parent]);
rootStyle.free();
tree.computeLayout(root, { width: 100, height: 100 });

// 可視化デバッガー関数
function debugDraw(node: bigint, parentX = 0, parentY = 0) {
  const layout = tree.getLayout(node);
  const x = parentX + layout.x;
  const y = parentY + layout.y;
  renderer.strokeRect(x, y, layout.width, layout.height, "red");
  layout.free();

  for (const child of tree.children(node)) {
    debugDraw(child, x, y);
  }
}
debugDraw(root);
// child の親相対位置 (10, 10) は描画時には (20, 20) になります
tree.free();
```

## 分離

特定のサブツリーの動作がおかしい場合：

1. 新しい `TaffyTree` を作成します。
2. そのサブツリー構造のみをレプリケートします。
3. 入力制約（サブツリーに提供される幅/高さ）をハードコードします。
4. `computeLayout` を実行して検査します。

これにより、外部の親制約から問題を分離できます。
