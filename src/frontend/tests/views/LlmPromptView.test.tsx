import { render, screen, fireEvent } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vitest";
import { LlmPromptView } from "../../src/views/LlmPromptView";
import { act } from "@testing-library/react";

// Mock the backend service
const mockService = {
  greet: vi.fn(),
  getCount: vi.fn(),
  incrementCounter: vi.fn(),
  sendLlmPrompt: vi.fn().mockResolvedValue("This is a mock LLM response"),
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

describe("LlmPromptView", () => {
  const mockOnError = vi.fn();
  const mockSetLoading = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should render the LLM prompt interface", async () => {
    // Setup
    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    // Assert
    expect(screen.getByText("LLM Prompt")).toBeInTheDocument();
    expect(
      screen.getByPlaceholderText("Ask the LLM something..."),
    ).toBeInTheDocument();
    expect(screen.getByText("Send Prompt")).toBeInTheDocument();
  });

  it("should update prompt value when user types in the textarea", async () => {
    // Setup
    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    // Execute
    const textArea = screen.getByPlaceholderText("Ask the LLM something...");
    await act(async () => {
      fireEvent.change(textArea, {
        target: { value: "What is the Internet Computer?" },
      });
    });

    // Assert - Since state is private, we can't assert directly, but we can check if the component behaves correctly later
    expect(textArea).toHaveValue("What is the Internet Computer?");
  });

  it("should send prompt and display response when Send Prompt button is clicked", async () => {
    // Setup
    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    const textArea = screen.getByPlaceholderText("Ask the LLM something...");
    const sendButton = screen.getByText("Send Prompt");

    // Execute
    await act(async () => {
      fireEvent.change(textArea, {
        target: { value: "What is the Internet Computer?" },
      });
    });

    await act(async () => {
      fireEvent.click(sendButton);
    });

    // Assert
    expect(mockService.sendLlmPrompt).toHaveBeenCalledWith(
      "What is the Internet Computer?",
    );
    expect(mockSetLoading).toHaveBeenCalledWith(true);
    expect(mockSetLoading).toHaveBeenCalledWith(false);
    expect(
      await screen.findByText("This is a mock LLM response"),
    ).toBeInTheDocument();
  });

  it("should not send empty prompts", async () => {
    // Setup
    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    const sendButton = screen.getByText("Send Prompt");

    // Execute - click without entering text
    await act(async () => {
      fireEvent.click(sendButton);
    });

    // Assert - should not call the service
    expect(mockService.sendLlmPrompt).not.toHaveBeenCalled();
  });

  it("should handle errors when sending LLM prompt fails", async () => {
    // Setup
    const errorMessage = "Failed to send prompt";
    vi.mocked(mockService.sendLlmPrompt).mockRejectedValueOnce(
      new Error(errorMessage),
    );

    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    const textArea = screen.getByPlaceholderText("Ask the LLM something...");
    const sendButton = screen.getByText("Send Prompt");

    // Execute
    await act(async () => {
      fireEvent.change(textArea, {
        target: { value: "What is the Internet Computer?" },
      });
    });

    await act(async () => {
      fireEvent.click(sendButton);
    });

    // Assert
    expect(mockOnError).toHaveBeenCalledWith(
      expect.stringContaining(errorMessage),
    );
    expect(mockSetLoading).toHaveBeenCalledWith(false);
  });

  it("should show loading state while waiting for response", async () => {
    // Setup
    let resolvePromise: (value: string) => void;
    const delayedPromise = new Promise<string>((resolve) => {
      resolvePromise = resolve;
    });
    vi.mocked(mockService.sendLlmPrompt).mockReturnValueOnce(delayedPromise);

    await act(async () => {
      render(
        <LlmPromptView onError={mockOnError} setLoading={mockSetLoading} />,
      );
    });

    const textArea = screen.getByPlaceholderText("Ask the LLM something...");
    const sendButton = screen.getByText("Send Prompt");

    // Execute
    await act(async () => {
      fireEvent.change(textArea, {
        target: { value: "What is the Internet Computer?" },
      });
    });

    // Click and check loading state
    fireEvent.click(sendButton);

    // Assert loading state
    expect(screen.getByText("Thinking...")).toBeInTheDocument();

    // Resolve the promise
    await act(async () => {
      resolvePromise!("Response received");
    });

    // Assert loading state is cleared
    expect(screen.getByText("Send Prompt")).toBeInTheDocument();
  });
});
