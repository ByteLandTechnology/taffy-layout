# 🚨 错误处理

**安全地处理异常和无效状态。**

Taffy 操作通常不会抛出错误，但无效的 API 使用（如访问不存在的节点）可能会引发 `TaffyError`。

## ⚠️ 常见错误场景

| 错误类型                    | 原因                                            | 解决方案                             |
| :-------------------------- | :---------------------------------------------- | :----------------------------------- |
| **`InvalidInputNode`**      | 访问已被释放或从未存在的节点 ID。               | 确保节点 ID 匹配一个有效的活动节点。 |
| **`ChildIndexOutOfBounds`** | 调用 `getChildAtIndex` 时索引 >= `childCount`。 | 在访问之前检查 `childCount`。        |
| **`InvalidParentNode`**     | 删除一个未附加到父节点的子节点。                | 仔细跟踪您的树结构。                 |

## 🛡️ 最佳实践

如果处理动态或用户生成的树结构，将树操作包装在 `try-catch` 块中。

```ts
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const someNodeId = tree.newLeaf(new Style());

try {
  // 示例：尝试访问可能无效的节点
  const layout = tree.getLayout(someNodeId);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error(`Taffy Layout Error: ${e.message}`);
  } else {
    throw e;
  }
}
```

## 🔍 验证模式

与其依赖 catch，不如验证索引：

```ts
const tree = new TaffyTree();
const parentNode = tree.newLeaf(new Style());
const index = 0;

const count = tree.childCount(parentNode);
if (index < count) {
  const child = tree.getChildAtIndex(parentNode, index);
  // ... 安全使用子节点
}
```
