---
title: 交差軸揃え (Align Items)
sidebar_position: 14
---

# 交差軸揃え (Align Items)

**交差軸に沿ったアイテムの配置を制御します。**

`alignItems` プロパティは、現在のライン上の**交差軸**におけるフレックスアイテムのデフォルトの配置動作を定義します。これは交差軸方向（主軸に垂直）の `justifyContent` と考えることができます。

## 値

| 值              | 説明                                                                                                                      |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------ |
| **`Stretch`**   | **デフォルト**。交差軸サイズが `auto` のアイテムを、その行のサイズまで引き伸ばします（min/max 制約に従います）。          |
| **`FlexStart`** | アイテムを交差軸の始端に配置します。                                                                                      |
| **`FlexEnd`**   | アイテムを交差軸の終端に配置します。                                                                                      |
| **`Center`**    | アイテムを交差軸の中央に配置します。                                                                                      |
| **`Baseline`**  | エンジンが計算したベースラインに基づいてアイテムを配置します。測定関数から独自の文字ベースラインを返す API はありません。 |

`Start` / `End` はコンテナの論理方向、`SelfStart` / `SelfEnd` はアイテム自身の `direction` に従って配置します。安全な配置には `SafeStart`、`SafeEnd`、`SafeFlexStart`、`SafeFlexEnd`、`SafeCenter`、`SafeSelfStart`、`SafeSelfEnd` があり、内容が収まらない場合は始端配置に戻して始端側のはみ出しを防ぎます。

`alignItems` は Grid のブロック軸にも適用されます。Grid のインライン軸には同じ `AlignItems` 列挙型を取る `justifyItems` を使います。未設定のプロパティは `undefined` を返し、表のデフォルト値はレイアウト時の動作を示します。

## 例

```tsx live
const tree = new TaffyTree();

const style = new Style({
  display: Display.Flex,
  size: { width: 50, height: 30 },
  alignItems: AlignItems.Center,
  justifyContent: JustifyContent.Center,
});

const labelStyle = new Style({
  flexGrow: 1,
});

// 配置をデモンストレーションするために異なる高さの子要素を作成
const child1 = tree.newLeaf(new Style({ size: { width: 50, height: 20 } }));
const child2 = tree.newLeaf(new Style({ size: { width: 50, height: 40 } }));
const child3 = tree.newLeaf(new Style({ size: { width: 50, height: 60 } }));

const rootStyle = new Style({
  display: Display.Flex,
  flexDirection: FlexDirection.Row,
  size: { width: 300, height: 100 },
  gap: { width: 10, height: 0 },
  padding: { left: 10, right: 10, top: 10, bottom: 10 },

  // ここを変更して様々な値をテスト
  alignItems: AlignItems.Center,
  // オプション: FlexStart, FlexEnd, Stretch, Baseline
});

const root = tree.newWithChildren(rootStyle, [child1, child2, child3]);

tree.computeLayout(root, { width: 300, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 次のステップ

- [自己配置 (Align Self)](./align-self.md)
- [主軸揃え (Justify Content)](./justify-content.md)
