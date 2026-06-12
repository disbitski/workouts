# Solana Workout Goal Tracker

A small Solana dApp prototype for recording progress toward a workout goal.

I built this project with Rust and Anchor while exploring how generative AI
tools such as Amazon CodeWhisperer and Amazon Q could assist with Solana
development.

## What It Does

- Initializes an on-chain workout-tracking account
- Stores a greeting for the tracker
- Calculates the remaining workouts against a goal of 100
- Tests the program through an Anchor TypeScript client

## Project Structure

| Path | Purpose |
| --- | --- |
| `programs/workouts/src/lib.rs` | On-chain Anchor program |
| `tests/workouts.ts` | TypeScript integration tests |
| `Anchor.toml` | Local Anchor workspace configuration |

## Development

You will need Rust, Solana CLI, Anchor, Node.js, and a local Solana wallet.

```sh
yarn
anchor build
anchor test
```

Use a disposable local-development wallet stored outside the repository.
Never commit a Solana keypair.

## Status

This is an archived learning prototype, not a production health application.
The current goal is fixed at 100 and the update instruction does not enforce
account ownership.

Useful next steps include:

- Make the workout goal configurable
- Track completed workouts and timestamps
- Require the tracker owner to authorize updates
- Correct the greeting assertion in the integration test
- Add input validation and complete tests
