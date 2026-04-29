/**
 * Seed Liquidity Script
 *
 * Adds initial liquidity to a freshly deployed pool.
 * Run this after deploy.ts to bootstrap the pool with starting reserves.
 *
 * Usage:
 *   npx ts-node scripts/seed-liquidity.ts \
 *     --amm <amm-contract-id> \
 *     --amount-a <amount> \
 *     --amount-b <amount> \
 *     --secret <your-secret-key>
 *
 * What it does:
 *   1. Reads AMM contract ID from args (or .env)
 *   2. Calls add_liquidity(account, amount_a, amount_b)
 *   3. Prints LP tokens received
 */

// TODO: implement using PoolClient from the SDK

async function main() {
  throw new Error("seed-liquidity script not yet implemented");
}

main().catch(console.error);
