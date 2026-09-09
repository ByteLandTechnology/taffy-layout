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

test("i18n_ja-JP_advanced_debugging example 1", async () => {
  const tree = new TaffyTree();
  const childStyle = new Style({ width: 50, height: 50 });
  const child = tree.newLeaf(childStyle);
  childStyle.free();
  const rootStyle = new Style({ width: 100, height: 100 });
  const root = tree.newWithChildren(rootStyle, [child]);
  rootStyle.free();
  tree.computeLayout(root, { width: 100, height: 100 });

  console.log(tree.printTree(root));
  tree.free();
});

test("i18n_ja-JP_advanced_debugging example 2", async () => {
  // モックレンダラー
  const renderer = {
    strokeRect: (x: number, y: number, w: number, h: number, c: string) => {},
  };
  const tree = new TaffyTree();
  const childStyle = new Style({ width: 20, height: 20 });
  const child = tree.newLeaf(childStyle);
  childStyle.free();
  const parentStyle = new Style({
    width: 60,
    height: 60,
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });
  const parent = tree.newWithChildren(parentStyle, [child]);
  parentStyle.free();
  const rootStyle = new Style({
    width: 100,
    height: 100,
    padding: { left: 10, right: 10, top: 10, bottom: 10 },
  });
  const root = tree.newWithChildren(rootStyle, [parent]);
  rootStyle.free();
  tree.computeLayout(root, { width: 100, height: 100 });

  // 可視化デバッガー関数
  function debugDraw(node: bigint, parentX = 0, parentY = 0) {
    const layout = tree.getLayout(node);
    const x = parentX + layout.x;
    const y = parentY + layout.y;
    renderer.strokeRect(x, y, layout.width, layout.height, "red");
    layout.free();

    for (const child of tree.children(node)) {
      debugDraw(child, x, y);
    }
  }
  debugDraw(root);
  // child の親相対位置 (10, 10) は描画時には (20, 20) になります
  tree.free();
});
