# ⚡ 性能

**保持 Taffy 布局快速运行的技巧。**

Taffy 旨在高效和高性能，但特定模式会影响性能。

## 1. 预分配容量

如果您知道节点数量，使用 `withCapacity` 避免重新分配。

```tsx live
const tree = TaffyTree.withCapacity(2000);
console.log(tree.totalNodeCount());

const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Row;
containerStyle.size = { width: 220, height: 70 };
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const itemStyle = new Style();
itemStyle.flexGrow = 1;

const first = tree.newLeaf(itemStyle);
const second = tree.newLeaf(itemStyle);
const root = tree.newWithChildren(containerStyle, [first, second]);

tree.computeLayout(root, { width: 220, height: 70 });

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <TaffyTreePreview tree={tree} root={root} />
    <div
      style={{
        padding: "8px 12px",
        borderRadius: 8,
        background: "rgba(0, 122, 255, 0.12)",
        color: "#0f172a",
        fontSize: 12,
        fontWeight: 600,
        display: "inline-flex",
        alignItems: "center",
      }}
    >
      Capacity: {tree.totalNodeCount()}
    </div>
  </div>
);
```

## 2. 增量布局

只有更改的节点会被重新计算。Taffy **懒惰地**行动，只重新计算受更改影响的分支。

```tsx live
const tree = new TaffyTree();

const childStyle = new Style();
childStyle.size = { width: 120, height: 60 };
const child = tree.newLeaf(childStyle);
const root = tree.newWithChildren(
  new Style({
    display: Display.Flex,
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  }),
  [child],
);

tree.computeLayout(root, { width: 200, height: 100 });

return <TaffyTreePreview tree={tree} root={root} />;
```

## 🐢 常见陷阱

### 1. 过度嵌套

每一层深度都会增加递归算法的复杂性。

- **坏**：`View -> View -> View -> Button` 仅仅为了内边距。
- **好**：在父节点上使用 `padding` 而不是包装器节点。

### 2. 深度测量函数

自定义测量函数（用于文本/图像）被频繁调用。

- **优化**：确保您的测量回调很快。避免在测量内进行 DOM 回流或繁重计算。

## 🚀 优化模式

### 重用样式

在紧密循环中创建 `Style` 对象（例如游戏渲染）在 JS 中可能很昂贵。尽可能重用定义对象。

```ts
// ✅ 好
const tree = new TaffyTree();
const ITEM_STYLE = new Style({ flexGrow: 1 });
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(ITEM_STYLE);
}
```

```ts
// ❌ 如果项目相同则避免
const tree = new TaffyTree();
for (let i = 0; i < 1000; i++) {
  tree.newLeaf(new Style({ flexGrow: 1 }));
}
```

### 批量属性访问

使用批量 getter 和 setter 最小化 WASM 桥接开销。

**样式批量更新：**
不要逐个设置属性：

```ts
const style = new Style();
// ❌ 多次调用 = 高开销
style.display = Display.Flex;
style.width = 100;
style.height = 100;
```

使用 `set()`：

```ts
const style = new Style();
// ✅ 单次调用 = 低开销
style.set({
  display: Display.Flex,
  width: 100,
  height: 100,
});
```

**布局批量读取：**
不要单独读取布局属性：

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ❌ 多次调用
const layout = tree.getLayout(node);
const x = layout.x;
const y = layout.y;
const w = layout.width;
const h = layout.height;
```

使用 `get()`：

```ts
const tree = new TaffyTree();
const node = tree.newLeaf(new Style());
tree.computeLayout(node, { width: 100, height: 100 });

// ✅ 单次调用返回值数组
const layout = tree.getLayout(node);
const [x, y, w, h] = layout.get("x", "y", "width", "height");
```

## 🔬 基准测试

使用 `performance.now()` 测量您的布局过程。

```ts
const tree = new TaffyTree();
const root = tree.newLeaf(new Style());

const start = performance.now();
tree.computeLayout(root, { width: 1000, height: 1000 });
const end = performance.now();
console.log(`Layout took ${end - start}ms`);
```

## 下一步

- [测量函数](../core-concepts/measure-functions.md)
- [调试布局](./debugging.md)
