import { useInternetIdentity } from "ic-use-internet-identity";

export function IcpLoginButton() {
  const { identity, login, loginStatus, clear, isInitializing } =
    useInternetIdentity();

  if (isInitializing) return null;

  const disabled = loginStatus === "logging-in" || loginStatus === "success";
  const text = loginStatus === "logging-in" ? "Logging in..." : "Login ICP";

  if (identity) return <button onClick={clear}>Logout ICP</button>;

  return (
    <button onClick={login} disabled={disabled}>
      {text}
    </button>
  );
}
