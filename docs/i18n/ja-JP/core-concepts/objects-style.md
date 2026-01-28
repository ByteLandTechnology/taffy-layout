---
title: Style オブジェクト
sidebar_position: 1
---

# Style オブジェクト

**`Style`** オブジェクトは、単一のノードのレイアウト規則を定義します。ノードのサイズ、配置、および子ノードの並べ方を決定するプロパティが含まれています。

## 主な役割

- **レイアウトモード**: ノードが Flexbox、Grid、または絶対配置のいずれを使用するかを決定します。
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

## 次のステップ

- [TaffyTree オブジェクト](./objects-taffy-tree.md)
- [Layout オブジェクト](./objects-layout.md)
