import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  TaffyTree,
  Style,
  Display,
  FlexDirection,
  FlexWrap,
  AlignItems,
  AlignSelf,
  AlignContent,
  JustifyContent,
  Direction,
} from "../src/index";

describe("Flex Style Properties", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Flex Direction", () => {
    it("flexDirection: defaults to Row, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexDirection).toBe(FlexDirection.Row);

      style.flexDirection = FlexDirection.Column;
      expect(style.flexDirection).toBe(FlexDirection.Column);

      style.flexDirection = FlexDirection.RowReverse;
      expect(style.flexDirection).toBe(FlexDirection.RowReverse);

      style.flexDirection = FlexDirection.ColumnReverse;
      expect(style.flexDirection).toBe(FlexDirection.ColumnReverse);

      style.flexDirection = FlexDirection.Row;
      expect(style.flexDirection).toBe(FlexDirection.Row);
    });
  });

  describe("Flex Wrap", () => {
    it("flexWrap: defaults to NoWrap, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexWrap).toBe(FlexWrap.NoWrap);

      style.flexWrap = FlexWrap.Wrap;
      expect(style.flexWrap).toBe(FlexWrap.Wrap);

      style.flexWrap = FlexWrap.WrapReverse;
      expect(style.flexWrap).toBe(FlexWrap.WrapReverse);

      style.flexWrap = FlexWrap.NoWrap;
      expect(style.flexWrap).toBe(FlexWrap.NoWrap);
    });
  });

  describe("Flex Grow and Shrink", () => {
    it("flexGrow: defaults to 0, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexGrow).toBe(0);

      style.flexGrow = 1;
      expect(style.flexGrow).toBe(1);

      style.flexGrow = 2.5;
      expect(style.flexGrow).toBe(2.5);

      style.flexGrow = 0;
      expect(style.flexGrow).toBe(0);
    });

    it("flexShrink: defaults to 1, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexShrink).toBe(1);

      style.flexShrink = 0;
      expect(style.flexShrink).toBe(0);

      style.flexShrink = 3.5;
      expect(style.flexShrink).toBe(3.5);

      style.flexShrink = 1;
      expect(style.flexShrink).toBe(1);
    });
  });

  describe("Flex Basis", () => {
    it("flexBasis: defaults to auto, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexBasis).toBe("auto");

      style.flexBasis = 100;
      expect(style.flexBasis).toBe(100);

      style.flexBasis = "50%";
      expect(style.flexBasis).toBe("50%");

      style.flexBasis = "auto";
      expect(style.flexBasis).toBe("auto");
    });

    it("flexBasis: handles various values", () => {
      const style = new Style();

      style.flexBasis = 0;
      expect(style.flexBasis).toBe(0);

      style.flexBasis = 200;
      expect(style.flexBasis).toBe(200);

      style.flexBasis = "100%";
      expect(style.flexBasis).toBe("100%");

      style.flexBasis = "25%";
      expect(style.flexBasis).toBe("25%");
    });
  });

  describe("Alignment Properties", () => {
    it("alignItems: sets and gets correctly", () => {
      const style = new Style();
      expect(style.alignItems).toBeUndefined();

      style.alignItems = AlignItems.Start;
      expect(style.alignItems).toBe(AlignItems.Start);

      style.alignItems = AlignItems.End;
      expect(style.alignItems).toBe(AlignItems.End);

      style.alignItems = AlignItems.Center;
      expect(style.alignItems).toBe(AlignItems.Center);

      style.alignItems = AlignItems.Baseline;
      expect(style.alignItems).toBe(AlignItems.Baseline);

      style.alignItems = AlignItems.Stretch;
      expect(style.alignItems).toBe(AlignItems.Stretch);

      style.alignItems = undefined;
      expect(style.alignItems).toBeUndefined();
    });

    it("alignSelf: sets and gets correctly", () => {
      const style = new Style();
      expect(style.alignSelf).toBe(AlignSelf.Auto);

      style.alignSelf = AlignSelf.FlexStart;
      expect(style.alignSelf).toBe(AlignSelf.FlexStart);

      style.alignSelf = AlignSelf.FlexEnd;
      expect(style.alignSelf).toBe(AlignSelf.FlexEnd);

      style.alignSelf = AlignSelf.Center;
      expect(style.alignSelf).toBe(AlignSelf.Center);

      style.alignSelf = AlignSelf.Auto;
      expect(style.alignSelf).toBe(AlignSelf.Auto);
    });

    it("alignContent: sets and gets correctly", () => {
      const style = new Style();
      expect(style.alignContent).toBeUndefined();

      style.alignContent = AlignContent.Start;
      expect(style.alignContent).toBe(AlignContent.Start);

      style.alignContent = AlignContent.End;
      expect(style.alignContent).toBe(AlignContent.End);

      style.alignContent = AlignContent.SpaceBetween;
      expect(style.alignContent).toBe(AlignContent.SpaceBetween);

      style.alignContent = AlignContent.SpaceAround;
      expect(style.alignContent).toBe(AlignContent.SpaceAround);

      style.alignContent = AlignContent.SpaceEvenly;
      expect(style.alignContent).toBe(AlignContent.SpaceEvenly);

      style.alignContent = undefined;
      expect(style.alignContent).toBeUndefined();
    });

    it("justifyContent: sets and gets correctly", () => {
      const style = new Style();
      expect(style.justifyContent).toBeUndefined();

      style.justifyContent = JustifyContent.Start;
      expect(style.justifyContent).toBe(JustifyContent.Start);

      style.justifyContent = JustifyContent.End;
      expect(style.justifyContent).toBe(JustifyContent.End);

      style.justifyContent = JustifyContent.Center;
      expect(style.justifyContent).toBe(JustifyContent.Center);

      style.justifyContent = JustifyContent.SpaceBetween;
      expect(style.justifyContent).toBe(JustifyContent.SpaceBetween);

      style.justifyContent = JustifyContent.SpaceAround;
      expect(style.justifyContent).toBe(JustifyContent.SpaceAround);

      style.justifyContent = JustifyContent.SpaceEvenly;
      expect(style.justifyContent).toBe(JustifyContent.SpaceEvenly);

      style.justifyContent = undefined;
      expect(style.justifyContent).toBeUndefined();
    });
  });
});

describe("Flex Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Basic Flex Layout", () => {
    it("validates Flex row layout", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 30, height: 30 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 30, height: 30 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.x).toBe(0);
      expect(child2Layout.x).toBe(30);
      expect(child1Layout.y).toBe(child2Layout.y);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Flex column layout", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Column;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 50, height: 50 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 50, height: 50 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.y).toBe(0);
      expect(child2Layout.y).toBe(50);
      expect(child1Layout.x).toBe(child2Layout.x);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Flex row-reverse layout", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.RowReverse;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 30, height: 30 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 30, height: 30 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // In row-reverse, first child should be on the right
      expect(child1Layout.x).toBeGreaterThan(child2Layout.x);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Flex column-reverse layout", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.ColumnReverse;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 50, height: 30 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 50, height: 30 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // In column-reverse, first child should be at the bottom
      expect(child1Layout.y).toBeGreaterThan(child2Layout.y);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Grow", () => {
    it("validates flexGrow distributes space", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.flexGrow = 1;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.flexGrow = 1;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // Both should share space equally
      expect(child1Layout.width).toBe(50);
      expect(child2Layout.width).toBe(50);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates flexGrow with different ratios", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.flexGrow = 1;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.flexGrow = 3;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // 1:3 ratio
      expect(child1Layout.width).toBe(25);
      expect(child2Layout.width).toBe(75);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Shrink", () => {
    it("validates flexShrink when content overflows", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 100, height: 30 };
      child1Style.flexShrink = 1;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 100, height: 30 };
      child2Style.flexShrink = 1;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // Both should shrink equally
      expect(child1Layout.width).toBe(50);
      expect(child2Layout.width).toBe(50);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates flexShrink = 0 prevents shrinking", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 60, height: 30 };
      child1Style.flexShrink = 0;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 60, height: 30 };
      child2Style.flexShrink = 1;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // Child1 should keep its size, child2 shrinks
      expect(child1Layout.width).toBe(60);
      expect(child2Layout.width).toBe(40);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Basis", () => {
    it("validates flexBasis sets initial size", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.flexBasis = 30;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.flexBasis = 70;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.width).toBe(30);
      expect(child2Layout.width).toBe(70);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates flexBasis with percentage", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.size = { width: 200, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.flexBasis = "50%";
      const child1 = tree.newLeaf(child1Style);

      tree.addChild(root, child1);

      tree.computeLayout(root, { width: 200, height: 50 });

      const child1Layout = tree.getLayout(child1);
      expect(child1Layout.width).toBe(100);

      tree.free();
      rootStyle.free();
      child1Style.free();
    });
  });

  describe("Flex Wrap", () => {
    it("validates flexWrap: wrap", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.flexWrap = FlexWrap.Wrap;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyles: Style[] = [];
      const children: bigint[] = [];
      for (let i = 0; i < 4; i++) {
        const style = new Style();
        style.size = { width: 60, height: 30 };
        childStyles.push(style);
        const child = tree.newLeaf(style);
        children.push(child);
        tree.addChild(root, child);
      }

      tree.computeLayout(root, { width: 100, height: 100 });

      // Each 60px child wraps to its own row since only 1 fits in 100px
      const layout0 = tree.getLayout(children[0]);
      const layout1 = tree.getLayout(children[1]);
      const layout2 = tree.getLayout(children[2]);

      // Each child should be on a different row
      expect(layout1.y).toBeGreaterThan(layout0.y);
      expect(layout2.y).toBeGreaterThan(layout1.y);

      tree.free();
      rootStyle.free();
      for (const style of childStyles) {
        style.free();
      }
    });

    it("validates flexWrap: wrap-reverse", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.flexWrap = FlexWrap.WrapReverse;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 60, height: 30 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 60, height: 30 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // In wrap-reverse, second row should appear above first
      expect(child2Layout.y).toBeLessThan(child1Layout.y);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Alignment", () => {
    it("validates alignItems: center", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.alignItems = AlignItems.Center;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 20, height: 20 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.y).toBe(40);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates justifyContent: center", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.justifyContent = JustifyContent.Center;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 20, height: 20 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(40);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates alignItems and justifyContent: center", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.alignItems = AlignItems.Center;
      rootStyle.justifyContent = JustifyContent.Center;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: 20, height: 20 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.x).toBe(40);
      expect(childLayout.y).toBe(40);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates justifyContent: space-between", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.justifyContent = JustifyContent.SpaceBetween;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 20, height: 20 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 20, height: 20 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.x).toBe(0);
      expect(child2Layout.x).toBe(80);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates justifyContent: space-around", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.justifyContent = JustifyContent.SpaceAround;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 20, height: 20 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 20, height: 20 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // Space around: (100 - 40) / 4 = 15 per half-space
      expect(child1Layout.x).toBe(15);
      expect(child2Layout.x).toBe(65);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates justifyContent: space-evenly", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.justifyContent = JustifyContent.SpaceEvenly;
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 20, height: 20 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 20, height: 20 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // Space evenly: (100 - 40) / 3 = 20 per space
      expect(child1Layout.x).toBe(20);
      expect(child2Layout.x).toBe(60);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates alignSelf overrides alignItems", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.alignItems = AlignItems.Start;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 20, height: 20 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 20, height: 20 };
      child2Style.alignSelf = AlignSelf.End;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.y).toBe(0);
      expect(child2Layout.y).toBe(80);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Gap", () => {
    it("validates gap between flex items", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Row;
      rootStyle.gap = { width: 10, height: 10 };
      rootStyle.size = { width: 100, height: 50 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 30, height: 30 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 30, height: 30 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 50 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child2Layout.x - (child1Layout.x + child1Layout.width)).toBe(10);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates gap in column direction", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Column;
      rootStyle.gap = { width: 10, height: 20 };
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.size = { width: 30, height: 20 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.size = { width: 30, height: 20 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child2Layout.y - (child1Layout.y + child1Layout.height)).toBe(20);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Flex Nested", () => {
    it("validates nested flex containers", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Flex;
      rootStyle.flexDirection = FlexDirection.Column;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      // First row container
      const row1Style = new Style();
      row1Style.display = Display.Flex;
      row1Style.flexDirection = FlexDirection.Row;
      row1Style.size = { width: 100, height: 50 };
      const row1 = tree.newLeaf(row1Style);

      // Children for first row
      const child1Style = new Style();
      child1Style.flexGrow = 1;
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.flexGrow = 1;
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(row1, child1);
      tree.addChild(row1, child2);
      tree.addChild(root, row1);

      tree.computeLayout(root, { width: 100, height: 100 });

      const row1Layout = tree.getLayout(row1);
      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(row1Layout.width).toBe(100);
      expect(child1Layout.width).toBe(50);
      expect(child2Layout.width).toBe(50);

      tree.free();
      rootStyle.free();
      row1Style.free();
      child1Style.free();
      child2Style.free();
    });
  });
});

describe("Writing direction and safe alignment", () => {
  beforeAll(setupTaffy);

  it("applies right-to-left direction through the public layout API", () => {
    const tree = new TaffyTree();
    const childStyle = new Style({ width: 20, height: 10 });
    const first = tree.newLeaf(childStyle);
    const second = tree.newLeaf(childStyle);
    const root = tree.newWithChildren(
      new Style({
        display: Display.Flex,
        direction: Direction.Rtl,
        width: 100,
        height: 10,
      }),
      [first, second],
    );

    tree.computeLayout(root, { width: 100, height: 10 });

    expect(tree.getLayout(first).x).toBe(80);
    expect(tree.getLayout(second).x).toBe(60);
  });

  it("applies safe and self-relative alignment", () => {
    const safeTree = new TaffyTree();
    const oversized = safeTree.newLeaf(
      new Style({ width: 150, height: 10, flexShrink: 0 }),
    );
    const safeRoot = safeTree.newWithChildren(
      new Style({
        display: Display.Flex,
        width: 100,
        height: 20,
        justifyContent: JustifyContent.SafeCenter,
      }),
      [oversized],
    );

    safeTree.computeLayout(safeRoot, { width: 100, height: 20 });
    expect(safeTree.getLayout(oversized).x).toBe(0);

    const selfTree = new TaffyTree();
    const selfAligned = selfTree.newLeaf(
      new Style({
        direction: Direction.Rtl,
        alignSelf: AlignSelf.SelfStart,
        width: 20,
        height: 10,
      }),
    );
    const selfRoot = selfTree.newWithChildren(
      new Style({
        display: Display.Flex,
        direction: Direction.Ltr,
        flexDirection: FlexDirection.Column,
        width: 100,
        height: 20,
      }),
      [selfAligned],
    );

    selfTree.computeLayout(selfRoot, { width: 100, height: 20 });
    expect(selfTree.getLayout(selfAligned).x).toBe(80);
  });
});
