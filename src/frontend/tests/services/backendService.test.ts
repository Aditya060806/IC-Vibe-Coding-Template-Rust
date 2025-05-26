import { describe, it, expect, vi, beforeEach } from "vitest";
import { createBackendService } from "../../src/services/backendService";
import type { _SERVICE } from "../../../declarations/backend/backend.did";

describe("createBackendService", () => {
  let mockAuthenticatedActor: _SERVICE;
  let mockUnauthenticatedActor: _SERVICE;

  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();

    // Create fresh mock actors for each test
    mockAuthenticatedActor = {
      get_count: vi.fn().mockResolvedValue(BigInt(42)),
      increment: vi.fn().mockResolvedValue(BigInt(43)),
      greet: vi.fn(),
      prompt: vi.fn(),
    } as unknown as _SERVICE;

    mockUnauthenticatedActor = {
      greet: vi.fn().mockResolvedValue("Hello, Test User!"),
      prompt: vi.fn().mockResolvedValue("This is a mock LLM response"),
      get_count: vi.fn(),
      increment: vi.fn(),
    } as unknown as _SERVICE;
  });

  describe("greet", () => {
    it("should call unauthenticated actor greet with the provided name", async () => {
      // Setup
      const service = createBackendService(
        mockAuthenticatedActor,
        mockUnauthenticatedActor,
      );

      // Execute
      const result = await service.greet("Test User");

      // Assert
      expect(mockUnauthenticatedActor.greet).toHaveBeenCalledWith("Test User");
      expect(result).toBe("Hello, Test User!");
    });

    it("should throw error when unauthenticated actor is not available", async () => {
      // Setup
      const service = createBackendService(mockAuthenticatedActor, undefined);

      // Execute & Assert
      await expect(service.greet("Test User")).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });

  describe("getCount", () => {
    it("should call authenticated actor get_count", async () => {
      // Setup
      const service = createBackendService(
        mockAuthenticatedActor,
        mockUnauthenticatedActor,
      );

      // Execute
      const result = await service.getCount();

      // Assert
      expect(mockAuthenticatedActor.get_count).toHaveBeenCalled();
      expect(result).toBe(BigInt(42));
    });

    it("should throw error when authenticated actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, mockUnauthenticatedActor);

      // Execute & Assert
      await expect(service.getCount()).rejects.toThrow(
        "Authentication required for counter operations",
      );
    });
  });

  describe("incrementCounter", () => {
    it("should call authenticated actor increment", async () => {
      // Setup
      const service = createBackendService(
        mockAuthenticatedActor,
        mockUnauthenticatedActor,
      );

      // Execute
      const result = await service.incrementCounter();

      // Assert
      expect(mockAuthenticatedActor.increment).toHaveBeenCalled();
      expect(result).toBe(BigInt(43));
    });

    it("should throw error when authenticated actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, mockUnauthenticatedActor);

      // Execute & Assert
      await expect(service.incrementCounter()).rejects.toThrow(
        "Authentication required for counter operations",
      );
    });
  });

  describe("sendLlmPrompt", () => {
    it("should call unauthenticated actor prompt with the provided prompt", async () => {
      // Setup
      const service = createBackendService(
        mockAuthenticatedActor,
        mockUnauthenticatedActor,
      );

      // Execute
      const result = await service.sendLlmPrompt("Test prompt");

      // Assert
      expect(mockUnauthenticatedActor.prompt).toHaveBeenCalledWith(
        "Test prompt",
      );
      expect(result).toBe("This is a mock LLM response");
    });

    it("should throw error when unauthenticated actor is not available", async () => {
      // Setup
      const service = createBackendService(mockAuthenticatedActor, undefined);

      // Execute & Assert
      await expect(service.sendLlmPrompt("Test prompt")).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });
});
