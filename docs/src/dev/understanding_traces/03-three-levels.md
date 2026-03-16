# Lesson 03 — Three levels: who chose what?

When execution spends a lot of time *outside* loop traces, you need to ask *why*. Three levels help:

1. **User program** — Is the program loop-heavy or full of branches/returns?
2. **yk** — Where did yk start and stop tracing? Did it build Loop / Coupler / Return and sensible side-traces?
3. **Interpreter (e.g. yklua)** — Where are control points and what bytecode do they cover? Bad placement means yk never gets a chance to form good loops.

---

## Level 1: The user’s program

If the program rarely stays in a single loop (lots of branches, recursion, or returns), then there simply isn’t one hot loop to capture. No amount of tuning will turn it into a single ControlPoint+Loop trace. So first ask: *does my program have a clear hot loop?*

---

## Level 2: yk’s trace selection

yk decides *where* to start tracing (control point or guard) and *where* to stop (same location → Loop, different location → Coupler, or return → Return). If the program *does* have a hot loop but you still see many connectors or side-traces, then either:

- yk stopped tracing too early (e.g. Return or Coupler instead of Loop), or
- guards are failing often, so you get many Guard (side-trace) compilations.

So second ask: *given the program, are the trace kinds (header/connector/side-trace) what I’d expect?*

---

## Level 3: The interpreter’s control points

The interpreter (e.g. yklua) chooses *where* to put control points (the places from which tracing can start). It also attaches a debug string (file:line:opcode) to each. If control points are in the wrong places (e.g. inside branches that are rarely taken, or not at loop headers), then yk will rarely form a nice loop trace even when the program has a hot loop. So third ask: *do the debugstr locations (file:line:opcode) match the loop headers and hot paths I expect?*

---

## Practical example: apply the three levels

**Run:**

```bash
$LUA tests/lua/nested_loops.lua 2>&1
```

**Observe:**

1. **Level 1 (program):** The program has two nested loops; we expect at least one “inner” loop trace and something that connects inner and outer (connector).
2. **Level 2 (yk):** You should see a **header** (inner loop, ControlPoint+Loop) and a **connector** (ControlPoint+Coupler) that links to the outer loop. So yk chose to form one loop and one connector — that’s sensible for nested loops.
3. **Level 3 (interpreter):** The debugstrs show *which* bytecode locations were traced (e.g. `nested_loops.lua:44: ADDI` for the inner body, `nested_loops.lua:42: ADDI` for the connector). Check in the Lua source: do those line numbers correspond to the inner loop body and the outer loop body? If yes, the interpreter placed control points where the hot loops are.

**Run:**

```bash
$LUA tests/lua/sidetrace.lua 2>&1
```

**Observe:**

1. **Level 1:** The program has one loop with a branch (`if i < 5 then ... else ...`). So we expect one main path (header) and another path when the branch goes the other way (side-trace).
2. **Level 2:** You see one **header** and one **side-trace**. The side-trace is Guard+Coupler (or Return): it was compiled when the guard failed. So yk’s behaviour matches the program.
3. **Level 3:** The debugstrs for the side-trace point to different lines (the “else” branch). That confirms the interpreter is giving yk the right locations for both paths.

---

## Try this

- Run a program that *doesn’t* have a clear hot loop (e.g. many different branches or a recursive function). You should see more connectors/returns and fewer “one big loop” headers. That’s Level 1.
- In a program that *does* have a hot loop, if you see lots of side-traces, ask: are those guards necessary (e.g. type checks) or could the interpreter/yk be improved (Level 2 or 3)?

---

## Takeaway

- **Level 1:** Program shape (loop-heavy vs branches/returns).
- **Level 2:** yk’s choices (where to start/stop, Loop vs Coupler vs Return, side-traces).
- **Level 3:** Interpreter’s control points and debug strings (do they match hot paths?).
- Use all three to explain “why isn’t my program spending more time in loop traces?”

**Next:** [04 — j2 trace graphs](04-j2-trace-graphs.md) — seeing the exact (TraceStart, TraceEnd) in HIR output.
