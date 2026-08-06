import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { ResetDataSection } from "./ResetDataSection";

describe("ResetDataSection", () => {
  it("offers credential-only and full reset actions", () => {
    const onReset = vi.fn();

    render(<ResetDataSection onReset={onReset} resetting={false} />);

    expect(screen.getByText(/remove the api key and keep local sql history/i)).toBeDefined();
    expect(screen.getByText(/clear the api key and local sql history/i)).toBeDefined();

    fireEvent.click(screen.getByRole("button", { name: /remove key and keep history/i }));
    expect(onReset).toHaveBeenCalledWith(true);
  });
});
