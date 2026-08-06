import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SetupRequiredState } from "./SetupRequiredState";

describe("SetupRequiredState", () => {
  it("shows setup guidance and opens settings", () => {
    const onOpenSettings = vi.fn();

    render(<SetupRequiredState onOpenSettings={onOpenSettings} />);

    expect(screen.getByText("Setup required")).toBeDefined();
    expect(screen.getByText(/add your openrouter api key/i)).toBeDefined();

    fireEvent.click(screen.getByRole("button", { name: /open settings/i }));

    expect(onOpenSettings).toHaveBeenCalledOnce();
  });
});
