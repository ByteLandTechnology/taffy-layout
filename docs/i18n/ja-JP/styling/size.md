---
title: サイジング (Sizing)
sidebar_position: 2
---

# サイジング (Sizing)

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

## 幅と高さ (Width and Height)

**要素の正確な寸法を制御します。**

`size`、`minSize`、`maxSize` を使用して、要素の寸法の境界を設定します。

> [!TIP]
> 🔗 **MDN ドキュメント**: [width](https://developer.mozilla.org/en-US/docs/Web/CSS/width), [height](https://developer.mozilla.org/en-US/docs/Web/CSS/height), [min-width](https://developer.mozilla.org/en-US/docs/Web/CSS/min-width), [max-width](https://developer.mozilla.org/en-US/docs/Web/CSS/max-width)

## プロパティ (Properties)

これらのプロパティは `width` と `height` を含む `Size` オブジェクトを受け取ります。

| プロパティ    | 説明                                                                                       |
| :------------ | :----------------------------------------------------------------------------------------- |
| **`size`**    | 理想サイズ。`auto` の場合、サイズはコンテンツまたは flex/grid ルールによって決定されます。 |
| **`minSize`** | 最小サイズ。アイテムがこの値以下に縮小するのを防ぎます。                                   |
| **`maxSize`** | 最大サイズ。アイテムがこの値以上に成長するのを防ぎます。                                   |

## 寸法値 (Dimension Values)

`width` と `height` プロパティは特定の値の型を受け取ります：

| 値          | 説明                                                             | 例 (JS)                                                                                          |
| :---------- | :--------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| **Auto**    | コンテンツに合わせたサイズ（一部の flex ケースではストレッチ）。 | `"auto"`                                                                                         |
| **Points**  | 正確なピクセル値。                                               | `150`                                                                                            |
| **Percent** | 親のサイズに対する割合。                                         | `"50%"` または `0.5`（float ヘルパーを使用する場合）。通常 JS バインディングでは文字列 `"50%"`。 |

## 例 (Example)

```tsx live
const tree = new TaffyTree();

const containerStyle = new Style({
  // 固定サイズ
  size: { width: 200, height: 200 },
  display: Display.Flex,
  flexDirection: FlexDirection.Column,
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
  gap: { width: 0, height: 10 },
});

const percentageChild = tree.newLeaf(
  new Style({
    // 親の幅の 80%、固定の高さ 30px
    size: { width: "80%", height: 30 },
  }),
);

const minMaxChild = tree.newLeaf(
  new Style({
    // 10px を目指すが、最低 50px に強制される
    size: { width: 10, height: 30 },
    minSize: { width: 50, height: "auto" },
  }),
);

const root = tree.newWithChildren(containerStyle, [
  percentageChild,
  minMaxChild,
]);

tree.computeLayout(root, { width: 200, height: 200 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## Flex との相互作用

- `flexGrow` は `size` を超えて拡張できる
- `flexShrink` はスペースが狭い場合に縮小

## 次のステップ

- [アスペクト比 (Aspect Ratio)](./aspect-ratio.md)
- [マージン (Margin)](./margin.md)
