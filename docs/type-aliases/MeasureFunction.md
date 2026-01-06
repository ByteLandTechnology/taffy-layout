[**Taffy-JS API Documentation**](../index.md)

---

[Taffy-JS API Documentation](../index.md) / MeasureFunction

# Type Alias: MeasureFunction()

```ts
type MeasureFunction = (
  knownDimensions,
  availableSpace,
  node,
  context,
  style,
) => Size<number>;
```

Custom measure function for leaf nodes with text or other dynamic content.

This callback is invoked during layout computation for leaf nodes that need
custom sizing based on their content (e.g., text nodes that need text measurement).

## Parameters

| Parameter         | Type                                                                     | Description                                                                                                           |
| ----------------- | ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| `knownDimensions` | [`Size`](../interfaces/Size.md)\<`number` \| `null`\>                    | Dimensions already determined by constraints. Each dimension is `number` if known, or `null` if needs to be measured. |
| `availableSpace`  | [`Size`](../interfaces/Size.md)\<[`AvailableSpace`](AvailableSpace.md)\> | The available space constraints for the node. Can be definite pixels, MinContent, or MaxContent.                      |
| `node`            | `bigint`                                                                 | The node ID (`bigint`) of the node being measured                                                                     |
| `context`         | `any`                                                                    | User-provided context attached to the node via `newLeafWithContext()`                                                 |
| `style`           | [`Style`](../classes/Style.md)                                           | The node's current Style configuration                                                                                |

## Returns

[`Size`](../interfaces/Size.md)\<`number`\>

The measured size of the content in pixels

<details>
<summary><strong>TypeScript Example</strong></summary>

```typescript
import init, {
  TaffyTree,
  Style,
  type MeasureFunction,
  type Size,
} from "taffy-js";

interface TextContext {
  text: string;
  fontSize: number;
}

await init();
const tree = new TaffyTree();

const style = new Style();
const context: TextContext = { text: "Hello, World!", fontSize: 16 };
const textNode: bigint = tree.newLeafWithContext(style, context);

// Typed measure function
const measureText: MeasureFunction = (
  knownDimensions,
  availableSpace,
  node,
  context,
  style,
): Size<number> => {
  const ctx = context as TextContext | undefined;
  if (!ctx?.text) return { width: 0, height: 0 };

  const width =
    knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
  const height = knownDimensions.height ?? ctx.fontSize * 1.2;

  return { width, height };
};

tree.computeLayoutWithMeasure(
  textNode,
  { width: { Definite: 200 }, height: "MaxContent" },
  measureText,
);
```

</details>
