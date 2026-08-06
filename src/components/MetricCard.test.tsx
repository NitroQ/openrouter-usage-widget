import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MetricCard } from "./MetricCard";

describe("MetricCard", () => {
  it("renders label and value", () => {
    render(<MetricCard label="Remaining" value="$74.50" />);
    expect(screen.getByText("Remaining")).toBeDefined();
    expect(screen.getByText("$74.50")).toBeDefined();
  });

  it("renders with custom className", () => {
    const { container } = render(
      <MetricCard label="Test" value="123" className="mt-4" />
    );
    expect(container.firstChild).toBeDefined();
  });

  it("handles empty value", () => {
    render(<MetricCard label="Label" value="" />);
    expect(screen.getByText("Label")).toBeDefined();
  });
});
