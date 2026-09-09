import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  TaffyTree,
  Style,
  Display,
  TextAlign,
  AlignContent,
  Clear,
  Float,
} from "../src/index";

describe("Block Style Properties", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Display Block", () => {
    it("display: sets to Block correctly", () => {
      const style = new Style();
      expect(style.display).toBe(Display.Flex); // default

      style.display = Display.Block;
      expect(style.display).toBe(Display.Block);
    });
  });

  describe("Item Properties", () => {
    it("itemIsTable: defaults to false, sets and gets correctly", () => {
      const style = new Style();
      expect(style.itemIsTable).toBe(false);

      style.itemIsTable = true;
      expect(style.itemIsTable).toBe(true);

      style.itemIsTable = false;
      expect(style.itemIsTable).toBe(false);
    });

    it("itemIsReplaced: defaults to false, sets and gets correctly", () => {
      const style = new Style();
      expect(style.itemIsReplaced).toBe(false);

      style.itemIsReplaced = true;
      expect(style.itemIsReplaced).toBe(true);

      style.itemIsReplaced = false;
      expect(style.itemIsReplaced).toBe(false);
    });

    it("itemIsTable and itemIsReplaced: can be toggled multiple times", () => {
      const style = new Style();

      for (let i = 0; i < 5; i++) {
        style.itemIsTable = true;
        expect(style.itemIsTable).toBe(true);
        style.itemIsTable = false;
        expect(style.itemIsTable).toBe(false);
      }

      for (let i = 0; i < 5; i++) {
        style.itemIsReplaced = true;
        expect(style.itemIsReplaced).toBe(true);
        style.itemIsReplaced = false;
        expect(style.itemIsReplaced).toBe(false);
      }
    });
  });

  describe("Scrollbar Width", () => {
    it("scrollbarWidth: defaults to 0, sets and gets correctly", () => {
      const style = new Style();
      expect(style.scrollbarWidth).toBe(0);

      style.scrollbarWidth = 15;
      expect(style.scrollbarWidth).toBe(15);

      style.scrollbarWidth = 0;
      expect(style.scrollbarWidth).toBe(0);
    });

    it("scrollbarWidth: handles decimal values", () => {
      const style = new Style();

      style.scrollbarWidth = 15.5;
      expect(style.scrollbarWidth).toBeCloseTo(15.5, 5);

      style.scrollbarWidth = 0.5;
      expect(style.scrollbarWidth).toBeCloseTo(0.5, 5);
    });

    it("scrollbarWidth: handles large values", () => {
      const style = new Style();

      style.scrollbarWidth = 100;
      expect(style.scrollbarWidth).toBe(100);
    });
  });

  describe("Text Align", () => {
    it("textAlign: defaults to Auto, sets and gets correctly", () => {
      const style = new Style();
      expect(style.textAlign).toBe(TextAlign.Auto);

      style.textAlign = TextAlign.LegacyLeft;
      expect(style.textAlign).toBe(TextAlign.LegacyLeft);

      style.textAlign = TextAlign.LegacyRight;
      expect(style.textAlign).toBe(TextAlign.LegacyRight);

      style.textAlign = TextAlign.LegacyCenter;
      expect(style.textAlign).toBe(TextAlign.LegacyCenter);

      style.textAlign = TextAlign.Auto;
      expect(style.textAlign).toBe(TextAlign.Auto);
    });

    it("textAlign: all variants work correctly", () => {
      const style = new Style();

      const variants = [
        TextAlign.Auto,
        TextAlign.LegacyLeft,
        TextAlign.LegacyRight,
        TextAlign.LegacyCenter,
      ];

      for (const variant of variants) {
        style.textAlign = variant;
        expect(style.textAlign).toBe(variant);
      }
    });
  });
});

describe("Block Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Basic Block Layout", () => {
    it("validates Block layout with fixed size", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 50, height: 50 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBe(50);
      expect(childLayout.height).toBe(50);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block stacked children", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 200 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 100, height: 50 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 100, height: 50 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 200 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.y).toBe(0);
      expect(child2Layout.y).toBe(50);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Block with percentage width", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 200, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: "50%", height: 50 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 200, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBe(100);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Block with Replaced Elements", () => {
    it("validates Block layout with itemIsReplaced", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.itemIsReplaced = true;
      childStyle.aspectRatio = 2;
      childStyle.size = { width: 50, height: "auto" };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBe(50);
      expect(childLayout.height).toBe(25);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block with aspect ratio and height", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.itemIsReplaced = true;
      childStyle.aspectRatio = 2;
      childStyle.size = { width: "auto", height: 30 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.height).toBe(30);
      expect(childLayout.width).toBe(60);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block with itemIsTable", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.itemIsTable = true;
      childStyle.size = { width: 50, height: 50 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBe(50);
      expect(childLayout.height).toBe(50);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Block with Margin and Padding", () => {
    it("validates Block with margin", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 50, height: 50 };
      childStyle.margin = { left: 10, right: 10, top: 10, bottom: 10 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(10);
      expect(childLayout.y).toBe(10);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block with auto margin for centering", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 50, height: 50 };
      childStyle.margin = { left: "auto", right: "auto", top: 0, bottom: 0 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(25);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block with padding", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.padding = { left: 10, right: 10, top: 10, bottom: 10 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: "100%", height: "100%" };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(10);
      expect(childLayout.y).toBe(10);
      expect(childLayout.width).toBe(80);
      expect(childLayout.height).toBe(80);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Block with Border", () => {
    it("validates Block with border", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.border = { left: 5, right: 5, top: 5, bottom: 5 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: "100%", height: "100%" };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(5);
      expect(childLayout.y).toBe(5);
      expect(childLayout.width).toBe(90);
      expect(childLayout.height).toBe(90);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Block with Scrollbar Width", () => {
    it("validates Block with scrollbarWidth affects content", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.scrollbarWidth = 10;

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: "100%", height: "100%" };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBeLessThanOrEqual(100);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Block with Min/Max Size", () => {
    it("validates Block with minSize", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 20, height: 20 };
      childStyle.minSize = { width: 50, height: 50 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBeGreaterThanOrEqual(50);
      expect(childLayout.height).toBeGreaterThanOrEqual(50);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Block with maxSize", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Block;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 100, height: 100 };
      childStyle.maxSize = { width: 50, height: 50 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBeLessThanOrEqual(50);
      expect(childLayout.height).toBeLessThanOrEqual(50);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });
});

describe("Float clearance and content alignment", () => {
  beforeAll(setupTaffy);

  it("contains floats in a flow root and applies clear", () => {
    const tree = new TaffyTree();
    const floated = tree.newLeaf(
      new Style({ float: Float.Left, width: 20, height: 30 }),
    );
    const cleared = tree.newLeaf(
      new Style({ clear: Clear.Both, width: 30, height: 10 }),
    );
    const root = tree.newWithChildren(
      new Style({ display: Display.FlowRoot, width: 100 }),
      [floated, cleared],
    );

    tree.computeLayout(root, { width: 100, height: "max-content" });

    expect(tree.getLayout(floated).y).toBe(0);
    expect(tree.getLayout(cleared).y).toBe(30);
    expect(tree.getLayout(root).height).toBe(40);
  });

  it("applies align-content to block layout", () => {
    const tree = new TaffyTree();
    const child = tree.newLeaf(new Style({ width: 20, height: 20 }));
    const root = tree.newWithChildren(
      new Style({
        display: Display.Block,
        alignContent: AlignContent.Center,
        width: 100,
        height: 100,
      }),
      [child],
    );

    tree.computeLayout(root, { width: 100, height: 100 });
    expect(tree.getLayout(child).y).toBe(40);
  });
});
