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

// Mock the useBackendActors hook to prevent HttpAgent creation during tests
vi.mock("../src/hooks/useBackendActors", () => ({
  useBackendActors: () => ({
    unauthenticatedActor: {
      greet: vi.fn().mockResolvedValue("Hello, World!"),
      get: vi.fn().mockResolvedValue(BigInt(0)),
      inc: vi.fn().mockResolvedValue(BigInt(1)),
      set_count: vi.fn().mockResolvedValue(BigInt(10)),
    },
    authenticatedActor: {
      greet: vi.fn().mockResolvedValue("Hello, World!"),
      get: vi.fn().mockResolvedValue(BigInt(0)),
      inc: vi.fn().mockResolvedValue(BigInt(1)),
      set_count: vi.fn().mockResolvedValue(BigInt(10)),
    },
    isReady: true,
    isAuthenticated: false,
  }),
}));

// Mock variables for NFID Identity Kit
const mockConnect = vi.fn();
const mockDisconnect = vi.fn();
const mockUseAuth = vi.fn();
let mockUser: { principal: string } | undefined = undefined;

// Mock NFID Identity Kit
vi.mock("@nfid/identitykit/react", () => ({
  IdentityKitProvider: ({ children }: { children: React.ReactNode }) =>
    children,
  ConnectWallet: () => <button>Connect Wallet</button>,
  useAuth: () => mockUseAuth(),
  useAgent: () => null, // Mock useAgent for useBackendActors hook
}));

// Set default mock return value
mockUseAuth.mockReturnValue({
  user: mockUser,
  connect: mockConnect,
  disconnect: mockDisconnect,
  isConnecting: false,
});

describe("App", () => {
  it("renders main app with greeting view as default", async () => {
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

    // Check connect wallet button is present
    expect(
      screen.getByRole("button", { name: "Connect Wallet" }),
    ).toBeInTheDocument();

    // Counter should NOT be visible when signed out
    expect(
      screen.queryByRole("button", { name: "Counter" }),
    ).not.toBeInTheDocument();

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

    // Set up authenticated state
    mockUser = { principal: "test-principal" };
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: false,
    });

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

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
});
