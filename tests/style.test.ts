import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  Style,
  Display,
  Position,
  FlexDirection,
  FlexWrap,
  AlignItems,
  AlignSelf,
  AlignContent,
  JustifyContent,
  Overflow,
  BoxSizing,
} from "../src/index";

describe("Style Class Properties", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Layout Modes", () => {
    it("display: defaults to Flex, sets and gets correctly", () => {
      const style = new Style();
      expect(style.display).toBe(Display.Flex);

      style.display = Display.None;
      expect(style.display).toBe(Display.None);

      style.display = Display.Flex;
      expect(style.display).toBe(Display.Flex);

      if (Display.Grid !== undefined) {
        style.display = Display.Grid;
        expect(style.display).toBe(Display.Grid);
      }

      style.display = Display.Block;
      expect(style.display).toBe(Display.Block);
    });

    it("position: defaults to Relative, sets and gets correctly", () => {
      const style = new Style();
      expect(style.position).toBe(Position.Relative);

      style.position = Position.Absolute;
      expect(style.position).toBe(Position.Absolute);

      style.position = Position.Relative;
      expect(style.position).toBe(Position.Relative);
    });

    it("boxSizing: defaults to BorderBox, sets and gets correctly", () => {
      const style = new Style();
      expect(style.boxSizing).toBe(BoxSizing.BorderBox);

      style.boxSizing = BoxSizing.ContentBox;
      expect(style.boxSizing).toBe(BoxSizing.ContentBox);

      style.boxSizing = BoxSizing.BorderBox;
      expect(style.boxSizing).toBe(BoxSizing.BorderBox);
    });

    it("overflow: defaults to Visible, sets and gets correctly", () => {
      const style = new Style();
      // Assuming default is visible for both x and y
      expect(style.overflow.x).toBe(Overflow.Visible);
      expect(style.overflow.y).toBe(Overflow.Visible);

      style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
      expect(style.overflow.x).toBe(Overflow.Hidden);
      expect(style.overflow.y).toBe(Overflow.Scroll);

      style.overflow = { x: Overflow.Clip, y: Overflow.Visible };
      expect(style.overflow.x).toBe(Overflow.Clip);
      expect(style.overflow.y).toBe(Overflow.Visible);

      // Test with raw numbers if supported (enums are numbers)
      style.overflow = { x: 2, y: 3 }; // Hidden(2), Scroll(3)
      expect(style.overflow.x).toBe(Overflow.Hidden);
      expect(style.overflow.y).toBe(Overflow.Scroll);
    });
  });

  describe("Flexbox Properties", () => {
    it("flexDirection: defaults to Row, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexDirection).toBe(FlexDirection.Row);

      style.flexDirection = FlexDirection.Column;
      expect(style.flexDirection).toBe(FlexDirection.Column);

      style.flexDirection = FlexDirection.RowReverse;
      expect(style.flexDirection).toBe(FlexDirection.RowReverse);

      style.flexDirection = FlexDirection.ColumnReverse;
      expect(style.flexDirection).toBe(FlexDirection.ColumnReverse);
    });

    it("flexWrap: defaults to NoWrap, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexWrap).toBe(FlexWrap.NoWrap);

      style.flexWrap = FlexWrap.Wrap;
      expect(style.flexWrap).toBe(FlexWrap.Wrap);

      style.flexWrap = FlexWrap.WrapReverse;
      expect(style.flexWrap).toBe(FlexWrap.WrapReverse);
    });

    it("flexGrow: defaults to 0, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexGrow).toBe(0);

      style.flexGrow = 1;
      expect(style.flexGrow).toBe(1);

      style.flexGrow = 2.5;
      expect(style.flexGrow).toBe(2.5);
    });

    it("flexShrink: defaults to 1, sets and gets correctly", () => {
      const style = new Style();
      expect(style.flexShrink).toBe(1);

      style.flexShrink = 0;
      expect(style.flexShrink).toBe(0);

      style.flexShrink = 3.5;
      expect(style.flexShrink).toBe(3.5);
    });

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
  });

  describe("Alignment Properties", () => {
    it("alignItems: sets and gets correctly (Option<Enum>)", () => {
      const style = new Style();
      // Default is usually Stretch in CSS, specifically for Taffy Flexbox it's technically 'Stretch' but represented as optional?
      // JS bindings .d.ts says: returns AlignItems | undefined
      // If undefined, it falls back to default.
      expect(style.alignItems).toBeUndefined();

      style.alignItems = AlignItems.Start;
      expect(style.alignItems).toBe(AlignItems.Start);

      style.alignItems = undefined;
      expect(style.alignItems).toBeUndefined();

      style.alignItems = AlignItems.Center;
      expect(style.alignItems).toBe(AlignItems.Center);
    });

    it("alignSelf: sets and gets correctly (Option<Enum> with Auto)", () => {
      const style = new Style();
      // alignSelf special case: undefined -> Auto
      expect(style.alignSelf).toBe(AlignSelf.Auto);

      style.alignSelf = AlignSelf.FlexStart;
      expect(style.alignSelf).toBe(AlignSelf.FlexStart);

      style.alignSelf = AlignSelf.Auto;
      expect(style.alignSelf).toBe(AlignSelf.Auto);

      style.alignSelf = undefined;
      expect(style.alignSelf).toBe(AlignSelf.Auto);
    });

    it("alignContent: sets and gets correctly (Option<Enum>)", () => {
      const style = new Style();
      expect(style.alignContent).toBeUndefined();

      style.alignContent = AlignContent.SpaceBetween;
      expect(style.alignContent).toBe(AlignContent.SpaceBetween);

      style.alignContent = undefined;
      expect(style.alignContent).toBeUndefined();
    });

    it("justifyContent: sets and gets correctly (Option<Enum>)", () => {
      const style = new Style();
      expect(style.justifyContent).toBeUndefined();

      style.justifyContent = JustifyContent.SpaceAround;
      expect(style.justifyContent).toBe(JustifyContent.SpaceAround);

      style.justifyContent = undefined;
      expect(style.justifyContent).toBeUndefined();
    });
  });

  describe("Sizing and Spacing", () => {
    it("size: defaults to auto, sets and gets correctly", () => {
      const style = new Style();
      // Default size is auto
      expect(style.size.width).toBe("auto");
      expect(style.size.height).toBe("auto");

      style.size = { width: 100, height: "50%" };
      expect(style.size.width).toBe(100);
      expect(style.size.height).toBe("50%");

      style.size = { width: "auto", height: 200 };
      expect(style.size.width).toBe("auto");
      expect(style.size.height).toBe(200);
    });

    it("minSize: defaults to auto, sets and gets correctly", () => {
      const style = new Style();
      // Default minSize is auto (which effectively means 0 for min-width/height usually, but strictly "auto" in Taffy types?)
      // Taffy's default for min_size is actually Auto.
      expect(style.minSize.width).toBe("auto");
      expect(style.minSize.height).toBe("auto");

      style.minSize = { width: 50, height: 50 };
      expect(style.minSize.width).toBe(50);
      expect(style.minSize.height).toBe(50);

      style.minSize = { width: "10%", height: "auto" };
      expect(style.minSize.width).toBe("10%");
      expect(style.minSize.height).toBe("auto");
    });

    it("maxSize: defaults to auto, sets and gets correctly", () => {
      const style = new Style();
      // Default maxSize is auto (no limit)
      expect(style.maxSize.width).toBe("auto");
      expect(style.maxSize.height).toBe("auto");

      style.maxSize = { width: 500, height: "100%" };
      expect(style.maxSize.width).toBe(500);
      expect(style.maxSize.height).toBe("100%");
    });

    it("aspectRatio: sets and gets correctly", () => {
      const style = new Style();
      expect(style.aspectRatio).toBeUndefined();

      style.aspectRatio = 16 / 9;
      // Rust uses f32, so we expect some precision loss, checking 5 decimal places is safe
      expect(style.aspectRatio).toBeCloseTo(16 / 9, 5);

      style.aspectRatio = undefined;
      expect(style.aspectRatio).toBeUndefined();
    });

    it("gap: sets and gets correctly (LengthPercentage)", () => {
      const style = new Style();
      // Default gap is 0
      expect(style.gap.width).toBe(0);
      expect(style.gap.height).toBe(0);

      style.gap = { width: 10, height: "5%" };
      expect(style.gap.width).toBe(10);
      expect(style.gap.height).toBe("5%");

      // Ensure setters work with new object
      style.gap = { width: 0, height: 0 };
      expect(style.gap.width).toBe(0);
      expect(style.gap.height).toBe(0);
    });

    it("margin: sets and gets correctly (LengthPercentageAuto)", () => {
      const style = new Style();
      // Default margin is 0
      expect(style.margin.left).toBe(0);
      expect(style.margin.right).toBe(0);
      expect(style.margin.top).toBe(0);
      expect(style.margin.bottom).toBe(0);

      style.margin = { left: 10, right: "auto", top: "5%", bottom: 0 };
      const m = style.margin;
      expect(m.left).toBe(10);
      expect(m.right).toBe("auto");
      expect(m.top).toBe("5%");
      expect(m.bottom).toBe(0);
    });

    it("padding: sets and gets correctly (LengthPercentage)", () => {
      const style = new Style();
      // Default padding is 0
      expect(style.padding.left).toBe(0);
      expect(style.padding.right).toBe(0);
      expect(style.padding.top).toBe(0);
      expect(style.padding.bottom).toBe(0);

      style.padding = { left: 10, right: 10, top: "2%", bottom: "2%" };
      const p = style.padding;
      expect(p.left).toBe(10);
      expect(p.top).toBe("2%");
    });

    it("border: sets and gets correctly (LengthPercentage)", () => {
      const style = new Style();
      // Default border is 0
      expect(style.border.left).toBe(0);
      expect(style.border.right).toBe(0);
      expect(style.border.top).toBe(0);
      expect(style.border.bottom).toBe(0);

      style.border = { left: 1, right: 1, top: 1, bottom: 1 };
      const b = style.border;
      expect(b.left).toBe(1);

      style.border = { left: "10%", right: 1, top: 1, bottom: 1 };
      expect(style.border.left).toBe("10%");
    });

    it("inset: sets and gets correctly (LengthPercentageAuto)", () => {
      const style = new Style();
      // Default inset is auto
      expect(style.inset.left).toBe("auto");
      expect(style.inset.right).toBe("auto");
      expect(style.inset.top).toBe("auto");
      expect(style.inset.bottom).toBe("auto");

      style.inset = { left: "auto", right: 20, top: "10%", bottom: 0 };
      const i = style.inset;
      expect(i.left).toBe("auto");
      expect(i.right).toBe(20);
      expect(i.top).toBe("10%");
      expect(i.bottom).toBe(0);
    });
  });
});
