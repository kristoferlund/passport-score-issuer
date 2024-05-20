import { createContext, useContext, useState } from "react";
import { type ReactNode } from "react";
import { VcFlowResponse, VcVerifiablePresentation } from "./types";
import { useInternetIdentity } from "ic-use-internet-identity";
import { jwtDecode } from "jwt-decode";
import { VcVerifiableCredential } from "./types";
import { usePassportCredentialRequest } from "./hooks/usePassportCredentialRequest";

export type VcProviderContextType = {
  startVcFlow: () => Promise<void>;
  credentials?: VcVerifiableCredential[];
};

export const VcContext = createContext<VcProviderContextType | undefined>(
  undefined
);

export const useVcProvider = (): VcProviderContextType => {
  const context = useContext(VcContext);
  if (!context) {
    throw new Error(
      "useVcProvider must be used within an VcProvider component."
    );
  }
  return context;
};

export function VcProvider({ children }: { children: ReactNode }) {
  const { identity } = useInternetIdentity();
  const [credentials, setCredentials] = useState<VcVerifiableCredential[]>();
  const passportCredentialRequest = usePassportCredentialRequest(5);

  /**
   * Handle the VC flow response and set the credentials state.
   */
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

  /**
   * When the VC flow window is ready, send the passport credential request to the VC flow window.
   */
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

  /**
   * Start the VC flow by opening the VC flow URL in a new window.
   */
  async function startVcFlow() {
    const vcFlowUrl = new URL("vc-flow/", process.env.II_URL);
    window.addEventListener("message", handleFlowReady);
    window.open(vcFlowUrl.toString());
  }

  return (
    <VcContext.Provider
      value={{
        startVcFlow,
        credentials,
      }}
    >
      {children}
    </VcContext.Provider>
  );
}
