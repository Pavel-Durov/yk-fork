# Understanding yk Traces: A Step-by-Step Course

This directory is a short technical course on how to read and interpret yk’s trace output. Each lesson introduces one concept and ends with a **practical example** you can run yourself.

**Goal:** By the end you will know what kind of trace you’re looking at (loop or not), how to read debugstrs, how to think about trace quality at three levels, and how to view j2 trace graphs.

---

## Prerequisites

- You have built the yk project (or at least the yk-instrumented Lua interpreter).
- The **yklua** binary is available (typically built from `tests/yklua`; the exact path depends on your build).
- You can run Lua scripts from the repository root, e.g.  
  `./path/to/yklua tests/lua/for_loop.lua`

We will use the same environment variables in every run so that tracing and logging are predictable:

```bash
YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
```

Use these (or the variant shown in each lesson) when running the examples.

---

## Course structure

| Lesson | Topic | What you’ll do |
|--------|--------|----------------|
| [01 — Trace kinds](01-trace-kinds.md) | The TraceStart / TraceEnd table; what is a “loop trace” | Run a simple loop and see one header trace |
| [02 — Debugstrs](02-debugstrs.md) | What debugstrs are; header vs connector vs side-trace | Compare output from a loop and a branching program |
| [03 — Three levels](03-three-levels.md) | Program vs yk vs interpreter: who chose what? | Interpret traces from nested loops and side-traces |
| [04 — j2 trace graphs](04-j2-trace-graphs.md) | Viewing HIR and the trace-kind JSON | Run with `YK_JITC=j2` and inspect jit-pre-opt output |
| [05 — Diagnosing trace quality](05-diagnosing-trace-quality.md) | Why isn’t my program in loops? A checklist | Apply the three levels to real output |

---

## Conventions

- **`LUA`** in commands means: your yklua (or yk-instrumented Lua) binary. Replace it with the actual path, e.g. `./tests/yklua/build/lua` or `lua`.
- **Repo root** means the root of the yk repository; paths like `tests/lua/for_loop.lua` are relative to that.
- **Stderr:** All trace and debugstr output goes to stderr. Use `2>&1` if you want to see it in the same stream as stdout.

---

## Quick reference: trace kinds

| Start        | End     | Meaning |
|-------------|---------|--------|
| ControlPoint | Loop    | Classic loop trace (best case). |
| ControlPoint | Coupler | Connector to another trace. |
| ControlPoint | Return  | Trace exited (e.g. return). |
| Guard       | Coupler | Side-trace jumping to another trace. |
| Guard       | Return  | Side-trace that returns. |

Only **ControlPoint + Loop** is a loop trace. In the debugstr output you see this as a **header** trace that forms a loop; **connector** = ControlPoint + Coupler; **side-trace** = Guard + Coupler or Return.

Next: [01 — Trace kinds](01-trace-kinds.md)
