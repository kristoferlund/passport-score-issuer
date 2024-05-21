import { useIssuerBackend } from "../IssuerBackendProvider";
import { useMutation } from "@tanstack/react-query";

export const useLinkScore = () => {
  const { actor: issuerBackend } = useIssuerBackend();
  return useMutation({
    mutationFn: ({
      address,
      signature,
    }: {
      address: string;
      signature: string;
    }) => {
      if (!issuerBackend) throw new Error("Issuer backend not available");
      return issuerBackend.score_link(signature, address);
    },
  });
};
