import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import userEvent from "@testing-library/user-event";
import App from "../src/App";
import { StrictMode } from "react";
import { act } from "react";

// Mock the backend canister for all views
vi.mock("../declarations/backend", () => ({
  backend: {
    greet: vi.fn().mockResolvedValue("Hello, World!"),
    get: vi.fn().mockResolvedValue(BigInt(0)),
    inc: vi.fn().mockResolvedValue(BigInt(1)),
    set_count: vi.fn().mockResolvedValue(BigInt(10)),
  },
}));

// Mock the LLM canister
vi.mock("../declarations/llm", () => ({
  llm: {
    prompt: vi.fn().mockResolvedValue("LLM response"),
  },
}));

describe("App", () => {
  it("renders the main headings and navigation", async () => {
    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Check header content
    expect(screen.getByText("Vibe Coding Template")).toBeInTheDocument();
    expect(
      screen.getByText("React + Rust + Internet Computer"),
    ).toBeInTheDocument();

    // Check navigation buttons
    expect(
      screen.getByRole("button", { name: "Greeting" }),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "LLM Prompt" }),
    ).toBeInTheDocument();

    // Check sign-in button is present
    expect(screen.getByRole("button", { name: "Sign In" })).toBeInTheDocument();

    // Counter should NOT be visible when signed out
    expect(
      screen.queryByRole("button", { name: "Counter" }),
    ).not.toBeInTheDocument();
  });

  it("starts with greeting view as default", async () => {
    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Greeting view should be visible by default
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Get Greeting" }),
    ).toBeInTheDocument();

    // Counter and LLM views should not be visible
    expect(screen.queryByText("Counter: 0")).not.toBeInTheDocument();
    expect(
      screen.queryByPlaceholderText("Ask the LLM something..."),
    ).not.toBeInTheDocument();
  });

  it("switches to counter view when counter navigation is clicked", async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Sign in first to access counter
    await user.click(screen.getByRole("button", { name: "Sign In" }));

    // Click on Counter navigation
    await user.click(screen.getByRole("button", { name: "Counter" }));

    // Counter view should be visible
    expect(screen.getByText("Counter: 0")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Increment" }),
    ).toBeInTheDocument();

    // Greeting and LLM views should not be visible
    expect(
      screen.queryByPlaceholderText("Enter your name"),
    ).not.toBeInTheDocument();
    expect(
      screen.queryByPlaceholderText("Ask the LLM something..."),
    ).not.toBeInTheDocument();
  });

  it("switches to LLM view when LLM navigation is clicked", async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Click on LLM Prompt navigation
    await user.click(screen.getByRole("button", { name: "LLM Prompt" }));

    // LLM view should be visible
    expect(
      screen.getByPlaceholderText("Ask the LLM something..."),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Send Prompt" }),
    ).toBeInTheDocument();

    // Greeting and Counter views should not be visible
    expect(
      screen.queryByPlaceholderText("Enter your name"),
    ).not.toBeInTheDocument();
    expect(screen.queryByText("Counter: 0")).not.toBeInTheDocument();
  });

  it("shows active state on current navigation button", async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Sign in first to access counter
    await user.click(screen.getByRole("button", { name: "Sign In" }));

    // Initially, Greeting button should have active styling
    const greetingButton = screen.getByRole("button", { name: "Greeting" });
    const counterButton = screen.getByRole("button", { name: "Counter" });

    expect(greetingButton).toHaveClass("bg-gray-600");
    expect(counterButton).not.toHaveClass("bg-gray-600");

    // Click on Counter navigation
    await user.click(counterButton);

    // Now Counter button should have active styling
    expect(counterButton).toHaveClass("bg-gray-600");
    expect(greetingButton).not.toHaveClass("bg-gray-600");
  });

  it("clears errors when switching views", async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Sign in first to access counter
    await user.click(screen.getByRole("button", { name: "Sign In" }));

    // This test would need the views to actually trigger errors
    // For now, we'll test that the navigation works without errors
    await user.click(screen.getByRole("button", { name: "Counter" }));
    await user.click(screen.getByRole("button", { name: "LLM Prompt" }));
    await user.click(screen.getByRole("button", { name: "Greeting" }));

    // Should be back to greeting view
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Get Greeting" }),
    ).toBeInTheDocument();
  });

  it("navigates between views when signed in", async () => {
    const user = userEvent.setup();

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Start with greeting (default)
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();

    // Sign in first to access counter
    await user.click(screen.getByRole("button", { name: "Sign In" }));
    expect(
      screen.getByRole("button", { name: "Sign Out" }),
    ).toBeInTheDocument();

    // Now counter should be visible and accessible
    expect(screen.getByRole("button", { name: "Counter" })).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Counter" }));
    expect(screen.getByText("Counter: 0")).toBeInTheDocument();

    // Go to LLM
    await user.click(screen.getByRole("button", { name: "LLM Prompt" }));
    expect(
      screen.getByPlaceholderText("Ask the LLM something..."),
    ).toBeInTheDocument();

    // Go back to greeting
    await user.click(screen.getByRole("button", { name: "Greeting" }));
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
  });

  describe("Authentication Feature", () => {
    it("starts in signed out state by default", async () => {
      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Should show Sign In button
      expect(
        screen.getByRole("button", { name: "Sign In" }),
      ).toBeInTheDocument();

      // Counter tab should not be visible
      expect(
        screen.queryByRole("button", { name: "Counter" }),
      ).not.toBeInTheDocument();

      // Other tabs should be visible
      expect(
        screen.getByRole("button", { name: "Greeting" }),
      ).toBeInTheDocument();
      expect(
        screen.getByRole("button", { name: "LLM Prompt" }),
      ).toBeInTheDocument();
    });

    it("shows counter tab when user signs in", async () => {
      const user = userEvent.setup();

      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Click Sign In
      await user.click(screen.getByRole("button", { name: "Sign In" }));

      // Should show Sign Out button
      expect(
        screen.getByRole("button", { name: "Sign Out" }),
      ).toBeInTheDocument();
      expect(
        screen.queryByRole("button", { name: "Sign In" }),
      ).not.toBeInTheDocument();

      // Counter tab should now be visible
      expect(
        screen.getByRole("button", { name: "Counter" }),
      ).toBeInTheDocument();
    });

    it("hides counter tab when user signs out", async () => {
      const user = userEvent.setup();

      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Sign in first
      await user.click(screen.getByRole("button", { name: "Sign In" }));
      expect(
        screen.getByRole("button", { name: "Counter" }),
      ).toBeInTheDocument();

      // Sign out
      await user.click(screen.getByRole("button", { name: "Sign Out" }));

      // Should show Sign In button again
      expect(
        screen.getByRole("button", { name: "Sign In" }),
      ).toBeInTheDocument();

      // Counter tab should be hidden
      expect(
        screen.queryByRole("button", { name: "Counter" }),
      ).not.toBeInTheDocument();
    });

    it("redirects to greeting when signing out from counter view", async () => {
      const user = userEvent.setup();

      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Sign in and navigate to counter
      await user.click(screen.getByRole("button", { name: "Sign In" }));
      await user.click(screen.getByRole("button", { name: "Counter" }));

      // Should be on counter view
      expect(screen.getByText("Counter: 0")).toBeInTheDocument();

      // Sign out
      await user.click(screen.getByRole("button", { name: "Sign Out" }));

      // Should be redirected to greeting view
      expect(
        screen.getByPlaceholderText("Enter your name"),
      ).toBeInTheDocument();
      expect(screen.queryByText("Counter: 0")).not.toBeInTheDocument();
    });

    it("allows access to counter view only when signed in", async () => {
      const user = userEvent.setup();

      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Try to access counter when signed out (button shouldn't exist)
      expect(
        screen.queryByRole("button", { name: "Counter" }),
      ).not.toBeInTheDocument();

      // Sign in
      await user.click(screen.getByRole("button", { name: "Sign In" }));

      // Now counter should be accessible
      expect(
        screen.getByRole("button", { name: "Counter" }),
      ).toBeInTheDocument();
      await user.click(screen.getByRole("button", { name: "Counter" }));

      // Should successfully navigate to counter view
      expect(screen.getByText("Counter: 0")).toBeInTheDocument();
    });

    it("preserves non-counter view when signing in and out", async () => {
      const user = userEvent.setup();

      await act(async () => {
        render(
          <StrictMode>
            <App />
          </StrictMode>,
        );
      });

      // Navigate to LLM view
      await user.click(screen.getByRole("button", { name: "LLM Prompt" }));
      expect(
        screen.getByPlaceholderText("Ask the LLM something..."),
      ).toBeInTheDocument();

      // Sign in and out while on LLM view
      await user.click(screen.getByRole("button", { name: "Sign In" }));
      await user.click(screen.getByRole("button", { name: "Sign Out" }));

      // Should still be on LLM view
      expect(
        screen.getByPlaceholderText("Ask the LLM something..."),
      ).toBeInTheDocument();
    });
  });
});
