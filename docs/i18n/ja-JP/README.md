# ![Taffy Layout Logo](../../taffy.svg) Taffy Layout

[English](../../../README.md) | [简体中文](../zh-CN/README.md) | [日本語](README.md)

[![npm version](https://badge.fury.io/js/taffy-layout.svg)](https://www.npmjs.com/package/taffy-layout)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

WebAssembly で高速に動作する [Taffy](https://github.com/DioxusLabs/taffy) レイアウトエンジンの JavaScript バインディング。CSS の Flexbox と Grid をほぼネイティブ性能で利用できます。

## 特徴

- **🚀 高性能**：WebAssembly によるレイアウト計算
- **📦 レイアウトアルゴリズム**：Flexbox、CSS Grid、Block、FlowRoot に対応
- **🔧 カスタム計測**：テキスト/コンテンツ計測用コールバックに対応
- **📝 TypeScript 対応**：型定義を同梱
- **🌳 ツリー型 API**：複雑な階層構造にも効率的
- **💡 なじみやすい API**：CSS 風のプロパティ名と値

## インストール

```bash
npm install taffy-layout
```

## クイックスタート

```typescript
import {
  loadTaffy,
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
} from "taffy-layout";

// WebAssembly モジュールを初期化
await loadTaffy();

// レイアウトツリーを作成
const tree = new TaffyTree();

// コンテナのスタイル
const containerStyle = new Style();
containerStyle.display = Display.Flex;
containerStyle.flexDirection = FlexDirection.Column;
containerStyle.alignItems = AlignItems.Center;

// size オブジェクトで設定
containerStyle.size = { width: 300, height: 200 };

// または個別の width/height プロパティを使用
containerStyle.width = 300;
containerStyle.height = 200;

// padding オブジェクトで設定
containerStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

// または個別の padding プロパティを使用
containerStyle.paddingLeft = 10;
containerStyle.paddingRight = 10;
containerStyle.paddingTop = 10;
containerStyle.paddingBottom = 10;

// 子要素のスタイル
const childStyle = new Style();
childStyle.flexGrow = 1;
childStyle.width = "100%";
childStyle.height = "auto";

// ノードを生成
const child1 = tree.newLeaf(childStyle);
const child2 = tree.newLeaf(childStyle);
const container = tree.newWithChildren(containerStyle, [child1, child2]);

// レイアウト計算
tree.computeLayout(container, { width: 300, height: 200 });

// 計算結果を取得
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

containerLayout.free();
child1Layout.free();
child2Layout.free();
containerStyle.free();
childStyle.free();
tree.free();
```

`Style` はツリーへコピーされ、`getStyle()` / `getLayout()` は独立した所有オブジェクトを返します。使用後はそれぞれ `.free()` で解放します。詳細は[メモリ管理](getting-started/configuration.md#メモリ管理)を参照してください。

## ドキュメント

- [はじめに](intro.md)
- [クイックスタート](getting-started/installation.md)
- [コアコンセプト](core-concepts/index.md)
- [スタイリングガイド](styling/index.md)
- [高度な使い方](advanced/index.md)
- [クックブック](cookbook/index.md)

## API リファレンス

### TaffyTree

レイアウトツリーを管理するメインクラス。

[ドキュメントを見る](../../api/classes/TaffyTree.md)

### Style

ノードのレイアウトプロパティを設定するオブジェクト。

[ドキュメントを見る](../../api/classes/Style.md)

### Layout

計算後のレイアウト結果 (読み取り専用)。

[ドキュメントを見る](../../api/classes/Layout.md)

### 列挙型

[ドキュメントを見る](../../api/index.md#enumerations)

### 型エイリアス

[ドキュメントを見る](../../api/index.md#type-aliases)

## カスタムテキスト計測

テキストなど動的な計測が必要な場合は、測定コールバックを渡せます。

```typescript
const tree = new TaffyTree();
const textStyle = new Style();
// 簡略化した等幅・文字単位折り返しの測定例（1文字8px、1行20px）

const textNode = tree.newLeafWithContext(textStyle, { text: "Hello, World!" });
const rootStyle = new Style();
const rootNode = tree.newWithChildren(rootStyle, [textNode]);
textStyle.free();
rootStyle.free();

tree.computeLayoutWithMeasure(
  rootNode,
  { width: 800, height: "max-content" },
  (known, available, node, context, style) => {
    style.free(); // 第5引数は所有コピー。この例では読み取る必要はありません
    const text = typeof context?.text === "string" ? context.text : "";
    const naturalWidth = text.length * 8;
    const limit =
      typeof available.width === "number"
        ? Math.max(0, available.width)
        : available.width === "min-content"
          ? Math.min(8, naturalWidth)
          : naturalWidth;
    const width = known.width ?? Math.min(naturalWidth, limit);
    const columns = Math.max(1, Math.floor(width / 8));
    const height = known.height ?? Math.ceil(text.length / columns) * 20;
    return { width, height };
  },
);

const textLayout = tree.getLayout(textNode);
console.log(`Text: ${textLayout.width}x${textLayout.height}`); // Text: 104x20
textLayout.free();
tree.free();
```

## エラーハンドリング

検査済みエラーを返すメソッドの `TaffyError` は try-catch で処理できます。ノード ID は同じツリーに属し、現在も有効である必要があります。無効な ID は WebAssembly panic になる場合もあります。[エラー処理](advanced/error-handling.md)も参照してください。

```typescript
import { TaffyTree, Style, TaffyError } from "taffy-layout";

const tree = new TaffyTree();
const style = new Style();
const nodeId = tree.newLeaf(style);
style.free();
try {
  tree.getChildAtIndex(nodeId, 0); // 有効な親ですが子がないため範囲外
} catch (e) {
  if (e instanceof TaffyError) {
    console.error("Error:", e.message);
    e.free();
  } else {
    throw e;
  }
} finally {
  tree.free();
}
```

## ブラウザ対応

WebAssembly reference types、JavaScript と WebAssembly 間の BigInt 連携、および ES Modules に対応するブラウザが必要です。基本的な WebAssembly 対応だけでは現在のバインディングを実行できません。

## サンプル

### Flexbox 行レイアウト

```typescript
const rowStyle = new Style();
rowStyle.display = Display.Flex;
rowStyle.flexDirection = FlexDirection.Row;
rowStyle.justifyContent = JustifyContent.SpaceBetween;
rowStyle.gap = { width: 10, height: 0 };
```

### CSS Grid レイアウト

```typescript
import { Style, Display, GridAutoFlow } from "taffy-layout";

const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridAutoFlow = GridAutoFlow.Row;
gridStyle.gap = { width: 10, height: 10 };

// グリッドアイテムの配置
const itemStyle = new Style();
itemStyle.gridRow = { start: 1, end: 3 }; // 2 行分
itemStyle.gridColumn = { start: 1, end: { span: 2 } }; // 2 列分
```

### グリッドテンプレート領域

```typescript
const gridStyle = new Style();
gridStyle.display = Display.Grid;
gridStyle.gridTemplateAreas = [
  { name: "header", rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 4 },
  { name: "sidebar", rowStart: 2, rowEnd: 4, columnStart: 1, columnEnd: 2 },
  { name: "main", rowStart: 2, rowEnd: 4, columnStart: 2, columnEnd: 4 },
  { name: "footer", rowStart: 4, rowEnd: 5, columnStart: 1, columnEnd: 4 },
];

// 名前付きグリッドライン
gridStyle.gridTemplateRowNames = [
  ["header-start"],
  ["header-end", "content-start"],
  [],
  ["content-end", "footer-start"],
  ["footer-end"],
];
```

### 絶対配置

```typescript
const absoluteStyle = new Style();
absoluteStyle.position = Position.Absolute;
absoluteStyle.inset = { left: 10, top: 10, right: "auto", bottom: "auto" };
absoluteStyle.size = { width: 100, height: 50 };
```

### パーセントサイズ

```typescript
const percentStyle = new Style();
percentStyle.size = {
  width: "50%", // 親幅の 50%
  height: "100%", // 親高の 100%
};
```

### 置換要素のブロックレイアウト

```typescript
const imgStyle = new Style();
imgStyle.itemIsReplaced = true;
imgStyle.aspectRatio = 16 / 9; // 16:9
imgStyle.size = { width: "100%", height: "auto" };
```

## ソースからのビルド

開発ツールには Node.js `22.14` 以降の `22.x`、または `24.10` 以降が必要です（リリースツールの要件）。Rust は `1.85` 以降と `wasm32-unknown-unknown` ターゲットを使用します。公開パッケージを利用するだけなら Node.js `18` 以降です。

```bash
# リポジトリを取得
git clone https://github.com/ByteLandTechnology/taffy-layout.git
cd taffy-layout

# WebAssembly ターゲットを追加
rustup target add wasm32-unknown-unknown

# 依存関係をインストール
npm install

# WebAssembly モジュールをビルド
npm run build
```

## テストの実行

```bash
npm test
```

## ライセンス

MIT License - 詳細は [LICENSE](../../../LICENSE) を参照してください。

## 謝辞

- [Taffy](https://github.com/DioxusLabs/taffy) - 本プロジェクトがラップしている Rust 製レイアウトエンジン
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WebAssembly の相互運用を支えるツール
