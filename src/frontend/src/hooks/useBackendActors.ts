import { Actor, HttpAgent } from "@dfinity/agent";
import { useAgent } from "@nfid/identitykit/react";
import { useState, useEffect, useMemo } from "react";
import { idlFactory } from "../../../declarations/backend";
import type { _SERVICE } from "../../../declarations/backend/backend.did";

// Environment configuration
const isDevelopment = process.env.DFX_NETWORK === "local";
const ICP_API_HOST = isDevelopment
  ? "http://localhost:4943"
  : "https://icp-api.io/";

// Get canister ID from environment
const BACKEND_CANISTER_ID =
  process.env.CANISTER_ID_BACKEND || "bkyz2-fmaaa-aaaaa-qaaaq-cai";

// Create unauthenticated agent singleton
let unauthenticatedAgentCache: HttpAgent | null = null;

const getUnauthenticatedAgent = async (): Promise<HttpAgent> => {
  if (!unauthenticatedAgentCache) {
    const agent = await HttpAgent.create({ host: ICP_API_HOST });

    if (isDevelopment) {
      await agent.fetchRootKey();
    }

    unauthenticatedAgentCache = agent;
  }
  return unauthenticatedAgentCache;
};

/**
 * Hook for creating a backend actor that uses authenticated agent when available,
 * otherwise falls back to unauthenticated agent
 */
export function useBackendActors() {
  const [unauthenticatedAgent, setUnauthenticatedAgent] =
    useState<HttpAgent | null>(null);
  const authenticatedAgent = useAgent({ host: ICP_API_HOST });

  // Initialize unauthenticated agent once
  useEffect(() => {
    getUnauthenticatedAgent()
      .then(setUnauthenticatedAgent)
      .catch((error) => {
        console.error("Failed to create unauthenticated agent:", error);
      });
  }, []);

  // Prepare authenticated agent for development if needed
  useEffect(() => {
    if (authenticatedAgent && isDevelopment) {
      authenticatedAgent.fetchRootKey().catch((error) => {
        console.error(
          "Failed to fetch root key for authenticated agent:",
          error,
        );
      });
    }
  }, [authenticatedAgent]);

  // Create backend actor - prefer authenticated agent when available
  const backendActor = useMemo(() => {
    const agent = authenticatedAgent || unauthenticatedAgent;
    if (!agent) return undefined;

    return Actor.createActor<_SERVICE>(idlFactory, {
      agent,
      canisterId: BACKEND_CANISTER_ID,
    });
  }, [authenticatedAgent, unauthenticatedAgent]);

  return {
    backendActor,
    isReady: !!unauthenticatedAgent,
    isAuthenticated: !!authenticatedAgent,
  };
}
