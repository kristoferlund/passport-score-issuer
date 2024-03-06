import { Identity } from "@dfinity/agent";
import { useIssuerBackend } from "./Actor";
import { useQuery } from "@tanstack/react-query";

export const useCredential = (address?: string, identity?: Identity) => {
  const { actor: issuerBackend } = useIssuerBackend();
  const principal = identity?.getPrincipal().toText();
  return useQuery({
    queryKey: ["Credential", address, principal],
    queryFn: async () => {
      if (!issuerBackend || !address) return null;
      const credential = await issuerBackend.lookup_credential(address);
      return credential;
    },
    enabled: !!issuerBackend && !!address && !!principal,
  });
};
