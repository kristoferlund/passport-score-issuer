import { useAccount, useConnect, useDisconnect, useSwitchChain } from "wagmi";

import { injected } from "wagmi/connectors";

export function EthLoginButton() {
  const { connect } = useConnect();
  const account = useAccount();
  const { disconnect } = useDisconnect();
  const { switchChain } = useSwitchChain();

  if (account.chainId !== 1) {
    return (
      <button
        onClick={() => switchChain({ chainId: 1 })}
        style={{ fontSize: "12px" }}
      >
        Switch to Mainnet
      </button>
    );
  }

  if (account.address) {
    return <button onClick={() => disconnect()}>Disconnect ETH</button>;
  }

  return (
    <button onClick={() => connect({ connector: injected() })}>
      Connect ETH
    </button>
  );
}
