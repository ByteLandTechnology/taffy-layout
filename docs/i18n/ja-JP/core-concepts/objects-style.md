---
title: Style オブジェクト
sidebar_position: 1
---

# Style オブジェクト

**`Style`** オブジェクトは、単一のノードのレイアウト規則を定義します。ノードのサイズ、配置、および子ノードの並べ方を決定するプロパティが含まれています。

## 主な役割

- **レイアウトモードと配置**: `display` で Flexbox、Grid、Block などのアルゴリズムを選び、`position` で相対配置または絶対配置を指定します。
- **寸法**: 幅、高さ、アスペクト比、および最小/最大制約を定義します。
- **間隔**: マージン、パディング、ボーダー、およびギャップ（gap）を制御します。
- **整列**: メイン軸およびクロス軸に沿って子ノードをどのように整列させるかを指定します。

## 使い方

スタイルは通常、ノード作成時に作成され、ノードに渡されます。

```typescript
const style = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 100, height: 100 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },
});
```

`new Style()` の `display` は `Display.Flex` です。ノード作成時と `setStyle()` 呼び出し時にスタイルがコピーされるため、元のオブジェクトや `getStyle()` が返すコピーを変更した後は `tree.setStyle(node, style)` でノードへ反映します。

コピー後に元の `Style` が不要なら `.free()` で解放できます。`getStyle()` が返したコピーも呼び出し側が所有するので、使用後に解放してください。

`Auto` 以外の配置値を設定した場合、`get()` と直接プロパティアクセスは同じ公開列挙値を返します。未設定または `Auto` の `alignSelf` / `justifySelf` は例外で、直接プロパティは `AlignSelf.Auto`、`get()` は `undefined` を返します。値の加減算や入れ替えをせず、エクスポートされた列挙型を使って比較・設定してください。

## 次のステップ

- [TaffyTree オブジェクト](./objects-taffy-tree.md)
- [Layout オブジェクト](./objects-layout.md)
