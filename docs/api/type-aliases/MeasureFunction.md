# MeasureFunction

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

| Parameter         | Type                                                       | Description                                                                                                                |
| ----------------- | ---------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `knownDimensions` | [`Size`](Size.md)\<`number` \| `undefined`\>               | Dimensions already determined by constraints. Each dimension is `number` if known, or `undefined` if needs to be measured. |
| `availableSpace`  | [`Size`](Size.md)\<[`AvailableSpace`](AvailableSpace.md)\> | The available space constraints for the node. Can be definite pixels, "min-content", or "max-content".                     |
| `node`            | `bigint`                                                   | The node ID (`bigint`) of the node being measured                                                                          |
| `context`         | `any`                                                      | Value attached via `newLeafWithContext()` or `setNodeContext()`, or `undefined` when the node has no attached context      |
| `style`           | [`Style`](../classes/Style.md)                             | An owned copy of the node's current Style; call `free()` when finished                                                     |

## Returns

[`Size`](Size.md)\<`number`\>

- The measured size of the content in pixels

## Remarks

Padding, borders, size constraints, and aspect ratios are applied by the layout
engine around this content measurement. Available space is adjusted for the
content box. Measurements may be cached, so the callback need not run for every
node on every layout pass. Call `markDirty()` after changing measured content
without changing its style or context.
A context is optional; nodes created with `newLeaf()` can also be measured.
Mutating an attached context object or changing the measurement function does
not invalidate cached measurements; mark the affected nodes dirty first.
The callback must be synchronous. Thrown exceptions and invalid return values
are currently converted to a zero content measurement by the binding. Record
failures and handle them outside `computeLayoutWithMeasure()` if needed.

## Example

```typescript
import init, {
  TaffyTree,
  Style,
  type MeasureFunction,
  type Size,
} from "taffy-layout";

interface TextContext {
  text: string;
  fontSize: number;
}

await init();
const tree = new TaffyTree();

const style = new Style();
const context: TextContext = { text: "Hello, World!", fontSize: 16 };
const textNode: bigint = tree.newLeafWithContext(style, context);

// Helper function to measure text width
const measureTextWidth = (text: string, fontSize: number) =>
  text.length * fontSize * 0.6;

// Typed measure function
const measureText: MeasureFunction = (
  knownDimensions,
  availableSpace,
  node,
  context,
  style,
): Size<number> => {
  const ctx = context as TextContext | undefined;
  style.free(); // This measurement does not need to read the style copy.
  if (!ctx?.text) {
    return {
      width: knownDimensions.width ?? 0,
      height: knownDimensions.height ?? 0,
    };
  }

  const width =
    knownDimensions.width ?? measureTextWidth(ctx.text, ctx.fontSize);
  const height = knownDimensions.height ?? ctx.fontSize * 1.2;

  return { width, height };
};

tree.computeLayoutWithMeasure(
  textNode,
  { width: 200, height: "max-content" },
  measureText,
);
```
