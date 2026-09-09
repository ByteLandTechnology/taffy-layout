---
title: 错误处理
sidebar_position: 3
---

# 错误处理

**安全地处理异常和无效状态。**

调用树方法时，必须传入由同一棵树创建且仍然有效的节点 ID。`remove()` 或 `clear()` 后不要复用旧 ID，也不要混用不同树的 ID。无效 ID 的检查并不统一，部分路径会触发 WebAssembly panic；不能依赖它们必定抛出可捕获的 `TaffyError`。

返回受检查错误的方法会将其转换为 `TaffyError`，例如有效父节点上的子索引越界。

## 常见错误场景

| 错误类型                    | 原因                                            | 解决方案                      |
| :-------------------------- | :---------------------------------------------- | :---------------------------- |
| **`ChildIndexOutOfBounds`** | 调用 `getChildAtIndex` 时索引 >= `childCount`。 | 在访问之前检查 `childCount`。 |

## 最佳实践

如果处理动态或用户生成的树结构，将树操作包装在 `try-catch` 块中。

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());

try {
  // 父节点有效，但没有子节点，因此索引 0 越界
  tree.getChildAtIndex(parentNode, 0);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
  } else {
    throw e;
  }
}
```

## 验证模式

与其依赖 catch，不如验证索引：

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (Number.isInteger(index) && index >= 0 && index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... 安全使用子节点
}
```
