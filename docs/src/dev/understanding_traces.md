# Understanding Traces

**New to traces?** A [step-by-step course](understanding_traces/README.md) in the same directory walks through trace kinds, debugstrs, three levels, and j2 trace graphs with practical examples for each topic.

---

yk can print the traces it has created to `stderr` to help with debugging.
However, these traces are often lengthy, and not always easy to understand.
This section explains how to get yk to print its traces, how to interpret
trace *kinds* (loop vs non-loop), and how to think about trace quality at
three levels: the user's program, yk's trace selection, and the interpreter's
(yklua's) control-point placement.


## Where to start: the TraceStart / TraceEnd table

Every trace has a *kind* given by a pair: where it **starts** and where it **ends**.
In the codebase this is represented as ([TraceStart], [TraceEnd]) in the HIR
(see `ykrt/src/compile/j2/hir.rs`). Five of the six combinations are valid:

|              | Coupler | Loop | Return |
|--------------|---------|------|--------|
| **ControlPoint** | ✓       | ✓    | ✓      |
| **Guard**        | ✓       | ✗    | ✓      |

- **TraceStart**: **ControlPoint** = trace started from an interpreter control
  point (a place the interpreter chose to allow tracing). **Guard** = trace
  started from a guard failure (side-trace).
- **TraceEnd**: **Loop** = trace ends back at the same location it started,
  i.e. a natural loop. **Coupler** = trace ends at a *different* location and
  jumps to another compiled trace. **Return** = trace ends on a return (e.g.
  out of the loop or function).

So for each trace you see, you can ask: *is it a loop or not?* Only
**ControlPoint + Loop** is a classic “loop trace”. Ideally, most execution
time is spent in loop traces; time spent outside loops can mean either that
“we've not traced something sensible” or that “the user's program isn't a great
match for tracing”. “Not traced something sensible” can in turn be either “yk
has not chosen good traces” or “the interpreter (e.g. yklua) has not chosen
good control points”.


## Three levels to think about

When interpreting trace output, it helps to reason at three levels:

1. **User program**  
   Is the program loop-heavy and predictable, or does it branch, recurse, or
   return often? That directly affects how much can be captured in loop traces.

2. **yk's trace selection**  
   Where did yk start and stop tracing? Did it form a Loop, a Coupler, or a
   Return? Did it create sensible side-traces (Guard → Coupler / Return)?

3. **Interpreter's control points (e.g. yklua)**  
   Where did the interpreter place control points and what debug strings
   (bytecode locations) do they carry? Poor placement means yk never gets a
   chance to build good loop traces.

So: first check whether the trace is a loop (ControlPoint + Loop); then, if
execution is often outside loops, ask whether that's due to program shape, yk
choices, or interpreter (yklua) choices.


## Debugstrs: a practical way to see what was traced

Debugstrs are short, human-readable strings (e.g. `file.lua:34: GETTABUP`)
that record the interpreter instructions that were executed along a trace.
They are the easiest way to see *what* was traced without reading raw IR.

### Enabling debugstrs

Set `YKD_LOG_IR=debugstrs` (or include `debugstrs` in a comma-separated list
for `YKD_LOG_IR`). Use `YKD_LOG=4` to see tracing start/stop messages, and
`YKD_SERIALISE_COMPILATION=1` if you want compilation to happen one trace at a
time so that debugstr output is easier to follow.

Example:

```bash
YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3 ./your_lua_program.lua
```

### What the debugstr labels mean

In the output you will see blocks like:

- **`--- Begin debugstrs: header: file.lua:34: GETTABUP ---`**  
  A **header** trace: it started from a **ControlPoint**. It may be a loop
  (TraceEnd: Loop), a coupler (TraceEnd: Coupler), or a return (TraceEnd: Return).

- **`--- Begin debugstrs: connector: file.lua:42: ADDI ---`**  
  A **connector** trace: ControlPoint start, **Coupler** end. It connects
  execution from one place to another (e.g. from an inner loop header to an
  outer loop).

- **`--- Begin debugstrs: side-trace: file.lua:58: LTI ---`**  
  A **side-trace**: it started from a **Guard** (e.g. a type check or branch
  that failed). It will end in Coupler or Return, never Loop.

The lines under each block are the sequence of interpreter instructions
(bytecode locations) that were recorded along that trace. That tells you
exactly which path through the interpreter was captured.


## Simple test programs and existing tests

A good way to build intuition is to run small Lua programs with debugstrs
and watch the order and kinds of traces (header vs connector vs side-trace).

There are several tests in `tests/lua/` that do exactly this; their
expected stderr is documented in the comments at the top of each file. For
example:

- **`tests/lua/for_loop.lua`** — simple loop; one header trace (ControlPoint +
  Loop), then execution in JIT code.
- **`tests/lua/while_loop.lua`** — similar; one header trace.
- **`tests/lua/sidetrace.lua`** — loop with a branch; you see a header trace
  and then a **side-trace** when the other branch is taken (Guard + Coupler
  or Return).
- **`tests/lua/sidetrace_to_loop.lua`** — more complex; header, then
  side-trace, then another header (e.g. loop formed from a side exit).
- **`tests/lua/nested_loops.lua`** — nested loops; you see **connector**
  traces (e.g. `connector: nested_loops.lua:42: ADDI`) linking inner and
  outer loop headers.

Run them with the same env vars as above (and the test harness if needed);
comparing the debugstr output to the Lua source and to the TraceStart/TraceEnd
table will make the trace kinds concrete.

### Minimal runnable example

Save this as `loop.lua`:

```lua
for i = 0, 5 do
  io.write(tostring(i))
end
```

Then run with the yk-instrumented Lua interpreter (built from `tests/yklua`);
the exact binary name depends on your build (e.g. `lua` or `yklua`):

```bash
YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3 /path/to/yklua loop.lua 2>&1
```

You should see: tracing start/stop at some bytecode location, then a single
`--- Begin debugstrs: header: ... ---` block listing the instructions in
the loop, then `yk-execution: enter-jit-code`. That one trace is a
ControlPoint+Loop; the debugstrs are the interpreter path that was recorded.


## Viewing j2 trace graphs (HIR trace kind)

When using the **j2** compiler (`YK_JITC=j2`), the **jit-pre-opt** stage logs
the HIR (high-level IR) for each trace. Each HIR module is prefixed with a
JSON line that gives the trace kind in terms of TraceStart and TraceEnd:

```text
; { "trid": "...", "start": { "kind": "ControlPoint" }, "end": { "kind": "Loop" } }
```

So you can see directly whether a trace is ControlPoint+Loop, ControlPoint+Coupler,
ControlPoint+Return, or Guard+Coupler/Return. That is the “j2 trace graph” in
terms of trace *kind*: the rest of the dump is the HIR block(s) (entry, optional
peel, etc.).

To view it:

```bash
YK_JITC=j2 YKD_LOG_IR=jit-pre-opt YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3 ./your_program
```

For a single loop, you should see one trace with `"start": { "kind": "ControlPoint" }`
and `"end": { "kind": "Loop" }`. Connectors show `"kind": "Coupler"` under `"end"`,
and side-traces show `"kind": "Guard"` under `"start"`.


## Producing a trace

### `YKD_LOG_IR`

`YKD_LOG_IR=[<path>:]<irstage_1>[,...,<irstage_n>]` logs IR from different stages
to `path`. The special value `-` (i.e. a single dash) can be used for `<path>`
to indicate stderr.

The following `ir_stage`s are supported:

 - `aot`: the entire AOT IR for the interpreter.
 - `debugstrs`: the sequence of interpreter locations (e.g. file:line: opcode) for each trace; see [Debugstrs: a practical way to see what was traced](#debugstrs-a-practical-way-to-see-what-was-traced).
 - `jit-pre-opt`: the JIT IR trace before optimisation (or, with `YK_JITC=j2`, the HIR with trace kind JSON).
 - `jit-post-opt`: the JIT IR trace after optimisation.
 - `jit-asm`: the assembler code of the compiled JIT IR trace.
 - `jit-asm-full`: the assembler code of the compiled JIT IR trace with
   instruction offsets and virtual addresses annotated.
