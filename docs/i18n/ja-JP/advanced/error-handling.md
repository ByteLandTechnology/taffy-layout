---
title: エラー処理
sidebar_position: 2
---

# エラー処理

**例外と無効な状態を安全に処理します。**

ツリーのメソッドには、同じツリーで作成され、現在も有効なノード ID を渡してください。`remove()` や `clear()` 後の古い ID、別のツリーの ID は使わないでください。無効な ID の検査は一律ではなく、WebAssembly panic になる経路もあるため、必ず `TaffyError` として捕捉できるとは限りません。

検査済みのエラーを返すメソッドでは、それを `TaffyError` に変換します。有効な親ノードに対して子のインデックスが範囲外の場合などが該当します。

測定コールバックの例外は別扱いで、現在のバインディングでは呼び出し元へ再送出されず測定値ゼロにフォールバックします。コールバック内で失敗を記録し、`computeLayoutWithMeasure()` の終了後に確認してください。

## 一般的なエラーシナリオ

| エラー型                    | 原因                                                             | 解決策                                       |
| :-------------------------- | :--------------------------------------------------------------- | :------------------------------------------- |
| **`ChildIndexOutOfBounds`** | `childCount` 以上のインデックスで `getChildAtIndex` を呼び出す。 | アクセス前に `childCount` をチェックします。 |

## ベストプラクティス

動的またはユーザー生成のツリー構造を扱う場合は、ツリー操作を `try-catch` ブロックで囲みます。

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());

try {
  // 親は有効ですが子がないため、インデックス 0 は範囲外です
  tree.getChildAtIndex(parentNode, 0);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
    e.free();
  } else {
    throw e;
  }
} finally {
  tree.free();
}
```

## 検証パターン

catch に依存するよりも、インデックスを検証します：

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (Number.isInteger(index) && index >= 0 && index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... 子を安全に使用
}
```
