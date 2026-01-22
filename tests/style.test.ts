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
  TextAlign,
  GridAutoFlow,
} from "../src/index";

describe("Style Class Properties", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Constructor", () => {
    it("creates Style with defaults when no arguments", () => {
      const style = new Style();
      expect(style.display).toBe(Display.Flex);
      expect(style.position).toBe(Position.Relative);
    });

    it("creates Style with initial properties", () => {
      const style = new Style({
        display: Display.Grid,
        flexDirection: FlexDirection.Column,
        width: 200,
        marginLeft: 10,
      });
      expect(style.display).toBe(Display.Grid);
      expect(style.flexDirection).toBe(FlexDirection.Column);
      expect(style.get("width")).toBe(200);
      expect(style.get("marginLeft")).toBe(10);
    });

    it("creates Style with dot notation properties", () => {
      const style = new Style({
        width: 300,
        height: "50%",
        marginLeft: 20,
        marginRight: "auto",
      });
      expect(style.get("width")).toBe(300);
      expect(style.get("height")).toBe("50%");
      expect(style.get("marginLeft")).toBe(20);
      expect(style.get("marginRight")).toBe("auto");
    });
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
      expect(style.overflow.x).toBe(Overflow.Visible);
      expect(style.overflow.y).toBe(Overflow.Visible);

      style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };
      expect(style.overflow.x).toBe(Overflow.Hidden);
      expect(style.overflow.y).toBe(Overflow.Scroll);

      style.overflow = { x: Overflow.Clip, y: Overflow.Visible };
      expect(style.overflow.x).toBe(Overflow.Clip);
      expect(style.overflow.y).toBe(Overflow.Visible);
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

    it("justifyItems: sets and gets correctly (Grid alignment)", () => {
      const style = new Style();
      expect(style.justifyItems).toBeUndefined();

      style.justifyItems = AlignItems.Center;
      expect(style.justifyItems).toBe(AlignItems.Center);

      style.justifyItems = AlignItems.Start;
      expect(style.justifyItems).toBe(AlignItems.Start);

      style.justifyItems = undefined;
      expect(style.justifyItems).toBeUndefined();
    });

    it("justifySelf: sets and gets correctly (Grid alignment)", () => {
      const style = new Style();
      expect(style.justifySelf).toBe(AlignSelf.Auto);

      style.justifySelf = AlignSelf.End;
      expect(style.justifySelf).toBe(AlignSelf.End);

      style.justifySelf = AlignSelf.Auto;
      expect(style.justifySelf).toBe(AlignSelf.Auto);

      style.justifySelf = undefined;
      expect(style.justifySelf).toBe(AlignSelf.Auto);
    });
  });

  describe("Sizing and Spacing", () => {
    it("size: defaults to auto, sets and gets correctly", () => {
      const style = new Style();
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
      expect(style.aspectRatio).toBeCloseTo(16 / 9, 5);

      style.aspectRatio = undefined;
      expect(style.aspectRatio).toBeUndefined();
    });

    it("gap: sets and gets correctly (LengthPercentage)", () => {
      const style = new Style();
      expect(style.gap.width).toBe(0);
      expect(style.gap.height).toBe(0);

      style.gap = { width: 10, height: "5%" };
      expect(style.gap.width).toBe(10);
      expect(style.gap.height).toBe("5%");

      style.gap = { width: 0, height: 0 };
      expect(style.gap.width).toBe(0);
      expect(style.gap.height).toBe(0);
    });

    it("margin: sets and gets correctly (LengthPercentageAuto)", () => {
      const style = new Style();
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

  describe("Block Layout Properties", () => {
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

    it("scrollbarWidth: defaults to 0, sets and gets correctly", () => {
      const style = new Style();
      expect(style.scrollbarWidth).toBe(0);

      style.scrollbarWidth = 15;
      expect(style.scrollbarWidth).toBe(15);

      style.scrollbarWidth = 0;
      expect(style.scrollbarWidth).toBe(0);
    });

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
  });

  describe("Grid Layout Properties", () => {
    it("gridAutoFlow: defaults to Row, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridAutoFlow).toBe(GridAutoFlow.Row);

      style.gridAutoFlow = GridAutoFlow.Column;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.Column);

      style.gridAutoFlow = GridAutoFlow.RowDense;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.RowDense);

      style.gridAutoFlow = GridAutoFlow.ColumnDense;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.ColumnDense);

      style.gridAutoFlow = GridAutoFlow.Row;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.Row);
    });

    it("gridRow: defaults to auto/auto, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe("auto");

      // Set to line numbers
      style.gridRow = { start: 1, end: 3 };
      expect(style.gridRow.start).toBe(1);
      expect(style.gridRow.end).toBe(3);

      // Set with span - serde_wasm_bindgen returns Map for objects
      style.gridRow = { start: 2, end: { span: 2 } };
      expect(style.gridRow.start).toBe(2);
      // Check if it's a Map (serde_wasm_bindgen default) or plain object
      const endVal = style.gridRow.end;
      if (endVal instanceof Map) {
        expect(endVal.get("span")).toBe(2);
      } else {
        expect(endVal).toEqual({ span: 2 });
      }

      // Set back to auto
      style.gridRow = { start: "auto", end: "auto" };
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe("auto");
    });

    it("gridColumn: defaults to auto/auto, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridColumn.start).toBe("auto");
      expect(style.gridColumn.end).toBe("auto");

      // Set to line numbers
      style.gridColumn = { start: 1, end: 4 };
      expect(style.gridColumn.start).toBe(1);
      expect(style.gridColumn.end).toBe(4);

      // Set with span - check for Map or plain object
      style.gridColumn = { start: "auto", end: { span: 3 } };
      expect(style.gridColumn.start).toBe("auto");
      const endVal = style.gridColumn.end;
      if (endVal instanceof Map) {
        expect(endVal.get("span")).toBe(3);
      } else {
        expect(endVal).toEqual({ span: 3 });
      }
    });

    it("gridTemplateRows: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(Array.isArray(style.gridTemplateRows)).toBe(true);
      expect(style.gridTemplateRows.length).toBe(0);

      // Set simple tracks
      // Set simple tracks with new DTO format
      // { min: 10, max: "auto" }
      const track1 = { min: 10, max: "auto" };
      style.gridTemplateRows = [track1] as any;

      expect(style.gridTemplateRows.length).toBe(1);
      const readback = style.gridTemplateRows[0] as any;
      expect(readback.min).toBe(10);
    });

    it("gridTemplateColumns: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(Array.isArray(style.gridTemplateColumns)).toBe(true);
      expect(style.gridTemplateColumns.length).toBe(0);

      style.gridTemplateColumns = [
        { min: "min-content", max: "max-content" },
      ] as any;
      expect(style.gridTemplateColumns.length).toBe(1);
    });

    it("gridAutoRows: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(Array.isArray(style.gridAutoRows)).toBe(true);
      expect(style.gridAutoRows.length).toBe(0);

      // gridAutoRows expects NonRepeatedTrackSizingFunctionDto[] (inner part of Single)
      style.gridAutoRows = [{ min: 20, max: "auto" }] as any;
      expect(style.gridAutoRows.length).toBe(1);
      const readback = style.gridAutoRows[0] as any;
      expect(readback.min).toBe(20);
    });

    it("gridAutoColumns: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(Array.isArray(style.gridAutoColumns)).toBe(true);
      expect(style.gridAutoColumns.length).toBe(0);

      style.gridAutoColumns = [{ min: "auto", max: "1fr" }];
      expect(style.gridAutoColumns.length).toBe(1);
    });

    it("gridTemplateAreas: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridTemplateAreas).toEqual([]);

      style.gridTemplateAreas = [
        {
          name: "header",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 4,
        },
        {
          name: "main",
          rowStart: 2,
          rowEnd: 4,
          columnStart: 2,
          columnEnd: 4,
        },
      ];
      expect(style.gridTemplateAreas.length).toBe(2);
      expect(style.gridTemplateAreas[0].name).toBe("header");

      style.gridTemplateAreas = [];
      expect(style.gridTemplateAreas).toEqual([]);
    });

    it("gridTemplateRowNames: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridTemplateRowNames).toEqual([]);

      style.gridTemplateRowNames = [
        ["header-start"],
        ["header-end", "main-start"],
        ["main-end"],
      ];
      expect(style.gridTemplateRowNames.length).toBe(3);
      expect(style.gridTemplateRowNames[0]).toEqual(["header-start"]);
      expect(style.gridTemplateRowNames[1]).toEqual([
        "header-end",
        "main-start",
      ]);

      style.gridTemplateRowNames = [];
      expect(style.gridTemplateRowNames).toEqual([]);
    });

    it("gridTemplateColumnNames: defaults to empty, sets and gets correctly", () => {
      const style = new Style();
      expect(style.gridTemplateColumnNames).toEqual([]);

      style.gridTemplateColumnNames = [
        ["sidebar-start"],
        ["sidebar-end", "main-start"],
        ["main-end"],
      ];
      expect(style.gridTemplateColumnNames.length).toBe(3);
      expect(style.gridTemplateColumnNames[0]).toEqual(["sidebar-start"]);

      style.gridTemplateColumnNames = [];
      expect(style.gridTemplateColumnNames).toEqual([]);
    });

    // Additional comprehensive tests

    it("gridRow: handles negative line indices", () => {
      const style = new Style();

      // Negative line indices count from the end
      style.gridRow = { start: -1, end: -2 };
      expect(style.gridRow.start).toBe(-1);
      expect(style.gridRow.end).toBe(-2);

      // Mixed positive and negative
      style.gridRow = { start: 1, end: -1 };
      expect(style.gridRow.start).toBe(1);
      expect(style.gridRow.end).toBe(-1);
    });

    it("gridColumn: handles negative line indices", () => {
      const style = new Style();

      style.gridColumn = { start: -3, end: -1 };
      expect(style.gridColumn.start).toBe(-3);
      expect(style.gridColumn.end).toBe(-1);
    });

    it("gridRow: handles span with various values", () => {
      const style = new Style();

      // Span 1 (default span)
      style.gridRow = { start: 1, end: { span: 1 } };
      expect(style.gridRow.start).toBe(1);
      const end1 = style.gridRow.end;
      if (end1 instanceof Map) {
        expect(end1.get("span")).toBe(1);
      }

      // Large span
      style.gridRow = { start: 1, end: { span: 10 } };
      expect(style.gridRow.start).toBe(1);
      const end10 = style.gridRow.end;
      if (end10 instanceof Map) {
        expect(end10.get("span")).toBe(10);
      }

      // Span on start instead of end
      style.gridRow = { start: { span: 3 }, end: 5 };
      const start3 = style.gridRow.start;
      if (start3 instanceof Map) {
        expect(start3.get("span")).toBe(3);
      }
      expect(style.gridRow.end).toBe(5);
    });

    it("gridColumn: handles span with various values", () => {
      const style = new Style();

      // Span on both start and end
      style.gridColumn = { start: { span: 2 }, end: { span: 3 } };
      const start = style.gridColumn.start;
      const end = style.gridColumn.end;
      if (start instanceof Map && end instanceof Map) {
        expect(start.get("span")).toBe(2);
        expect(end.get("span")).toBe(3);
      }
    });

    it("gridRow: all auto placement", () => {
      const style = new Style();

      // Both start and end as auto
      style.gridRow = { start: "auto", end: "auto" };
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe("auto");

      // Auto start with line end
      style.gridRow = { start: "auto", end: 3 };
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe(3);
    });

    it("gridTemplateAreas: handles complex grid layout", () => {
      const style = new Style();

      // Holy grail layout
      style.gridTemplateAreas = [
        {
          name: "header",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 4,
        },
        {
          name: "nav",
          rowStart: 2,
          rowEnd: 3,
          columnStart: 1,
          columnEnd: 2,
        },
        {
          name: "main",
          rowStart: 2,
          rowEnd: 3,
          columnStart: 2,
          columnEnd: 3,
        },
        {
          name: "aside",
          rowStart: 2,
          rowEnd: 3,
          columnStart: 3,
          columnEnd: 4,
        },
        {
          name: "footer",
          rowStart: 3,
          rowEnd: 4,
          columnStart: 1,
          columnEnd: 4,
        },
      ];

      expect(style.gridTemplateAreas.length).toBe(5);
      expect(style.gridTemplateAreas[0].name).toBe("header");
      expect(style.gridTemplateAreas[1].name).toBe("nav");
      expect(style.gridTemplateAreas[2].name).toBe("main");
      expect(style.gridTemplateAreas[3].name).toBe("aside");
      expect(style.gridTemplateAreas[4].name).toBe("footer");

      // Verify area bounds
      const header = style.gridTemplateAreas[0];
      expect(header.rowStart).toBe(1);
      expect(header.rowEnd).toBe(2);
      expect(header.columnStart).toBe(1);
      expect(header.columnEnd).toBe(4);
    });

    it("gridTemplateRowNames: handles empty line names", () => {
      const style = new Style();

      // Some lines have no names
      style.gridTemplateRowNames = [[], ["named-line"], [], ["another-name"]];
      expect(style.gridTemplateRowNames.length).toBe(4);
      expect(style.gridTemplateRowNames[0]).toEqual([]);
      expect(style.gridTemplateRowNames[1]).toEqual(["named-line"]);
      expect(style.gridTemplateRowNames[2]).toEqual([]);
      expect(style.gridTemplateRowNames[3]).toEqual(["another-name"]);
    });

    it("gridTemplateColumnNames: handles multiple names per line", () => {
      const style = new Style();

      // Multiple names on same line
      style.gridTemplateColumnNames = [
        ["content-start", "sidebar-end", "main-start"],
        ["content-end", "sidebar-start"],
      ];
      expect(style.gridTemplateColumnNames.length).toBe(2);
      expect(style.gridTemplateColumnNames[0]).toEqual([
        "content-start",
        "sidebar-end",
        "main-start",
      ]);
      expect(style.gridTemplateColumnNames[1]).toEqual([
        "content-end",
        "sidebar-start",
      ]);
    });

    it("gridAutoFlow: all variants work correctly", () => {
      const style = new Style();

      // Test each variant
      const variants = [
        GridAutoFlow.Row,
        GridAutoFlow.Column,
        GridAutoFlow.RowDense,
        GridAutoFlow.ColumnDense,
      ];

      for (const variant of variants) {
        style.gridAutoFlow = variant;
        expect(style.gridAutoFlow).toBe(variant);
      }
    });
  });

  describe("Block Layout Edge Cases", () => {
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

  describe("Additional Alignment Edge Cases", () => {
    it("justifyItems: all AlignItems variants work", () => {
      const style = new Style();

      const variants = [
        AlignItems.Start,
        AlignItems.End,
        AlignItems.FlexStart,
        AlignItems.FlexEnd,
        AlignItems.Center,
        AlignItems.Baseline,
        AlignItems.Stretch,
      ];

      for (const variant of variants) {
        style.justifyItems = variant;
        expect(style.justifyItems).toBe(variant);
      }

      // Can be unset
      style.justifyItems = undefined;
      expect(style.justifyItems).toBeUndefined();
    });

    it("justifySelf: all AlignSelf variants work", () => {
      const style = new Style();

      const variants = [
        AlignSelf.Auto,
        AlignSelf.Start,
        AlignSelf.End,
        AlignSelf.FlexStart,
        AlignSelf.FlexEnd,
        AlignSelf.Center,
        AlignSelf.Baseline,
        AlignSelf.Stretch,
      ];

      for (const variant of variants) {
        style.justifySelf = variant;
        // Auto is treated as None internally
        if (variant === AlignSelf.Auto) {
          expect(style.justifySelf).toBe(AlignSelf.Auto);
        } else {
          expect(style.justifySelf).toBe(variant);
        }
      }
    });
  });

  describe("Grid with Display Mode", () => {
    it("grid properties work when display is Grid", () => {
      const style = new Style();
      style.display = Display.Grid;

      // Set up grid template
      style.gridAutoFlow = GridAutoFlow.Column;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.Column);

      // Set grid placement
      style.gridRow = { start: 1, end: 3 };
      style.gridColumn = { start: 2, end: 4 };

      expect(style.gridRow.start).toBe(1);
      expect(style.gridRow.end).toBe(3);
      expect(style.gridColumn.start).toBe(2);
      expect(style.gridColumn.end).toBe(4);

      // Set grid areas
      style.gridTemplateAreas = [
        {
          name: "content",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 2,
        },
      ];
      expect(style.gridTemplateAreas.length).toBe(1);

      // Set line names
      style.gridTemplateRowNames = [["top"], ["bottom"]];
      style.gridTemplateColumnNames = [["left"], ["right"]];
      expect(style.gridTemplateRowNames.length).toBe(2);
      expect(style.gridTemplateColumnNames.length).toBe(2);
    });

    it("grid properties can be set even when display is not Grid", () => {
      const style = new Style();
      // Default display is Flex, but we can still set grid properties
      expect(style.display).toBe(Display.Flex);

      style.gridAutoFlow = GridAutoFlow.RowDense;
      expect(style.gridAutoFlow).toBe(GridAutoFlow.RowDense);

      style.gridRow = { start: 1, end: 2 };
      expect(style.gridRow.start).toBe(1);
    });
  });

  describe("Batch Property Reading (Style.get)", () => {
    it("reads single top-level property", () => {
      const style = new Style();
      style.display = Display.Flex;
      expect(style.get("display")).toBe(Display.Flex);
    });

    it("reads single numeric property", () => {
      const style = new Style();
      style.flexGrow = 2.5;
      expect(style.get("flexGrow")).toBe(2.5);
    });

    it("reads multiple top-level properties", () => {
      const style = new Style();
      style.display = Display.Grid;
      style.flexGrow = 3;
      style.flexShrink = 0.5;

      const [display, flexGrow, flexShrink] = style.get(
        "display",
        "flexGrow",
        "flexShrink",
      );
      expect(display).toBe(Display.Grid);
      expect(flexGrow).toBe(3);
      expect(flexShrink).toBeCloseTo(0.5);
    });

    it("reads nested size properties with dot notation", () => {
      const style = new Style();
      style.size = { width: 100, height: "50%" };

      expect(style.get("width")).toBe(100);
      expect(style.get("height")).toBe("50%");
    });

    it("reads nested margin properties with dot notation", () => {
      const style = new Style();
      style.margin = { left: 10, right: "auto", top: "5%", bottom: 0 };

      expect(style.get("marginLeft")).toBe(10);
      expect(style.get("marginRight")).toBe("auto");
      expect(style.get("marginTop")).toBe("5%");
      expect(style.get("marginBottom")).toBe(0);
    });

    it("reads mixed top-level and nested properties", () => {
      const style = new Style();
      style.display = Display.Flex;
      style.flexDirection = FlexDirection.Column;
      style.size = { width: 200, height: 100 };
      style.padding = { left: 10, right: 10, top: 5, bottom: 5 };

      const [display, width, paddingLeft] = style.get(
        "display",
        "width",
        "paddingLeft",
      );
      expect(display).toBe(Display.Flex);
      expect(width).toBe(200);
      expect(paddingLeft).toBe(10);
    });

    it("reads full nested object (size)", () => {
      const style = new Style();
      style.size = { width: 150, height: "75%" };

      const size = style.get("size")!;
      expect(size.width).toBe(150);
      expect(size.height).toBe("75%");
    });

    it("reads overflow nested properties", () => {
      const style = new Style();
      style.overflow = { x: Overflow.Hidden, y: Overflow.Scroll };

      expect(style.get("overflowX")).toBe(Overflow.Hidden);
      expect(style.get("overflowY")).toBe(Overflow.Scroll);
    });

    it("reads gap nested properties", () => {
      const style = new Style();
      style.gap = { width: 10, height: "5%" };

      expect(style.get("columnGap")).toBe(10);
      expect(style.get("rowGap")).toBe("5%");
    });

    it("reads boolean properties", () => {
      const style = new Style();
      style.itemIsTable = true;
      style.itemIsReplaced = true;

      expect(style.get("itemIsTable")).toBe(true);
      expect(style.get("itemIsReplaced")).toBe(true);
    });

    it("returns undefined for optional properties that are not set", () => {
      const style = new Style();
      expect(style.get("alignItems")).toBeUndefined();
      expect(style.get("aspectRatio")).toBeUndefined();
    });

    it("handles grid row/column properties", () => {
      const style = new Style();
      style.gridRow = { start: 1, end: 3 };
      style.gridColumn = { start: 2, end: 4 };

      expect(style.get("gridRowStart")).toBe(1);
      expect(style.get("gridRowEnd")).toBe(3);
      expect(style.get("gridColumnStart")).toBe(2);
      expect(style.get("gridColumnEnd")).toBe(4);
    });

    it("returns undefined for empty keys array", () => {
      const style = new Style();
      expect(style.get()).toBeUndefined();
    });

    it("returns undefined for unknown property path", () => {
      const style = new Style();
      expect(style.get("unknownProperty")).toBeUndefined();
      expect(style.get("size.unknown")).toBeUndefined();
    });
  });

  describe("Batch Property Writing (Style.set)", () => {
    it("sets single top-level property", () => {
      const style = new Style();
      style.set({ display: Display.Grid });
      expect(style.display).toBe(Display.Grid);
    });

    it("sets multiple top-level properties", () => {
      const style = new Style();
      style.set({
        display: Display.Flex,
        flexDirection: FlexDirection.Column,
        flexGrow: 2,
        flexShrink: 0.5,
      });

      expect(style.display).toBe(Display.Flex);
      expect(style.flexDirection).toBe(FlexDirection.Column);
      expect(style.flexGrow).toBe(2);
      expect(style.flexShrink).toBeCloseTo(0.5);
    });

    it("sets nested size properties with dot notation", () => {
      const style = new Style();
      style.set({
        width: 100,
        height: "50%",
      });

      expect(style.size.width).toBe(100);
      expect(style.size.height).toBe("50%");
    });

    it("sets nested margin properties with dot notation", () => {
      const style = new Style();
      style.set({
        marginLeft: 10,
        marginRight: "auto",
        marginTop: "5%",
        marginBottom: 0,
      });

      expect(style.margin.left).toBe(10);
      expect(style.margin.right).toBe("auto");
      expect(style.margin.top).toBe("5%");
      expect(style.margin.bottom).toBe(0);
    });

    it("sets mixed top-level and nested properties", () => {
      const style = new Style();
      style.set({
        display: Display.Flex,
        flexDirection: FlexDirection.Row,
        width: 200,
        paddingLeft: 10,
        paddingRight: 10,
      });

      expect(style.display).toBe(Display.Flex);
      expect(style.flexDirection).toBe(FlexDirection.Row);
      expect(style.size.width).toBe(200);
      expect(style.padding.left).toBe(10);
      expect(style.padding.right).toBe(10);
    });

    it("sets full nested object (size)", () => {
      const style = new Style();
      style.set({
        size: { width: 150, height: "75%" },
      });

      expect(style.size.width).toBe(150);
      expect(style.size.height).toBe("75%");
    });

    it("sets overflow nested properties", () => {
      const style = new Style();
      style.set({
        overflowX: Overflow.Hidden,
        overflowY: Overflow.Scroll,
      });

      expect(style.overflow.x).toBe(Overflow.Hidden);
      expect(style.overflow.y).toBe(Overflow.Scroll);
    });

    it("sets boolean properties", () => {
      const style = new Style();
      style.set({
        itemIsTable: true,
        itemIsReplaced: true,
      });

      expect(style.itemIsTable).toBe(true);
      expect(style.itemIsReplaced).toBe(true);
    });

    it("set then get roundtrip", () => {
      const style = new Style();
      style.set({
        display: Display.Grid,
        width: 300,
        marginLeft: 20,
        flexGrow: 1.5,
      });

      const [display, width, marginLeft, flexGrow] = style.get(
        "display",
        "width",
        "marginLeft",
        "flexGrow",
      );
      expect(display).toBe(Display.Grid);
      expect(width).toBe(300);
      expect(marginLeft).toBe(20);
      expect(flexGrow).toBe(1.5);
    });

    it("handles grid row/column properties", () => {
      const style = new Style();
      style.set({
        gridRowStart: 1,
        gridRowEnd: 3,
        gridColumnStart: 2,
        gridColumnEnd: 4,
      });

      expect(style.gridRow.start).toBe(1);
      expect(style.gridRow.end).toBe(3);
      expect(style.gridColumn.start).toBe(2);
      expect(style.gridColumn.end).toBe(4);
    });
  });

  describe("Individual Property Getters and Setters", () => {
    describe("Overflow Individual Properties", () => {
      it("overflowX: sets and gets correctly", () => {
        const style = new Style();
        expect(style.overflowX).toBe(Overflow.Visible);

        style.overflowX = Overflow.Hidden;
        expect(style.overflowX).toBe(Overflow.Hidden);

        style.overflowX = Overflow.Scroll;
        expect(style.overflowX).toBe(Overflow.Scroll);

        // Setting overflowX should not affect overflowY
        style.overflowY = Overflow.Clip;
        expect(style.overflowX).toBe(Overflow.Scroll);
        expect(style.overflowY).toBe(Overflow.Clip);
      });

      it("overflowY: sets and gets correctly", () => {
        const style = new Style();
        expect(style.overflowY).toBe(Overflow.Visible);

        style.overflowY = Overflow.Hidden;
        expect(style.overflowY).toBe(Overflow.Hidden);

        style.overflowY = Overflow.Scroll;
        expect(style.overflowY).toBe(Overflow.Scroll);
      });

      it("overflowX and overflowY: independent of each other", () => {
        const style = new Style();
        style.overflowX = Overflow.Hidden;
        style.overflowY = Overflow.Scroll;

        expect(style.overflowX).toBe(Overflow.Hidden);
        expect(style.overflowY).toBe(Overflow.Scroll);
        expect(style.overflow.x).toBe(Overflow.Hidden);
        expect(style.overflow.y).toBe(Overflow.Scroll);
      });
    });

    describe("Size Individual Properties", () => {
      it("width: sets and gets correctly", () => {
        const style = new Style();
        expect(style.width).toBe("auto");

        style.width = 200;
        expect(style.width).toBe(200);

        style.width = "50%";
        expect(style.width).toBe("50%");

        style.width = "auto";
        expect(style.width).toBe("auto");
      });

      it("height: sets and gets correctly", () => {
        const style = new Style();
        expect(style.height).toBe("auto");

        style.height = 100;
        expect(style.height).toBe(100);

        style.height = "75%";
        expect(style.height).toBe("75%");

        style.height = "auto";
        expect(style.height).toBe("auto");
      });

      it("width and height: independent of each other", () => {
        const style = new Style();
        style.width = 300;
        style.height = "50%";

        expect(style.width).toBe(300);
        expect(style.height).toBe("50%");
        expect(style.size.width).toBe(300);
        expect(style.size.height).toBe("50%");
      });

      it("minWidth: sets and gets correctly", () => {
        const style = new Style();
        expect(style.minWidth).toBe("auto");

        style.minWidth = 100;
        expect(style.minWidth).toBe(100);

        style.minWidth = "10%";
        expect(style.minWidth).toBe("10%");

        style.minWidth = "auto";
        expect(style.minWidth).toBe("auto");
      });

      it("minHeight: sets and gets correctly", () => {
        const style = new Style();
        expect(style.minHeight).toBe("auto");

        style.minHeight = 50;
        expect(style.minHeight).toBe(50);

        style.minHeight = "auto";
        expect(style.minHeight).toBe("auto");
      });

      it("maxWidth: sets and gets correctly", () => {
        const style = new Style();
        expect(style.maxWidth).toBe("auto");

        style.maxWidth = 500;
        expect(style.maxWidth).toBe(500);

        style.maxWidth = "100%";
        expect(style.maxWidth).toBe("100%");

        style.maxWidth = "auto";
        expect(style.maxWidth).toBe("auto");
      });

      it("maxHeight: sets and gets correctly", () => {
        const style = new Style();
        expect(style.maxHeight).toBe("auto");

        style.maxHeight = 300;
        expect(style.maxHeight).toBe(300);

        style.maxHeight = "auto";
        expect(style.maxHeight).toBe("auto");
      });

      it("min/max width/height: all independent", () => {
        const style = new Style();
        style.minWidth = 100;
        style.maxWidth = 500;
        style.minHeight = 50;
        style.maxHeight = 300;

        expect(style.minWidth).toBe(100);
        expect(style.maxWidth).toBe(500);
        expect(style.minHeight).toBe(50);
        expect(style.maxHeight).toBe(300);
      });
    });

    describe("Margin Individual Properties", () => {
      it("marginLeft: sets and gets correctly", () => {
        const style = new Style();
        expect(style.marginLeft).toBe(0);

        style.marginLeft = 10;
        expect(style.marginLeft).toBe(10);

        style.marginLeft = "5%";
        expect(style.marginLeft).toBe("5%");

        style.marginLeft = "auto";
        expect(style.marginLeft).toBe("auto");
      });

      it("marginRight: sets and gets correctly", () => {
        const style = new Style();
        expect(style.marginRight).toBe(0);

        style.marginRight = 20;
        expect(style.marginRight).toBe(20);

        style.marginRight = "auto";
        expect(style.marginRight).toBe("auto");
      });

      it("marginTop: sets and gets correctly", () => {
        const style = new Style();
        expect(style.marginTop).toBe(0);

        style.marginTop = 5;
        expect(style.marginTop).toBe(5);

        style.marginTop = "10%";
        expect(style.marginTop).toBe("10%");
      });

      it("marginBottom: sets and gets correctly", () => {
        const style = new Style();
        expect(style.marginBottom).toBe(0);

        style.marginBottom = 15;
        expect(style.marginBottom).toBe(15);

        style.marginBottom = "auto";
        expect(style.marginBottom).toBe("auto");
      });

      it("all margin properties: independent of each other", () => {
        const style = new Style();
        style.marginLeft = 10;
        style.marginRight = "auto";
        style.marginTop = "5%";
        style.marginBottom = 0;

        expect(style.marginLeft).toBe(10);
        expect(style.marginRight).toBe("auto");
        expect(style.marginTop).toBe("5%");
        expect(style.marginBottom).toBe(0);
        expect(style.margin.left).toBe(10);
        expect(style.margin.right).toBe("auto");
        expect(style.margin.top).toBe("5%");
        expect(style.margin.bottom).toBe(0);
      });
    });

    describe("Padding Individual Properties", () => {
      it("paddingLeft: sets and gets correctly", () => {
        const style = new Style();
        expect(style.paddingLeft).toBe(0);

        style.paddingLeft = 20;
        expect(style.paddingLeft).toBe(20);

        style.paddingLeft = "10%";
        expect(style.paddingLeft).toBe("10%");
      });

      it("paddingRight: sets and gets correctly", () => {
        const style = new Style();
        expect(style.paddingRight).toBe(0);

        style.paddingRight = 25;
        expect(style.paddingRight).toBe(25);
      });

      it("paddingTop: sets and gets correctly", () => {
        const style = new Style();
        expect(style.paddingTop).toBe(0);

        style.paddingTop = 10;
        expect(style.paddingTop).toBe(10);

        style.paddingTop = "5%";
        expect(style.paddingTop).toBe("5%");
      });

      it("paddingBottom: sets and gets correctly", () => {
        const style = new Style();
        expect(style.paddingBottom).toBe(0);

        style.paddingBottom = 15;
        expect(style.paddingBottom).toBe(15);
      });

      it("all padding properties: independent of each other", () => {
        const style = new Style();
        style.paddingLeft = 20;
        style.paddingRight = 30;
        style.paddingTop = "5%";
        style.paddingBottom = 10;

        expect(style.paddingLeft).toBe(20);
        expect(style.paddingRight).toBe(30);
        expect(style.paddingTop).toBe("5%");
        expect(style.paddingBottom).toBe(10);
      });
    });

    describe("Border Individual Properties", () => {
      it("borderLeft: sets and gets correctly", () => {
        const style = new Style();
        expect(style.borderLeft).toBe(0);

        style.borderLeft = 1;
        expect(style.borderLeft).toBe(1);

        style.borderLeft = "2%";
        expect(style.borderLeft).toBe("2%");
      });

      it("borderRight: sets and gets correctly", () => {
        const style = new Style();
        expect(style.borderRight).toBe(0);

        style.borderRight = 2;
        expect(style.borderRight).toBe(2);
      });

      it("borderTop: sets and gets correctly", () => {
        const style = new Style();
        expect(style.borderTop).toBe(0);

        style.borderTop = 1;
        expect(style.borderTop).toBe(1);
      });

      it("borderBottom: sets and gets correctly", () => {
        const style = new Style();
        expect(style.borderBottom).toBe(0);

        style.borderBottom = 3;
        expect(style.borderBottom).toBe(3);
      });

      it("all border properties: independent of each other", () => {
        const style = new Style();
        style.borderLeft = 1;
        style.borderRight = 2;
        style.borderTop = 3;
        style.borderBottom = 4;

        expect(style.borderLeft).toBe(1);
        expect(style.borderRight).toBe(2);
        expect(style.borderTop).toBe(3);
        expect(style.borderBottom).toBe(4);
      });
    });

    describe("Inset Individual Properties", () => {
      it("left: sets and gets correctly", () => {
        const style = new Style();
        expect(style.left).toBe("auto");

        style.position = Position.Absolute;
        style.left = 0;
        expect(style.left).toBe(0);

        style.left = "10%";
        expect(style.left).toBe("10%");

        style.left = "auto";
        expect(style.left).toBe("auto");
      });

      it("right: sets and gets correctly", () => {
        const style = new Style();
        expect(style.right).toBe("auto");

        style.position = Position.Absolute;
        style.right = 20;
        expect(style.right).toBe(20);

        style.right = "auto";
        expect(style.right).toBe("auto");
      });

      it("top: sets and gets correctly", () => {
        const style = new Style();
        expect(style.top).toBe("auto");

        style.position = Position.Absolute;
        style.top = 0;
        expect(style.top).toBe(0);

        style.top = "5%";
        expect(style.top).toBe("5%");
      });

      it("bottom: sets and gets correctly", () => {
        const style = new Style();
        expect(style.bottom).toBe("auto");

        style.position = Position.Absolute;
        style.bottom = 10;
        expect(style.bottom).toBe(10);

        style.bottom = "auto";
        expect(style.bottom).toBe("auto");
      });

      it("all inset properties: independent of each other", () => {
        const style = new Style();
        style.position = Position.Absolute;
        style.left = 0;
        style.right = 20;
        style.top = "10%";
        style.bottom = "auto";

        expect(style.left).toBe(0);
        expect(style.right).toBe(20);
        expect(style.top).toBe("10%");
        expect(style.bottom).toBe("auto");
        expect(style.inset.left).toBe(0);
        expect(style.inset.right).toBe(20);
        expect(style.inset.top).toBe("10%");
        expect(style.inset.bottom).toBe("auto");
      });
    });

    describe("Gap Individual Properties", () => {
      it("columnGap: sets and gets correctly", () => {
        const style = new Style();
        expect(style.columnGap).toBe(0);

        style.columnGap = 10;
        expect(style.columnGap).toBe(10);

        style.columnGap = "5%";
        expect(style.columnGap).toBe("5%");
      });

      it("rowGap: sets and gets correctly", () => {
        const style = new Style();
        expect(style.rowGap).toBe(0);

        style.rowGap = 15;
        expect(style.rowGap).toBe(15);

        style.rowGap = "10%";
        expect(style.rowGap).toBe("10%");
      });

      it("columnGap and rowGap: independent of each other", () => {
        const style = new Style();
        style.columnGap = 10;
        style.rowGap = "5%";

        expect(style.columnGap).toBe(10);
        expect(style.rowGap).toBe("5%");
        expect(style.gap.width).toBe(10);
        expect(style.gap.height).toBe("5%");
      });
    });

    describe("Grid Row/Column Individual Properties", () => {
      it("gridRowStart: sets and gets correctly", () => {
        const style = new Style();
        style.display = Display.Grid;
        expect(style.gridRowStart).toBe("auto");

        style.gridRowStart = 1;
        expect(style.gridRowStart).toBe(1);

        style.gridRowStart = "auto";
        expect(style.gridRowStart).toBe("auto");

        style.gridRowStart = { span: 2 };
        const start = style.gridRowStart;
        if (start instanceof Map) {
          expect(start.get("span")).toBe(2);
        } else {
          expect(start).toEqual({ span: 2 });
        }
      });

      it("gridRowEnd: sets and gets correctly", () => {
        const style = new Style();
        style.display = Display.Grid;
        expect(style.gridRowEnd).toBe("auto");

        style.gridRowEnd = 3;
        expect(style.gridRowEnd).toBe(3);

        style.gridRowEnd = { span: 2 };
        const end = style.gridRowEnd;
        if (end instanceof Map) {
          expect(end.get("span")).toBe(2);
        } else {
          expect(end).toEqual({ span: 2 });
        }
      });

      it("gridColumnStart: sets and gets correctly", () => {
        const style = new Style();
        style.display = Display.Grid;
        expect(style.gridColumnStart).toBe("auto");

        style.gridColumnStart = 1;
        expect(style.gridColumnStart).toBe(1);

        style.gridColumnStart = "auto";
        expect(style.gridColumnStart).toBe("auto");
      });

      it("gridColumnEnd: sets and gets correctly", () => {
        const style = new Style();
        style.display = Display.Grid;
        expect(style.gridColumnEnd).toBe("auto");

        style.gridColumnEnd = 4;
        expect(style.gridColumnEnd).toBe(4);

        style.gridColumnEnd = { span: 3 };
        const end = style.gridColumnEnd;
        if (end instanceof Map) {
          expect(end.get("span")).toBe(3);
        } else {
          expect(end).toEqual({ span: 3 });
        }
      });

      it("all grid row/column properties: independent of each other", () => {
        const style = new Style();
        style.display = Display.Grid;
        style.gridRowStart = 1;
        style.gridRowEnd = 3;
        style.gridColumnStart = 2;
        style.gridColumnEnd = 4;

        expect(style.gridRowStart).toBe(1);
        expect(style.gridRowEnd).toBe(3);
        expect(style.gridColumnStart).toBe(2);
        expect(style.gridColumnEnd).toBe(4);
        expect(style.gridRow.start).toBe(1);
        expect(style.gridRow.end).toBe(3);
        expect(style.gridColumn.start).toBe(2);
        expect(style.gridColumn.end).toBe(4);
      });
    });

    describe("Individual Properties with Batch get/set", () => {
      it("get() works with new individual property names", () => {
        const style = new Style();
        style.width = 200;
        style.marginLeft = 10;
        style.paddingTop = 5;
        style.overflowX = Overflow.Hidden;

        expect(style.get("width")).toBe(200);
        expect(style.get("marginLeft")).toBe(10);
        expect(style.get("paddingTop")).toBe(5);
        expect(style.get("overflowX")).toBe(Overflow.Hidden);
      });

      it("set() works with new individual property names", () => {
        const style = new Style();
        style.set({
          width: 300,
          height: "50%",
          marginLeft: 20,
          marginRight: "auto",
          paddingTop: 10,
          paddingBottom: 15,
          borderLeft: 1,
          borderRight: 2,
          overflowX: Overflow.Scroll,
          overflowY: Overflow.Hidden,
        });

        expect(style.width).toBe(300);
        expect(style.height).toBe("50%");
        expect(style.marginLeft).toBe(20);
        expect(style.marginRight).toBe("auto");
        expect(style.paddingTop).toBe(10);
        expect(style.paddingBottom).toBe(15);
        expect(style.borderLeft).toBe(1);
        expect(style.borderRight).toBe(2);
        expect(style.overflowX).toBe(Overflow.Scroll);
        expect(style.overflowY).toBe(Overflow.Hidden);
      });

      it("get() multiple individual properties at once", () => {
        const style = new Style();
        style.width = 200;
        style.height = 100;
        style.marginLeft = 10;
        style.paddingTop = 5;

        const [width, height, marginLeft, paddingTop] = style.get(
          "width",
          "height",
          "marginLeft",
          "paddingTop",
        );

        expect(width).toBe(200);
        expect(height).toBe(100);
        expect(marginLeft).toBe(10);
        expect(paddingTop).toBe(5);
      });
    });
  });
});
