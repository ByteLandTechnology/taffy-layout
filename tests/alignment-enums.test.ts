import { beforeAll, describe, expect, it } from "vitest";
import {
  AlignContent,
  AlignSelf,
  JustifyContent,
  Style,
  AlignItems,
  Display,
} from "../src/index";
import { setupTaffy } from "./utils";

describe.each(["constructor", "set", "assignment"] as const)(
  "Alignment enum reads after %s",
  (update) => {
    beforeAll(async () => {
      await setupTaffy();
    });

    it.each([
      AlignSelf.Start,
      AlignSelf.End,
      AlignSelf.FlexStart,
      AlignSelf.FlexEnd,
      AlignSelf.Center,
      AlignSelf.Baseline,
      AlignSelf.Stretch,
    ])("returns the public self-alignment value %s", (value) => {
      const props = { alignSelf: value, justifySelf: value };
      const style = new Style(update === "constructor" ? props : undefined);
      if (update === "set") {
        style.set(props);
      } else if (update === "assignment") {
        style.alignSelf = value;
        style.justifySelf = value;
      }

      expect(style.alignSelf).toBe(value);
      expect(style.justifySelf).toBe(value);
      expect(style.get("alignSelf")).toBe(value);
      expect(style.get("justifySelf")).toBe(value);
      expect(style.get("alignSelf", "justifySelf")).toEqual([value, value]);
    });

    it.each([
      {
        name: "SpaceAround",
        alignContent: AlignContent.SpaceAround,
        justifyContent: JustifyContent.SpaceAround,
      },
      {
        name: "SpaceEvenly",
        alignContent: AlignContent.SpaceEvenly,
        justifyContent: JustifyContent.SpaceEvenly,
      },
    ])(
      "returns public $name values without swapping distribution modes",
      ({ alignContent, justifyContent }) => {
        const props = { alignContent, justifyContent };
        const style = new Style(update === "constructor" ? props : undefined);
        if (update === "set") {
          style.set(props);
        } else if (update === "assignment") {
          style.alignContent = alignContent;
          style.justifyContent = justifyContent;
        }

        expect(style.alignContent).toBe(alignContent);
        expect(style.justifyContent).toBe(justifyContent);
        expect(style.get("alignContent")).toBe(alignContent);
        expect(style.get("justifyContent")).toBe(justifyContent);
        expect(style.get("alignContent", "justifyContent")).toEqual([
          alignContent,
          justifyContent,
        ]);
      },
    );
  },
);

describe("Display and alignment values", () => {
  beforeAll(setupTaffy);

  it("round-trips public display and alignment values", () => {
    expect([
      Display.Block,
      Display.Flex,
      Display.Grid,
      Display.None,
      Display.FlowRoot,
    ]).toEqual([0, 1, 2, 3, 4]);

    const style = new Style({
      display: Display.FlowRoot,
      alignItems: AlignItems.SafeSelfStart,
      alignSelf: AlignSelf.SafeCenter,
      alignContent: AlignContent.SafeEnd,
      justifyContent: JustifyContent.SafeFlexEnd,
      justifyItems: AlignItems.SelfEnd,
      justifySelf: AlignSelf.SafeSelfEnd,
    });

    expect(
      style.get(
        "display",
        "alignItems",
        "alignSelf",
        "alignContent",
        "justifyContent",
        "justifyItems",
        "justifySelf",
      ),
    ).toEqual([
      Display.FlowRoot,
      AlignItems.SafeSelfStart,
      AlignSelf.SafeCenter,
      AlignContent.SafeEnd,
      JustifyContent.SafeFlexEnd,
      AlignItems.SelfEnd,
      AlignSelf.SafeSelfEnd,
    ]);
  });
});
