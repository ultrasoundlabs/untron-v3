import { describe, expect, it } from "@effect/vitest";
import * as fs from "node:fs/promises";
import * as path from "node:path";
import { fileURLToPath } from "node:url";

type Violation = {
  file: string;
  line: number;
  column: number;
  alias: string;
};

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const ignoredDirNames = new Set(["node_modules", ".ponder", "generated"]);

const listSourceFiles = async (dir: string): Promise<string[]> => {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (!ignoredDirNames.has(entry.name)) files.push(...(await listSourceFiles(fullPath)));
      continue;
    }
    if (!entry.isFile()) continue;

    if (fullPath.endsWith(".ts") || fullPath.endsWith(".tsx") || fullPath.endsWith(".js")) {
      files.push(fullPath);
    }
  }

  return files;
};

const indexToLineColumn = (source: string, index: number) => {
  let line = 1;
  let lineStart = 0;
  for (let i = 0; i < index; i++) {
    if (source.charCodeAt(i) === 10) {
      line++;
      lineStart = i + 1;
    }
  }
  return { line, column: index - lineStart + 1 };
};

const extractSqlTemplateTextParts = (source: string) => {
  const templates: Array<{ text: string; startIndex: number }> = [];

  for (let i = 0; i < source.length; i++) {
    const start = source.indexOf("sql`", i);
    if (start === -1) break;

    let cursor = start + 4;
    let exprDepth = 0;
    let text = "";

    while (cursor < source.length) {
      const char = source[cursor]!;

      if (exprDepth === 0) {
        if (char === "`") break;

        if (char === "$" && source[cursor + 1] === "{") {
          exprDepth = 1;
          cursor += 2;
          continue;
        }

        text += char;
        cursor += 1;
        continue;
      }

      if (char === "{") exprDepth += 1;
      if (char === "}") exprDepth -= 1;
      cursor += 1;
    }

    templates.push({ text, startIndex: start });
    i = cursor + 1;
  }

  return templates;
};

const findUnquotedCamelCaseAliases = (sqlText: string) => {
  const violations: Array<{ alias: string; offset: number }> = [];
  const re = /\bAS\s+(?!")([A-Za-z_][A-Za-z0-9_]*[A-Z][A-Za-z0-9_]*)\b/g;
  for (;;) {
    const match = re.exec(sqlText);
    if (!match) break;
    violations.push({ alias: match[1]!, offset: match.index });
  }
  return violations;
};

describe("sql aliases", () => {
  it("does not use unquoted camelCase aliases in sql`` templates", async () => {
    const files = await listSourceFiles(repoRoot);

    const violations: Violation[] = [];

    for (const file of files) {
      const source = await fs.readFile(file, "utf8");
      const templates = extractSqlTemplateTextParts(source);

      for (const template of templates) {
        const matches = findUnquotedCamelCaseAliases(template.text);
        for (const match of matches) {
          const { line, column } = indexToLineColumn(source, template.startIndex + match.offset);
          violations.push({
            file: path.relative(repoRoot, file),
            line,
            column,
            alias: match.alias,
          });
        }
      }
    }

    expect(
      violations.map((v) => `${v.file}:${v.line}:${v.column} unquoted alias: ${v.alias}`)
    ).toEqual([]);
  });
});
