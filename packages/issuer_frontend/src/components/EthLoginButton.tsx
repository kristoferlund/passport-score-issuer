import { useAccount, useConnect, useDisconnect } from "wagmi";

import { injected } from "wagmi/connectors";

export function EthLoginButton() {
  const { connect } = useConnect();
  const { address } = useAccount();
  const { disconnect } = useDisconnect();

  if (address)
    return <button onClick={() => disconnect()}>Disconnect ETH</button>;

  return (
    <button onClick={() => connect({ connector: injected() })}>
      Connect ETH
    </button>
  );
}
