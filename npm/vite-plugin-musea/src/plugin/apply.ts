import type { ConfigEnv } from "vite";

export function shouldApplyMuseaPlugin(env: Pick<ConfigEnv, "mode">): boolean {
  return env.mode !== "test";
}
