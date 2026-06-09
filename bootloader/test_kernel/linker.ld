ENTRY(_start)

PHDRS
{
    text PT_LOAD FLAGS(5);
    rodata PT_LOAD FLAGS(4);
    data PT_LOAD FLAGS(6);
    note PT_NOTE FLAGS(4);
}

SECTIONS
{
    . = 1M;

    .text : ALIGN(4K)
    {
        *(.text*)
    } :text

    .rodata : ALIGN(4K)
    {
        *(.rodata*)
    } :rodata

    .note.test : ALIGN(4)
    {
        *(.note.test*)
    } :rodata :note

    .data : ALIGN(4K)
    {
        *(.data*)
    } :data

    .bss : ALIGN(4K)
    {
        *(.bss*)
        *(COMMON)
    } :data
}
