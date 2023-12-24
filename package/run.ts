// don't worry about the types here asc knows this types
export function fibonacci(n: i64): i64 {
  if (n <= 1) {
    return n;
  }

  let fibPrev = 0;
  let fibCurrent = 1;

  for (let i = 2; i <= n; i++) {
    const fibNext = fibPrev + fibCurrent;
    fibPrev = fibCurrent;
    fibCurrent = fibNext;
  }

  return fibCurrent;
}
