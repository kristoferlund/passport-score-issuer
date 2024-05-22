import { IcpLoginButton } from "./components/IcpLoginButton";
import { Toaster } from "react-hot-toast";
import { useInternetIdentity } from "ic-use-internet-identity";
import ScoreSection from "./components/ScoreSection";
import ChainButton from "./components/ChainButton";

function App() {
  const { identity, clear } = useInternetIdentity();

  const principal = identity?.getPrincipal();

  return (
    <main className="col">
      <img
        src="/header.svg"
        alt="The Internet Computer"
        style={{ width: "300px" }}
      />
      <h1>Gitcoin Passport Issuer</h1>
      <div style={{ textAlign: "center" }}>
        This issuer links your{" "}
        <a href="https://passport.gitcoin.co" target="_blank">
          Gitcoin Passport Score
        </a>{" "}
        to your IC identity. Once the link has been established, this service
        can issue{" "}
        <a
          href="https://en.wikipedia.org/wiki/Verifiable_credentials"
          target="_blank"
        >
          verifiable credentials
        </a>{" "}
        that prove your Gitcoin Passport Score to other apps, for instance the{" "}
        <a href="https://jzi4k-7qaaa-aaaal-qdncq-cai.icp0.io" target="_blank">
          VC Flow Demo App
        </a>
        .
      </div>

      <div style={{ textAlign: "center" }}>
        Credentials are shared in a privacy preserving way, without revealing
        the Ethereum address or IC identity used to create it.
      </div>

      <div className="row" style={{ width: "170px" }}>
        {identity && principal && (
          <ChainButton img="/ic.svg" disconnect={clear}>
            {principal.toText().slice(0, 5)}...{principal.toText().slice(-5)}
          </ChainButton>
        )}
        {!identity && <IcpLoginButton />}
      </div>
      {identity && <ScoreSection />}
      <Toaster />

      <div className="links">
        <a
          href="https://github.com/kristoferlund/passport-score-issuer"
          target="_blank"
        >
          <img src="https://img.shields.io/github/license/kristoferlund/passport-score-issuer" />
        </a>

        <a
          href="https://github.com/kristoferlund/passport-score-issuer"
          target="_blank"
        >
          <img src="https://img.shields.io/github/stars/kristoferlund/passport-score-issuer" />
        </a>
        <a href="https://github.com/kristoferlund" target="_blank">
          <img src="https://img.shields.io/github/followers/kristoferlund" />
        </a>
      </div>
    </main>
  );
}

export default App;
