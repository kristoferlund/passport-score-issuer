export type PassportContextType = {
  startVcFlow: () => Promise<void>;
  passportScore?: string;
};
