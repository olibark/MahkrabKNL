.section .text
.global _start

_start:
    cli
1:
    hlt
    jmp 1b
