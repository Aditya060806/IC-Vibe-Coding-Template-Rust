import React from "react";
import ReactDOM from "react-dom/client";
import { IdentityKitProvider } from "@nfid/identitykit/react";
import App from "./src/App";
import "./src/index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <IdentityKitProvider
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
