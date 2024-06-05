import {
  ActorProvider,
  createActorContext,
  createUseActorHook,
} from "ic-use-actor";
import { canisterId, idlFactory } from "../../../demo_app_backend/declarations";

import React from "react";
import { _SERVICE } from "../../../demo_app_backend/declarations/demo_app.did";
import { useInternetIdentity } from "ic-use-internet-identity";

export const actorContext = createActorContext<_SERVICE>();
export const useDemoAppBackend = createUseActorHook<_SERVICE>(actorContext);

export default function DemoAppBackendProvider({
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
