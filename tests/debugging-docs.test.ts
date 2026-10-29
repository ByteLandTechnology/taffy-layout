import { readFileSync } from "node:fs";
import { transpile } from "typescript";
import { beforeAll, describe, expect, it, vi } from "vitest";
import { Position, Style, TaffyTree } from "../src/index";
import { setupTaffy } from "./utils";

// Execute the documented traversal itself so a regression in any translation
// cannot be hidden by a separate, correct copy of the algorithm in this test.
describe.each([
  "../docs/advanced/debugging.md",
  "../docs/i18n/zh-CN/advanced/debugging.md",
  "../docs/i18n/ja-JP/advanced/debugging.md",
])("Canvas positions in %s", (path) => {
  beforeAll(setupTaffy);
  const markdown = readFileSync(new URL(path, import.meta.url), "utf8");
  const snippet = markdown.match(
    /function debugDraw\([\s\S]*?\n}\ndebugDraw\(root\);/,
  );
  if (!snippet) throw new Error(`Missing debugDraw example in ${path}`);
  const draw = new Function(
    "tree",
    "root",
    "renderer",
    "Position",
    transpile(snippet[0]),
  );

  it.each([
    {
      name: "in-flow child uses its direct parent",
      position: Position.Static,
      parentPosition: Position.Static,
      rootPosition: Position.Relative,
      expected: { x: 45, y: 65 },
    },
    {
      name: "absolute child skips a static parent",
      position: Position.Absolute,
      parentPosition: Position.Static,
      rootPosition: Position.Relative,
      expected: { x: 25, y: 35 },
    },
    {
      name: "absolute child uses its relative parent",
      position: Position.Absolute,
      parentPosition: Position.Relative,
      rootPosition: Position.Relative,
      expected: { x: 45, y: 65 },
    },
    {
      name: "absolute child uses its sticky parent",
      position: Position.Absolute,
      parentPosition: Position.Sticky,
      rootPosition: Position.Relative,
      expected: { x: 45, y: 65 },
    },
    {
      name: "absolute child without a positioned ancestor uses the root origin",
      position: Position.Absolute,
      parentPosition: Position.Static,
      rootPosition: Position.Static,
      expected: { x: 0, y: 0 },
    },
    {
      name: "fixed child skips positioned ancestors",
      position: Position.Fixed,
      parentPosition: Position.Relative,
      rootPosition: Position.Relative,
      expected: { x: 0, y: 0 },
    },
  ])("$name", ({ position, parentPosition, rootPosition, expected }) => {
    const tree = new TaffyTree();
    const childStyle = new Style({
      position,
      size: { width: 10, height: 10 },
      left: 0,
      top: 0,
    });
    const child = tree.newLeaf(childStyle);
    childStyle.free();
    const parentStyle = new Style({
      position: parentPosition,
      size: { width: 50, height: 50 },
      margin: { left: 20, top: 30, right: 0, bottom: 0 },
    });
    const parent = tree.newWithChildren(parentStyle, [child]);
    parentStyle.free();
    const rootStyle = new Style({
      position: rootPosition,
      size: { width: 100, height: 100 },
      margin: { left: 20, top: 30, right: 0, bottom: 0 },
      border: { left: 5, top: 5, right: 5, bottom: 5 },
    });
    const root = tree.newWithChildren(rootStyle, [parent]);
    rootStyle.free();

    try {
      tree.computeLayout(root, { width: 300, height: 200 });
      const renderer = { strokeRect: vi.fn() };
      draw(tree, root, renderer, Position);
      expect(renderer.strokeRect).toHaveBeenCalledTimes(3);
      expect(renderer.strokeRect).toHaveBeenNthCalledWith(
        3,
        expected.x,
        expected.y,
        10,
        10,
        "red",
      );
    } finally {
      tree.free();
    }
  });
});
