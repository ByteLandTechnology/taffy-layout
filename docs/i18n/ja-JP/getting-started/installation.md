---
title: インストール
sidebar_position: 1
---

# インストール

```bash
npm install taffy-layout
```

## 動作環境

- **Node.js**: バージョン 12 以上。
- **ブラウザ**: **WebAssembly** をサポートするモダンブラウザ：
  - Chrome 57+
  - Firefox 52+
  - Safari 11+
  - Edge 16+

## ブラウザでの使用

ES Modules を通じてブラウザで Taffy を使用します：

```html
<script type="module">
  import {
    loadTaffy,
    TaffyTree,
  } from "https://cdn.jsdelivr.net/npm/taffy-layout/dist/index.js";

  // 1. WebAssembly を初期化（必須）
  await loadTaffy();

  // 2. Taffy の使用を開始
  const tree = new TaffyTree();
</script>
```

## Node.js での使用

Node.js 環境で Taffy を使用します：

```typescript
import { loadTaffy, TaffyTree } from "taffy-layout";

// 1. WebAssembly を初期化（必須）
await loadTaffy();

// 2. Taffy の使用を開始
const tree = new TaffyTree();
```

## TypeScript サポート

Taffy Layout は**完全な TypeScript 型定義**を含んでおり、すぐに使用できます。追加のインストール（`@types/taffy-layout` など）は不要です。`Style`、`Layout`、`TaffyTree` の完全なインテリセンスが利用できます。

## 次のステップ

- **[クイックスタートガイド](./quick-start.md)** - 最初の Taffy レイアウトを作成します。
- **[API リファレンス](../../api/index.md)** - 完全な API ドキュメントを確認します。
