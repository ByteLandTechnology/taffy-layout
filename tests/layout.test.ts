import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  AlignItems,
  JustifyContent,
} from "../src/index";

describe("Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  it("validates basic Flex layout", () => {
    const tree = new TaffyTree();

    // Root: 100x100 Flex Column
    const rootStyle = new Style();
    rootStyle.display = Display.Flex;
    rootStyle.flexDirection = FlexDirection.Column;
    rootStyle.size = { width: 100, height: 100 };

    const root = tree.newLeaf(rootStyle); // Leaf for now

    // Child 1: 50x50
    const childStyle = new Style();
    childStyle.size = { width: 50, height: 50 };
    const child1 = tree.newLeaf(childStyle);

    // Child 2: 50x50
    const child2 = tree.newLeaf(childStyle);

    // Add children
    tree.addChild(root, child1);
    tree.addChild(root, child2);

    // Compute
    tree.computeLayout(root, { width: 100, height: 100 });

    // Verify Root
    const rootLayout = tree.getLayout(root);
    expect(rootLayout.width).toBe(100);
    expect(rootLayout.height).toBe(100);

    // Verify Children positions (stacked vertically)
    const child1Layout = tree.getLayout(child1);
    expect(child1Layout.x).toBe(0);
    expect(child1Layout.y).toBe(0);
    expect(child1Layout.width).toBe(50);
    expect(child1Layout.height).toBe(50);

    const child2Layout = tree.getLayout(child2);
    expect(child2Layout.x).toBe(0);
    expect(child2Layout.y).toBe(50); // Stacked below child1
    expect(child2Layout.width).toBe(50);
    expect(child2Layout.height).toBe(50);

    // Clean up
    tree.free();
    // Note: Styles are copied into tree or referenced?
    // TaffyTree owns the styles once added? No, Style passed to newLeaf is copied.
    rootStyle.free();
    childStyle.free();
  });

  it("validates Center alignment", () => {
    const tree = new TaffyTree();
    const rootStyle = new Style();
    rootStyle.display = Display.Flex;
    rootStyle.size = { width: 100, height: 100 };
    rootStyle.alignItems = AlignItems.Center;
    rootStyle.justifyContent = JustifyContent.Center;

    const root = tree.newLeaf(rootStyle);

    const childStyle = new Style();
    childStyle.size = { width: 20, height: 20 };
    const child = tree.newLeaf(childStyle);

    tree.addChild(root, child);

    tree.computeLayout(root, { width: 100, height: 100 });

    const layout = tree.getLayout(child);
    // Centered in 100x100: (100-20)/2 = 40
    expect(layout.x).toBe(40);
    expect(layout.y).toBe(40);

    tree.free();
    rootStyle.free();
    childStyle.free();
  });
});
