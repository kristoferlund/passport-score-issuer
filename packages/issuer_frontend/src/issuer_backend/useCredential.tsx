import { useIssuerBackend } from "./Actor";
import { useQuery } from "@tanstack/react-query";
import { useInternetIdentity } from "ic-use-internet-identity";

export const useLookupCredential = () => {
  const { actor: issuerBackend } = useIssuerBackend();
  const { identity } = useInternetIdentity();
  const principal = identity?.getPrincipal().toText();

  return useQuery({
    queryKey: ["Credential", principal],
    queryFn: async () => {
      if (!issuerBackend) return null;
      const credential = await issuerBackend.lookup_credential();
      console.log("credential", credential);
      return credential;
    },
    enabled: !!issuerBackend && !!principal,
  });
};
