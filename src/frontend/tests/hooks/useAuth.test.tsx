import { renderHook } from "@testing-library/react";
import { describe, expect, it, vi, beforeEach } from "vitest";

// Mock the Identity Kit module
vi.mock("@nfid/identitykit/react", () => ({
  useAuth: vi.fn(() => ({
    user: undefined,
    connect: vi.fn(),
    disconnect: vi.fn(),
    isConnecting: false,
  })),
}));

// Import after mocking
import { useAuth } from "../../src/hooks/useAuth";

describe("useAuth", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should return isSignedIn as false when user is null", () => {
    const { result } = renderHook(() => useAuth());

    expect(result.current.isSignedIn).toBe(false);
    expect(result.current.user).toBe(undefined);
  });

  it("should return isSignedIn as true when user is present", async () => {
    const { useAuth: mockUseAuth } = await import("@nfid/identitykit/react");
    const testUser = { principal: "test-principal" };

    vi.mocked(mockUseAuth).mockReturnValue({
      user: testUser as any,
      connect: vi.fn(),
      disconnect: vi.fn(),
      isConnecting: false,
    });

    const { result } = renderHook(() => useAuth());

    expect(result.current.isSignedIn).toBe(true);
    expect(result.current.user).toBe(testUser);
  });

  it("should call connect when signIn is called", async () => {
    const { useAuth: mockUseAuth } = await import("@nfid/identitykit/react");
    const mockConnect = vi.fn();

    vi.mocked(mockUseAuth).mockReturnValue({
      user: undefined,
      connect: mockConnect,
      disconnect: vi.fn(),
      isConnecting: false,
    });

    const { result } = renderHook(() => useAuth());

    await result.current.signIn();

    expect(mockConnect).toHaveBeenCalledOnce();
  });

  it("should call disconnect when signOut is called", async () => {
    const { useAuth: mockUseAuth } = await import("@nfid/identitykit/react");
    const mockDisconnect = vi.fn();

    vi.mocked(mockUseAuth).mockReturnValue({
      user: undefined,
      connect: vi.fn(),
      disconnect: mockDisconnect,
      isConnecting: false,
    });

    const { result } = renderHook(() => useAuth());

    await result.current.signOut();

    expect(mockDisconnect).toHaveBeenCalledOnce();
  });

  it("should handle signIn errors", async () => {
    const { useAuth: mockUseAuth } = await import("@nfid/identitykit/react");
    const consoleErrorSpy = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    const mockConnect = vi
      .fn()
      .mockRejectedValueOnce(new Error("Connection failed"));

    vi.mocked(mockUseAuth).mockReturnValue({
      user: undefined,
      connect: mockConnect,
      disconnect: vi.fn(),
      isConnecting: false,
    });

    const { result } = renderHook(() => useAuth());

    await expect(result.current.signIn()).rejects.toThrow("Connection failed");
    expect(consoleErrorSpy).toHaveBeenCalledWith(
      "Sign in failed:",
      expect.any(Error),
    );

    consoleErrorSpy.mockRestore();
  });

  it("should handle signOut errors", async () => {
    const { useAuth: mockUseAuth } = await import("@nfid/identitykit/react");
    const consoleErrorSpy = vi
      .spyOn(console, "error")
      .mockImplementation(() => {});
    const mockDisconnect = vi
      .fn()
      .mockRejectedValueOnce(new Error("Disconnect failed"));

    vi.mocked(mockUseAuth).mockReturnValue({
      user: undefined,
      connect: vi.fn(),
      disconnect: mockDisconnect,
      isConnecting: false,
    });

    const { result } = renderHook(() => useAuth());

    await expect(result.current.signOut()).rejects.toThrow("Disconnect failed");
    expect(consoleErrorSpy).toHaveBeenCalledWith(
      "Sign out failed:",
      expect.any(Error),
    );

    consoleErrorSpy.mockRestore();
  });
});
