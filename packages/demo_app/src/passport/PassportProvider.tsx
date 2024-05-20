import { createContext, useContext, useState } from "react";
import { type ReactNode } from "react";
import { PassportContextType } from "./context.type";
import {
  PassportIssuerOrigin,
  VcFlowRequestWire,
  VcFlowResponse,
  VcVerifiableCredential,
  VcVerifiablePresentation,
  getPassportCredentialSpec,
  usePassportCredentialRequest,
} from "../vc-api";
import { useInternetIdentity } from "ic-use-internet-identity";
import { jwtDecode } from "jwt-decode";
import { z } from "zod";

export const PassportContext = createContext<PassportContextType | undefined>(
  undefined
);

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
  const [credentials, setCredentials] = useState<VcVerifiableCredential[]>();
  const passportCredentialRequest = usePassportCredentialRequest(5);

  async function handleFlowFinished(event: MessageEvent) {
    try {
      const vcFlowResponse = VcFlowResponse.parse(event.data);

      if (
        vcFlowResponse.result &&
        "verifiablePresentation" in vcFlowResponse.result
      ) {
        const verifiablePresentation = VcVerifiablePresentation.parse(
          jwtDecode(vcFlowResponse.result.verifiablePresentation)
        );

        const credentials = verifiablePresentation.vp.verifiableCredential.map(
          (vc) => VcVerifiableCredential.parse(jwtDecode(vc))
        );

        setCredentials(credentials);
      }

      if (
        event.source &&
        "close" in event.source &&
        typeof event.source.close === "function"
      ) {
        event.source.close();
      }
      window.removeEventListener("message", handleFlowFinished);
    } catch (e) {
      // Not a VC response
    }
  }

  async function handleFlowReady(event: MessageEvent) {
    if (!identity || event.data?.method !== "vc-flow-ready") {
      return;
    }

    try {
      window.addEventListener("message", handleFlowFinished);
      event.source?.postMessage(passportCredentialRequest, {
        targetOrigin: event.origin,
      });
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
        credentials,
      }}
    >
      {children}
    </PassportContext.Provider>
  );
}

export * from "./context.type";
