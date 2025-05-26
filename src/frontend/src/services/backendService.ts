import type { _SERVICE } from "../../../declarations/backend/backend.did";

/**
 * Service for handling all backend canister API calls
 * This service accepts a single backend actor and uses isAuthenticated flag for auth-required operations
 */
export const createBackendService = (
  backendActor?: _SERVICE,
  isAuthenticated?: boolean,
) => ({
  /**
   * Sends a greeting to the backend and returns the response
   * This is a public call that doesn't require authentication
   * @param name Name to greet
   * @returns Promise with the greeting response
   */
  async greet(name: string): Promise<string> {
    if (!backendActor) {
      throw new Error("Backend service not ready");
    }
    return await backendActor.greet(name || "World");
  },

  /**
   * Fetches the current counter value for the authenticated user
   * Requires authentication
   * @returns Promise with the current count
   */
  async getCount(): Promise<bigint> {
    if (!backendActor) {
      throw new Error("Backend service not ready");
    }
    if (!isAuthenticated) {
      throw new Error("Authentication required for counter operations");
    }
    return await backendActor.get_count();
  },

  /**
   * Increments the counter for the authenticated user
   * Requires authentication
   * @returns Promise with the new count
   */
  async incrementCounter(): Promise<bigint> {
    if (!backendActor) {
      throw new Error("Backend service not ready");
    }
    if (!isAuthenticated) {
      throw new Error("Authentication required for counter operations");
    }
    return await backendActor.increment();
  },

  /**
   * Sends a prompt to the LLM backend
   * This is a public call that doesn't require authentication
   * @param prompt The user's prompt text
   * @returns Promise with the LLM response
   */
  async sendLlmPrompt(prompt: string): Promise<string> {
    if (!backendActor) {
      throw new Error("Backend service not ready");
    }
    return await backendActor.prompt(prompt);
  },
});
