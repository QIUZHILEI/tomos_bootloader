.global entry
entry:
    csrr t0, mhartid
    slli t1, t0 ,14
    la t2, _stack_start
    sub t2, t2, t1
    mv sp, t2
    j rust_entry