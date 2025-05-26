import type { _SERVICE } from "../../../declarations/backend/backend.did";

/**
 * Service for handling all backend canister API calls
 * This service now supports both authenticated and unauthenticated calls
 */
export const createBackendService = (
  authenticatedActor?: _SERVICE,
  unauthenticatedActor?: _SERVICE,
) => ({
  /**
   * Sends a greeting to the backend and returns the response
   * Uses unauthenticated actor as this is a public call
   * @param name Name to greet
   * @returns Promise with the greeting response
   */
  async greet(name: string): Promise<string> {
    if (!unauthenticatedActor) {
      throw new Error("Backend service not ready");
    }
    return await unauthenticatedActor.greet(name || "World");
  },

  /**
   * Fetches the current counter value for the authenticated user
   * Uses authenticated actor as this is user-specific
   * @returns Promise with the current count
   */
  async getCount(): Promise<bigint> {
    if (!authenticatedActor) {
      throw new Error("Authentication required for counter operations");
    }
    return await authenticatedActor.get_count();
  },

  /**
   * Increments the counter for the authenticated user
   * Uses authenticated actor as this is user-specific
   * @returns Promise with the new count
   */
  async incrementCounter(): Promise<bigint> {
    if (!authenticatedActor) {
      throw new Error("Authentication required for counter operations");
    }
    return await authenticatedActor.increment();
  },

  /**
   * Sends a prompt to the LLM backend
   * Uses unauthenticated actor as this is a public call
   * @param prompt The user's prompt text
   * @returns Promise with the LLM response
   */
  async sendLlmPrompt(prompt: string): Promise<string> {
    if (!unauthenticatedActor) {
      throw new Error("Backend service not ready");
    }
    return await unauthenticatedActor.prompt(prompt);
  },
});

// Legacy export for backward compatibility
// This will be removed after updating all components
export const backendService = {
  async greet(_name: string): Promise<string> {
    throw new Error(
      "Legacy backend service no longer supported. Use authenticated service.",
    );
  },
  async getCount(): Promise<bigint> {
    throw new Error(
      "Legacy backend service no longer supported. Use authenticated service.",
    );
  },
  async incrementCounter(): Promise<bigint> {
    throw new Error(
      "Legacy backend service no longer supported. Use authenticated service.",
    );
  },
  async sendLlmPrompt(_prompt: string): Promise<string> {
    throw new Error(
      "Legacy backend service no longer supported. Use authenticated service.",
    );
  },
};
