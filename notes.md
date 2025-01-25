# NOTES

## Test with Indirect live vars

bf.O3.c:
@@ src live vars - smid: Opt
0 - Register(14, 8, [])
1 - Indirect(6, -48, 8)
2 - Indirect(6, -56, 8)
3 - Register(13, 8, [])
4 - Register(12, 8, [])
5 - Register(15, 8, [])
6 - Indirect(6, -96, 8)
7 - Register(3, 8, [])
@@ dst live vars - smid: UnOpt
0 - Register(13, 8, [])
1 - Indirect(6, -80, 8)
2 - Indirect(6, -88, 8)
3 - Register(14, 8, [])
4 - Register(3, 8, [])
5 - Register(12, 8, [])
6 - Indirect(6, -48, 8)
7 - Register(15, 8, [])
