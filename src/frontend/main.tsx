import React from "react";
import ReactDOM from "react-dom/client";
import App from "./src/App";
import { IdentityKitProvider } from "@nfid/identitykit/react";
import { IdentityKitSignerConfig, InternetIdentity } from "@nfid/identitykit";
import "./src/index.css";

// Initialize IdentityKit with Internet Identity and Mocked Signer
const localInternetIdentity: IdentityKitSignerConfig = {
  ...InternetIdentity,
  providerUrl: process.env.INTERNET_IDENTITY_URL || "https://identity.ic0.app",
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <IdentityKitProvider
      signers={
        process.env.DFX_NETWORK === "local"
          ? [localInternetIdentity]
          : undefined
      }
      onConnectSuccess={() => console.log("Successfully connected to wallet")}
      onConnectFailure={(error: Error) =>
        console.error("Failed to connect:", error)
      }
      onDisconnect={() => console.log("Disconnected from wallet")}
    >
      <App />
    </IdentityKitProvider>
  </React.StrictMode>,
);
