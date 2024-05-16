import { createContext, useContext, useState } from "react";
import { type ReactNode } from "react";
import { PassportContextType } from "./context.type";
import {
  VcFlowRequestWire,
  VcFlowResponse,
  VcGitcoinPassportScoreCredentialSubject,
  VcInternetIdentityIdAliasCredentialSubject,
  VcVerifiableCredential,
  VcVerifiablePresentation,
} from "../vc-api";
import { useInternetIdentity } from "ic-use-internet-identity";
import { jwtDecode } from "jwt-decode";

export const PassportContext = createContext<PassportContextType | undefined>(
  undefined,
);

const PassportCredentialSpec = {
  credentialType: "GitcoinPassportScore",
};

const PassportIssuerOrigin =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.CANISTER_ID_ISSUER}.localhost:4943`
    : `https://${process.env.CANISTER_ID_ISSUER}.icp0.io`;

console.log("PassportIssuerOrigin", PassportIssuerOrigin);

export const usePassportScore = (): PassportContextType => {
  const context = useContext(PassportContext);
  if (!context) {
    throw new Error(
      "usePassportScore must be used within an InternetIdentityProvider",
    );
  }
  return context;
};

export function PassportProvider({ children }: { children: ReactNode }) {
  const { identity } = useInternetIdentity();
  const [passportScore, setPassportScore] = useState<string>();

  async function handleFlowFinished(event: MessageEvent) {
    console.log("handleFlowFinished", event);
    try {
      const vcFlowResponse = VcFlowResponse.parse(event.data);
      if (
        vcFlowResponse.result &&
        "verifiablePresentation" in vcFlowResponse.result
      ) {
        const verifiablePresentation = VcVerifiablePresentation.parse(
          jwtDecode(vcFlowResponse.result.verifiablePresentation),
        );

        const credential1 = VcVerifiableCredential.parse(
          jwtDecode(verifiablePresentation.vp.verifiableCredential[0]),
        );

        const internetIdentityIdAliasCredentialSubject =
          VcInternetIdentityIdAliasCredentialSubject.parse(
            credential1.vc.credentialSubject,
          );

        const credential2 = VcVerifiableCredential.parse(
          jwtDecode(verifiablePresentation.vp.verifiableCredential[1]),
        );

        const gitcoinPassportScoreCredentialSubject =
          VcGitcoinPassportScoreCredentialSubject.parse(
            credential2.vc.credentialSubject,
          );

        console.log(
          "cred2",
          jwtDecode(verifiablePresentation.vp.verifiableCredential[1]),
        );

        setPassportScore(JSON.stringify(gitcoinPassportScoreCredentialSubject));
      }

      console.log("vcFlowResponse", vcFlowResponse);
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
        passportScore,
      }}
    >
      {children}
    </PassportContext.Provider>
  );
}

export * from "./context.type";
