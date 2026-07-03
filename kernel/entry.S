.section .text
.global _start

.equ COM1_BASE, 0x3f8        # First PC serial port base address.
.equ COM1_LSR, COM1_BASE + 5 # Line status register.
.equ COM1_THR_EMPTY, 0x20    # Transmit buffer can accept a byte.
.equ DEBUGCON_PORT, 0xe9     # QEMU debug console proof port.
# Offsets mirror bootprotocol::BootInfo.
.equ BI_FRAMEBUFFER_BASE, 0  # BootInfo framebuffer address.
.equ BI_FRAMEBUFFER_SIZE, 8  # BootInfo framebuffer byte count.
.equ BI_WIDTH, 16            # BootInfo horizontal resolution.
.equ BI_HEIGHT, 20           # BootInfo vertical resolution.
.equ BI_PIXELS_PER_LINE, 24  # BootInfo framebuffer stride in pixels.
.equ BI_PIXEL_FORMAT, 28     # BootInfo UEFI GOP pixel format.
.equ PAINT_MAX_WIDTH, 500    # Maximum proof rectangle width.
.equ PAINT_MAX_HEIGHT, 500   # Maximum proof rectangle height.
.equ PIXEL_BYTES, 4          # GOP direct framebuffer pixel size.

_start:
    cli
    mov %rdi, %r12
    lea entry_reached_message(%rip), %rsi
    call debugcon_write_string

    call serial_init
    lea entry_reached_message(%rip), %rsi
    call serial_write_string

    call paint_framebuffer

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

paint_framebuffer:
    test %r12, %r12
    jz framebuffer_invalid

    mov BI_FRAMEBUFFER_BASE(%r12), %r8
    mov BI_FRAMEBUFFER_SIZE(%r12), %r9
    mov BI_WIDTH(%r12), %r10d
    mov BI_HEIGHT(%r12), %r11d
    mov BI_PIXELS_PER_LINE(%r12), %ebx
    mov BI_PIXEL_FORMAT(%r12), %edx

    test %r8, %r8
    jz framebuffer_invalid
    test %r9, %r9
    jz framebuffer_invalid
    test %r10d, %r10d
    jz framebuffer_invalid
    test %r11d, %r11d
    jz framebuffer_invalid
    cmp %r10d, %ebx
    jb framebuffer_invalid
    cmp $3, %edx
    jae framebuffer_invalid

    cmp $PAINT_MAX_WIDTH, %r10d
    jbe 1f
    mov $PAINT_MAX_WIDTH, %r10d
1:
    cmp $PAINT_MAX_HEIGHT, %r11d
    jbe 2f
    mov $PAINT_MAX_HEIGHT, %r11d
2:
    shl $2, %rbx
    mov %r8, %r13
    mov %r11d, %r14d

3:
    test %r14d, %r14d
    jz framebuffer_painted
    mov %r13, %rdi
    mov %r10d, %ecx

4:
    test %ecx, %ecx
    jz 5f
    movl $0x0000ff00, (%rdi)
    add $PIXEL_BYTES, %rdi
    dec %ecx
    jmp 4b

5:
    add %rbx, %r13
    dec %r14d
    jmp 3b

framebuffer_painted:
    lea framebuffer_painted_message(%rip), %rsi
    call debugcon_write_string
    ret

framebuffer_invalid:
    lea framebuffer_invalid_message(%rip), %rsi
    call debugcon_write_string
    ret

.section .rodata
.global test_message
test_message:
    .asciz "MahkrabKNL ELF loader test kernel"
entry_reached_message:
    .asciz "\r\nKERNEL ENTRY REACHED\r\n"
framebuffer_painted_message:
    .asciz "FRAMEBUFFER PAINTED\r\n"
framebuffer_invalid_message:
    .asciz "FRAMEBUFFER INVALID\r\n"

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
