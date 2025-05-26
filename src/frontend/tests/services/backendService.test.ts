import { describe, it, expect, vi, beforeEach } from "vitest";
import { createBackendService } from "../../src/services/backendService";
import type { _SERVICE } from "../../../declarations/backend/backend.did";

describe("createBackendService", () => {
  let mockBackendActor: _SERVICE;

  beforeEach(() => {
    // Clear all mocks before each test
    vi.clearAllMocks();

    // Create fresh mock actor for each test
    mockBackendActor = {
      get_count: vi.fn().mockResolvedValue(BigInt(42)),
      increment: vi.fn().mockResolvedValue(BigInt(43)),
      greet: vi.fn().mockResolvedValue("Hello, Test User!"),
      prompt: vi.fn().mockResolvedValue("This is a mock LLM response"),
    } as unknown as _SERVICE;
  });

  describe("greet", () => {
    it("should call backend actor greet with the provided name", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, false);

      // Execute
      const result = await service.greet("Test User");

      // Assert
      expect(mockBackendActor.greet).toHaveBeenCalledWith("Test User");
      expect(result).toBe("Hello, Test User!");
    });

    it("should call backend actor greet when authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, true);

      // Execute
      const result = await service.greet("Test User");

      // Assert
      expect(mockBackendActor.greet).toHaveBeenCalledWith("Test User");
      expect(result).toBe("Hello, Test User!");
    });

    it("should throw error when backend actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, false);

      // Execute & Assert
      await expect(service.greet("Test User")).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });

  describe("getCount", () => {
    it("should call backend actor get_count when authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, true);

      // Execute
      const result = await service.getCount();

      // Assert
      expect(mockBackendActor.get_count).toHaveBeenCalled();
      expect(result).toBe(BigInt(42));
    });

    it("should throw error when not authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, false);

      // Execute & Assert
      await expect(service.getCount()).rejects.toThrow(
        "Authentication required for counter operations",
      );
    });

    it("should throw error when backend actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, true);

      // Execute & Assert
      await expect(service.getCount()).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });

  describe("incrementCounter", () => {
    it("should call backend actor increment when authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, true);

      // Execute
      const result = await service.incrementCounter();

      // Assert
      expect(mockBackendActor.increment).toHaveBeenCalled();
      expect(result).toBe(BigInt(43));
    });

    it("should throw error when not authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, false);

      // Execute & Assert
      await expect(service.incrementCounter()).rejects.toThrow(
        "Authentication required for counter operations",
      );
    });

    it("should throw error when backend actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, true);

      // Execute & Assert
      await expect(service.incrementCounter()).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });

  describe("sendLlmPrompt", () => {
    it("should call backend actor prompt with the provided prompt", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, false);

      // Execute
      const result = await service.sendLlmPrompt("Test prompt");

      // Assert
      expect(mockBackendActor.prompt).toHaveBeenCalledWith("Test prompt");
      expect(result).toBe("This is a mock LLM response");
    });

    it("should call backend actor prompt when authenticated", async () => {
      // Setup
      const service = createBackendService(mockBackendActor, true);

      // Execute
      const result = await service.sendLlmPrompt("Test prompt");

      // Assert
      expect(mockBackendActor.prompt).toHaveBeenCalledWith("Test prompt");
      expect(result).toBe("This is a mock LLM response");
    });

    it("should throw error when backend actor is not available", async () => {
      // Setup
      const service = createBackendService(undefined, false);

      // Execute & Assert
      await expect(service.sendLlmPrompt("Test prompt")).rejects.toThrow(
        "Backend service not ready",
      );
    });
  });
});
