-- Run-time:
--   env-var: YK_HOT_THRESHOLD=2
--   env-var: YK_SIDETRACE_THRESHOLD=2
--   env-var: YKD_LOG=4
--   env-var: YKD_LOG_IR=debugstrs
--   env-var: YKD_SERIALISE_COMPILATION=1
--   stderr:
--     h1
--     yk-tracing: start-tracing: sidetrace_to_loop.lua:48: GTI
--     yk-tracing: stop-tracing: sidetrace_to_loop.lua:48: GTI
--     --- Begin debugstrs: header: sidetrace_to_loop.lua:48: GTI ---
--       sidetrace_to_loop.lua:48: GTI
--       sidetrace_to_loop.lua:49: TEST
--       sidetrace_to_loop.lua:49: JMP
--       sidetrace_to_loop.lua:58: ADDI
--       sidetrace_to_loop.lua:58: JMP
--     --- End debugstrs ---
--     h2
--     yk-execution: enter-jit-code: sidetrace_to_loop.lua:48: GTI
--     yk-execution: deoptimise ...
--     yk-execution: enter-jit-code: sidetrace_to_loop.lua:48: GTI
--     yk-execution: deoptimise ...
--     h3
--     yk-execution: enter-jit-code: sidetrace_to_loop.lua:48: GTI
--     yk-execution: deoptimise ...
--     yk-tracing: start-side-tracing: sidetrace_to_loop.lua:48: GTI
--     yk-tracing: stop-tracing: sidetrace_to_loop.lua:53: FORLOOP
--     yk-tracing: start-tracing: sidetrace_to_loop.lua:53: FORLOOP
--     yk-tracing: stop-tracing: sidetrace_to_loop.lua:53: FORLOOP
--     --- Begin debugstrs: header: sidetrace_to_loop.lua:53: FORLOOP ---
--       sidetrace_to_loop.lua:53: FORLOOP
--     --- End debugstrs ---
--     --- Begin debugstrs: side-trace: sidetrace_to_loop.lua:48: GTI ---
--       sidetrace_to_loop.lua:52: TEST
--       sidetrace_to_loop.lua:53: LOADI
--       sidetrace_to_loop.lua:53: LOADI
--       sidetrace_to_loop.lua:53: LOADI
--       sidetrace_to_loop.lua:53: FORPREP
--     --- End debugstrs ---
--     yk-execution: enter-jit-code: sidetrace_to_loop.lua:53: FORLOOP
--     yk-execution: deoptimise ...
--     yk-execution: enter-jit-code: sidetrace_to_loop.lua:48: GTI
--     yk-execution: deoptimise ...
--     yk-tracing: start-side-tracing: sidetrace_to_loop.lua:48: GTI
--     exit

function h(i, b1, b2)
  while i > 0 do
    if b1 then
     --
    else
      if b2 then
        for j = 0, 3 do
          --
        end
      end
    end
    i = i - 1
  end
end

io.stderr:write("h1\n")
h(3, true, true)
io.stderr:write("h2\n")
h(1, false, false)
io.stderr:write("h3\n")
h(1, false, true)
io.stderr:write("exit\n")
