/**
 * Multiply an integer `x` by a Perbill-like fraction given by `parts`
 * (0 <= parts <= PERBILL), rounding to "nearest, prefer down"
 * like Substrate's `Perbill * N` aka `overflow_prune_mul`.
 *
 * This is different from `floor(x * parts / PERBILL)`, it is similar to
 * `round(float(x) * parts / PERBILL)` if floats had infinite precision.
 *
 * By default PERBILL = 1_000_000_000n (Perbill), but you can override it
 * to emulate other fixed-point scales.
 */
export function perbillMul(x: bigint, parts: bigint, PERBILL = 1_000_000_000n): bigint {
    if (PERBILL <= 0n) {
        throw new RangeError(`PERBILL must be > 0, got ${PERBILL}`);
    }

    if (parts < 0n || parts > PERBILL) {
        throw new RangeError(`parts must be between 0 and ${PERBILL}, got ${parts}`);
    }

    if (x === 0n || parts === 0n) {
        return 0n;
    }

    // x = q * PERBILL + r, with 0 <= r < PERBILL
    const q = x / PERBILL;
    const r = x % PERBILL;

    // Base term: floor((x / PERBILL) * parts) = q * parts
    const base = q * parts;

    // "rational_mul_correction" part:
    // Compute (r * parts) / PERBILL with NearestPrefDown rounding.
    const remMul = r * parts; // exact numerator
    const div = remMul / PERBILL; // truncated part
    const rem = remMul % PERBILL; // remainder
    const half = PERBILL / 2n;

    // NearestPrefDown:
    // - round up only if fractional part > 0.5
    // - if exactly 0.5, stay down
    const correction = rem > half ? div + 1n : div;

    return base + correction;
}
