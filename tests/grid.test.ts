import { describe, it, expect, beforeAll } from "vitest";
import { setupTaffy } from "./utils";
import {
  TaffyTree,
  Style,
  Display,
  GridAutoFlow,
  AlignItems,
  JustifyContent,
  type TrackSizingFunction,
} from "../src/index";

describe("Grid Style Properties", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Grid Template Tracks", () => {
    it("gridTemplateRows: sets and gets fixed length tracks", () => {
      const style = new Style();
      style.display = Display.Grid;

      style.gridTemplateRows = [
        { min: 100, max: 100 },
        { min: 50, max: 50 },
      ] as any;

      expect(style.gridTemplateRows.length).toBe(2);
      const readback = style.gridTemplateRows as any;
      expect(readback[0].min).toBe(100);
      expect(readback[1].min).toBe(50);
    });

    it("gridTemplateRows: sets and gets auto and fr tracks", () => {
      const style = new Style();
      style.display = Display.Grid;

      style.gridTemplateRows = [
        { min: 100, max: 100 },
        { min: "auto", max: "1fr" },
      ] as any;

      expect(style.gridTemplateRows.length).toBe(2);
    });

    it("gridTemplateColumns: sets and gets content sizing tracks", () => {
      const style = new Style();
      style.display = Display.Grid;

      style.gridTemplateColumns = [
        { min: "50%", max: "max-content" },
        { min: "min-content", max: "fit-content" },
      ] as any;

      expect(style.gridTemplateColumns.length).toBe(2);
    });

    it("gridTemplateColumns: sets and gets fixed length tracks", () => {
      const style = new Style();

      style.gridTemplateColumns = [
        { min: 60, max: 60 },
        { min: "auto", max: "1fr" },
      ] as any;

      expect(style.gridTemplateColumns.length).toBe(2);
    });
  });

  describe("Grid Auto Tracks", () => {
    it("gridAutoRows: sets and gets content sizing tracks", () => {
      const style = new Style();
      style.display = Display.Grid;

      style.gridAutoRows = [{ min: "min-content", max: "max-content" }] as any;

      expect(style.gridAutoRows.length).toBe(1);
      const readback = style.gridAutoRows[0] as any;
      expect(readback.min).toBe("min-content");
      expect(readback.max).toBe("max-content");
    });

    it("gridAutoColumns: sets and gets fr tracks", () => {
      const style = new Style();
      style.display = Display.Grid;

      style.gridAutoColumns = [{ min: "auto", max: "1fr" }];

      expect(style.gridAutoColumns.length).toBe(1);
    });

    it("gridAutoRows: sets multiple tracks", () => {
      const style = new Style();

      style.gridAutoRows = [
        { min: 50, max: 50 },
        { min: "auto", max: "1fr" },
      ] as any;

      expect(style.gridAutoRows.length).toBe(2);
    });
  });

  describe("Grid Repetition", () => {
    it("gridTemplateRows: sets repetition tracks", () => {
      const style = new Style();

      style.gridTemplateRows = [
        { count: 3, tracks: [{ min: 10, max: 10 }] },
      ] as any;

      expect(style.gridTemplateRows.length).toBe(1);
    });

    it("gridTemplateColumns: sets repetition with lineNames", () => {
      const style = new Style();

      style.gridTemplateColumns = [
        {
          count: 2,
          tracks: [{ min: 10, max: 10 }],
          lineNames: [["start"], ["middle"], ["end"]],
        },
      ] as any;

      expect(style.gridTemplateColumns.length).toBe(1);
    });

    it("gridTemplateRows: sets auto-fill repetition", () => {
      const style = new Style();

      style.gridTemplateRows = [
        { count: "auto-fill", tracks: [{ min: 100, max: 100 }] },
      ] as any;

      expect(style.gridTemplateRows.length).toBe(1);
    });

    it("gridTemplateColumns: sets auto-fit repetition", () => {
      const style = new Style();

      style.gridTemplateColumns = [
        { count: "auto-fit", tracks: [{ min: 50, max: "1fr" }] },
      ] as any;

      expect(style.gridTemplateColumns.length).toBe(1);
    });
  });

  describe("Grid Placement", () => {
    it("gridRow: handles line numbers", () => {
      const style = new Style();

      style.gridRow = { start: 1, end: 3 };
      expect(style.gridRow.start).toBe(1);
      expect(style.gridRow.end).toBe(3);
    });

    it("gridColumn: handles line numbers", () => {
      const style = new Style();

      style.gridColumn = { start: 1, end: 4 };
      expect(style.gridColumn.start).toBe(1);
      expect(style.gridColumn.end).toBe(4);
    });

    it("gridRow: handles negative line indices", () => {
      const style = new Style();

      style.gridRow = { start: -1, end: -2 };
      expect(style.gridRow.start).toBe(-1);
      expect(style.gridRow.end).toBe(-2);

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

    it("gridRow: handles span", () => {
      const style = new Style();

      style.gridRow = { start: 2, end: { span: 2 } };
      expect(style.gridRow.start).toBe(2);
      const endVal = style.gridRow.end;
      if (endVal instanceof Map) {
        expect(endVal.get("span")).toBe(2);
      } else {
        expect(endVal).toEqual({ span: 2 });
      }
    });

    it("gridColumn: handles span", () => {
      const style = new Style();

      style.gridColumn = { start: "auto", end: { span: 3 } };
      expect(style.gridColumn.start).toBe("auto");
      const endVal = style.gridColumn.end;
      if (endVal instanceof Map) {
        expect(endVal.get("span")).toBe(3);
      } else {
        expect(endVal).toEqual({ span: 3 });
      }
    });

    it("gridRow: handles named line placement", () => {
      const style = new Style();

      style.gridRow = {
        start: { line: 1, ident: "header" },
        end: "auto",
      } as any;
      const row = style.gridRow as any;
      expect(row.start.ident).toBe("header");
      expect(row.start.line).toBe(1);
      expect(row.end).toBe("auto");
    });

    it("gridColumn: handles named span", () => {
      const style = new Style();

      style.gridColumn = {
        start: { span: 2, ident: "sidebar" },
        end: "auto",
      } as any;
      const col = style.gridColumn as any;
      expect(col.start.span).toBe(2);
      expect(col.start.ident).toBe("sidebar");
    });

    it("gridRow: handles span with various values", () => {
      const style = new Style();

      // Span 1
      style.gridRow = { start: 1, end: { span: 1 } };
      const end1 = style.gridRow.end;
      if (end1 instanceof Map) {
        expect(end1.get("span")).toBe(1);
      }

      // Large span
      style.gridRow = { start: 1, end: { span: 10 } };
      const end10 = style.gridRow.end;
      if (end10 instanceof Map) {
        expect(end10.get("span")).toBe(10);
      }

      // Span on start
      style.gridRow = { start: { span: 3 }, end: 5 };
      const start3 = style.gridRow.start;
      if (start3 instanceof Map) {
        expect(start3.get("span")).toBe(3);
      }
      expect(style.gridRow.end).toBe(5);
    });

    it("gridColumn: handles span on both sides", () => {
      const style = new Style();

      style.gridColumn = { start: { span: 2 }, end: { span: 3 } };
      const start = style.gridColumn.start;
      const end = style.gridColumn.end;
      if (start instanceof Map && end instanceof Map) {
        expect(start.get("span")).toBe(2);
        expect(end.get("span")).toBe(3);
      }
    });

    it("gridRow: handles auto placement", () => {
      const style = new Style();

      style.gridRow = { start: "auto", end: "auto" };
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe("auto");

      style.gridRow = { start: "auto", end: 3 };
      expect(style.gridRow.start).toBe("auto");
      expect(style.gridRow.end).toBe(3);
    });
  });

  describe("Grid Line Names", () => {
    it("gridTemplateRowNames: sets and gets line names", () => {
      const style = new Style();

      style.gridTemplateRowNames = [
        ["header-start"],
        ["header-end", "main-start"],
      ];

      const names = style.gridTemplateRowNames;
      expect(names.length).toBe(2);
      expect(names[0][0]).toBe("header-start");
      expect(names[1][1]).toBe("main-start");
    });

    it("gridTemplateColumnNames: sets and gets line names", () => {
      const style = new Style();

      style.gridTemplateColumnNames = [["col-start"], ["col-end"]];

      expect(style.gridTemplateColumnNames.length).toBe(2);
    });

    it("gridTemplateRowNames: handles empty line names", () => {
      const style = new Style();

      style.gridTemplateRowNames = [[], ["named-line"], [], ["another-name"]];
      expect(style.gridTemplateRowNames.length).toBe(4);
      expect(style.gridTemplateRowNames[0]).toEqual([]);
      expect(style.gridTemplateRowNames[1]).toEqual(["named-line"]);
    });

    it("gridTemplateColumnNames: handles multiple names per line", () => {
      const style = new Style();

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
    });
  });

  describe("Grid Template Areas", () => {
    it("gridTemplateAreas: sets and gets areas", () => {
      const style = new Style();

      const areas = [
        {
          name: "header",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 3,
        },
      ];
      style.gridTemplateAreas = areas as any;

      expect(style.gridTemplateAreas.length).toBe(1);
      const area = style.gridTemplateAreas[0] as any;
      expect(area.name).toBe("header");
      expect(area.rowEnd).toBe(2);
    });

    it("gridTemplateAreas: handles complex holy grail layout", () => {
      const style = new Style();

      style.gridTemplateAreas = [
        {
          name: "header",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 4,
        },
        { name: "nav", rowStart: 2, rowEnd: 3, columnStart: 1, columnEnd: 2 },
        { name: "main", rowStart: 2, rowEnd: 3, columnStart: 2, columnEnd: 3 },
        { name: "aside", rowStart: 2, rowEnd: 3, columnStart: 3, columnEnd: 4 },
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
      expect(style.gridTemplateAreas[4].name).toBe("footer");

      const header = style.gridTemplateAreas[0];
      expect(header.rowStart).toBe(1);
      expect(header.columnEnd).toBe(4);
    });
  });

  describe("Grid Auto Flow", () => {
    it("gridAutoFlow: defaults to Row", () => {
      const style = new Style();
      expect(style.gridAutoFlow).toBe(GridAutoFlow.Row);
    });

    it("gridAutoFlow: sets and gets all variants", () => {
      const style = new Style();

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
});

describe("Grid Layout Computation", () => {
  beforeAll(async () => {
    await setupTaffy();
  });

  describe("Basic Grid Layout", () => {
    it("validates basic Grid layout with explicit placement", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.gridRow = { start: 1, end: 2 };
      child1Style.gridColumn = { start: 1, end: 2 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.gridRow = { start: 1, end: 2 };
      child2Style.gridColumn = { start: 2, end: 3 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const rootLayout = tree.getLayout(root);
      expect(rootLayout.width).toBe(100);
      expect(rootLayout.height).toBe(100);

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);
      expect(child2Layout.x).toBeGreaterThanOrEqual(child1Layout.x);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Grid with fixed column widths", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 200, height: 100 };
      rootStyle.gridTemplateColumns = [
        { min: 50, max: 50 },
        { min: 100, max: 100 },
        { min: 50, max: 50 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.gridRow = { start: 1, end: 2 };
      child1Style.gridColumn = { start: 1, end: 2 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.gridRow = { start: 1, end: 2 };
      child2Style.gridColumn = { start: 2, end: 3 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 200, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.width).toBe(50);
      expect(child2Layout.width).toBe(100);
      expect(child2Layout.x).toBe(50);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });

  describe("Grid Auto Flow", () => {
    it("validates Grid auto-flow row", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridAutoFlow = GridAutoFlow.Row;
      rootStyle.gridTemplateColumns = [
        { min: 50, max: 50 },
        { min: 50, max: 50 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const childStyles: Style[] = [];
      const children: bigint[] = [];
      for (let i = 0; i < 4; i++) {
        const style = new Style();
        style.size = { width: 20, height: 20 };
        childStyles.push(style);
        const child = tree.newLeaf(style);
        children.push(child);
        tree.addChild(root, child);
      }

      tree.computeLayout(root, { width: 100, height: 100 });

      // Children should be arranged in a 2x2 grid
      const layout0 = tree.getLayout(children[0]);
      const layout1 = tree.getLayout(children[1]);
      const layout2 = tree.getLayout(children[2]);
      const layout3 = tree.getLayout(children[3]);

      // First row
      expect(layout0.y).toBe(layout1.y);
      expect(layout1.x).toBeGreaterThan(layout0.x);

      // Second row
      expect(layout2.y).toBe(layout3.y);
      expect(layout2.y).toBeGreaterThan(layout0.y);

      tree.free();
      rootStyle.free();
      for (const style of childStyles) {
        style.free();
      }
    });

    it("validates Grid auto-flow column", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridAutoFlow = GridAutoFlow.Column;
      rootStyle.gridTemplateRows = [
        { min: 50, max: 50 },
        { min: 50, max: 50 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const childStyles: Style[] = [];
      const children: bigint[] = [];
      for (let i = 0; i < 4; i++) {
        const style = new Style();
        style.size = { width: 20, height: 20 };
        childStyles.push(style);
        const child = tree.newLeaf(style);
        children.push(child);
        tree.addChild(root, child);
      }

      tree.computeLayout(root, { width: 100, height: 100 });

      // Children should be arranged column-first
      const layout0 = tree.getLayout(children[0]);
      const layout1 = tree.getLayout(children[1]);
      const layout2 = tree.getLayout(children[2]);

      // First column
      expect(layout0.x).toBe(layout1.x);
      expect(layout1.y).toBeGreaterThan(layout0.y);

      // Second column
      expect(layout2.y).toBe(layout0.y);
      expect(layout2.x).toBeGreaterThan(layout0.x);

      tree.free();
      rootStyle.free();
      for (const style of childStyles) {
        style.free();
      }
    });
  });

  describe("Grid Span", () => {
    it("validates Grid with row span", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridTemplateRows = [
        { min: 50, max: 50 },
        { min: 50, max: 50 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      // Child spans 2 rows
      const childStyle = new Style();
      childStyle.gridRow = { start: 1, end: { span: 2 } };
      childStyle.gridColumn = { start: 1, end: 2 };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.height).toBe(100);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });

    it("validates Grid with column span", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridTemplateColumns = [
        { min: 50, max: 50 },
        { min: 50, max: 50 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.gridRow = { start: 1, end: 2 };
      childStyle.gridColumn = { start: 1, end: { span: 2 } };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBe(100);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Grid Gap", () => {
    it("validates Grid with gap between children", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gap = { width: 10, height: 10 };
      rootStyle.gridTemplateColumns = [
        { min: 45, max: 45 },
        { min: 45, max: 45 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.gridRow = { start: 1, end: 2 };
      child1Style.gridColumn = { start: 1, end: 2 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.gridRow = { start: 1, end: 2 };
      child2Style.gridColumn = { start: 2, end: 3 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      const child1Right = child1Layout.x + child1Layout.width;
      expect(child2Layout.x - child1Right).toBeCloseTo(10, 0);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Grid with row and column gaps", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gap = { width: 10, height: 20 };
      rootStyle.gridTemplateColumns = [
        { min: 45, max: 45 },
        { min: 45, max: 45 },
      ] as any;
      rootStyle.gridTemplateRows = [
        { min: 40, max: 40 },
        { min: 40, max: 40 },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const styles: Style[] = [];
      const children: bigint[] = [];
      for (let row = 1; row <= 2; row++) {
        for (let col = 1; col <= 2; col++) {
          const style = new Style();
          style.gridRow = { start: row, end: row + 1 };
          style.gridColumn = { start: col, end: col + 1 };
          styles.push(style);
          const child = tree.newLeaf(style);
          children.push(child);
          tree.addChild(root, child);
        }
      }

      tree.computeLayout(root, { width: 100, height: 100 });

      // Check column gap
      const layout0 = tree.getLayout(children[0]);
      const layout1 = tree.getLayout(children[1]);
      expect(layout1.x - (layout0.x + layout0.width)).toBeCloseTo(10, 0);

      // Check row gap
      const layout2 = tree.getLayout(children[2]);
      expect(layout2.y - (layout0.y + layout0.height)).toBeCloseTo(20, 0);

      tree.free();
      rootStyle.free();
      for (const style of styles) {
        style.free();
      }
    });
  });

  describe("Grid Alignment", () => {
    it("validates Grid with center alignment", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.justifyContent = JustifyContent.Center;
      rootStyle.alignItems = AlignItems.Center;

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
  });

  describe("Grid Scrollbar Width", () => {
    it("validates Grid with scrollbarWidth", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.scrollbarWidth = 15;

      const root = tree.newLeaf(rootStyle);

      const childStyle = new Style();
      childStyle.size = { width: "100%", height: "100%" };
      const child = tree.newLeaf(childStyle);

      tree.addChild(root, child);

      tree.computeLayout(root, { width: 100, height: 100 });

      const childLayout = tree.getLayout(child);
      expect(childLayout.width).toBeLessThanOrEqual(100);
      expect(childLayout.height).toBeLessThanOrEqual(100);

      tree.free();
      rootStyle.free();
      childStyle.free();
    });
  });

  describe("Grid Fr Units", () => {
    it("validates Grid with fr units", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridTemplateColumns = [
        { min: "auto", max: "1fr" },
        { min: "auto", max: "2fr" },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.gridRow = { start: 1, end: 2 };
      child1Style.gridColumn = { start: 1, end: 2 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.gridRow = { start: 1, end: 2 };
      child2Style.gridColumn = { start: 2, end: 3 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      // 1fr : 2fr = 1:2 ratio (with tolerance for integer rounding)
      expect(
        Math.abs(child2Layout.width - child1Layout.width * 2),
      ).toBeLessThanOrEqual(2);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });

    it("validates Grid with mixed fixed and fr units", () => {
      const tree = new TaffyTree();

      const rootStyle = new Style();
      rootStyle.display = Display.Grid;
      rootStyle.size = { width: 100, height: 100 };
      rootStyle.gridTemplateColumns = [
        { min: 20, max: 20 },
        { min: "auto", max: "1fr" },
      ] as any;

      const root = tree.newLeaf(rootStyle);

      const child1Style = new Style();
      child1Style.gridRow = { start: 1, end: 2 };
      child1Style.gridColumn = { start: 1, end: 2 };
      const child1 = tree.newLeaf(child1Style);

      const child2Style = new Style();
      child2Style.gridRow = { start: 1, end: 2 };
      child2Style.gridColumn = { start: 2, end: 3 };
      const child2 = tree.newLeaf(child2Style);

      tree.addChild(root, child1);
      tree.addChild(root, child2);

      tree.computeLayout(root, { width: 100, height: 100 });

      const child1Layout = tree.getLayout(child1);
      const child2Layout = tree.getLayout(child2);

      expect(child1Layout.width).toBe(20);
      expect(child2Layout.width).toBe(80);

      tree.free();
      rootStyle.free();
      child1Style.free();
      child2Style.free();
    });
  });
});

describe("Implicit grid row sizing", () => {
  beforeAll(setupTaffy);

  it("resolves percentage heights in implicit grid rows", () => {
    const tree = new TaffyTree();
    const first = tree.newLeaf(new Style({ height: "50%" }));
    const second = tree.newLeaf(new Style({ width: 50, height: 20 }));
    const root = tree.newWithChildren(
      new Style({ display: Display.Grid, width: 100, height: 100 }),
      [first, second],
    );

    tree.computeLayout(root, { width: 100, height: 100 });
    expect(tree.getLayout(first).get("x", "y", "width", "height")).toEqual([
      0, 0, 100, 20,
    ]);
    expect(tree.getLayout(second).get("x", "y", "width", "height")).toEqual([
      0, 40, 50, 20,
    ]);
  });
});

describe("Grid track sizing and serialization", () => {
  beforeAll(setupTaffy);

  it.each([
    "gridAutoRows",
    "gridAutoColumns",
    "gridTemplateRows",
    "gridTemplateColumns",
  ] as const)(
    "resolves percentages in %s against the container",
    (property) => {
      const columns = property.endsWith("Columns");
      const tracks: TrackSizingFunction[] = [
        { min: "50%", max: "50%" },
        { min: "50%", max: "50%" },
      ];
      const tree = new TaffyTree();
      const children = [tree.newLeaf(new Style()), tree.newLeaf(new Style())];
      const style = new Style({
        display: Display.Grid,
        width: 200,
        height: 200,
        gridAutoFlow: columns ? GridAutoFlow.Column : GridAutoFlow.Row,
      });
      style[property] = tracks;
      const root = tree.newWithChildren(style, children);

      tree.computeLayout(root, { width: 200, height: 200 });
      expect(
        children.map((child) =>
          tree.getLayout(child).get("x", "y", "width", "height"),
        ),
      ).toEqual(
        columns
          ? [
              [0, 0, 100, 200],
              [100, 0, 100, 200],
            ]
          : [
              [0, 0, 200, 100],
              [0, 100, 200, 100],
            ],
      );
      expect(tree.getStyle(root).get(property)).toEqual(tracks);
      expect(style[property]).toEqual(tracks);
    },
  );

  it("resolves percentage tracks nested in a repeat", () => {
    const tree = new TaffyTree();
    const children = [tree.newLeaf(new Style()), tree.newLeaf(new Style())];
    const style = new Style({
      display: Display.Grid,
      width: 200,
      height: 20,
      gridTemplateColumns: [{ count: 2, tracks: [{ min: "50%", max: "50%" }] }],
    });
    const root = tree.newWithChildren(style, children);

    tree.computeLayout(root, { width: 200, height: 20 });
    expect(tree.getLayout(children[0]).width).toBe(100);
    expect(tree.getLayout(children[1]).get("x", "width")).toEqual([100, 100]);
    expect(style.gridTemplateColumns).toEqual([
      {
        count: 2,
        tracks: [{ min: "50%", max: "50%" }],
        lineNames: [],
      },
    ]);
  });

  it("limits a track with a percentage maximum and a numeric minimum", () => {
    const tree = new TaffyTree();
    const child = tree.newLeaf(new Style());
    const root = tree.newWithChildren(
      new Style({
        display: Display.Grid,
        width: 200,
        height: 20,
        gridTemplateColumns: [{ min: 0, max: "50%" }],
      }),
      [child],
    );

    tree.computeLayout(root, { width: 200, height: 20 });
    expect(tree.getLayout(child).width).toBe(100);
  });

  it.each([
    "gridAutoRows",
    "gridAutoColumns",
    "gridTemplateRows",
    "gridTemplateColumns",
  ] as const)(
    "preserves existing track values through %s reads",
    (property) => {
      const tracks: TrackSizingFunction[] = [
        { min: 12, max: 24 },
        { min: "25%", max: "75%" },
        { min: "auto", max: "1.5fr" },
        { min: "min-content", max: "max-content" },
        { min: "max-content", max: "min-content" },
        { min: "auto", max: "fit-content" },
        { min: "auto", max: "auto" },
      ];
      const tree = new TaffyTree();
      const original = new Style({ [property]: tracks });
      const node = tree.newLeaf(original);
      const copied = tree.getStyle(node);

      expect(copied[property]).toEqual(tracks);
      expect(copied.get(property)).toEqual(tracks);
      original.set({ [property]: copied.get(property) });
      expect(original.get(property)).toEqual(tracks);
    },
  );
});
