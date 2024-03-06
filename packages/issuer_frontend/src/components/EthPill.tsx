import { useAccount } from "wagmi";

export default function EthPill() {
  const { address } = useAccount();

  if (!address) return null;

  return (
    <div className="pill">
      {address.slice(0, 5)}...{address.slice(-5)}
    </div>
  );
}
