# ![Taffy Layout Logo](../../taffy.svg) Taffy Layout

[English](../../../README.md) | [简体中文](README.md) | [日本語](../ja-JP/README.md)

[![npm version](https://badge.fury.io/js/taffy-layout.svg)](https://www.npmjs.com/package/taffy-layout)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

基于 WebAssembly 的高性能 [Taffy](https://github.com/DioxusLabs/taffy) 布局引擎 JavaScript 绑定，将 CSS Flexbox 和 Grid 布局算法带到前端，接近原生性能。

## 特性

- **🚀 高性能**：WebAssembly 驱动的布局计算
- **📦 布局算法**：支持 Flexbox、CSS Grid、Block 与 FlowRoot
- **🔧 自定义测量**：支持自定义文本/内容测量回调
- **📝 TypeScript 友好**：完整类型定义
- **🌳 基于树的 API**：适合复杂场景的高效树结构
- **💡 熟悉的 API**：类 CSS 的属性名称与取值

## 安装

```bash
npm install taffy-layout
```

## 快速上手

```typescript
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// 初始化 WebAssembly 模块
await loadTaffy();

// 创建布局树
const tree = new TaffyTree();

// 容器样式
const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Column;
containerStyle.alignItems = AlignItems.Center;

// 可以设置 size 对象
containerStyle.size = { width: 300, height: 200 };

// 或使用独立的 width/height 属性
containerStyle.width = 300;
containerStyle.height = 200;

// 设置 padding 对象
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

// 或使用独立的 padding 属性
containerStyle.paddingLeft = 10;
containerStyle.paddingRight = 10;
containerStyle.paddingTop = 10;
containerStyle.paddingBottom = 10;

// 子元素样式
const childStyle = new Style();
childStyle.flexGrow = 1;
childStyle.width = "100%";
childStyle.height = "auto";

// 创建节点
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// 计算布局
tree.computeLayout(container, { width: 300, height: 200 });

// 读取结果
const containerLayout = tree.getLayout(container);
const child1Layout = tree.getLayout(child1);
const child2Layout = tree.getLayout(child2);

console.log(`Container: ${containerLayout.width}x${containerLayout.height}`);
console.log(
  `Child 1: ${child1Layout.width}x${child1Layout.height} at (${child1Layout.x}, ${child1Layout.y})`,
);
console.log(
  `Child 2: ${child2Layout.width}x${child2Layout.height} at (${child2Layout.x}, ${child2Layout.y})`,
);
```

## 详细文档

- [简介](intro.md)
- [快速上手](getting-started/installation.md)
- [核心概念](core-concepts/index.md)
- [样式指南](styling/index.md)
- [高级用法](advanced/index.md)
- [Cookbook](cookbook/index.md)

## API 参考

### TaffyTree

管理布局树的核心类。

[查看文档](../../api/classes/TaffyTree.md)

### Style

用于配置节点布局属性的对象。

[查看文档](../../api/classes/Style.md)

### Layout

只读的布局计算结果。

[查看文档](../../api/classes/Layout.md)

### 枚举

[查看文档](../../api/index.md#enumerations)

### 类型别名

[查看文档](../../api/index.md#type-aliases)

## 自定义文本测量

对文本节点或需要动态测量的内容，可传入测量回调：

```typescript
const tree = new TaffyTree();
const textStyle = new Style();
const measureTextWidth = (text: string) => text.length * 8;
const measureTextHeight = (text: string, width: number) => 20;

const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });
const rootNode = tree.newWithChildren(new Style(), [textNode]);

tree.computeLayoutWithMeasure(
  rootNode,
  { width: 800, height: "max-content" },
  (known, available, node, context, style) => {
    style.free(); // 本例不需要测量回调传入的独立样式副本
    if (context?.text) {
      // 在这里实现文本测量逻辑
      const limit =
        typeof available.width === "number" ? available.width : Infinity;
      const width =
        known.width ?? Math.min(measureTextWidth(context.text), limit);
      const height = known.height ?? measureTextHeight(context.text, width);
      return { width, height };
    }
    return { width: 0, height: 0 };
  },
);
```

## 错误处理

对返回受检查错误的方法，可用 try-catch 处理 `TaffyError`。节点 ID 必须来自同一棵树且仍有效；无效 ID 也可能触发 WebAssembly panic。详见[错误处理](advanced/error-handling.md)。

```typescript
try {
  const tree = new TaffyTree();
  const style = new Style();
  const nodeId = tree.newLeaf(style);
  console.log("Created node:", nodeId);
} catch (e) {
  if (e instanceof TaffyError) {
    console.error("Error:", e.message);
  }
}
```

## 浏览器支持

浏览器需支持 ES Modules、BigInt、JavaScript 与 WebAssembly 间的 BigInt 互操作，以及 WebAssembly reference types；仅支持基础 WebAssembly 不足以运行当前绑定。

## 示例

### Flexbox 行布局

```typescript
const rowStyle = new Style();
rowStyle.display = Display.Flex;
rowStyle.flexDirection = FlexDirection.Row;
rowStyle.justifyContent = JustifyContent.SpaceBetween;
rowStyle.gap = { width: 10, height: 0 };
```

### CSS Grid 布局

```typescript
import { Style, Display, GridAutoFlow } from "taffy-layout";

const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridAutoFlow = GridAutoFlow.Row;
gridStyle.gap = { width: 10, height: 10 };

// 网格项定位
const itemStyle = new Style();
itemStyle.gridRow = { start: 1, end: 3 }; // 跨 2 行
itemStyle.gridColumn = { start: 1, end: { span: 2 } }; // 跨 2 列
```

### 网格区域模板

```typescript
const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridTemplateAreas = [
  { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
  { name: "sidebar", rowStart: 2, rowEnd: 4, columnStart: 1, columnEnd: 2 },
  { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 },
  { name: "footer", rowStart: 4, rowEnd: 5, columnStart: 1, columnEnd: 4 },
];

// 命名网格线
gridStyle.gridTemplateRowNames = [
  ["header-start"],
  ["header-end", "content-start"],
  [],
  ["content-end", "footer-start"],
  ["footer-end"],
];
```

### 绝对定位

```typescript
const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
absoluteStyle.size = { width: 100, height: 50 };
```

### 百分比尺寸

```typescript
const percentStyle = new Style();
percentStyle.size = {
  width: "50%", // 父级宽度的 50%
  height: "100%", // 父级高度的 100%
};
```

### 替换元素的块级布局

```typescript
const imgStyle = new Style();
imgStyle.itemIsReplaced = true;
imgStyle.aspectRatio = 16 / 9; // 16:9 宽高比
imgStyle.size = { width: "100%", height: "auto" };
```

## 从源码构建

开发工具要求 Node.js 22.14 或更高的 22.x 版本，或 Node.js 24.10 及以上；这高于已发布包的 Node.js 18 运行要求。还需安装 Rust 1.85 或更高版本及 Cargo，并准备 WebAssembly 编译目标：

```bash
# 克隆仓库
git clone https://github.com/ByteLandTechnology/taffy-layout.git
cd taffy-layout

# 安装 WebAssembly 编译目标
rustup target add wasm32-unknown-unknown

# 安装依赖
npm install

# 构建 WebAssembly、TypeScript 产物及文档
npm run build
```

## 运行测试

```bash
npm test
```

## 许可证

MIT License - 详见 [LICENSE](../../../LICENSE)。

## 致谢

- [Taffy](https://github.com/DioxusLabs/taffy) - 本项目封装的 Rust 布局引擎
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WebAssembly 互操作工具
