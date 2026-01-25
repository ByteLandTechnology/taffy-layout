---
title: 測定関数
---

# 測定関数

リーフノードのサイズがそのコンテンツ（例：テキスト、画像、プラットフォーム固有のウィジェット）に依存する場合、Taffy はスタイルプロパティのみからサイズを計算できません。このような場合、**測定関数**を提供する必要があります。

## 使用タイミング

ツリーがコンテンツベースのサイジングを必要とするノード（例：`width: auto` または `measure` モード）を含む場合、標準の `computeLayout()` の代わりに `computeLayoutWithMeasure()` を使用します。Taffy はコンテンツベースのサイジングを必要とするリーフノードに対してコールバックを呼び出します。

## 動作原理

測定関数は、レイアウトプロセス中に Taffy が呼び出すコールバックです。「これらの制約が与えられた場合、このコンテンツはどのくらいのサイズですか？」と尋ねます。

```text
measure(known_dimensions, available_space) -> Size { width, height }
```

### 引数

1. **`knownDimensions`**: ノードのスタイルで明示的に定義された寸法（例：`width: 100` が設定されている場合、`knownDimensions.width` は `100` になります）。
2. **`availableSpace`**: 親ノードによって提供されるスペース。これにより、コンテンツがどのくらい大きくなれるかが制約されます。

### 戻り値

関数は、測定された `width` と `height`（ピクセル単位）を含む `Size` オブジェクトを**返す必要があります**。

## 例

```tsx live
const tree = new TaffyTree();

const style = new Style();
// このノードは固定サイズを持たないため、Taffy は測定関数に問い合わせます
style.size = { width: "auto", height: "auto" };

const measuredNode = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.display = Display.Flex;
rootStyle.size = { width: 300, height: 100 };
rootStyle.alignItems = AlignItems.Center;
rootStyle.justifyContent = JustifyContent.Center;

const root = tree.newWithChildren(rootStyle, [measuredNode]);

// computeLayout の代わりに computeLayoutWithMeasure を使用します
tree.computeLayoutWithMeasure(
  root,
  { width: 300, height: 100 },
  (knownDims, availableSpace) => {
    // 1. 既知の寸法（スタイルオーバーライド）があるか確認
    // 2. そうでない場合、利用可能なスペースまたはコンテンツの固有サイズに基づいて計算
    const width =
      knownDims.width ??
      (typeof availableSpace.width === "number"
        ? Math.min(availableSpace.width, 150)
        : 150);

    const height = knownDims.height ?? 50;

    return { width, height };
  },
);

return (
  <div style={{ display: "flex", gap: 10 }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
      <strong>Measured Size:</strong>
      <br />
      {tree.getLayout(measuredNode).width} x{" "}
      {tree.getLayout(measuredNode).height}
    </div>
  </div>
);
```

## 典型的なユースケース

- **テキストレイアウト**: フォントサイズ、テキストコンテンツ、折り返し幅に基づいて幅/高さを計算します。
- **画像**: 画像の固有寸法を返します。
- **ネイティブ UI ウィジェット**: 独自のサイジングロジックを持つプラットフォーム固有のコントロールをラップします。

## パフォーマンスのヒント

- **結果をキャッシュ**: 測定は高コストになる可能性があります。入力（`knownDimensions`、`availableSpace`、コンテンツ文字列など）に基づいて結果をキャッシュし、同じ測定を再計算することを避けてください。
- **副作用を避ける**: 測定関数は純粋である必要があります。その中で DOM や外部状態を変更しないでください。

## 次のステップ

- [パフォーマンスチューニング](../advanced/performance.md)
- [レイアウトクックブック](../cookbook/)
