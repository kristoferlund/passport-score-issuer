import { createContext, useContext, useEffect } from "react";
import { type ReactNode, useState } from "react";
import { PassportContextType } from "./context.type";
import { VcFlowRequestWire } from "../vc-api";
import { useInternetIdentity } from "ic-use-internet-identity";

export const PassportContext = createContext<PassportContextType | undefined>(
  undefined
);

const PassportCredentialSpec = {
  credentialType: "GitcoinPassportScore",
};

const PassportIssuerOrigin =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.CANISTER_ID_issuer_backend}.localhost:4943`
    : `https://${process.env.CANISTER_ID_issuer_backend}.icp0.io`;

export const usePassportScore = (): PassportContextType => {
  const context = useContext(PassportContext);
  if (!context) {
    throw new Error(
      "usePassportScore must be used within an InternetIdentityProvider"
    );
  }
  return context;
};

export function PassportProvider({ children }: { children: ReactNode }) {
  const { identity } = useInternetIdentity();

  async function handleFlowFinished(evnt: MessageEvent) {
    console.log("handleFlowFinished", evnt);
    // TODO: handle the response
  }

  async function handleFlowReady(event: MessageEvent) {
    if (!identity || event.data?.method !== "vc-flow-ready") {
      return;
    }

    const req: VcFlowRequestWire = {
      id: 1,
      jsonrpc: "2.0",
      method: "request_credential",
      params: {
        issuer: {
          origin: PassportIssuerOrigin,
        },
        credentialSpec: PassportCredentialSpec,
        credentialSubject: identity?.getPrincipal().toString(),
      },
    };

    console.log("postMessage", req);

    try {
      window.addEventListener("message", handleFlowFinished);
      event.source?.postMessage(req, { targetOrigin: event.origin });
    } finally {
      window.removeEventListener("message", handleFlowReady);
    }
  }

  async function startVcFlow() {
    const vcFlowUrl = new URL("vc-flow/", process.env.II_URL);
    window.addEventListener("message", handleFlowReady);
    window.open(vcFlowUrl.toString());
  }

  return (
    <PassportContext.Provider
      value={{
        startVcFlow,
      }}
    >
      {children}
    </PassportContext.Provider>
  );
}

export * from "./context.type";
