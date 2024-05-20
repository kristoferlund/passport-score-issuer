import { IcpLoginButton } from "./components/IcpLoginButton";
import IcpPill from "./components/IcpPill";
import { useInternetIdentity } from "ic-use-internet-identity";
import { usePassportCredentialRequest } from "./vc/hooks/usePassportCredentialRequest";
import { useVcProvider } from "./vc/VcProvider";

function App() {
  const { identity } = useInternetIdentity();
  const { startVcFlow, credentials } = useVcProvider();

  const passportCredentialRequest = usePassportCredentialRequest(5);

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
          Gitcoin Passport Issuer for ICP
        </a>
        . This application will never have any knowledge of the Ethereum address
        or ICP identity used to create the credential.
      </div>
      <div className="row">
        <IcpPill />
        <IcpLoginButton />
      </div>

      {identity && (
        <div className="col">
          <h2>Credential Request</h2>
          <div style={{ textAlign: "center" }}>
            We will request a credential proving that the user hasa a Passport
            score of at least 5.
          </div>
          <code>
            <pre>{JSON.stringify(passportCredentialRequest, null, 2)}</pre>
          </code>
          <button onClick={startVcFlow} style={{ display: "block" }}>
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
    </main>
  );
}

export default App;
