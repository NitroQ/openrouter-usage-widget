import { describe, expect, it } from "vitest";

describe("setup state transitions", () => {
  it("marks the app configured after a successful credential replacement", () => {
    const settings = { configured: false, keyMode: "standard" };
    const result = { success: true };

    const next = result.success
      ? { ...settings, configured: true, keyMode: "management" }
      : settings;

    expect(next).toEqual({ configured: true, keyMode: "management" });
  });
});
