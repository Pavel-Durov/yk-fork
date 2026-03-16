-- Minimal loop for the "Understanding yk Traces" course.
-- Run with: YKD_LOG=4 YKD_LOG_IR=debugstrs YKD_SERIALISE_COMPILATION=1 YK_HOT_THRESHOLD=3 <yklua> simple_loop.lua 2>&1
for i = 0, 6 do
  io.write(tostring(i))
end
io.write("\n")
