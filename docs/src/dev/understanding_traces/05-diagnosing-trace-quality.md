# Lesson 05 — Diagnosing trace quality: a checklist

You want the program to spend as much time as possible in **loop traces** (ControlPoint + Loop). If it doesn’t, use the three levels and the tools from the previous lessons to diagnose.

---

## Checklist: “Why isn’t my program in loops?”

**1. Confirm what you’re seeing**

- Run with **debugstrs** and note how many blocks are **header** vs **connector** vs **side-trace**.
- Optionally run with **YK_JITC=j2** and **YKD_LOG_IR=jit-pre-opt** and check the JSON: how many traces have `"end": { "kind": "Loop" }`?

**2. Level 1 — Program**

- Does the program have a clear hot loop, or is it branchy/recursive/return-heavy?
- If there’s no single hot loop, you’ll naturally see more connectors/returns and side-traces. That’s expected.

**3. Level 2 — yk**

- For a program that *does* have a hot loop: do you see a header trace that forms a loop (one header, then “enter-jit-code” and repeated execution)?
- If you see many connectors or returns instead of one loop, yk may be stopping tracing too early. If you see many side-traces, guards may be failing often (e.g. type or branch guards).

**4. Level 3 — Interpreter**

- Look at the **debugstr** locations (file:line:opcode). Do they correspond to the loop headers and hot paths in the source?
- If control points are in cold branches or in the wrong places, yk will rarely get a chance to build a good loop trace. That’s an interpreter (e.g. yklua) control-point placement issue.

---

## Practical example: full diagnosis pass

**Run two programs and compare.**

**A. Simple loop (expected: one loop trace):**

```bash
export YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3
$LUA tests/lua/for_loop.lua 2>&1
```

- You should see: one **header** block, then “enter-jit-code” and the loop running in JIT. So: one ControlPoint+Loop trace; Level 1 (program has one loop), Level 2 (yk formed a loop), Level 3 (control point at the loop header).

**B. Branching loop (expected: one header + one side-trace):**

```bash
$LUA tests/lua/sidetrace.lua 2>&1
```

- You should see: one **header** (main branch), then after a deoptimise a **side-trace** (other branch). So: Level 1 (one loop with two paths), Level 2 (yk built loop + one Guard trace), Level 3 (control point at the branch; guard captures the other path).

If you run a program of your own and the output doesn’t match what you expect from the program shape, use the checklist above and the three levels to decide whether the cause is the program, yk’s choices, or the interpreter’s control points.

---

## Summary of the course

| Lesson | Concept | Practical tool |
|--------|---------|----------------|
| 01 | Trace kinds = (TraceStart, TraceEnd); only ControlPoint+Loop is a loop | Run a simple loop, see one header |
| 02 | Debugstrs = interpreter path; header / connector / side-trace labels | Compare for_loop, sidetrace, nested_loops output |
| 03 | Three levels: program, yk, interpreter | Interpret nested_loops and sidetrace with the three levels |
| 04 | j2 HIR JSON = explicit trace kind | YK_JITC=j2, jit-pre-opt to see start/end kind |
| 05 | Diagnose “why not in loops?” with the checklist and three levels | Apply to your own program |

---

## Further reading

- Main [Understanding traces](../understanding_traces.md) doc: `YKD_LOG_IR` stages, reference.
- Source: `ykrt/src/compile/j2/hir.rs` — TraceStart / TraceEnd definitions and the validity table.
- Tests: `tests/lua/*.lua` — expected stderr (including debugstrs) in the top-of-file comments.

[← Back to course index](README.md)
