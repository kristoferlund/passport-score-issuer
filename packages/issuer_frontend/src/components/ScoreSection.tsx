import { useScore } from "../issuer_backend/hooks/useScore";
import { useAccount } from "wagmi";
import ScoreButton from "./ScoreButton";

export default function ScoreSection() {
  const { address } = useAccount();
  const { data: scoreResponse } = useScore();

  return (
    <div className="col" style={{ textAlign: "center" }}>
      {scoreResponse &&
        "Err" in scoreResponse &&
        !address &&
        "This principal has not yet a linked Gitcoin Passport Score. To link, connect your Ethereum wallet. "}
      <h2>Gitcoin Passport Score</h2>
      <div className="credential">
        {scoreResponse && "Ok" in scoreResponse ? (
          <div className="score">{scoreResponse.Ok.toFixed(2)}</div>
        ) : (
          <div className="score">–</div>
        )}
        <ScoreButton />
      </div>
    </div>
  );
}
