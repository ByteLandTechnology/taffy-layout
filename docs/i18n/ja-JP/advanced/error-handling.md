# 🚨 エラー処理

**例外と無効な状態を安全に処理します。**

Taffy の操作は通常エラーをスローしませんが、無効な API 使用（存在しないノードへのアクセスなど）により `TaffyError` が発生する可能性があります。

## ⚠️ 一般的なエラーシナリオ

| エラー型                    | 原因                                                             | ソリューション                                                   |
| :-------------------------- | :--------------------------------------------------------------- | :--------------------------------------------------------------- |
| **`InvalidInputNode`**      | 解放されたまたは存在しなかったノード ID にアクセスする。         | ノード ID が有効なアクティブなノードと一致することを確認します。 |
| **`ChildIndexOutOfBounds`** | `childCount` 以上のインデックスで `getChildAtIndex` を呼び出す。 | アクセス前に `childCount` をチェックします。                     |
| **`InvalidParentNode`**     | 親にアタッチされていない子を削除する。                           | ツリー構造を慎重に追跡します。                                   |

## 🛡️ ベストプラクティス

動的またはユーザー生成のツリー構造を扱う場合は、ツリー操作を `try-catch` ブロックでラップします。

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const someNodeId = tree.newLeaf(new Style());

try {
  // 例：無効な可能性があるノードにアクセスしようとする
  const layout = tree.getLayout(someNodeId);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
  } else {
    throw e;
  }
}
```

## 🔍 検証パターン

catch に依存するよりも、インデックスを検証します：

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... 子を安全に使用
}
```
