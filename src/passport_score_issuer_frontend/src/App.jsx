import { LoginButton } from "./components/LoginButton";
import { useInternetIdentity } from "ic-use-internet-identity";
import { usePassportScore } from "./passport/PassportProvider";

function App() {
  const { identity } = useInternetIdentity();
  const { startVcFlow } = usePassportScore();

  return (
    <main
      style={{
        width: "100%",
        display: "flex",
        flexDirection: "column",
        gap: "20px",
        alignItems: "center",
        fontFamily: "sans-serif",
      }}
    >
      <img src="/logo2.svg" alt="DFINITY logo" style={{ width: "300px" }} />
      {identity ? (
        <>
          You are logged in as: {identity.getPrincipal().toText().slice(0, 5)}
          ...{identity.getPrincipal().toText().slice(-5)}
          <button onClick={startVcFlow} style={{ display: "block" }}>
            Issue Passport
          </button>
        </>
      ) : (
        <LoginButton />
      )}
    </main>
  );
}

export default App;
