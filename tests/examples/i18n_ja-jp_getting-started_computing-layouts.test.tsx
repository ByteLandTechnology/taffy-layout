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

test("i18n_ja-JP_getting-started_computing-layouts example 1", async () => {
  const tree = new TaffyTree();
  const rootStyle = new Style({
    display: Display.Flex,
    alignItems: AlignItems.Center,
    justifyContent: JustifyContent.Center,
    size: { width: 400, height: 100 },
  });

  const child = tree.newLeaf(
    new Style({
      size: { width: 50, height: 50 },
    }),
  );
  const root = tree.newWithChildren(rootStyle, [child]);

  // 1. レイアウトを計算
  //    制約を渡します：width: 400, height: 100
  tree.computeLayout(root, { width: 400, height: 100 });

  // 2. 結果を読み取り
  //    エンジンがすべてのノードのレイアウトデータを入力しました。
  const rootLayout = tree.getLayout(root);
  const childLayout = tree.getLayout(child);

  console.log(`Root Size: ${rootLayout.width}x${rootLayout.height}`);
  console.log(`Child Pos: ${childLayout.x}, ${childLayout.y}`);

  return (
    <div
      style={{
        width: rootLayout.width,
        height: rootLayout.height,
        background: "#f0f0f0",
        position: "relative",
      }}
    >
      <div
        style={{
          width: childLayout.width,
          height: childLayout.height,
          left: childLayout.x,
          top: childLayout.y,
          position: "absolute",
          background: "#007aff",
        }}
      />
      <div
        style={{
          position: "absolute",
          bottom: 5,
          right: 5,
          fontSize: 10,
          color: "#666",
        }}
      >
        Child at ({childLayout.x}, {childLayout.y})
      </div>
    </div>
  );
});

test("i18n_ja-JP_getting-started_computing-layouts example 2", async () => {
  const tree = new TaffyTree();
  const root = tree.newLeaf(new Style());
  const childNode = tree.newLeaf(new Style());
  tree.addChild(root, childNode);

  // 1. 最初のレイアウト
  tree.computeLayout(root, { width: 800, height: 600 });

  // 2. リーフノードを変更
  const newStyle = new Style({ width: 250 });
  tree.setStyle(childNode, newStyle);

  // 3. 再計算
  //    Taffy は影響を受けないブランチの再計算をスキップします。
  tree.computeLayout(root, { width: 800, height: 600 });
});

test("i18n_ja-JP_getting-started_computing-layouts example 3", async () => {
  const tree = new TaffyTree();

  // サブピクセル精度を有効化
  tree.disableRounding();

  // ... レイアウトを計算 ...
  const node = tree.newLeaf(new Style());
  const layout = tree.getLayout(node);
  console.log(layout.width); // 33 ではなく 33.33333... になる可能性があります
});
