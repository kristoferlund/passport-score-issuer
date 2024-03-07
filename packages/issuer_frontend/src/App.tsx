import CredentialSection from "./components/CredentialSection";
import { EthLoginButton } from "./components/EthLoginButton";
import EthPill from "./components/EthPill";
import { IcpLoginButton } from "./components/IcpLoginButton";
import IcpPill from "./components/IcpPill";
import { Toaster } from "react-hot-toast";
import { useAccount } from "wagmi";
import { useInternetIdentity } from "ic-use-internet-identity";

function App() {
  const { identity } = useInternetIdentity();
  const { address } = useAccount();

  return (
    <main className="col">
      <img
        src="/header.svg"
        alt="The Internet Computer"
        style={{ width: "300px" }}
      />
      <h1>Gitcoin Passport issuer for ICP</h1>
      <div style={{ textAlign: "center" }}>
        This issuer links your Ethereum address and Gitcoin Passport Score to
        your ICP identity. The issued credential can be used to prove your
        Gitcoin Passport Score to other ICP services in a secure way, without
        revealing your Ethereum address.
      </div>
      {!identity && !address && (
        <div style={{ textAlign: "center", fontWeight: "700" }}>
          Login to ICP and connect to ETH to view or create a passport
          credential.
        </div>
      )}

      <div className="row">
        <IcpPill />
        <IcpLoginButton />
      </div>
      <div className="row">
        <EthPill />
        <EthLoginButton />
      </div>
      {identity && address && <CredentialSection />}
      <Toaster />
    </main>
  );
}

export default App;
