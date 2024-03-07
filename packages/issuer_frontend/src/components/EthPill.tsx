import { getAccount } from "@wagmi/core";
import { useAccount } from "wagmi";
import { useEffect } from "react";

export default function EthPill() {
  const account = useAccount();

  if (!account.address) return null;

  if (account.chainId !== 1)
    return (
      <div className="pill" style={{ fontSize: "12px" }}>
        Wrong network
      </div>
    );

  return (
    <div className="pill">
      {account.address.slice(0, 5)}...{account.address.slice(-5)}
    </div>
  );
}
