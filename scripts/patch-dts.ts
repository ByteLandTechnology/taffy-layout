import { readFileSync, writeFileSync, readdirSync } from "fs";
import { join } from "path";

interface ClassDoc {
  jsName: string;
  docComment: string;
}

function extractClassDocs(rustSource: string): ClassDoc[] {
  const results: ClassDoc[] = [];
  const lines = rustSource.split("\n");

  let i = 0;
  while (i < lines.length) {
    const line = lines[i];

    // Look for #[wasm_bindgen(js_name = ClassName)] followed by pub struct
    const wasmBindgenMatch = line.match(
      /^\s*#\[wasm_bindgen\(js_name\s*=\s*(\w+)\)\]\s*$/,
    );
    if (wasmBindgenMatch) {
      const jsName = wasmBindgenMatch[1];

      // Check if next non-empty, non-attribute line is a pub struct
      let j = i + 1;
      while (j < lines.length) {
        const nextLine = lines[j].trim();
        if (nextLine.startsWith("#[") || nextLine === "") {
          j++;
          continue;
        }
        if (nextLine.startsWith("pub struct")) {
          // Found a struct, now collect doc comments before the wasm_bindgen attribute
          const docLines: string[] = [];
          let k = i - 1;

          // Go backwards to collect doc comments
          while (k >= 0) {
            const prevLine = lines[k].trim();
            if (prevLine.startsWith("///")) {
              // Extract the doc content (keep original formatting)
              const docContent = prevLine.replace(/^\/\/\/\s?/, "");
              docLines.unshift(docContent);
              k--;
            } else if (prevLine === "" || prevLine.startsWith("//!")) {
              // Skip empty lines and module-level comments
              k--;
            } else if (prevLine.startsWith("// =")) {
              // Stop at section dividers
              break;
            } else {
              break;
            }
          }

          if (docLines.length > 0) {
            // Convert to TSDoc format - keep all content
            const tsDocComment = convertToTsDoc(docLines);
            results.push({ jsName, docComment: tsDocComment });
          }
        }
        break;
      }
    }
    i++;
  }

  return results;
}

function convertToTsDoc(docLines: string[]): string {
  const lines: string[] = ["/**"];

  for (const line of docLines) {
    // Convert Rust # headers to @-prefixed sections where appropriate
    if (line.startsWith("# ")) {
      const header = line.substring(2);
      // Map common Rust doc headers to TSDoc
      if (header === "Example" || header === "Examples") {
        lines.push(` * @example`);
      } else if (header === "Returns") {
        lines.push(` * @returns`);
      } else if (header === "Arguments" || header === "Parameters") {
        lines.push(` *`);
        lines.push(` * **${header}:**`);
      } else {
        // Keep other headers as bold text
        lines.push(` *`);
        lines.push(` * **${header}**`);
      }
    } else {
      lines.push(` * ${line}`);
    }
  }

  lines.push(" */");
  return lines.join("\n");
}

function patchDtsFile(dtsPath: string, classDocs: ClassDoc[]): void {
  let content = readFileSync(dtsPath, "utf-8");

  for (const { jsName, docComment } of classDocs) {
    // Match "export class ClassName" that doesn't already have a JSDoc comment
    const classPattern = new RegExp(`(^|\\n)(export class ${jsName}\\s)`, "g");

    // Check if already has a JSDoc comment
    const alreadyDocumented = new RegExp(
      `\\/\\*\\*[^*]*\\*\\/\\s*\\nexport class ${jsName}\\s`,
    );
    if (alreadyDocumented.test(content)) {
      continue;
    }

    content = content.replace(classPattern, `$1${docComment}\n$2`);
  }

  writeFileSync(dtsPath, content);
}

function main(): void {
  const projectRoot = join(import.meta.dirname, "..");
  const srcDir = join(projectRoot, "src");
  const dtsPath = join(projectRoot, "pkg", "taffy_wasm.d.ts");

  const allClassDocs: ClassDoc[] = [];

  // Parse all .rs files in src directory
  const rustFiles = readdirSync(srcDir).filter((f) => f.endsWith(".rs"));

  for (const file of rustFiles) {
    const filePath = join(srcDir, file);
    const source = readFileSync(filePath, "utf-8");
    const docs = extractClassDocs(source);
    allClassDocs.push(...docs);
  }

  if (allClassDocs.length === 0) {
    console.log("No class documentation found in Rust sources");
    return;
  }

  patchDtsFile(dtsPath, allClassDocs);
  console.log(
    `✓ Added documentation for ${allClassDocs.length} classes: ${allClassDocs.map((d) => d.jsName).join(", ")}`,
  );
}

main();
