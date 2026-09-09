import { beforeAll, describe, expect, it } from "vitest";
import {
  AlignContent,
  Direction,
  Display,
  JustifyContent,
  Position,
  Style,
  TaffyTree,
} from "../src/index";
import { setupTaffy } from "./utils";

describe("Detailed grid layout", () => {
  beforeAll(setupTaffy);

  it("returns null when no detailed grid layout has been computed", () => {
    const tree = new TaffyTree();
    const leafStyle = new Style();
    const gridStyle = new Style({ display: Display.Grid });
    const leaf = tree.newLeaf(leafStyle);
    const emptyGrid = tree.newLeaf(gridStyle);
    const grid = tree.newWithChildren(gridStyle, [leaf]);

    expect(tree.detailedLayoutInfo(grid)).toBeNull();
    tree.computeLayout(grid, { width: 100, height: 100 });
    expect(tree.detailedLayoutInfo(grid)).not.toBeNull();
    expect(tree.detailedLayoutInfo(leaf)).toBeNull();
    tree.computeLayout(emptyGrid, { width: 100, height: 100 });
    expect(tree.detailedLayoutInfo(emptyGrid)).toBeNull();

    leafStyle.free();
    gridStyle.free();
    tree.free();
  });

  it("reports grid gaps and border-relative track positions", () => {
    const tree = new TaffyTree();
    const childStyle = new Style();
    const rootStyle = new Style({
      display: Display.Grid,
      width: 200,
      height: 100,
      padding: { left: 10, right: 10, top: 10, bottom: 10 },
      border: { left: 2, right: 2, top: 2, bottom: 2 },
      columnGap: 10,
      rowGap: 5,
      gridTemplateColumns: [
        { min: 50, max: 50 },
        { min: 70, max: 70 },
      ],
      gridTemplateRows: [
        { min: 20, max: 20 },
        { min: 30, max: 30 },
      ],
    });
    const root = tree.newWithChildren(rootStyle, [tree.newLeaf(childStyle)]);
    tree.computeLayout(root, { width: 200, height: 100 });

    const info = tree.detailedLayoutInfo(root);
    if (info === null)
      throw new Error("Expected detailed grid layout information");
    expect(info.columns).toEqual({
      negativeImplicitTracks: 0,
      explicitTracks: 2,
      positiveImplicitTracks: 0,
      sizes: [50, 70],
      gutters: [0, 10, 0],
      positions: [
        { start: 12, end: 62 },
        { start: 72, end: 142 },
      ],
    });
    expect(info.rows.sizes).toEqual([20, 30]);
    expect(info.rows.gutters).toEqual([0, 5, 0]);
    expect(info.rows.positions).toEqual([
      { start: 12, end: 32 },
      { start: 37, end: 67 },
    ]);
    expect(info.items).toEqual([
      { rowStart: 1, rowEnd: 2, columnStart: 1, columnEnd: 2 },
    ]);
    childStyle.free();
    rootStyle.free();
    tree.free();
  });

  it("includes distributed alignment space in grid gutters", () => {
    const tree = new TaffyTree();
    const childStyle = new Style();
    const rootStyle = new Style({
      display: Display.Grid,
      width: 300,
      height: 100,
      columnGap: 10,
      justifyContent: JustifyContent.SpaceBetween,
      alignContent: AlignContent.Center,
      gridTemplateColumns: [
        { min: 50, max: 50 },
        { min: 50, max: 50 },
      ],
      gridTemplateRows: [{ min: 40, max: 40 }],
    });
    const root = tree.newWithChildren(rootStyle, [tree.newLeaf(childStyle)]);
    tree.computeLayout(root, { width: 300, height: 100 });

    const info = tree.detailedLayoutInfo(root);
    if (info === null)
      throw new Error("Expected detailed grid layout information");
    expect(info.columns.sizes).toEqual([50, 50]);
    expect(info.columns.gutters).toEqual([0, 200, 0]);
    expect(info.columns.positions).toEqual([
      { start: 0, end: 50 },
      { start: 250, end: 300 },
    ]);
    expect(info.rows.positions).toEqual([{ start: 30, end: 70 }]);
    expect(info.rows.gutters).toEqual([0, 0]);
    childStyle.free();
    rootStyle.free();
    tree.free();
  });

  it.each([Direction.Ltr, Direction.Rtl])(
    "keeps collapsed auto-fit tracks in logical order for direction %s",
    (direction) => {
      const tree = new TaffyTree();
      const firstStyle = new Style({ gridColumn: { start: 1, end: 2 } });
      const secondStyle = new Style({ gridColumn: { start: 3, end: 4 } });
      const rootStyle = new Style({
        display: Display.Grid,
        direction,
        width: 300,
        height: 100,
        columnGap: 10,
        justifyContent: JustifyContent.SpaceBetween,
        gridTemplateColumns: [
          { count: "auto-fit", tracks: [{ min: 50, max: 50 }] },
        ],
      });
      const root = tree.newWithChildren(rootStyle, [
        tree.newLeaf(firstStyle),
        tree.newLeaf(secondStyle),
      ]);
      tree.computeLayout(root, { width: 300, height: 100 });

      const info = tree.detailedLayoutInfo(root);
      if (info === null)
        throw new Error("Expected detailed grid layout information");
      expect(info.columns.explicitTracks).toBe(5);
      expect(info.columns.sizes).toEqual([50, 0, 50, 0, 0]);
      expect(info.columns.gutters).toEqual(
        direction === Direction.Ltr
          ? [0, 10, 190, 0, 0, 0]
          : [0, 200, 0, 0, 0, 0],
      );
      expect(info.columns.positions).toEqual(
        direction === Direction.Ltr
          ? [
              { start: 0, end: 50 },
              { start: 60, end: 60 },
              { start: 250, end: 300 },
              { start: 300, end: 300 },
              { start: 300, end: 300 },
            ]
          : [
              { start: 250, end: 300 },
              { start: 50, end: 50 },
              { start: 0, end: 50 },
              { start: 0, end: 0 },
              { start: 0, end: 0 },
            ],
      );
      expect(info.items.map((item) => item.columnStart)).toEqual([1, 3]);
      firstStyle.free();
      secondStyle.free();
      rootStyle.free();
      tree.free();
    },
  );

  it("preserves empty track arrays when the grid has only absolute children", () => {
    const tree = new TaffyTree();
    const childStyle = new Style({
      position: Position.Absolute,
      width: 10,
      height: 10,
    });
    const rootStyle = new Style({ display: Display.Grid });
    const root = tree.newWithChildren(rootStyle, [tree.newLeaf(childStyle)]);
    tree.computeLayout(root, { width: 100, height: 100 });

    const info = tree.detailedLayoutInfo(root);
    if (info === null)
      throw new Error("Expected detailed grid layout information");
    expect(info.columns.sizes).toEqual([]);
    expect(info.columns.positions).toEqual([]);
    expect(info.columns.gutters).toEqual([0]);
    expect(info.rows.sizes).toEqual([]);
    expect(info.rows.positions).toEqual([]);
    expect(info.rows.gutters).toEqual([0]);
    expect(info.items).toEqual([]);
    childStyle.free();
    rootStyle.free();
    tree.free();
  });
});
