import { Principal } from "@dfinity/principal";
import { isNullish } from "@dfinity/utils";
import { z } from "zod";
import { ArgumentValue } from "./generated/vc_issuer_types";
import { useInternetIdentity } from "ic-use-internet-identity";

export type * from "./generated/vc_issuer_types";

// The type of messages that kick start the flow (II -> RP)
export const VcFlowReady = {
  jsonrpc: "2.0",
  method: "vc-flow-ready",
};

const zodPrincipal = z.string().transform((val, ctx) => {
  let principal;
  try {
    principal = Principal.fromText(val);
  } catch {
    ctx.addIssue({ code: z.ZodIssueCode.custom, message: "Not a principal " });
    return z.NEVER;
  }

  return principal;
});

// Parse & validate a CredentialSpec
const zodCredentialSpec = z
  /* The input object */
  .object({
    credentialType: z.string(),
    arguments: z.optional(
      z.record(z.string(), z.union([z.string(), z.number()]))
    ),
  })
  /* Transform to make the type easier to use:
   *    - transform the arguments to whatever the did spec expects
   *    - work around the JS type gen weirdness for optionals
   * XXX: TS needs the type annotation or it gets slightly confused
   */
  .transform<{
    credential_type: string;
    arguments: [] | [Array<[string, ArgumentValue]>];
  }>(({ credentialType, arguments: args }) => ({
    credential_type: credentialType,
    arguments: isNullish(args) ? [] : [fixupArgs(args)],
  }));

/* Convert the JSON map/record into what the did spec expects */
const fixupArgs = (
  arg: Record<string, string | number>
): Array<[string, ArgumentValue]> => {
  return Object.entries(arg).map(([k, v]) => [
    k,
    typeof v === "string" ? { String: v } : { Int: v },
  ]);
};

export const VcFlowRequest = z.object({
  id: z.union([
    z.number(),
    z.string(),
  ]) /* Slightly lax; in principle jsonrpc does not allow fractional numbers as id */,
  jsonrpc: z.literal("2.0"),
  method: z.literal("request_credential"),
  params: z.object({
    issuer: z.object({
      origin: z
        .string()
        .url() /* XXX: we limit to URLs, but in practice should even be an origin */,
      canisterId: z.optional(zodPrincipal),
    }),
    credentialSpec: zodCredentialSpec,
    credentialSubject: zodPrincipal,
    derivationOrigin: z.optional(z.string()),
  }),
});

// The wire format of a VC flow request
export type VcFlowRequestWire = z.input<typeof VcFlowRequest>;
export type VcFlowRequest = z.output<typeof VcFlowRequest>;

export const VcFlowResponse = z.object({
  id: z.union([z.number(), z.string()]),
  jsonrpc: z.literal("2.0"),
  result: z
    .object({
      verifiablePresentation: z.string(),
    })
    .or(
      z.object({
        error: z.object({
          version: z.literal("1"),
          code: z.string(),
        }),
      })
    ),
});

export const VcVerifiablePresentation = z.object({
  iss: z.string(),
  vp: z.object({
    "@context": z.literal("https://www.w3.org/2018/credentials/v1"),
    type: z.literal("VerifiablePresentation"),
    verifiableCredential: z.array(z.string()),
  }),
});

export const VcInternetIdentityIdAliasCredentialSubject = z.object({
  InternetIdentityIdAlias: z.object({
    hasIdAlias: z.string(),
  }),
});

export const VcGitcoinPassportScoreCredentialSubject = z.object({
  GitcoinPassportScore: z.object({
    minScore: z.number(),
  }),
});

export const VcVerifiableCredential = z.object({
  exp: z.number(),
  iss: z.string(),
  nbf: z.number(),
  jti: z.string(),
  sub: z.string(),
  vc: z.object({
    "@context": z.literal("https://www.w3.org/2018/credentials/v1"),
    type: z.array(z.string()),
    credentialSubject: z.union([
      VcInternetIdentityIdAliasCredentialSubject,
      VcGitcoinPassportScoreCredentialSubject,
    ]),
  }),
});
export type VcVerifiableCredential = z.infer<typeof VcVerifiableCredential>;

export function getPassportCredentialSpec(minScore: number) {
  return {
    credentialType: "GitcoinPassportScore",
    arguments: {
      minScore,
    },
  };
}

export const PassportIssuerOrigin =
  process.env.DFX_NETWORK === "local"
    ? `http://${process.env.CANISTER_ID_ISSUER}.localhost:4943`
    : `https://${process.env.CANISTER_ID_ISSUER}.icp0.io`;

export function usePassportCredentialRequest(
  minScore: number
): VcFlowRequestWire | undefined {
  const { identity } = useInternetIdentity();

  if (!identity) {
    return undefined;
  }

  return {
    id: 1,
    jsonrpc: "2.0",
    method: "request_credential",
    params: {
      issuer: {
        origin: PassportIssuerOrigin,
      },
      credentialSpec: {
        credentialType: "GitcoinPassportScore",
        arguments: {
          minScore,
        },
      },
      credentialSubject: identity?.getPrincipal().toString(),
    },
  };
}
