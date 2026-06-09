.section .text
.global _start

_start:
    cli
    lea test_message(%rip), %rax
    mov test_counter(%rip), %rbx
    mov %rbx, test_scratch(%rip)
1:
    hlt
    jmp 1b

.section .rodata
.global test_message
test_message:
    .asciz "MahkrabKNL ELF loader test kernel"

.section .data
.global test_counter
test_counter:
    .quad 0x1122334455667788

.section .bss
.balign 16
.global test_stack
test_stack:
    .skip 4096
.global test_scratch
test_scratch:
    .skip 8

.section .note.test, "a", @note
    .balign 4
    .long 8
    .long 4
    .long 0x4d4b4e4c
    .asciz "MKNL"
    .balign 4
    .long 1
