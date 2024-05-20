import CredentialButton from "./CredentialButton";
import { useAccount } from "wagmi";
import { useLookupCredential } from "../issuer_backend/useCredential";

export default function CredentialSection() {
  const { address } = useAccount();
  const { data: credentialResponse } = useLookupCredential();

  return (
    <div className="col" style={{ textAlign: "center" }}>
      {credentialResponse &&
        "Err" in credentialResponse &&
        !address &&
        "This principal has not yet a linked Gitcoin Passport Score. To link, connect your Ethereum wallet. "}
      <h2>Gitcoin Passport Score</h2>
      <div className="credential">
        {credentialResponse && "Ok" in credentialResponse ? (
          <div className="score">{credentialResponse.Ok.toFixed(2)}</div>
        ) : (
          <div className="score">–</div>
        )}
        <CredentialButton />
      </div>
    </div>
  );
}
