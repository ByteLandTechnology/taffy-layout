import React from "react";
import { test } from "vitest";
import init, {
  TaffyTree,
  Style,
  // Add all other exports that might be needed
  Display,
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
  const root = tree.newLeaf(new Style());
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
  function debugDraw(node: any) {
    const layout = tree.getLayout(node);
    renderer.strokeRect(layout.x, layout.y, layout.width, layout.height, "red");

    for (const child of tree.children(node)) {
      debugDraw(child);
    }
  }
  debugDraw(root);
});
