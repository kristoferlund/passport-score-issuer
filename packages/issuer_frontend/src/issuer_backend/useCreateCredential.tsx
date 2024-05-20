import { useIssuerBackend } from "./Actor";
import { useMutation } from "@tanstack/react-query";

export const useCreateOrRefreshCredential = () => {
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
      return issuerBackend.create_or_refresh_credential(signature, address);
    },
  });
};
