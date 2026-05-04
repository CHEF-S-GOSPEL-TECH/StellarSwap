/**
 * Library helpers
 *
 * Periphery convenience functions for sorting token pairs, deriving pair
 * lookups, and calculating quote paths before router transactions are built.
 */

export function sortTokens(tokenA: string, tokenB: string): [string, string] {
  if (tokenA === tokenB) {
    throw new Error("identical token addresses");
  }

  return tokenA < tokenB ? [tokenA, tokenB] : [tokenB, tokenA];
}

export function quote(amountA: bigint, reserveA: bigint, reserveB: bigint): bigint {
  if (amountA <= 0n) {
    throw new Error("amountA must be positive");
  }
  if (reserveA <= 0n || reserveB <= 0n) {
    throw new Error("reserves must be positive");
  }

  return (amountA * reserveB) / reserveA;
}
