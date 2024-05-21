import { useAccount, useSignMessage } from "wagmi";

import { Principal } from "@dfinity/principal";
import { queryClient } from "../main";
import toast from "react-hot-toast";
import { useEffect } from "react";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useLinkScore } from "../issuer_backend/hooks/useLinkScore";
import { useScore } from "../issuer_backend/hooks/useScore";

function createLinkSignatureMessage(
  address: `0x${string}`,
  principal: Principal,
) {
  return `Sign this message to link your Ethereum address to your Internet Computer identity.\n\nEthereum address: ${address}\n\nInternet Computer principal: ${principal.toText()}`;
}

export default function ScoreButton() {
  const { address } = useAccount();
  const { identity } = useInternetIdentity();
  const { signMessage, isPending: isSignaturePending } = useSignMessage();
  const {
    mutate: linkScore,
    isPending: isLinkPending,
    isSuccess,
    isError,
    error,
    data,
  } = useLinkScore();
  const { data: scoreResponse } = useScore();

  const isCreating = isSignaturePending || isLinkPending;

  const buttonText = () => {
    if (scoreResponse && "Ok" in scoreResponse) {
      if (isCreating) return "Refreshing …";
      return "Refresh";
    }
    if (isCreating) return "Creating …";
    return "Create link";
  };

  useEffect(() => {
    if (!isSuccess) return;
    if ("Err" in data) {
      toast.error("Error linking score: " + data.Err);
      return;
    }
    toast.success("Score linked");
    queryClient.invalidateQueries();
  }, [isSuccess]);

  useEffect(() => {
    if (!isError) return;
    toast.error(`Error linking score: ${error?.message}`);
  }, [isError, error]);

  const register = async () => {
    if (!address || !identity) return;
    const message = createLinkSignatureMessage(
      address,
      identity.getPrincipal(),
    );
    signMessage(
      { message },
      {
        onSuccess(signature) {
          linkScore({ address, signature });
        },
      },
    );
  };

  return (
    <div className="col">
      <button onClick={register}>{buttonText()}</button>
    </div>
  );
}
