import "./styles/window.scss";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createConfig, http } from "wagmi";
import { injected, walletConnect } from "wagmi/connectors";

import App from "./App";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import IssuerBackendProvider from "./issuer_backend/Actor";
import React from "react";
import ReactDOM from "react-dom/client";
import { WagmiProvider } from "wagmi";
import { mainnet } from "wagmi/chains";

const WALLETCONNECT_PROJECT_ID = "72b848352694de83eb0eb8505384c308";
export const wagmiConfig = createConfig({
  chains: [mainnet],
  connectors: [
    injected(),
    walletConnect({ projectId: WALLETCONNECT_PROJECT_ID }),
  ],
  transports: {
    [mainnet.id]: http(),
  },
});

export const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <InternetIdentityProvider>
      <IssuerBackendProvider>
        <WagmiProvider config={wagmiConfig}>
          <QueryClientProvider client={queryClient}>
            <App />
          </QueryClientProvider>
        </WagmiProvider>
      </IssuerBackendProvider>
    </InternetIdentityProvider>
  </React.StrictMode>
);
