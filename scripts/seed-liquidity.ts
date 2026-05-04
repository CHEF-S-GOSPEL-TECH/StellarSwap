/**
 * Seed Liquidity Script
 *
 * Adds initial liquidity to a freshly deployed pair.
 * Run this after deploy.ts to bootstrap the pair with starting reserves.
 *
 * Usage:
 *   npx ts-node scripts/seed-liquidity.ts \
 *     --pair <pair-contract-id> \
 *     --amount-a <amount> \
 *     --amount-b <amount> \
 *     --secret <your-secret-key>
 *
 * What it does:
 *   1. Reads pair contract ID from args (or .env)
 *   2. Calls add_liquidity(account, amount_a, amount_b)
 *   3. Prints LP tokens received
 */

// TODO: implement using PairClient from the SDK

async function main() {
  throw new Error("seed-liquidity script not yet implemented");
}

main().catch(console.error);
