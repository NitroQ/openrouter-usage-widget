import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SettingsSection } from "./SettingsSection";

describe("SettingsSection", () => {
  it("renders title", () => {
    render(
      <SettingsSection title="Account">
        <p>Content</p>
      </SettingsSection>
    );
    expect(screen.getByText("Account")).toBeDefined();
  });

  it("renders children", () => {
    render(
      <SettingsSection title="Widget">
        <input type="text" placeholder="Refresh interval" />
      </SettingsSection>
    );
    expect(screen.getByPlaceholderText("Refresh interval")).toBeDefined();
  });

  it("renders multiple children", () => {
    render(
      <SettingsSection title="Privacy">
        <p>First</p>
        <p>Second</p>
      </SettingsSection>
    );
    expect(screen.getByText("First")).toBeDefined();
    expect(screen.getByText("Second")).toBeDefined();
  });
});
