---
title: サイズ、スペース、単位
sidebar_position: 4
---

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

`"auto"` はスタイル寸法に使う値で、`AvailableSpace` では使えません。

## ボックスモデル

`boxSizing` のデフォルトは `BoxSizing.BorderBox` で、スタイル寸法に padding と border が含まれます。`BoxSizing.ContentBox` ではスタイル寸法がコンテンツボックスを指定し、最終的な `Layout.size` は引き続きボーダーボックスのサイズになります：

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

- 計算結果の `Layout.size` には padding + border が含まれる
- margin は外部スペーシング

## パーセンテージ

パーセントサイズは、そのレイアウトで与えられる包含領域を基準に解決されます。通常フローでは親のコンテンツボックス、Grid アイテムでは配置先のグリッド領域、絶対配置では親の配置領域が基準になります。基準寸法が未確定の場合は、数値に解決できない段階もあります。次の例は通常の Flexbox の子です：

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

`margin`、`padding`、`border` のパーセント値は、上下の辺も含めて包含領域の幅を基準にします。`inset` の上下は高さ、左右は幅を基準にするため、同じパーセントでもプロパティにより基準が異なります。

## よくある落とし穴

- サイズ指定も測定コンテンツもない空のリーフは、利用可能なスペースを与えるだけではその大きさに広がらない
- `"auto"` のサイズはレイアウトアルゴリズム、コンテンツ、他の制約で決まる
- コンテンツ測定コールバックは `computeLayoutWithMeasure()` で渡す。ノードの context は任意で、`"max-content"` だけではコールバックは登録されない

## 次のステップ

- [計測関数 (Measure Functions)](./measure-functions.md)
- [スタイリングガイド (Styling Guide)](../styling/index.md)
