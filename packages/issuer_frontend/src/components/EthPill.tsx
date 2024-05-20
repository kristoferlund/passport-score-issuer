import { useAccount } from "wagmi";

export default function EthPill() {
  const account = useAccount();

  if (!account.address) return null;

  return (
    <div className="pill">
      {account.address.slice(0, 5)}...{account.address.slice(-5)}
    </div>
  );
}
