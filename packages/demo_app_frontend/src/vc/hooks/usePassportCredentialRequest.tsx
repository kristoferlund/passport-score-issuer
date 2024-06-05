import { PassportIssuerOrigin } from "..";
import { VcFlowRequest } from "../types";
import { useInternetIdentity } from "ic-use-internet-identity";

export function usePassportCredentialRequest(
  minScore: number
): VcFlowRequest | undefined {
  const { identity } = useInternetIdentity();

  if (!identity) {
    return undefined;
  }

  return {
    id: 1,
    jsonrpc: "2.0",
    method: "request_credential",
    params: {
      issuer: {
        origin: PassportIssuerOrigin,
      },
      credentialSpec: {
        credentialType: "GitcoinPassportScore",
        arguments: {
          minScore,
        },
      },
      credentialSubject: identity?.getPrincipal().toString(),
    },
  };
}
