import { beforeAll, describe, expect, it } from "vitest";
import { BoxSizing, Style, TaffyTree, type AvailableSpace } from "../src/index";
import { setupTaffy } from "./utils";

describe("Content measurement", () => {
  beforeAll(setupTaffy);

  it("measures a leaf without attached context", () => {
    const tree = new TaffyTree();
    const style = new Style();
    const node = tree.newLeaf(style);
    let calls = 0;
    let measuredContext: unknown = "not measured";

    tree.computeLayoutWithMeasure(
      node,
      { width: 300, height: 100 },
      (known, available, measuredNode, context, measuredStyle) => {
        calls++;
        measuredContext = context;
        measuredStyle.free();
        return { width: 150, height: 50 };
      },
    );

    const layout = tree.getLayout(node);
    expect(calls).toBeGreaterThan(0);
    expect(measuredContext).toBeUndefined();
    expect(layout.size).toEqual({ width: 150, height: 50 });
    layout.free();
    style.free();
    tree.free();
  });

  it.each([
    [BoxSizing.BorderBox, 100, 76],
    [BoxSizing.ContentBox, 124, 100],
  ])(
    "keeps content measurement and box sizing for mode %s",
    (boxSizing, expectedWidth, availableWidth) => {
      const tree = new TaffyTree();
      const style = new Style({
        width: 100,
        boxSizing,
        padding: { left: 10, right: 10, top: 10, bottom: 10 },
        border: { left: 2, right: 2, top: 2, bottom: 2 },
      });
      const context = { text: "measured content" };
      const node = tree.newLeafWithContext(style, context);
      const measuredWidths: AvailableSpace[] = [];

      tree.computeLayoutWithMeasure(
        node,
        { width: 500, height: 500 },
        (known, available, measuredNode, measuredContext, measuredStyle) => {
          expect(measuredNode).toBe(node);
          expect(measuredContext).toBe(context);
          expect(measuredStyle.width).toBe(100);
          expect(measuredStyle.boxSizing).toBe(boxSizing);
          measuredWidths.push(available.width);
          measuredStyle.free();
          return { width: known.width ?? 40, height: 30 };
        },
      );

      const layout = tree.getLayout(node);
      expect(measuredWidths).toContain(availableWidth);
      expect(layout.size).toEqual({ width: expectedWidth, height: 54 });
      expect(tree.printTree(node)).toContain("content_w:");
      layout.free();
      style.free();
      tree.free();
    },
  );
});
