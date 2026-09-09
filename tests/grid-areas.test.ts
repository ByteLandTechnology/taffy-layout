import { beforeAll, describe, expect, it } from "vitest";
import { Display, GridAutoFlow, Style, TaffyTree } from "../src/index";
import { setupTaffy } from "./utils";

describe("Grid template area dimensions", () => {
  beforeAll(setupTaffy);

  it("preserves unnamed rows and columns in grid template areas", () => {
    const style = new Style({
      display: Display.Grid,
      width: 300,
      height: 200,
      gridTemplateAreaRowCount: 2,
      gridTemplateAreaColumnCount: 3,
      gridTemplateAreas: [
        {
          name: "main",
          rowStart: 1,
          rowEnd: 2,
          columnStart: 1,
          columnEnd: 2,
        },
      ],
    });

    expect(style.gridTemplateAreaRowCount).toBe(2);
    expect(style.gridTemplateAreaColumnCount).toBe(3);

    const tree = new TaffyTree();
    const children = Array.from({ length: 6 }, () => tree.newLeaf(new Style()));
    const root = tree.newWithChildren(style, children);

    tree.computeLayout(root, { width: 300, height: 200 });

    expect(tree.getLayout(children[2]).x).toBe(200);
    expect(tree.getLayout(children[3]).y).toBe(100);
  });

  it("clamps explicit grid area counts to the named area extents", () => {
    const style = new Style({
      gridTemplateAreaRowCount: 1,
      gridTemplateAreaColumnCount: 1,
      gridTemplateAreas: [
        {
          name: "content",
          rowStart: 1,
          rowEnd: 4,
          columnStart: 1,
          columnEnd: 5,
        },
      ],
    });

    expect(style.gridTemplateAreaRowCount).toBe(3);
    expect(style.gridTemplateAreaColumnCount).toBe(4);
  });

  it("preserves explicit grid area counts when areas are assigned later", () => {
    const style = new Style({
      gridTemplateAreaRowCount: 2,
      gridTemplateAreaColumnCount: 3,
    });

    style.gridTemplateAreas = [
      {
        name: "main",
        rowStart: 1,
        rowEnd: 2,
        columnStart: 1,
        columnEnd: 2,
      },
    ];

    expect(style.gridTemplateAreaRowCount).toBe(2);
    expect(style.gridTemplateAreaColumnCount).toBe(3);

    style.gridTemplateAreas = [];

    expect(style.gridTemplateAreas).toEqual([]);
    expect(style.gridTemplateAreaRowCount).toBe(2);
    expect(style.gridTemplateAreaColumnCount).toBe(3);
  });

  it.each(["assignment", "set"] as const)(
    "shrinks and clears inferred grid dimensions through %s",
    (update) => {
      const tree = new TaffyTree();
      const children = [tree.newLeaf(new Style()), tree.newLeaf(new Style())];
      const style = new Style({
        display: Display.Grid,
        width: 200,
        height: 100,
        gridTemplateAreas: [
          {
            name: "main",
            rowStart: 1,
            rowEnd: 3,
            columnStart: 1,
            columnEnd: 3,
          },
        ],
      });
      const root = tree.newWithChildren(style, children);

      for (const areas of [
        [
          {
            name: "main",
            rowStart: 1,
            rowEnd: 2,
            columnStart: 1,
            columnEnd: 2,
          },
        ],
        [],
      ]) {
        if (update === "assignment") {
          style.gridTemplateAreas = areas;
        } else {
          style.set({ gridTemplateAreas: areas });
        }
        expect(
          style.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
        ).toEqual([areas.length, areas.length]);
        tree.setStyle(root, style);
        tree.computeLayout(root, { width: 200, height: 100 });

        expect(
          tree.getLayout(children[0]).get("x", "y", "width", "height"),
        ).toEqual([0, 0, 200, 50]);
        expect(
          tree.getLayout(children[1]).get("x", "y", "width", "height"),
        ).toEqual([0, 50, 200, 50]);
      }
    },
  );

  it.each(["leaf", "context", "children"] as const)(
    "preserves explicit dimensions through %s node style round-trips",
    (creation) => {
      const tree = new TaffyTree();
      const area = {
        name: "main",
        rowStart: 1,
        rowEnd: 3,
        columnStart: 1,
        columnEnd: 4,
      };
      const original = new Style({
        gridTemplateAreas: [area],
        gridTemplateAreaRowCount: 2,
        gridTemplateAreaColumnCount: 3,
      });
      const node =
        creation === "leaf"
          ? tree.newLeaf(original)
          : creation === "context"
            ? tree.newLeafWithContext(original, { text: "hello" })
            : tree.newWithChildren(original, []);
      const grown = tree.getStyle(node);
      grown.gridTemplateAreas = [{ ...area, rowEnd: 5, columnEnd: 6 }];
      expect(
        grown.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual([4, 5]);
      tree.setStyle(node, grown);

      const cleared = tree.getStyle(node);
      cleared.gridTemplateAreas = [];
      expect(
        cleared.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual([2, 3]);

      // Replacing the entire style must discard the previous explicit counts.
      tree.setStyle(node, new Style({ gridTemplateAreas: [area] }));
      const inferred = tree.getStyle(node);
      inferred.gridTemplateAreas = [];
      expect(
        inferred.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual([0, 0]);
    },
  );

  it.each(["row", "column"] as const)(
    "preserves only the explicitly set %s dimension",
    (axis) => {
      const tree = new TaffyTree();
      const style = new Style({
        display: Display.Grid,
        width: 200,
        height: 100,
        gridAutoFlow: axis === "row" ? GridAutoFlow.Column : GridAutoFlow.Row,
        gridTemplateAreas: [
          {
            name: "main",
            rowStart: 1,
            rowEnd: 4,
            columnStart: 1,
            columnEnd: 4,
          },
        ],
      });
      if (axis === "row") {
        style.gridTemplateAreaRowCount = 2;
      } else {
        style.gridTemplateAreaColumnCount = 2;
      }
      style.gridTemplateAreas = [];
      expect(
        style.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual(axis === "row" ? [2, 0] : [0, 2]);

      const children = [tree.newLeaf(new Style()), tree.newLeaf(new Style())];
      const root = tree.newWithChildren(style, children);
      tree.computeLayout(root, { width: 200, height: 100 });
      expect(
        tree.getLayout(children[1]).get("x", "y", "width", "height"),
      ).toEqual(axis === "row" ? [0, 50, 200, 50] : [100, 0, 100, 100]);

      style.set({
        gridTemplateAreaRowCount: 0,
        gridTemplateAreaColumnCount: 0,
      });
      expect(
        style.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual([0, 0]);
    },
  );

  it("preserves explicit dimensions in styles passed to measure callbacks", () => {
    const tree = new TaffyTree();
    const context = { text: "hello" };
    const node = tree.newLeafWithContext(
      new Style({
        gridTemplateAreaRowCount: 2,
        gridTemplateAreaColumnCount: 3,
        gridTemplateAreas: [
          {
            name: "main",
            rowStart: 1,
            rowEnd: 5,
            columnStart: 1,
            columnEnd: 6,
          },
        ],
      }),
      context,
    );
    const measuredStyles: Style[] = [];
    const contexts: unknown[] = [];
    tree.computeLayoutWithMeasure(
      node,
      { width: 200, height: 100 },
      (_known, _available, _node, receivedContext, style) => {
        measuredStyles.push(style);
        contexts.push(receivedContext);
        return { width: 20, height: 10 };
      },
    );
    expect(measuredStyles.length).toBeGreaterThan(0);
    for (const style of measuredStyles) {
      style.gridTemplateAreas = [];
      expect(
        style.get("gridTemplateAreaRowCount", "gridTemplateAreaColumnCount"),
      ).toEqual([2, 3]);
    }
    expect(contexts.every((value) => value === context)).toBe(true);
  });
});
