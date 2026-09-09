---
title: 測定関数
sidebar_position: 5
---

# 測定関数

リーフノードのサイズがそのコンテンツ（例：テキスト、画像、プラットフォーム固有のウィジェット）に依存する場合、Taffy はスタイルプロパティのみからサイズを計算できません。このような場合、**測定関数**を提供する必要があります。

## 使用タイミング

ツリーがコンテンツベースのサイジングを必要とするノード（例：サイズが `"auto"` のテキストノード）を含む場合、標準の `computeLayout()` の代わりに `computeLayoutWithMeasure()` を使用します。Taffy はコンテンツベースのサイジングを必要とするリーフノードに対してコールバックを呼び出します。

## 動作原理

測定関数は、レイアウトプロセス中に Taffy が呼び出すコールバックです。「これらの制約が与えられた場合、このコンテンツはどのくらいのサイズですか？」と尋ねます。

### 引数

1. **`knownDimensions`**: エンジンが今回のコールバックに渡す既知の寸法。既知の値が渡されない軸は `undefined` です。レイアウト段階によっては、スタイルに固定サイズがあっても `undefined` になります。スタイルの幅・高さをそのままコピーした値ではありません。
2. **`availableSpace`**: 各軸の値はピクセル数、`"min-content"`、`"max-content"` のいずれかで、今回の測定条件を表します。数値の場合、padding、border、スクロールバー用スペースを差し引いた内容測定用のスペースです。
3. **`node`**: 測定対象ノードの ID（`bigint`）。
4. **`context`**: `newLeafWithContext()` または `setNodeContext()` で関連付けた任意のコンテンツデータ。未設定の場合は `undefined` です。
5. **`style`**: ノードの現在のスタイルの独立した所有コピー。明示的なグリッド領域の行列数も保持します。使用後に `.free()` を呼んでください。

通常の `newLeaf()` で作成したリーフでも測定コールバックは呼ばれます。context は任意のコンテンツデータを渡すためのものです。コールバック内のスタイルコピーを変更してもツリーは更新されません。寸法が確定済みの場合、非表示ノード、キャッシュが使える場合は、コールバックが不要なこともあります。

### 戻り値

関数は、測定された `width` と `height`（ピクセル単位）を含む `Size` オブジェクトを**返す必要があります**。

コールバックで発生した例外は現在のバインディングでは外側へ再送出されず、測定値がゼロにフォールバックします。失敗を検知したい場合はコールバック内で捕捉して外部変数などに記録し、`computeLayoutWithMeasure()` が戻った後に確認してください。

## 例

```tsx live
const tree = new TaffyTree();

const style = new Style();
// このノードは固定サイズを持たないため、Taffy は測定関数に問い合わせます
style.size = { width: "auto", height: "auto" };

const measuredNode = tree.newLeafWithContext(style, { width: 150, height: 50 });

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
  (knownDims, availableSpace, node, context, measuredStyle) => {
    measuredStyle.free(); // この例ではスタイルを参照しないため、先に解放します
    // 1. 今回の測定で既知の寸法があるか確認
    // 2. そうでない場合、利用可能なスペースまたはコンテンツの固有サイズに基づいて計算
    const contentWidth = context?.width ?? 150;
    const contentHeight = context?.height ?? 50;
    const width =
      knownDims.width ??
      (typeof availableSpace.width === "number"
        ? Math.min(availableSpace.width, contentWidth)
        : contentWidth);

    const height = knownDims.height ?? contentHeight;

    return { width, height };
  },
);

const measuredLayout = tree.getLayout(measuredNode);
const [measuredWidth, measuredHeight] = measuredLayout.get("width", "height");
measuredLayout.free();
style.free();
rootStyle.free();

return (
  <div style={{ display: "flex", gap: 10 }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
      <strong>Measured Size:</strong>
      <br />
      {measuredWidth} x {measuredHeight}
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

1 回のレイアウトで同じノードが異なる制約で複数回測定される場合も、キャッシュにより測定されない場合もあります。context の中身を直接書き換えたり、測定関数やフォントを変更したりした場合は、影響するノードに `markDirty(node)` を呼んでから再計算します。`setNodeContext()` による更新は自動的にダーティになります。

## 次のステップ

- [スタイリングガイド (Styling Guide)](../styling/index.md)
- [レイアウトクックブック (Cookbook)](../cookbook/)
