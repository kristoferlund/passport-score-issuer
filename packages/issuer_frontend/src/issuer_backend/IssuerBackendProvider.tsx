import {
  ActorProvider,
  createActorContext,
  createUseActorHook,
} from "ic-use-actor";
import { canisterId, idlFactory } from "../../../issuer_backend/declarations";
import React from "react";
import { _SERVICE } from "../../../issuer_backend/declarations/issuer.did";
import { useInternetIdentity } from "ic-use-internet-identity";

export const actorContext = createActorContext<_SERVICE>();
export const useIssuerBackend = createUseActorHook<_SERVICE>(actorContext);

export default function IssuerBackendProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const { identity } = useInternetIdentity();

  return (
    <ActorProvider<_SERVICE>
      canisterId={canisterId}
      context={actorContext}
      identity={identity}
      idlFactory={idlFactory}
    >
      {children}
    </ActorProvider>
  );
}
