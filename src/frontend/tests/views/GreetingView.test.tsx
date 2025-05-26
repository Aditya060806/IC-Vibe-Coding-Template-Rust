import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { StrictMode } from "react";
import { GreetingView } from "../../src/views/GreetingView";
import userEvent from "@testing-library/user-event";

// Mock the useBackendActors hook
const mockService = {
  greet: vi.fn().mockResolvedValue("Hello, Test User!"),
  getCount: vi.fn(),
  incrementCounter: vi.fn(),
  sendLlmPrompt: vi.fn(),
};

vi.mock("../../src/services/backendService", () => ({
  createBackendService: vi.fn(() => mockService),
}));

vi.mock("../../src/hooks/useBackendActors", () => ({
  useBackendActors: vi.fn(() => ({
    authenticatedActor: {} as any,
    unauthenticatedActor: {} as any,
    isReady: true,
    isAuthenticated: false,
  })),
}));

describe("GreetingView", () => {
  const mockSetLoading = vi.fn();
  const mockOnError = vi.fn();

  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();
  });

  it("should render the greeting input and button", () => {
    // Setup
    render(
      <StrictMode>
        <GreetingView onError={mockOnError} setLoading={mockSetLoading} />
      </StrictMode>,
    );

    // Assert
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
    expect(screen.getByText("Get Greeting")).toBeInTheDocument();
  });

  it("should call the greet service and display the response when button is clicked", async () => {
    // Setup
    render(
      <StrictMode>
        <GreetingView onError={mockOnError} setLoading={mockSetLoading} />
      </StrictMode>,
    );

    // Execute
    const input = screen.getByPlaceholderText("Enter your name");
    await userEvent.type(input, "Test User");
    await userEvent.click(screen.getByText("Get Greeting"));

    // Assert
    expect(mockSetLoading).toHaveBeenCalledWith(true);
    expect(mockService.greet).toHaveBeenCalledWith("Test User");
    expect(await screen.findByText("Hello, Test User!")).toBeInTheDocument();
    expect(mockSetLoading).toHaveBeenLastCalledWith(false);
  });

  it("should handle error when service call fails", async () => {
    // Setup - override mock to throw an error
    const errorMessage = "Failed to fetch greeting";
    vi.mocked(mockService.greet).mockRejectedValueOnce(new Error(errorMessage));

    render(
      <StrictMode>
        <GreetingView onError={mockOnError} setLoading={mockSetLoading} />
      </StrictMode>,
    );

    // Execute
    await userEvent.click(screen.getByText("Get Greeting"));

    // Assert
    expect(mockOnError).toHaveBeenCalledWith(
      expect.stringContaining(errorMessage),
    );
    expect(mockSetLoading).toHaveBeenLastCalledWith(false);
  });
});
