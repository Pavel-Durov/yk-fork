# Lesson 02 — Debugstrs: what was traced?

**Debugstrs** are short strings (e.g. `file.lua:34: GETTABUP`) that record which interpreter instructions ran along a trace. They are the easiest way to see *what* was traced without reading raw IR.

---

## Enabling debugstrs

Use the same run as in Lesson 01:

```bash
export YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
$LUA your_script.lua 2>&1
```

- **YKD_LOG=4** — prints tracing start/stop and enter-jit-code messages.
- **YKD_LOG_IR=debugstrs** — prints one block of debugstrs per compiled trace.
- **YKD_SERIALISE_COMPILATION=1** — compiles one trace at a time so the order of output is clear.
- **YK_HOT_THRESHOLD=3** — how many times a location must be hit before tracing starts (lower = trace sooner).

---

## The three labels: header, connector, side-trace

Each block of debugstrs starts with a line like:

```text
--- Begin debugstrs: <label>: file.lua:LINE: OPCODE ---
```

The **label** tells you the trace kind in terms of TraceStart/TraceEnd:

| Label        | TraceStart   | TraceEnd   | Meaning |
|-------------|--------------|------------|--------|
| **header**  | ControlPoint | Loop / Coupler / Return | Trace started at a control point. |
| **connector** | ControlPoint | Coupler    | Connector from one trace to another (e.g. inner → outer loop). |
| **side-trace** | Guard      | Coupler / Return | Trace started from a guard failure (branch or type check). |

The lines *under* the header are the sequence of interpreter locations (file:line:opcode) that were recorded. That is the exact path through the interpreter for that trace.

---

## Practical example 1: loop only (header)

**Run:**

```bash
$LUA tests/lua/for_loop.lua 2>&1
```

**Look for:** One block starting with `--- Begin debugstrs: header: ...`. The lines inside are the bytecode path for the loop body (GETTABUP, GETFIELD, FORLOOP, etc.). This trace is **ControlPoint + Loop**.

---

## Practical example 2: loop with branch (header + side-trace)

**Run** (from repo root):

```bash
$LUA tests/lua/sidetrace.lua 2>&1
```

Or use the course example (same idea; you may need `YK_SIDETRACE_THRESHOLD=2`):

```bash
$LUA docs/src/dev/understanding_traces/examples/branching_loop.lua 2>&1
```

**Look for:**

1. First, a **header** block (e.g. `header: sidetrace.lua:58: LTI`). That’s the main loop path (e.g. `i < 5`).
2. Later, after a deoptimise and “start-side-tracing”, a **side-trace** block (e.g. `side-trace: sidetrace.lua:58: LTI`). That’s the path when the guard failed (e.g. `i >= 5`). The instructions listed (e.g. different line numbers) show the *other* branch.

So: one **header** = one ControlPoint trace (here, a loop). One **side-trace** = one Guard trace (here, Coupler or Return back into the loop). Comparing the two blocks shows which interpreter path each trace captured.

---

## Practical example 3: nested loops (header + connector)

**Run:**

```bash
$LUA tests/lua/nested_loops.lua 2>&1
```

**Look for:**

- A **header** block for the inner loop (e.g. `header: nested_loops.lua:44: ADDI`).
- Later, a **connector** block (e.g. `connector: nested_loops.lua:42: ADDI`). That trace doesn’t form a loop by itself; it connects execution from one place (e.g. inner loop) to another (e.g. outer loop). The debugstrs in the connector block show the path between those two loop headers.

So: **header** = “this trace can be a loop”. **connector** = “this trace just links two other traces”.

---

## Try this

- Run `tests/lua/while_loop.lua` and confirm you only see **header** blocks (one loop).
- Run `tests/lua/sidetrace_to_loop.lua`: you should see **header**, then **side-trace**, then another **header** (a new loop formed from a side exit). Follow the debugstr line numbers and match them to the Lua source.

---

## Takeaway

- Debugstrs show the interpreter path (file:line:opcode) for each trace.
- **header** = ControlPoint start (Loop/Coupler/Return). **connector** = ControlPoint + Coupler. **side-trace** = Guard start.
- Use the instruction list under each block to see exactly which path was traced.

**Next:** [03 — Three levels](03-three-levels.md) — program vs yk vs interpreter.
