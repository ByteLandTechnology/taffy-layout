---
title: Layout オブジェクト
sidebar_position: 3
---

# Layout オブジェクト

**`Layout`** オブジェクトには、レイアウトプロセスが終了した後のノードの最終的な計算結果が含まれます。

`getLayout()` はその時点の独立したスナップショットを返します。再計算後は新しく取得し、不要になった各 `Layout` は `.free()` で解放してください。ツリーを解放しても、取得済みのコピーは別に解放する必要があります。

## 主なプロパティ

- **`position`**: 親ノードの左上隅を基準としたノードの `x` および `y` 座標。
- **`size`**: ノードの計算された幅（`width`）と高さ（`height`）（ピクセル単位）。
- **`margin` / `padding` / `border`**: 解決されたエッジのサイズ。

`contentSize`（および `contentWidth` / `contentHeight`）は、スクロール原点から到達可能なコンテンツ範囲を示し、ノードのサイズを超える場合があります。padding と border を差し引いた CSS コンテンツボックスのサイズではありません。スクロール原点は LTR では padding box の左上、RTL では右上で、始端側のスクロールできない負のはみ出しは含みません。

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

## Grid の詳細結果

`tree.detailedLayoutInfo(node)` は最後に保存された Grid 詳細結果を取得し、`rows`、`columns`、`items` を持つ通常の JavaScript オブジェクトを直接返します。`.Grid` のラッパーや `.free()` はありません。一度も詳細結果を生成していないノードでは `null` です。初めから Grid 以外のノードや、リーフとして計算された子のない Grid もこれに該当します。

スタイルや子を変更しても、以前の Grid 詳細は自動的に消去されません。例えば Grid を Flex に変更して再計算しても古い詳細が残ることがあります。現在のノードが Grid コンテナとして計算されたことを確認してから結果を使用してください。

各軸には `sizes`、`positions`、`gutters` が含まれます。`positions` の `{ start, end }` はコンテナのボーダーボックスを基準とする物理座標で、列の左端・右端または行の上端・下端を表します。配列は論理トラック順なので、RTL の列座標は減少する場合がありますが、各トラックでは `start <= end` です。

`gutters` は隣接トラック間の実際の間隔で、コンテンツ配置によって追加された空間も含みます。長さは `sizes.length + 1`、先頭と末尾は `0`、トラックがない場合は `[0]` です。コンテナの padding と端側の配置余白は `positions` に反映されます。

## 次のステップ

- [サイズ、スペース、および単位](./size-and-space.md)
- [計測関数 (Measure Functions)](./measure-functions.md)
