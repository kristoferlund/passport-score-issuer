export const PassportIssuerOrigin =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.CANISTER_ID_ISSUER}.localhost:4943`
    : `https://${process.env.CANISTER_ID_ISSUER}.icp0.io`;
