import { z } from "zod";

export const VcFlowRequest = z.object({
  id: z.union([z.number(), z.string()]),
  jsonrpc: z.literal("2.0"),
  method: z.literal("request_credential"),
  params: z.object({
    issuer: z.object({
      origin: z.string(),
    }),
    credentialSpec: z.object({
      credentialType: z.string(),
      arguments: z.record(z.string(), z.union([z.string(), z.number()])),
    }),
    credentialSubject: z.string(),
    derivationOrigin: z.optional(z.string()),
  }),
});
export type VcFlowRequest = z.infer<typeof VcFlowRequest>;

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
