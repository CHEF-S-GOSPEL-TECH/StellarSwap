/**
 * Deploy Script
 *
 * Deploys the factory, pair, and LP token contracts to Stellar testnet (or mainnet).
 *
 * Usage:
 *   npx ts-node scripts/deploy.ts --network testnet --secret <your-secret-key>
 *
 * What it does:
 *   1. Uploads factory WASM -> gets a wasm_hash
 *   2. Deploys factory contract instance
 *   3. Uploads pair WASM -> gets a wasm_hash
 *   4. Deploys pair contract instance
 *   5. Uploads LP token WASM -> gets a wasm_hash
 *   6. Deploys LP token contract instance
 *   7. Calls Pair.initialize(token_a, token_b, lp_token)
 *   8. Calls LpToken.initialize(pair_address, "DEX LP Token", "DLP")
 *   9. Prints deployed contract IDs
 */

// TODO: implement deploy script using @stellar/stellar-sdk
//       Reference: https://developers.stellar.org/docs/smart-contracts/guides/cli/deploy-contract

async function main() {
  throw new Error("deploy script not yet implemented");
}

main().catch(console.error);
