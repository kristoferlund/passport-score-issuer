import { useAccount, useSignMessage } from "wagmi";

import { Principal } from "@dfinity/principal";
import { queryClient } from "../main";
import toast from "react-hot-toast";
import { useCreateOrRefreshCredential } from "../issuer_backend/useCreateCredential";
import { useEffect } from "react";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useLookupCredential } from "../issuer_backend/useCredential";

function createRegisterMessage(address: `0x${string}`, principal: Principal) {
  return `Sign this message to link your Ethereum address to your Internet Computer identity.\n\nEthereum address: ${address}\n\nInternet Computer principal: ${principal.toText()}`;
}

export default function CredentialButton() {
  const { address } = useAccount();
  const { identity } = useInternetIdentity();
  const { signMessage, isPending: isSignaturePending } = useSignMessage();
  const {
    mutate: createCredential,
    isPending: isCreatePending,
    isSuccess,
    isError,
    error,
    data,
  } = useCreateOrRefreshCredential();
  const { data: credentialResponse } = useLookupCredential();

  const isCreating = isSignaturePending || isCreatePending;

  const buttonText = () => {
    if (credentialResponse && "Ok" in credentialResponse) {
      if (isCreating) return "Refreshing …";
      return "Refresh";
    }
    if (isCreating) return "Creating …";
    return "Create link";
  };

  useEffect(() => {
    if (!isSuccess) return;
    if ("Err" in data) {
      toast.error("Error creating credential: " + data.Err);
      return;
    }
    toast.success("Credential created");
    queryClient.invalidateQueries();
  }, [isSuccess]);

  useEffect(() => {
    if (!isError) return;
    toast.error(`Error creating credential: ${error?.message}`);
  }, [isError, error]);

  const register = async () => {
    if (!address || !identity) return;
    const message = createRegisterMessage(address, identity.getPrincipal());
    signMessage(
      { message },
      {
        onSuccess(signature) {
          createCredential({ address, signature });
        },
      }
    );
  };

  return (
    <div className="col">
      <button onClick={register}>{buttonText()}</button>
    </div>
  );
}
