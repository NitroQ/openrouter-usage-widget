import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ConnectionStatus } from "./ConnectionStatus";

describe("ConnectionStatus", () => {
  it("renders live status", () => {
    render(
      <ConnectionStatus status="live" lastUpdated="just now" />
    );
    expect(screen.getByText(/Live/)).toBeDefined();
    expect(screen.getByText("just now")).toBeDefined();
  });

  it("renders offline status", () => {
    render(
      <ConnectionStatus status="offline" lastUpdated="2m ago" />
    );
    expect(screen.getByText(/Offline/)).toBeDefined();
    expect(screen.getByText("2m ago")).toBeDefined();
  });

  it("renders auth_error status", () => {
    render(
      <ConnectionStatus status="auth_error" lastUpdated="5m ago" />
    );
    expect(screen.getByText(/Auth failed/)).toBeDefined();
  });

  it("renders refreshing status", () => {
    render(
      <ConnectionStatus status="refreshing" lastUpdated="just now" />
    );
    expect(screen.getByText(/Refreshing/)).toBeDefined();
  });
});
