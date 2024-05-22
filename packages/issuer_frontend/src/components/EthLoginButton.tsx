import { useAccount, useConnect, useDisconnect } from "wagmi";

import { injected } from "wagmi/connectors";

export function EthLoginButton() {
  const { connect } = useConnect();
  const account = useAccount();
  const { disconnect } = useDisconnect();

  if (account.address) {
    return <button onClick={() => disconnect()}>Disconnect ETH</button>;
  }

  return (
    <button onClick={() => connect({ connector: injected() })}>
      <img src="/eth.svg" alt="ETH" />
      Connect wallet
    </button>
  );
}
