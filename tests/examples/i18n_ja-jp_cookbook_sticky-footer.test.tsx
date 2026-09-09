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

test("i18n_ja-JP_cookbook_sticky-footer example 1", async () => {
  const tree = new TaffyTree();

  // ページコンテナ
  const pageStyle = new Style({
    display: Display.Flex,
    flexDirection: FlexDirection.Column,
    size: { width: 300, height: "auto" },
    minSize: { width: "auto", height: 300 }, // 少なくともビューポートの高さ
  });

  const header = tree.newLeaf(
    new Style({
      size: { width: "100%", height: 50 },
      marginBottom: 10,
      flexShrink: 0,
    }),
  );
  const footer = tree.newLeaf(
    new Style({
      size: { width: "100%", height: 50 },
      marginTop: 10,
      flexShrink: 0,
    }),
  );

  // コンテンツがスペースを埋めるために成長
  const content = tree.newLeaf(
    new Style({
      flexGrow: 1,
      flexShrink: 0,
      size: { width: "100%", height: "auto" },
    }),
  );

  const root = tree.newWithChildren(pageStyle, [header, content, footer]);

  tree.computeLayout(root, { width: 300, height: 300 });

  return <TaffyTreePreview tree={tree} root={root} />;
});
