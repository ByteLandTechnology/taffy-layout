---
title: サイズ、スペース、単位
---

sidebar_position: 4

# サイズ、スペース、単位

Taffy レイアウトは利用可能なスペースとサイズ制約で駆動されます。このモデルを理解することで、レイアウトを予測可能にします。

## 利用可能なスペース

`computeLayout` の 2 番目の引数が利用可能なスペースを定義します：

```tsx live
// 固定
const fixedTree = new TaffyTree();
const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: 40 };
const fixedChild = fixedTree.newLeaf(fixedStyle);

const fixedRootStyle = new Style();
fixedRootStyle.display = Display.Flex;
fixedRootStyle.flexDirection = FlexDirection.Row;
fixedRootStyle.size = { width: 200, height: 80 };
fixedRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const fixedRoot = fixedTree.newWithChildren(fixedRootStyle, [fixedChild]);

fixedTree.computeLayout(fixedRoot, { width: 200, height: 80 });
console.log(fixedTree.printTree(fixedRoot));

// フレキシブル
const flexibleTree = new TaffyTree();
const flexibleStyle = new Style();
flexibleStyle.size = { width: "auto", height: 40 };
const flexibleChild = flexibleTree.newLeaf(flexibleStyle);

const flexibleRootStyle = new Style();
flexibleRootStyle.display = Display.Flex;
flexibleRootStyle.flexDirection = FlexDirection.Row;
flexibleRootStyle.size = { width: 200, height: 80 };
flexibleRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const flexibleRoot = flexibleTree.newWithChildren(flexibleRootStyle, [
  flexibleChild,
]);

flexibleTree.computeLayout(flexibleRoot, {
  width: "max-content",
  height: 80,
});
console.log(flexibleTree.printTree(flexibleRoot));

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>固定スペース</div>
      <TaffyTreePreview tree={fixedTree} root={fixedRoot} />
    </div>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>最大コンテンツ幅</div>
      <TaffyTreePreview tree={flexibleTree} root={flexibleRoot} />
    </div>
  </div>
);
```

### 許可される値

- `number`: 絶対サイズ（通常はピクセル）
- `"min-content"`: 最小コンテンツサイズ
- `"max-content"`: 最大コンテンツサイズ
- `"auto"`: レイアウトに任せる

## ボックスモデル

Taffy は `box-sizing: border-box` のように動作します：

```text
┌─────────────────────────┐
│  Margin                 │
│  ┌───────────────────┐  │
│  │ Border            │  │
│  │  ┌─────────────┐  │  │
│  │  │ Padding     │  │  │
│  │  │  Content    │  │  │
│  │  └─────────────┘  │  │
│  └───────────────────┘  │
└─────────────────────────┘
```

- `size` には padding + border が含まれる
- margin は外部スペーシング

## パーセンテージ

パーセントサイズは親コンテンツボックスに対する相対値です：

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: "50%", height: "100%" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.size = { width: 260, height: 160 };
rootStyle.padding = { left: 16, right: 16, top: 16, bottom: 16 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 260,
  height: 160,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## よくある落とし穴

- サイズのないルートは子を 0 にすることが多い
- Flex 内の `auto` はコンテンツサイズを意味する
- `max-content` は測定コールバックをトリガーする

## 次のステップ

- [計測関数 (Measure Functions)](./measure-functions.md)
- [スタイリングガイド (Styling Guide)](../styling/index.md)
