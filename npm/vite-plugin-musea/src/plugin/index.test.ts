import assert from "node:assert/strict";
import test from "node:test";
import { shouldApplyMuseaPlugin } from "./apply.js";

void test("musea plugin is inactive in Vite test mode", () => {
  assert.equal(shouldApplyMuseaPlugin({ mode: "test" }), false);
});

void test("musea plugin remains active outside Vite test mode", () => {
  assert.equal(shouldApplyMuseaPlugin({ mode: "development" }), true);
});
