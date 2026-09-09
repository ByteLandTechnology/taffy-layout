---
title: 尺寸、空间与单位
sidebar_position: 4
---

# 尺寸、空间和单位

Taffy 布局由可用空间加上尺寸约束驱动。理解这个模型可以使布局变得可预测。

## 可用空间

`computeLayout` 的第二个参数定义可用空间：

```tsx live
// 固定
const fixedTree = new TaffyTree();
const fixedStyle = new Style();
fixedStyle.size = { width: 120, height: 40 };
const fixedChild = fixedTree.newLeaf(fixedStyle);

const fixedRootStyle = new Style();
fixedRootStyle.display = Display.Flex;
fixedRootStyle.flexDirection = FlexDirection.Row;
fixedRootStyle.size = { width: 200, height: 80 };
fixedRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const fixedRoot = fixedTree.newWithChildren(fixedRootStyle, [fixedChild]);

fixedTree.computeLayout(fixedRoot, { width: 200, height: 80 });
console.log(fixedTree.printTree(fixedRoot));

// 弹性
const flexibleTree = new TaffyTree();
const flexibleStyle = new Style();
flexibleStyle.size = { width: "auto", height: 40 };
const flexibleChild = flexibleTree.newLeaf(flexibleStyle);

const flexibleRootStyle = new Style();
flexibleRootStyle.display = Display.Flex;
flexibleRootStyle.flexDirection = FlexDirection.Row;
flexibleRootStyle.size = { width: 200, height: 80 };
flexibleRootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

const flexibleRoot = flexibleTree.newWithChildren(flexibleRootStyle, [
  flexibleChild,
]);

flexibleTree.computeLayout(flexibleRoot, {
  width: "max-content",
  height: 80,
});
console.log(flexibleTree.printTree(flexibleRoot));

return (
  <div style={{ display: "flex", gap: 12, flexWrap: "wrap" }}>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>固定空间</div>
      <TaffyTreePreview tree={fixedTree} root={fixedRoot} />
    </div>
    <div>
      <div style={{ marginBottom: 6, fontSize: 12 }}>最大内容宽度</div>
      <TaffyTreePreview tree={flexibleTree} root={flexibleRoot} />
    </div>
  </div>
);
```

### 允许的值

- `number`: 绝对尺寸（通常是像素）
- `"min-content"`: 最小内容尺寸
- `"max-content"`: 最大内容尺寸

`"auto"` 用于样式尺寸，不是 `AvailableSpace` 的有效值。

## 盒模型

`boxSizing` 默认为 `BoxSizing.BorderBox`，此时样式尺寸包含 padding 和 border。设为 `BoxSizing.ContentBox` 后，样式尺寸仅指定内容框；最终 `Layout.size` 仍为边框框尺寸：

```text
┌─────────────────────────┐
│  Margin                 │
│  ┌───────────────────┐  │
│  │ Border            │  │
│  │  ┌─────────────┐  │  │
│  │  │ Padding     │  │  │
│  │  │  Content    │  │  │
│  │  └─────────────┘  │  │
│  └───────────────────┘  │
└─────────────────────────┘
```

- `Layout.size` 包含 padding + border
- margin 是外部间距

## 百分比

百分比尺寸相对于该轴的包含块尺寸解析。普通 Flex 子元素通常使用父内容框，Grid 子元素使用分配到的网格区域；绝对定位子元素通常使用父 padding box（Grid 放置还可能限定区域）。根节点的百分比相对于传入的确定可用尺寸。若参照尺寸尚未确定，百分比可能在该布局阶段无法解析。

```tsx live
const tree = new TaffyTree();

const style = new Style();
style.size = { width: "50%", height: "100%" };
const child = tree.newLeaf(style);

const rootStyle = new Style();
rootStyle.size = { width: 260, height: 160 };
rootStyle.padding = { left: 16, right: 16, top: 16, bottom: 16 };

const root = tree.newWithChildren(rootStyle, [child]);

tree.computeLayout(root, {
  width: 260,
  height: 160,
});

console.log(tree.printTree(root));

return <TaffyTreePreview tree={tree} root={root} />;
```

## 常见陷阱

- 可用空间不会强制所有节点填满它；没有固定尺寸、内容测量或拉伸规则的空叶子可得到零尺寸
- `"auto"` 的实际尺寸取决于布局算法、内容以及其他约束
- 使用 `computeLayoutWithMeasure()` 提供内容测量回调；节点 context 是可选的，`"max-content"` 本身不会注册回调

## 下一步

- [计算函数 (Measure Functions)](./measure-functions.md)
- [样式指南 (Styling Guide)](../styling/index.md)
