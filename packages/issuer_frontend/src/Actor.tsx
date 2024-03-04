import {
  ActorProvider,
  createActorContext,
  createUseActorHook,
} from "ic-use-actor";
import {
  canisterId,
  idlFactory,
} from "../../../declarations/issuer_backend/index";

import { PassportProvider } from "./passport/PassportProvider";
import React from "react";
import type { _SERVICE } from "../../../declarations/issuer_backend/issuer_backend.did";
import { useInternetIdentity } from "ic-use-internet-identity";

const actorContext = createActorContext<_SERVICE>();
export const useBackend = createUseActorHook<_SERVICE>(actorContext);

export default function Actor({ children }: { children: React.ReactNode }) {
  const { identity } = useInternetIdentity();

  return (
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
    >
      <PassportProvider>{children}</PassportProvider>
    </ActorProvider>
  );
}
