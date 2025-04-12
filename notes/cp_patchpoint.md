CP_PATCHPOINT=0 CP_VERBOSE=1 YKB_TRACER=swt  cargo test ::idempotent.c -- --nocapture

src_rbp: 0x7fff5db50d10, reg_store: 0x7fff5db50c00, src_frame_size: 0xa0, dst_frame_size: 0x90, rbp_offset_reg_store: 0x110
Register2Register - src: Register(14, 8, [-152]) dst: Register(14, 8, [-104])
Register2Register - src: Register(3, 8, []) dst: Register(13, 8, [-72])
Register2Register - src: Register(15, 8, [-96]) dst: Register(15, 8, [-64])
Register2Register - src: Register(12, 8, [-64]) dst: Register(12, 8, [-112])
Register2Register - src: Register(13, 8, []) dst: Register(3, 8, [])



CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt  cargo test ::idempotent.c -- --nocapture

Register2Register - src: Register(14, 8, [-152]) dst: Register(14, 8, [-104])
Register2Register - src: Register(3, 8, []) dst: Register(13, 8, [-72])
Register2Register - src: Register(15, 8, [-96]) dst: Register(15, 8, [-64])
Register2Register - src: Register(12, 8, [-64]) dst: Register(12, 8, [-112])
Register2Register - src: Register(13, 8, [-72]) dst: Register(3, 8, [-80])


## udiv.c

CP_PATCHPOINT=0 CP_VERBOSE=1 YKB_TRACER=swt  cargo test ::udiv.c -- --nocapture

Register2Register - src: Register(14, 8, []) dst: Register(13, 8, [])
Register2Register - src: Register(13, 8, [-80]) dst: Register(14, 8, [-80])
Register2Register - src: Register(15, 8, [-72]) dst: Register(12, 8, [-64])
Register2Register - src: Register(12, 8, [-56]) dst: Register(15, 8, [-72])
Register2Register - src: Register(3, 8, [-64]) dst: Register(3, 8, [-88])
Register2Register - src: Register(0, 8, []) dst: Register(0, 8, [])


CP_PATCHPOINT=1 CP_VERBOSE=1 YKB_TRACER=swt  cargo test ::udiv.c -- --nocapture

Register2Register - src: Register(14, 8, []) dst: Register(13, 8, [])
Register2Register - src: Register(13, 8, [-80]) dst: Register(14, 8, [-80])
Register2Register - src: Register(15, 8, [-72]) dst: Register(12, 8, [-64])
Register2Register - src: Register(12, 8, [-56]) dst: Register(15, 8, [-72])
Register2Register - src: Register(3, 8, [-64]) dst: Register(3, 8, [-88])
Register2Register - src: Register(0, 8, [-88]) dst: Register(0, 8, [-96])

