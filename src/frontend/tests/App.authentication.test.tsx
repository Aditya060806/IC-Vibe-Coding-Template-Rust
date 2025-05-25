import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vitest";
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

// Declare mock variables first (hoisted)
const mockConnect = vi.fn();
const mockDisconnect = vi.fn();
let mockUser: any = undefined;
let mockIsConnecting = false;

const mockUseAuth = vi.fn();

// Mock NFID Identity Kit
vi.mock("@nfid/identitykit/react", () => ({
  IdentityKitProvider: ({ children }: { children: React.ReactNode }) =>
    children,
  ConnectWallet: () => <button>Connect Wallet</button>,
  useAuth: () => mockUseAuth(),
}));

describe("App Authentication", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUser = undefined;
    mockIsConnecting = false;

    // Set default mock return value
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });
  });

  it("shows counter navigation only when authenticated", async () => {
    // Test unauthenticated state first
    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Counter should NOT be visible when signed out
    expect(
      screen.queryByRole("button", { name: "Counter" }),
    ).not.toBeInTheDocument();

    // Setup authenticated state
    mockUser = { principal: "test-principal", subaccount: null };
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });

    // Re-render with authenticated state
    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Counter navigation should be visible when authenticated
    expect(screen.getByRole("button", { name: "Counter" })).toBeInTheDocument();
  });

  it("redirects to greeting view when logging out from counter view", async () => {
    const user = userEvent.setup();

    // Start with authenticated user on counter view
    mockUser = { principal: "test-principal", subaccount: null };
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });

    const { rerender } = render(
      <StrictMode>
        <App />
      </StrictMode>,
    );

    // Navigate to counter view
    await user.click(screen.getByRole("button", { name: "Counter" }));
    expect(screen.getByText("Counter: 0")).toBeInTheDocument();

    // Simulate logout by changing mock to unauthenticated
    mockUser = undefined;
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });

    await act(async () => {
      rerender(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Should automatically redirect to greeting view
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
    expect(screen.queryByText("Counter: 0")).not.toBeInTheDocument();
  });

  it("can access all views when authenticated", async () => {
    const user = userEvent.setup();

    // Setup - Mock authenticated user
    mockUser = { principal: "test-principal", subaccount: null };
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Test greeting view (default)
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();

    // Test counter view
    await user.click(screen.getByRole("button", { name: "Counter" }));
    expect(screen.getByText("Counter: 0")).toBeInTheDocument();

    // Test LLM view
    await user.click(screen.getByRole("button", { name: "LLM Prompt" }));
    expect(
      screen.getByPlaceholderText("Ask the LLM something..."),
    ).toBeInTheDocument();

    // Test back to greeting
    await user.click(screen.getByRole("button", { name: "Greeting" }));
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();
  });

  it("can access greeting and LLM views when not authenticated", async () => {
    const user = userEvent.setup();

    // Setup - Mock unauthenticated user
    mockUser = undefined;
    mockUseAuth.mockReturnValue({
      user: mockUser,
      connect: mockConnect,
      disconnect: mockDisconnect,
      isConnecting: mockIsConnecting,
    });

    await act(async () => {
      render(
        <StrictMode>
          <App />
        </StrictMode>,
      );
    });

    // Test greeting view (default)
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();

    // Test LLM view
    await user.click(screen.getByRole("button", { name: "LLM Prompt" }));
    expect(
      screen.getByPlaceholderText("Ask the LLM something..."),
    ).toBeInTheDocument();

    // Test back to greeting
    await user.click(screen.getByRole("button", { name: "Greeting" }));
    expect(screen.getByPlaceholderText("Enter your name")).toBeInTheDocument();

    // Counter navigation should not be present
    expect(
      screen.queryByRole("button", { name: "Counter" }),
    ).not.toBeInTheDocument();
  });
});
