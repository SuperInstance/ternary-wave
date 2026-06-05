# ternary-wave

**Waveform generation in {-1, 0, +1}. Square, sawtooth, triangle, and sine-quantized.**

Every sound starts with a waveform. This crate generates the fundamental wave shapes — square, sawtooth, triangle — using only the three ternary values. The constraint produces distinctive waveforms: a ternary square wave alternates between +1 and -1. A ternary sawtooth steps through the three values sequentially. A ternary triangle mirrors the sawtooth. A ternary sine approximates the sinusoid by quantizing to the nearest value.

These aren't approximations of continuous waves — they're the *actual* waves, defined in ternary space. The harmonic content is different from continuous waveforms, producing a grittier, more digital character.

## What's Inside

- **`square(freq, ticks)`** — alternates between +1 and -1 at the given frequency
- **`saw(freq, ticks)`** — steps through -1 → 0 → +1 → -1 → ... (ascending sawtooth)
- **`triangle(freq, ticks)`** — -1 → 0 → +1 → 0 → -1 → ... (symmetric)
- **`sine(freq, ticks)`** — quantized sine: sample sin() at each tick, snap to nearest {-1, 0, +1}
- **`noise(ticks, seed)`** — pseudo-random ternary noise (deterministic from seed)
- **`pulse(freq, ticks, duty)`** — pulse wave with configurable duty cycle
- **`combine(waves)`** — elementwise addition of multiple waveforms (wraps mod 3)

## Quick Example

```rust
use ternary_wave::*;

// Square wave: 2 cycles in 8 ticks
let sq = square(2, 8);
// [1, 1, -1, -1, 1, 1, -1, -1]

// Sawtooth: ascending through ternary values
let saw_wave = saw(2, 9);
// [-1, 0, 1, -1, 0, 1, -1, 0, 1]

// Triangle wave
let tri = triangle(1, 8);
// [-1, 0, 1, 0, -1, 0, 1, 0]

// Sine approximation
let sin_approx = sine(1, 8);
// Quantized from sin(): [0, 1, 0, -1, 0, 1, 0, -1]

// Combine two waves
let layered = combine(&[square(1, 8), triangle(2, 8)]);
// Elementwise ternary addition (mod 3 wrapping)
```

## The Deeper Truth

**Ternary waveforms have different harmonic content than continuous ones.** A continuous square wave has odd harmonics (1f, 3f, 5f...). A ternary square wave — which has the *same shape* — has the same harmonics but with different amplitudes because the quantization to {-1, 0, +1} means there's no "in-between" values. The result is a purer, more extreme version of the square wave character.

The sine approximation is particularly interesting: because ternary has only 3 levels, the quantization error is large. The ternary sine sounds more like a stepped wave than a smooth sinusoid. But this "error" is also a feature — it adds harmonic content that wouldn't exist in a pure sine, making the ternary sine richer and more interesting than its continuous counterpart.

**Use cases:**
- **Audio synthesis** — generate raw ternary waveforms for synthesis
- **Algorithmic music** — use ternary waves as the foundation for compositions
- **Signal processing education** — the simplest waveform generators
- **Testing** — generate known signals for testing ternary processing pipelines
- **Art** — ternary waveforms as visual patterns

## See Also

- **ternary-harmonic** — harmonic analysis of these waveforms
- **ternary-loop** — find the period of generated waveforms
- **ternary-echo** — add echoes to generated waveforms
- **ternary-bite** — degrade generated waveforms for lo-fi effects
- **ternary-vu** — meter the generated waveforms

## Install

```bash
cargo add ternary-wave
```

## License

MIT
