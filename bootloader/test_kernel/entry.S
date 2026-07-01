.section .text
.global _start

.equ COM1_BASE, 0x3f8        # First PC serial port base address.
.equ COM1_LSR, COM1_BASE + 5 # Line status register.
.equ COM1_THR_EMPTY, 0x20    # Transmit buffer can accept a byte.
.equ DEBUGCON_PORT, 0xe9     # QEMU debug console proof port.

_start:
    cli
    lea entry_reached_message(%rip), %rsi
    call debugcon_write_string

    call serial_init
    lea entry_reached_message(%rip), %rsi
    call serial_write_string

    mov $0xb8000, %rdi
    movw $0x0f4b, 0(%rdi)
    movw $0x0f45, 2(%rdi)
    movw $0x0f52, 4(%rdi)
    movw $0x0f4e, 6(%rdi)
    movw $0x0f45, 8(%rdi)
    movw $0x0f4c, 10(%rdi)
    movw $0x0f20, 12(%rdi)
    movw $0x0f4f, 14(%rdi)
    movw $0x0f4b, 16(%rdi)
    lea test_message(%rip), %rax
    mov test_counter(%rip), %rbx
    mov %rbx, test_scratch(%rip)
1:
    hlt
    jmp 1b

serial_init:
    mov $(COM1_BASE + 1), %dx
    mov $0x00, %al
    out %al, %dx
    mov $(COM1_BASE + 3), %dx
    mov $0x80, %al
    out %al, %dx
    mov $(COM1_BASE + 0), %dx
    mov $0x03, %al
    out %al, %dx
    mov $(COM1_BASE + 1), %dx
    mov $0x00, %al
    out %al, %dx
    mov $(COM1_BASE + 3), %dx
    mov $0x03, %al
    out %al, %dx
    mov $(COM1_BASE + 2), %dx
    mov $0xc7, %al
    out %al, %dx
    mov $(COM1_BASE + 4), %dx
    mov $0x0b, %al
    out %al, %dx
    ret

serial_write_string:
    lodsb
    test %al, %al
    jz 2f
1:
    mov $COM1_LSR, %dx
    in %dx, %al
    test $COM1_THR_EMPTY, %al
    jz 1b
    mov $COM1_BASE, %dx
    mov -1(%rsi), %al
    out %al, %dx
    jmp serial_write_string
2:
    ret

debugcon_write_string:
    lodsb
    test %al, %al
    jz 1f
    mov $DEBUGCON_PORT, %dx
    out %al, %dx
    jmp debugcon_write_string
1:
    ret

.section .rodata
.global test_message
test_message:
    .asciz "MahkrabKNL ELF loader test kernel"
entry_reached_message:
    .asciz "\r\nKERNEL ENTRY REACHED\r\n"

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
