import { IcpLoginButton } from "./components/IcpLoginButton";
import { useInternetIdentity } from "ic-use-internet-identity";
import { usePassportCredentialRequest } from "./vc/hooks/usePassportCredentialRequest";
import { useVcProvider } from "./vc/VcProvider";
import ChainButton from "./components/ChainButton";

function App() {
  const { identity, clear } = useInternetIdentity();
  const { startVcFlow, credentials } = useVcProvider();

  const passportCredentialRequest = usePassportCredentialRequest(1);

  const principal = identity?.getPrincipal();

  return (
    <main className="col">
      <img
        src="/header.svg"
        alt="The Internet Computer"
        style={{ width: "300px" }}
      />
      <h1>VC Flow Demo App</h1>
      <div style={{ textAlign: "center" }}>
        This app allows you to request a{" "}
        <a href="https://passport.gitcoin.co">Gitcoin Passport Score</a>{" "}
        Credential issued by the{" "}
        <a href="https://ycons-daaaa-aaaal-qja3q-cai.icp0.io">
          Gitcoin Passport Issuer for IC
        </a>
        . This application will never have any knowledge of the Ethereum address
        or ICP identity used to create the credential.
      </div>
      <div className="row" style={{ width: "170px" }}>
        {identity && principal && (
          <ChainButton img="/ic.svg" disconnect={clear}>
            {principal.toText().slice(0, 5)}...{principal.toText().slice(-5)}
          </ChainButton>
        )}
        {!identity && <IcpLoginButton />}
      </div>

      {identity && (
        <div className="col">
          <h2>Credential Request</h2>
          <div style={{ textAlign: "center" }}>
            We will request a credential proving that the user has a Passport
            score of at least 1.
          </div>
          <code>
            <pre>{JSON.stringify(passportCredentialRequest, null, 2)}</pre>
          </code>
          <button
            onClick={startVcFlow}
            style={{ display: "block", width: "170px" }}
          >
            Send request
          </button>
        </div>
      )}
      {identity && credentials && (
        <div className="col">
          <h2>Received Credentials</h2>
          {credentials.map((vc) => (
            <div key={vc.exp}>
              <code>
                <pre>{JSON.stringify(vc, null, 2)}</pre>
              </code>
            </div>
          ))}
        </div>
      )}
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
