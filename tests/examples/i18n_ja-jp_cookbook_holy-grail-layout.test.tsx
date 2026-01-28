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

test("i18n_ja-JP_cookbook_holy-grail-layout example 1", async () => {
  const tree = new TaffyTree();

  const pageStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    size: { width: 600, height: 400 },
    gap: { width: 0, height: 10 },
  });

  const header = tree.newLeaf(
    new Style({
      size: { width: "100%", height: 50 },
      flexShrink: 0,
    }),
  );
  const footer = tree.newLeaf(
    new Style({
      size: { width: "100%", height: 50 },
      flexShrink: 0,
    }),
  );

  const bodyRowStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Row,
    flexGrow: 1, // ヘッダー/フッター間の垂直スペースを埋める
    gap: { width: 10, height: 0 },
  });

  const left = tree.newLeaf(
    new Style({ size: { width: 100, height: "auto" } }),
  );
  const right = tree.newLeaf(
    new Style({ size: { width: 100, height: "auto" } }),
  );
  const main = tree.newLeaf(
    new Style({
      flexGrow: 1,
    }),
  );

  const bodyRow = tree.newWithChildren(bodyRowStyle, [left, main, right]);
  const root = tree.newWithChildren(pageStyle, [header, bodyRow, footer]);

  tree.computeLayout(root, { width: 600, height: 400 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
