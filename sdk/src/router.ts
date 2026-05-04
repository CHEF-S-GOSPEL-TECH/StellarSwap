/**
 * Router client
 *
 * Periphery helpers for quotes and user-facing swap transactions.
 */

import { PairClient } from "./pair";

export class RouterClient {
  constructor(private pair: PairClient) {}

  /**
   * Returns how much `tokenOut` you'd receive for `amountIn` of `tokenIn`,
   * accounting for the 0.3% fee.
   */
  async quote(tokenIn: string, amountIn: bigint): Promise<bigint> {
    // TODO: delegate to pair.getQuote
    throw new Error("not implemented");
  }

  /**
   * Builds an unsigned swap transaction.
   *
   * @param sourceAccount - the swapper's Stellar account address
   * @param tokenIn       - address of the token being sold
   * @param amountIn      - amount to sell (in stroops / base units)
   * @param slippageBps   - max acceptable slippage in basis points (e.g. 50 = 0.5%)
   * @returns unsigned transaction XDR
   */
  async buildSwapTx(
    sourceAccount: string,
    tokenIn: string,
    amountIn: bigint,
    slippageBps: number = 50
  ): Promise<string /* XDR */> {
    // TODO:
    // 1. get quote
    // 2. calculate min_amount_out from slippage
    // 3. build transaction invoking pair contract's swap()
    throw new Error("not implemented");
  }
}
