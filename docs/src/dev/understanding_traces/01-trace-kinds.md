# Lesson 01 — Trace kinds: the TraceStart / TraceEnd table

Every trace in yk has a **kind** defined by two things: **where it started** and **where it ended**. In the code this is the pair `(TraceStart, TraceEnd)` (see `ykrt/src/compile/j2/hir.rs`).

---

## The table

Five of the six combinations are valid:

|              | Coupler | Loop | Return |
|--------------|---------|------|--------|
| **ControlPoint** | ✓       | ✓    | ✓      |
| **Guard**        | ✓       | ✗    | ✓      |

- **TraceStart**
  - **ControlPoint** — The trace started at a place where the interpreter allows tracing (a “control point”).
  - **Guard** — The trace started when a guard failed (a **side-trace**).
- **TraceEnd**
  - **Loop** — The trace ends back at the *same* location it started → a natural loop.
  - **Coupler** — The trace ends at a *different* location and jumps to another compiled trace.
  - **Return** — The trace ends on a return (e.g. leaving the loop or function).

So when you look at a trace, the first question is: **is it a loop?** Only **ControlPoint + Loop** is a loop trace. Ideally, most execution time is spent in such traces; time outside loops can mean the program isn’t loop-heavy, or that yk/the interpreter didn’t choose good traces or control points.

---

## Practical example: one loop, one trace

**Goal:** Run a program that has a single obvious loop and see exactly one **header** trace (which will be ControlPoint + Loop).

**1. Use the provided example script**

From the repo root, run the small loop script in this course directory (replace `$LUA` with your yklua binary path):

```bash
export YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
$LUA docs/src/dev/understanding_traces/examples/simple_loop.lua 2>&1
```

Or use the existing test program (same idea):

```bash
$LUA tests/lua/for_loop.lua 2>&1
```

**2. What you should see**

- Several lines of program output (numbers, etc.).
- A line like: `yk-tracing: start-tracing: ... : GETTABUP` (or similar).
- A line like: `yk-tracing: stop-tracing: ... : GETTABUP`.
- One block that looks like:
  ```text
  --- Begin debugstrs: header: for_loop.lua:34: GETTABUP ---
    for_loop.lua:34: GETTABUP
    ...
  --- End debugstrs ---
  ```
- Then: `yk-execution: enter-jit-code: ...` and more program output.

That single **header** block is one trace. In this case it is a **ControlPoint + Loop** trace: it started at a control point and ended back at the same place (the loop). We’ll see in the next lesson how to recognise “header” vs “connector” vs “side-trace” in the labels.

**3. Try this**

- Change the loop bound (e.g. run 2 iterations vs 20). You still get single trace that is executed multiple times.
- **Experiment with different thresholds.** The threshold is how many times a control point must be hit before yk starts tracing. Run the same script with `YK_HOT_THRESHOLD=1`, then `YK_HOT_THRESHOLD=6`, then `YK_HOT_THRESHOLD=666` (the default). With a **low** value (1–5), tracing starts after just a few iterations and you should see the debugstr block quickly even for short loops. With a **high** value (e.g. 131), tracing only starts after many hits—if your loop runs only a handful of iterations, you may see no tracing at all. Compare when “start-tracing” and “enter-jit-code” appear in each run.


## Takeaway

- Every trace has a (TraceStart, TraceEnd) kind.
- Only **ControlPoint + Loop** is a loop trace.
- A simple loop program should produce one header trace that is exactly that.

**Next:** [02 — Debugstrs](02-debugstrs.md) — what the “header” / “connector” / “side-trace” labels mean and how to read the instruction list.
