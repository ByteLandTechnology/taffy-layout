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

test("i18n_ja-JP_core-concepts_measure-functions example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  // このノードは固定サイズを持たないため、Taffy は測定関数に問い合わせます
  style.size = { width: "auto", height: "auto" };

  const measuredNode = tree.newLeaf(style);

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.size = { width: 300, height: 100 };
  rootStyle.alignItems = AlignItems.Center;
  rootStyle.justifyContent = JustifyContent.Center;

  const root = tree.newWithChildren(rootStyle, [measuredNode]);

  // computeLayout の代わりに computeLayoutWithMeasure を使用します
  tree.computeLayoutWithMeasure(
    root,
    { width: 300, height: 100 },
    (knownDims, availableSpace) => {
      // 1. 既知の寸法（スタイルオーバーライド）があるか確認
      // 2. そうでない場合、利用可能なスペースまたはコンテンツの固有サイズに基づいて計算
      const width =
        knownDims.width ??
        (typeof availableSpace.width === "number"
          ? Math.min(availableSpace.width, 150)
          : 150);

      const height = knownDims.height ?? 50;

      return { width, height };
    },
  );

  return (
    <div style={{ display: "flex", gap: 10 }}>
      <TaffyTreePreview tree={tree} root={root} />
      <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
        <strong>Measured Size:</strong>
        <br />
        {tree.getLayout(measuredNode).width} x{" "}
        {tree.getLayout(measuredNode).height}
      </div>
    </div>
  );
});
