import React from "react";
import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
  Direction,
  Float,
  Clear,
  FlexDirection,
  AlignItems,
  AlignContent,
  JustifyContent,
  Position,
  FlexWrap,
  BoxSizing,
  GridAutoFlow,
  Overflow,
  AlignSelf,
  TextAlign,
  Dimension,
  AvailableSpace,
  Size,
  GridPlacement,
  Rect,
  LengthPercentage,
  LengthPercentageAuto,
  DetailedLayoutInfo,
  DetailedGridInfo,
  DetailedGridTracksInfo,
  DetailedGridItemsInfo,
  TrackSizingFunction,
  MinTrackSizingFunction,
  MaxTrackSizingFunction,
  GridTemplateArea,
  GridTemplateComponent,
  GridTemplateRepetition,
  RepetitionCount,
  StyleProperty,
  StylePropertyValues,
  LayoutProperty,
  Line,
  Point,
  TaffyError,
  Layout,
  MeasureFunction,
} from "taffy-layout";

// Global init for the suite
await init();

// Mock TaffyTreePreview component
const TaffyTreePreview = (_props: any) => null;

test("advanced_debugging example 1", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style({ width: 100, height: 100 }));
  tree.computeLayout(root, { width: 100, height: 100 });

  console.log(tree.printTree(root));
});

test("advanced_debugging example 2", async () => {
  // Mock renderer
  const renderer = {
    strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
  };
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  tree.computeLayout(root, { width: 100, height: 100 });

  // Visual debugger function
  function debugDraw(
    node: bigint,
    parentOrigin = { x: 0, y: 0 },
    absoluteOrigin = parentOrigin,
    rootOrigin = parentOrigin,
  ) {
    const style = tree.getStyle(node);
    const position = style.position;
    style.free();

    const origin =
      position === Position.Fixed
        ? rootOrigin
        : position === Position.Absolute
          ? absoluteOrigin
          : parentOrigin;
    const layout = tree.getLayout(node);
    const point = { x: origin.x + layout.x, y: origin.y + layout.y };
    renderer.strokeRect(point.x, point.y, layout.width, layout.height, "red");
    layout.free();

    const childRootOrigin = node === root ? point : rootOrigin;
    const childAbsoluteOrigin =
      node === root || position !== Position.Static ? point : absoluteOrigin;
    for (const child of tree.children(node)) {
      debugDraw(child, point, childAbsoluteOrigin, childRootOrigin);
    }
  }
  debugDraw(root);
});
