# Provenance Machine

Cryptographic provenance infrastructure for the Crypto World’s Fair by Colosseum.

## Architecture

`artifact → local SHA-256 → wallet signature → Solana PDA record → verification`

The browser hashes the artifact locally. The underlying file is never uploaded.

## Build

```powershell
anchor build
anchor test
```

Deploy to Devnet after configuring a real program keypair:

```powershell
anchor keys list
anchor build
anchor deploy --provider.cluster devnet
```

Run the frontend from a normal HTTP origin:

```powershell
npx serve app
```

## Before deployment

The `declare_id!` and `[programs.devnet]` values are intentionally a placeholder. Run `anchor keys list`, use the generated program keypair address, update both locations, then rebuild and deploy.

The frontend instruction currently uses the standard Anchor instruction layout with an eight-byte placeholder discriminator. Once Anchor generates the IDL, replace the hand-built instruction with the generated Anchor client call so the discriminator is guaranteed to match the deployed program.
