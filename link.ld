OUTPUT_ARCH(riscv)
ENTRY(entry)
SECTIONS
{
    .text 0xC0000000: {
        *(.text*) 
        . = ALIGN(8);
    }
    .rodata : {
        . = ALIGN(8);
        __global_pointer$ = . + 0x800;
        *(.rodata*)
        *(.srodata*)
        . = ALIGN(8);
    }
    .data : { 
        . = ALIGN(8);
        *(.sdata*)
        *(.data*)
        . = ALIGN(8);
    } 
    .bss : {
        . = ALIGN(8);
        _bss_start = .;
        *(.sbss*)
        *(.bss*)
        *(COMMON)
        . = ALIGN(8);
        _bss_end = .;
    }
    . = ALIGN(4096);
    .kstack : {
        . = ALIGN(8);
        . += 5*4096*4;
        _stack_start = .;
    }
}
