# ternary-wave

**Three values. Infinite sound.**

Before there was a synth, there was a waveform. Before the waveform, there was a shape — up, steady, down. +1, 0, -1. The entire history of electronic sound, compressed into three states.

Ternary waves aren't approximations of "real" waves. They ARE the waves — defined in the simplest possible space. A square wave that alternates between +1 and -1. A sawtooth that climbs -1 → 0 → +1 → -1. A triangle that breathes -1 → 0 → +1 → 0 → -1. A sine that's been dragged through a quantizer and emerged with scars.

The result sounds *nothing* like continuous audio. It sounds digital in the most honest way — the sound of a universe that only has three opinions.

## What's Inside

- **`square(freq, ticks)`** — the primitive pulse. Alternates between +1 and -1. The heartbeat of every drum machine ever made
- **`saw(freq, ticks)`** — climbing through the three states. Harmonically rich, harmonically *rude*
- **`triangle(freq, ticks)`** — the gentle one. -1 → 0 → +1 → 0 → -1. Sounds almost like a sine, but sharper
- **`sine(freq, ticks)`** — a sine wave dragged through a three-level quantizer. The ghost of a sine, haunting the ternary domain
- **`noise(ticks, seed)`** — deterministic chaos. Pseudo-random ternary noise from a seed. Same seed, same chaos
- **`pulse(freq, ticks, duty)`** — a square wave that's not 50/50. The sound ofPWM synths and NES pulse channels
- **`combine(waves)`** — stack waveforms. Elementwise ternary addition. When shapes collide

## Quick Example

```rust
use ternary_wave::*;

// The fundamental bass: a square wave at frequency 2, 8 ticks long
let bass = square(2, 8);
// [1, 1, -1, -1, 1, 1, -1, -1]

// A sawtooth climbing endlessly
let saw_wave = saw(2, 9);
// [-1, 0, 1, -1, 0, 1, -1, 0, 1]

// The ghost of a sine
let ghost = sine(1, 12);
// Quantized sin(): [0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1]

// Layer them: square + triangle = something new
let layered = combine(&[square(1, 8), triangle(2, 8)]);
// The shapes interfere. New harmonics emerge from the collision.
```

## The Deeper Truth

**Ternary waveforms are the DNA of digital sound.** Every continuous waveform, when you strip away the smooth curves and infinite resolution, reduces to a sequence of discrete states. Ternary waveforms skip the pretense — they're already discrete, already quantized, already honest about what they are.

The harmonic content is *different* from continuous waves. A ternary square wave has the same odd harmonics (1f, 3f, 5f...) as a continuous square, but the amplitudes are quantized too — each harmonic is either there (+1), not there (0), or inverted (-1). The result is a square wave on steroids: more extreme, more digital, more *itself*.

The ternary sine is the most revealing. A continuous sine is the purest possible sound — one frequency, nothing else. Quantized to three levels, it gains harmonics it never had. The quantization error *is* the character. It's like taking a photograph and reducing it to three colors — you lose fidelity but gain a specific, unmistakable aesthetic. The ternary sine doesn't approximate a sine. It *translates* a sine into a different artistic medium.

**Why it matters:** These six functions are the raw material for everything else in the ternary audio ecosystem. Wave → envelope → filter → pan → output. Every synth patch starts here. Every drum sound starts here. Every ambient texture starts here. Six functions. Three values. The whole universe.

**Use cases:**
- **Synthesis** — generate raw material for any ternary audio chain
- **Algorithmic music** — use these as the foundation for generative composition
- **Sound design** — layer and mangle for textures you can't get from continuous synthesis
- **Education** — the simplest possible waveform generators. Show students what waves *actually are*
- **Art** — ternary waveforms as visual patterns, textile designs, architectural rhythms

## See Also

- **ternary-harmonic** — what happens when you analyze these waveforms for harmonic content
- **ternary-envelope** — shape these raw waves into musical sounds (ADSR)
- **ternary-bite** — destroy these waves for lo-fi textures
- **ternary-loop** — find the period, loop it, play it forever
- **ternary-rack** — the modular synth that wires these together
- **ternary-mixer** — blend multiple waves into one signal
- **ternary-echo** — add space and depth to the raw waveforms

## Install

```bash
cargo add ternary-wave
```

## License

MIT
