import CredentialBox from "./CredentialBox";
import { useAccount } from "wagmi";
import { useCredential } from "../issuer_backend/useCredential";
import { useInternetIdentity } from "ic-use-internet-identity";

export default function CredentialSection() {
  const { address } = useAccount();
  const { identity } = useInternetIdentity();
  const { data: credentialResponse } = useCredential(address, identity);

  return (
    <div className="col" style={{ textAlign: "center" }}>
      {credentialResponse &&
        "Err" in credentialResponse &&
        !address &&
        "This principal has not yet been issued a Gitcoin Passport credential. To issue one, connect your Ethereum wallet. "}
      <h2>Credential – Gitcoin Passport Score</h2>
      <div className="credential">
        {credentialResponse && "Ok" in credentialResponse ? (
          <div className="score">{credentialResponse.Ok.toFixed(2)}</div>
        ) : (
          <CredentialBox />
        )}
      </div>
    </div>
  );
}
