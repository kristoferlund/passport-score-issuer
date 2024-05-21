import { useIssuerBackend } from "../IssuerBackendProvider";
import { useQuery } from "@tanstack/react-query";
import { useInternetIdentity } from "ic-use-internet-identity";

export const useScore = () => {
  const { actor: issuerBackend } = useIssuerBackend();
  const { identity } = useInternetIdentity();
  const principal = identity?.getPrincipal().toText();

  return useQuery({
    queryKey: ["score", principal],
    queryFn: async () => {
      if (!issuerBackend) return null;
      return issuerBackend.score_get();
    },
    enabled: !!issuerBackend && !!principal,
  });
};
