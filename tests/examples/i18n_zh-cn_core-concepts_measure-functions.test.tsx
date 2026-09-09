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

test("i18n_zh-CN_core-concepts_measure-functions example 1", async () => {
  const tree = new TaffyTree();

  const style = new Style();
  // 此节点没有固定尺寸，因此 Taffy 将询问测量函数
  style.size = { width: "auto", height: "auto" };

  const measuredNode = tree.newLeafWithContext(style, {
    width: 150,
    height: 50,
  });

  const rootStyle = new Style();
  rootStyle.display = Display.Flex;
  rootStyle.size = { width: 300, height: 100 };
  rootStyle.alignItems = AlignItems.Center;
  rootStyle.justifyContent = JustifyContent.Center;

  const root = tree.newWithChildren(rootStyle, [measuredNode]);

  // 我们使用 computeLayoutWithMeasure 而不是 computeLayout
  tree.computeLayoutWithMeasure(
    root,
    { width: 300, height: 100 },
    (knownDims, availableSpace, node, context, measuredStyle) => {
      measuredStyle.free(); // 本例只使用上下文，不需要样式副本
      // 1. 使用本次测量提供的尺寸提示（本例没有 padding 或 border）
      // 2. 否则，基于可用空间或内容固有尺寸计算
      const contentWidth = context?.width ?? 150;
      const contentHeight = context?.height ?? 50;
      const width =
        knownDims.width ??
        (typeof availableSpace.width === "number"
          ? Math.min(availableSpace.width, contentWidth)
          : contentWidth);

      const height = knownDims.height ?? contentHeight;

      return { width, height };
    },
  );

  const measuredLayout = tree.getLayout(measuredNode);
  const measuredSize = measuredLayout.size;
  measuredLayout.free();

  return (
    <div style={{ display: "flex", gap: 10 }}>
      <TaffyTreePreview tree={tree} root={root} />
      <div style={{ padding: 10, background: "#f0f0f0", borderRadius: 4 }}>
        <strong>Measured Size:</strong>
        <br />
        {measuredSize.width} x {measuredSize.height}
      </div>
    </div>
  );
});
