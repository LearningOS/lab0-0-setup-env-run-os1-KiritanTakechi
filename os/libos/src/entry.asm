.section .text.entry
.globl _start
.globl boot_stack_lower_bound
.globl boot_stack_top

_start:
    la sp, boot_stack_top
    call main

.section .bss.stack

boot_stack_lower_bound:
    .space 4096 * 16
boot_stack_top: