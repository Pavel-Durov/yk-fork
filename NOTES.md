## Cloned control point
  call void (i64, i32, ptr, i32, ...) @llvm.experimental.patchpoint.void(i64 1, i32 13, ptr @__ykrt_control_point, i32 3, ptr %28, ptr %7, i64 1, ptr %6, ptr %7, ptr %8, ptr %9, ptr %28), !dbg !119

call void (i64, i32, ptr, i32, ...) @llvm.experimental.patchpoint.void(
    i64 1,          ; ID
    i32 13,         ; NumBytes
    ptr @__ykrt_control_point, ; Target function
    i32 3,          ; NumArgs
    ptr %28,        ; Argument 1
    ptr %7,         ; Argument 2
    i64 1,          ; Argument 3
    ptr %6,         ; Stack map live variables
    ptr %7,
    ptr %8,
    ptr %9,
    ptr %28
), !dbg !119

ID: 1


## Original control point:

ID: 0
  call void (i64, i32, ptr, i32, ...) @llvm.experimental.patchpoint.void(
    i64 0,          ; ID
    i32 13,         ; NumBytes
    ptr @__ykrt_control_point, ; Target function
    i32 3,          ; NumArgs
    ptr %28,        ; Argument 1
    ptr %7,         ; Argument 2
    i64 0,          ; Argument 3
    ptr %6,         ; Stack map live variables
    ptr %7,
    ptr %8,
    ptr %9,
    ptr %28
), !dbg !74

## General form
```
call void @llvm.experimental.patchpoint.void(
    i64 <ID>,
    i32 <NumBytes>,
    ptr <Target>,
    i32 <NumArgs>,
    <Args...>,
    <Live Variables...>
)
```
D (i64) - Unique identifier for the patchpoint.
NumBytes (i32) - Number of bytes to reserve for the patchpoint (size of the machine code).
Target (ptr) - Pointer to the target function.
NumArgs (i32) - Number of arguments to pass to the target function.
Args (...) - Arguments to pass to the target function.
Live Variables (...) - Variables that should be considered live at the safepoint.