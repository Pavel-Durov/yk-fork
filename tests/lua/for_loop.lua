-- Run-time:
--   env-var: YK_HOT_THRESHOLD=3
--   env-var: YKD_LOG=4
--   env-var: YKD_LOG_IR=debugstrs
--   env-var: YKD_SERIALISE_COMPILATION=1
--   stderr:
--     0
--     1
--     2
--     yk-tracing: start-tracing: for_loop.lua:34: GETTABUP
--     3
--     yk-tracing: stop-tracing: for_loop.lua:34: GETTABUP
--     --- Begin debugstrs: header: for_loop.lua:34: GETTABUP ---
--       for_loop.lua:34: GETTABUP
--       for_loop.lua:34: GETFIELD
--       for_loop.lua:34: SELF
--       for_loop.lua:34: GETTABUP
--       for_loop.lua:34: MOVE
--       for_loop.lua:34: CALL
--       for_loop.lua:34: LOADK
--       for_loop.lua:34: CALL
--       for_loop.lua:35: ADDI
--       for_loop.lua:33: FORLOOP
--     --- End debugstrs ---
--     4
--     yk-execution: enter-jit-code: for_loop.lua:34: GETTABUP
--     5
--     6
--     yk-execution: deoptimise ...
--     exit

local x = 0
for _ = 0, 6 do
  io.stderr:write(tostring(x), "\n")
  x = x + 1
end
io.stderr:write("exit\n")
