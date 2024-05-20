import { VcVerifiableCredential } from "../vc-api";

export type PassportContextType = {
  startVcFlow: () => Promise<void>;
  credentials?: VcVerifiableCredential[];
};
