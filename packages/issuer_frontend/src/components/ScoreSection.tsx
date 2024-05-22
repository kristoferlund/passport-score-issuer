import { useScore } from "../issuer_backend/hooks/useScore";
import { useAccount, useDisconnect } from "wagmi";
import ScoreButton from "./ScoreButton";
import { EthLoginButton } from "./EthLoginButton";
import ChainButton from "./ChainButton";

export default function ScoreSection() {
  const { address } = useAccount();
  const { data: scoreResponse } = useScore();
  const { disconnect } = useDisconnect();

  return (
    <div className="col" style={{ textAlign: "center" }}>
      <h2>Linked Gitcoin Passport Score</h2>
      <div className="credential">
        {scoreResponse && "Ok" in scoreResponse ? (
          <div className="score">{scoreResponse.Ok.toFixed(2)}</div>
        ) : (
          <div className="score">–</div>
        )}
        {address && (
          <ChainButton img="/eth.svg" disconnect={disconnect}>
            {address.slice(0, 5)}...{address.slice(-5)}
          </ChainButton>
        )}
      </div>
      {scoreResponse &&
        "Err" in scoreResponse &&
        !address &&
        "This IC identity has not yet a linked Gitcoin Passport Score. To link, connect your Ethereum wallet. "}
      {scoreResponse &&
        "Err" in scoreResponse &&
        address &&
        "This IC identity has not yet a linked Gitcoin Passport Score."}
      {scoreResponse &&
        "Ok" in scoreResponse &&
        !address &&
        "To refresh the score, connect your Ethereum wallet."}
      {!address && (
        <div style={{ width: "200px" }}>
          <EthLoginButton />
        </div>
      )}
      {address && <ScoreButton />}
    </div>
  );
}
