-- Loop with a branch: one path is the "header", the other triggers a "side-trace".
-- Run with: YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3 YK_SIDETRACE_THRESHOLD=2 <yklua> branching_loop.lua 2>&1
for i = 0, 10 do
  if i < 5 then
    io.write("L")
  else
    io.write("R")
  end
end
io.write("\n")
