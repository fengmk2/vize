import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { test } from "node:test";

import { repoRoot, runMoonScript } from "./_helpers/moonbit.ts";

const cargoToml = fs.readFileSync(path.join(repoRoot, "Cargo.toml"), "utf8");
const currentVersion = cargoToml.match(/^version = "(.+)"$/m)?.[1];

assert.ok(currentVersion, "Failed to read current version from Cargo.toml");

test("release script fails clearly when stdin is not interactive", () => {
  const result = runMoonScript("release", ["minor"]);

  assert.equal(result.status, 1);
  assert.match(
    result.stderr,
    /Error: Confirmation requires an interactive terminal\. Re-run with -y to skip the prompt\.\n$/,
  );
  assert.match(
    result.stdout,
    new RegExp(
      `^Current version: ${currentVersion.replaceAll(".", "\\.")}\\nNew version: .+ \\(tag: v.+\\)\\n\\n$`,
    ),
  );
});

test("release script clears prerelease suffixes for stable bumps", () => {
  const cases = [
    ["1.2.3-alpha.1", "patch", "1.2.4"],
    ["1.2.3-beta", "minor", "1.3.0"],
    ["1.2.3-rc.1", "major", "2.0.0"],
    ["1.2.3-alpha.1", "release", "1.2.3"],
    ["1.2.3-alpha.1", "alpha", "1.2.3-alpha.2"],
  ] as const;

  for (const [current, bump, expected] of cases) {
    const result = runMoonScript("release", ["--print-bump", current, bump]);

    assert.equal(result.status, 0, `${result.stderr}\n${result.stdout}`.trim());
    assert.equal(result.stdout.trim(), expected);
  }
});
