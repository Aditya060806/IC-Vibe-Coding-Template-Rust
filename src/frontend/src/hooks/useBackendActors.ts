import { Actor, HttpAgent } from "@dfinity/agent";
import { useAgent } from "@nfid/identitykit/react";
import { useEffect, useState, useMemo } from "react";
import { idlFactory } from "../../../declarations/backend";
import type { _SERVICE } from "../../../declarations/backend/backend.did";

// Environment configuration
const isDevelopment = process.env.NODE_ENV === "development";
const ICP_API_HOST = isDevelopment
  ? "http://localhost:4943"
  : "https://icp-api.io/";

// Get canister ID from environment
const BACKEND_CANISTER_ID =
  process.env.CANISTER_ID_BACKEND || "bkyz2-fmaaa-aaaaa-qaaaq-cai";

/**
 * Hook for creating a backend actor that uses authenticated agent when available,
 * otherwise falls back to unauthenticated agent
 */
export function useBackendActors() {
  const [unauthenticatedAgent, setUnauthenticatedAgent] = useState<
    HttpAgent | undefined
  >();
  const authenticatedAgent = useAgent();

  // Create unauthenticated agent for public calls
  useEffect(() => {
    const createUnauthenticatedAgent = async () => {
      try {
        const agent = await HttpAgent.create({ host: ICP_API_HOST });

        // Fetch root key for local development
        if (isDevelopment) {
          await agent.fetchRootKey();
        }

        setUnauthenticatedAgent(agent);
      } catch (error) {
        console.error("Failed to create unauthenticated agent:", error);
      }
    };

    createUnauthenticatedAgent();
  }, []);

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
