---
title: サイジング
---

# サイジング

`size`、`minSize`、`maxSize` を使用して寸法を制御します。

## 一般的なプロパティ

- `size`：メインサイズ
- `minSize`：最小サイズ
- `maxSize`：最大サイズ

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: 200, height: 100 };
style.minSize = { width: 120, height: 60 };
style.maxSize = { width: 300, height: 160 };

const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 240, height: 140 };
rootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 240,
  height: 140,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## Flex との相互作用

- `flexGrow` は `size` を超えて拡張できる
- `flexShrink` はスペースが狭い場合に縮小

## パーセンテージサイズ

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: "50%", height: "100%" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.flexDirection = FlexDirection.Row;
rootStyle.size = { width: 200, height: 60 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 200,
  height: 60,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [スペーシング](./gap.md)
- [ポジショニング](./position.md)
