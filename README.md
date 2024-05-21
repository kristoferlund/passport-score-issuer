# ICP Passport Score Issuer

This project demos the use of verifiable credentials on the Internet Computer. It uses [Gitcoin Passport](https://passport.gitcoin.co) as an example source of verifiable credentials.

> Passport helps you collect “stamps” that prove your humanity and reputation.

The project consists of three main packages:

### 1. [`issuer_backend`](./packages/issuer_backend)

Stores the link between Gitcoin Passport and II account and issues verifiable credentials to prove the Gitcoin Passport Score.

### 2. [`issuer_frontend`](./packages/issuer_frontend)

In this interface, the user logs in with their Ethereum address and II credentials and then links their Gitcoin Passport to their II account. 

Try it out: https://ycons-daaaa-aaaal-qja3q-cai.icp0.io/

### 3. [`demo_app`](./packages/demo_app)

Here, the user can securely request a verifiable credential from the issuer proving their Gitcoin Passport Score.

Try it out: https://jzi4k-7qaaa-aaaal-qdncq-cai.icp0.io/

## Run locally

### 1. Start the local replica

```bash
dfx start --clean
```

### 2. Deploy the canisters

```bash
make deploy-all
```
