/**
 * Pool client
 *
 * Wraps the AMM contract's read and write methods.
 * Use this to interact with a deployed pool from a frontend or script.
 */

import { Contract, SorobanRpc, TransactionBuilder, Networks } from "@stellar/stellar-sdk";

export interface PoolConfig {
  contractId: string;       // deployed AMM contract address
  rpcUrl: string;           // Soroban RPC endpoint
  networkPassphrase: string; // e.g. Networks.TESTNET
}

export class PoolClient {
  constructor(private config: PoolConfig) {}

  /**
   * Returns the current reserves of the pool as [reserveA, reserveB].
   */
  async getReserves(): Promise<[bigint, bigint]> {
    // TODO: call AMM contract's get_reserves() via simulateTransaction
    throw new Error("not implemented");
  }

  /**
   * Returns a price quote: how much token_out you'd receive for `amountIn` of `tokenIn`.
   */
  async getQuote(tokenIn: string, amountIn: bigint): Promise<bigint> {
    // TODO: call AMM contract's get_quote(token_in, amount_in)
    throw new Error("not implemented");
  }

  /**
   * Builds and returns an unsigned transaction to add liquidity.
   * Caller is responsible for signing and submitting.
   */
  async addLiquidity(
    sourceAccount: string,
    amountA: bigint,
    amountB: bigint
  ): Promise<string /* XDR */> {
    // TODO: build transaction invoking add_liquidity(to, amount_a, amount_b)
    throw new Error("not implemented");
  }

  /**
   * Builds and returns an unsigned transaction to remove liquidity.
   */
  async removeLiquidity(
    sourceAccount: string,
    lpAmount: bigint
  ): Promise<string /* XDR */> {
    // TODO: build transaction invoking remove_liquidity(to, lp_amount)
    throw new Error("not implemented");
  }
}
