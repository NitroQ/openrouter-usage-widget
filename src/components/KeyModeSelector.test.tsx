import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { KeyModeSelector } from "./KeyModeSelector";

describe("KeyModeSelector", () => {
  it("renders both radio options", () => {
    render(<KeyModeSelector value="standard" onChange={() => {}} />);
    expect(screen.getByText("Standard API key")).toBeDefined();
    expect(screen.getByText("Management API key")).toBeDefined();
  });

  it("shows standard description when standard is selected", () => {
    render(<KeyModeSelector value="standard" onChange={() => {}} />);
    expect(screen.getByText(/Shows usage and spending limits/)).toBeDefined();
  });

  it("shows management description when management is selected", () => {
    render(<KeyModeSelector value="management" onChange={() => {}} />);
    expect(screen.getByText(/Shows account-wide credits/)).toBeDefined();
  });

  it("calls onChange when standard radio is clicked", () => {
    const onChange = vi.fn();
    render(<KeyModeSelector value="management" onChange={onChange} />);
    fireEvent.click(screen.getByText("Standard API key"));
    expect(onChange).toHaveBeenCalledWith("standard");
  });

  it("calls onChange when management radio is clicked", () => {
    const onChange = vi.fn();
    render(<KeyModeSelector value="standard" onChange={onChange} />);
    fireEvent.click(screen.getByText("Management API key"));
    expect(onChange).toHaveBeenCalledWith("management");
  });

  it("does not show standard description when management is selected", () => {
    render(<KeyModeSelector value="management" onChange={() => {}} />);
    expect(screen.queryByText(/Shows usage and spending limits/)).toBeNull();
  });
});
