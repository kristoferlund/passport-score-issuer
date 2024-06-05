import { useInternetIdentity } from "ic-use-internet-identity";

export default function IcpPill() {
  const { identity } = useInternetIdentity();

  const principal = identity?.getPrincipal();

  if (!principal) return null;

  return (
    <div className="chain-button">
      {principal.toText().slice(0, 5)}...{principal.toText().slice(-5)}
    </div>
  );
}
