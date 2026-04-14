# Lesson 04 — j2 trace graphs: seeing TraceStart and TraceEnd in HIR

When you use the **j2** compiler, the **jit-pre-opt** log prints the HIR (high-level IR) for each trace. Each trace is prefixed with a **JSON line** that states its kind explicitly: `TraceStart` and `TraceEnd`. That is the “j2 trace graph” in terms of trace *kind*.

---

## Enabling j2 and jit-pre-opt

Use the j2 compiler and ask for jit-pre-opt output:

```bash
export YK_JITC=j2 YKD_LOG_IR=jit-pre-opt YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
$LUA your_script.lua 2>&1
```

- **YK_JITC=j2** — use the j2 backend (which produces HIR and the trace-kind JSON).
- **YKD_LOG_IR=jit-pre-opt** — log the HIR (with the JSON header) for each trace.

You can combine stages: e.g. `YKD_LOG_IR=jit-pre-opt,debugstrs` to see both HIR and debugstrs (if your build supports it).

---

## What you see: the JSON line

Before the HIR block for each trace you get a comment line like:

```text
; { "trid": "...", "start": { "kind": "ControlPoint" }, "end": { "kind": "Loop" } }
```

So you can read off:

- **start.kind** — `"ControlPoint"` or `"Guard"`.
- **end.kind** — `"Loop"`, `"Coupler"`, or `"Return"`.

For a Coupler, the JSON also includes `"tgt_trid": "..."`. For a Guard start, it includes `"src_trid"` and `"gidx"`.

---

## Practical example 1: one loop (ControlPoint + Loop)

**Run:**

```bash
export YK_JITC=j2 YKD_LOG_IR=jit-pre-opt YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
$LUA tests/lua/for_loop.lua 2>&1
```

**Look for:** A line like:

```text
; { "trid": "...", "start": { "kind": "ControlPoint" }, "end": { "kind": "Loop" } }
```

That confirms this trace is **ControlPoint + Loop**. The rest of the block is the HIR (entry block, and optionally a peel block).

---

## Practical example 2: connector (ControlPoint + Coupler)

**Run:**

```bash
$LUA tests/lua/nested_loops.lua 2>&1
```

**Look for:**

- One trace with `"end": { "kind": "Loop" }` (the inner loop).
- Another with `"end": { "kind": "Coupler", "tgt_trid": "..." }` — the connector. So **ControlPoint + Coupler**.

---

## Practical example 3: side-trace (Guard + Coupler or Return)

**Run:**

```bash
$LUA tests/lua/sidetrace.lua 2>&1
```

**Look for:** A trace whose JSON has:

```text
"start": { "kind": "Guard", "src_trid": "...", "gidx": "..." }
```

and `"end": { "kind": "Coupler" }` or `"end": { "kind": "Return" }`. That’s the side-trace (Guard start, never Loop end).

---

## Try this

- Run the same scripts with **debugstrs** as well (`YKD_LOG_IR=jit-pre-opt,debugstrs`). Match the order of traces: first trace = first JSON + first debugstr block. That ties the formal (TraceStart, TraceEnd) to the human-readable instruction list.
- For a program that only has one loop, confirm you get exactly one trace with `"end": { "kind": "Loop" }`.

---

## Takeaway

- With **YK_JITC=j2** and **YKD_LOG_IR=jit-pre-opt**, each trace’s kind is explicit in the leading JSON.
- **start.kind** = ControlPoint or Guard; **end.kind** = Loop, Coupler, or Return.
- Use this when you need to be sure what kind of trace you’re looking at (e.g. when debugging “why is this not a loop?”).

**Next:** [05 — Diagnosing trace quality](05-diagnosing-trace-quality.md) — a short checklist and summary.
