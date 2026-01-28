---
title: Installation
sidebar_position: 1
---

# Installation

```bash
npm install taffy-layout
```

## Requirements

- **Node.js**: Version 12 or higher.
- **Browser**: Modern browser with **WebAssembly** support:
  - Chrome 57+
  - Firefox 52+
  - Safari 11+
  - Edge 16+

## Browser Usage

Using Taffy in the browser via ES Modules:

```html
<script type="module">
  import {
    loadTaffy,
    TaffyTree,
  } from "https://cdn.jsdelivr.net/npm/taffy-layout/dist/index.js";

  // 1. Initialize WebAssembly (Required)
  await loadTaffy();

  // 2. Start using Taffy
  const tree = new TaffyTree();
</script>
```

## Node.js Usage

Using Taffy in a Node.js environment:

```typescript
import { loadTaffy, TaffyTree } from "taffy-layout";

// 1. Initialize WebAssembly (Required)
await loadTaffy();

// 2. Start using Taffy
const tree = new TaffyTree();
```

## TypeScript Support

Taffy Layout includes **complete TypeScript definitions** out of the box. No additional installation (like `@types/taffy-layout`) is needed. You get full Intellisense for `Style`, `Layout`, and `TaffyTree`.

## Next Steps

- 👉 **[Quick Start Guide](./quick-start.md)** - Create your first Taffy layout.
- 📖 **[API Reference](../api/index.md)** - View the full API documentation.
