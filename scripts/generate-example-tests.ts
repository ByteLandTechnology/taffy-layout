import {
  readFileSync,
  writeFileSync,
  mkdirSync,
  rmSync,
  existsSync,
  readdirSync,
} from "fs";
import { execSync } from "child_process";
import { resolve, join, basename, extname } from "path";

const srcDir = resolve(process.cwd(), "src");
const readmePath = resolve(process.cwd(), "README.md");
const outDir = resolve(process.cwd(), "tests/examples");

// Clean output dir
if (existsSync(outDir)) {
  rmSync(outDir, { recursive: true, force: true });
}
mkdirSync(outDir, { recursive: true });

try {
  const snippets: { source: string; code: string; index: number }[] = [];

  // Helper to extract snippets from content
  const extractSnippets = (
    content: string,
    sourceName: string,
    isRust: boolean,
  ) => {
    // Regex to find typescript code blocks
    // For Rust files, we look for blocks that are inside comments.
    // But typically we can just find the fenced blocks and then clean the lines.
    const regex = /```typescript([\s\S]*?)```/g;
    let match;
    let counter = 0; // Reset counter for each source if we want per-source indexing (or keep global if distinctness is key)
    // Actually user probably wants sequential per source.
    while ((match = regex.exec(content)) !== null) {
      counter++;
      let code = match[1];
      if (isRust) {
        // Strip Rust comment prefixes (/// or //!) and JSDoc stars (*) from each line
        code = code
          .split("\n")
          .map((line) => {
            return line
              .replace(/^\s*\/\/[!/]?\s?/, "") // Strip /// or //!
              .replace(/^\s*\*\s?/, ""); // Strip *
          })
          .join("\n");
      }
      snippets.push({ source: sourceName, code, index: counter });
    }
  };

  // 1. Process src/*.rs files
  const files = readdirSync(srcDir).filter(
    (f) => f.endsWith(".rs") && f !== "utils.rs",
  );
  for (const file of files) {
    const content = readFileSync(join(srcDir, file), "utf-8");
    extractSnippets(content, basename(file, ".rs"), true);
  }

  // 2. Process README.md
  if (existsSync(readmePath)) {
    const content = readFileSync(readmePath, "utf-8");
    extractSnippets(content, "readme", false);
  }

  console.log(`Found ${snippets.length} examples.`);

  // Group snippets by source
  const snippetsBySource: Record<string, typeof snippets> = {};
  snippets.forEach((item) => {
    if (!snippetsBySource[item.source]) {
      snippetsBySource[item.source] = [];
    }
    snippetsBySource[item.source].push(item);
  });

  Object.entries(snippetsBySource).forEach(([source, items]) => {
    const fileName = `${source.toLowerCase()}.test.ts`;
    const filePath = join(outDir, fileName);

    let fileContent = `
import { test } from 'vitest';
import init, { 
    TaffyTree, 
    Style,
    // Add all other exports that might be needed
    Display, FlexDirection, AlignItems, AlignContent, JustifyContent, 
    Position, FlexWrap, BoxSizing, GridAutoFlow, Overflow, AlignSelf, 
    TextAlign, Dimension, AvailableSpace, Size, GridPlacement, Rect, 
    LengthPercentage, LengthPercentageAuto,
    DetailedLayoutInfo, DetailedGridInfo, DetailedGridTracksInfo, DetailedGridItemsInfo,
    TrackSizingFunction, Point, TaffyError, Layout, MeasureFunction
} from 'taffy-layout';

// Global init for the suite
await init();
`;

    items.forEach((item) => {
      let exampleCode = item.code;

      // Robustly strip imports using regex
      exampleCode = exampleCode.replace(
        /import\s+(?:[\s\S]*?from\s+)?['"][^'"]+['"];?/g,
        "",
      );

      const bodyLines: string[] = [];
      exampleCode.split("\n").forEach((line) => {
        const trimmed = line.trim();
        if (trimmed !== "await init();" && trimmed !== "await loadTaffy();") {
          bodyLines.push(line);
        }
      });

      fileContent += `
test('${item.source} example ${item.index}', async () => {
    ${bodyLines.join("\n")}
});
`;
    });

    writeFileSync(filePath, fileContent);

    try {
      execSync(`npx prettier --write "${filePath}"`, { stdio: "inherit" });
    } catch (error) {
      console.warn(`⚠️  Formatting failed for ${fileName}`, error);
    }
  });

  console.log(`Generated test files in ${outDir}`);

  // Run tsc once for all generated files
  try {
    execSync(`npx tsc --noEmit -p tsconfig.test.json`, { stdio: "inherit" });
    console.log("✅ TypeScript type checking passed");
  } catch (error) {
    console.error("❌ TypeScript type checking failed");
    process.exit(1);
  }
} catch (error) {
  console.error("❌ Generation failed.", error);
  process.exit(1);
}
