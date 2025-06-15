## compile bd test

1. set: `USE_LOCAL_DIR=true` in tests/langtest_c.rs

2. run: 

```shell
YKB_TRACER=swt cargo test --release ::bf.bench.c
```

3. Varify release build:

```shell
$ file ~/temp/bf.bench
/home/pd/temp/bf.bench: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, not stripped
```


4. Hyperfine runs

## SWT MULTI - Manual patch

```shell
[13:42:15] $ YKD_SERIALISE_COMPILATION=1  YKB_TRACER=swt  hyperfine '~/temp/bf.bench ~/yk-fork/tests/bf_examples/loop.22000000.bf'
Benchmark 1: ~/temp/bf.bench ~/yk-fork/tests/bf_examples/loop.22000000.bf
  Time (mean ± σ):     776.5 ms ±   8.9 ms    [User: 724.1 ms, System: 52.9 ms]
  Range (min … max):   767.6 ms … 792.3 ms    10 runs
```

## SWT MULTI
```
YKD_SERIALISE_COMPILATION=1  YKB_TRACER=swt  hyperfine '~/temp/bf.bench ./tests/bf_examples/loop.22000000.bf'
Benchmark 1: ~/temp/bf.bench ./tests/bf_examples/loop.22000000.bf
  Time (mean ± σ):      1.237 s ±  0.002 s    [User: 1.166 s, System: 0.071 s]
  Range (min … max):    1.234 s …  1.239 s    10 runs
```

## SWT
```
YKD_SERIALISE_COMPILATION=1  YKB_TRACER=swt  hyperfine '~/temp/yk/bf.bench ~/yk-fork/tests/bf_examples/loop.22000000.bf'
Benchmark 1: ~/temp/yk/bf.bench ~/yk-fork/tests/bf_examples/loop.22000000.bf
  Time (mean ± σ):      2.441 s ±  0.015 s    [User: 2.384 s, System: 0.058 s]
  Range (min … max):    2.421 s …  2.461 s    10 runs

```
## HWT

```
YKD_SERIALISE_COMPILATION=1  YKB_TRACER=hwt  hyperfine '~/temp/bf.bench.hwt ~/yk-fork/tests/bf_examples/loop.22000000.bf' 
Benchmark 1: ~/temp/bf.bench.hwt ~/yk-fork/tests/bf_examples/loop.22000000.bf
  Time (mean ± σ):     814.5 ms ±   3.1 ms    [User: 742.3 ms, System: 73.1 ms]
  Range (min … max):   807.4 ms … 819.6 ms    10 runs
```

# use for debugging
YKD_SERIALISE_COMPILATION=1  YKD_LOG=4 YKB_TRACER=swt CP_VERBOSE=1 hyperfine '~/temp/bf.bench ./test.bf'
```

https://github.com/rizinorg/rizin/tree/dev
rizin

## MAnually patching with rizin

Load:
```
aaa
```
List all funcitons: 
```
afl
```

Enable write mode:
```
oo+
```

Patch function with return instruction:
```shell
[0x00209c40]> s sym.imp.__yk_trace_basicblock_dummy
[0x0020b610]> pdf
            ; XREFS(76)
┌ sym.imp.__yk_trace_basicblock_dummy();
└           0x0020b610      jmp   qword reloc.__yk_trace_basicblock_dummy ; [0x20ddd8:8]=0x20b616
[0x0020b610]> wx C3 # patch first instruction as routine return
[0x0020b610]> pdf
            ; XREFS(76)
┌ sym.imp.__yk_trace_basicblock_dummy();
│           0x0020b610      ret
```