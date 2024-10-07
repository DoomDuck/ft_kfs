# Program entrypoint symbol
ENTRY(_start)

SECTIONS
{
  . = 1M,

  .text BLOCK(4K): ALIGN(4K)
  {
      *(.multiboot)
      *(.text*)
  }

  .rodata BLOCK(4K): ALIGN(4K)
  {
      *(.rodata*)
  }

  .data BLOCK(4K): ALIGN(4K)
  {
      *(.data*)
      *(.bss*)
  }

  .tdata BLOCK(4K): ALIGN(4K)
  {
      *(.tdata*)
      *(.tbss*)
      # TODO(Clément): See why this is here
      . += 8;
  }

  /DISCARD/ : {
    *(.comment*)
    *(.eh_frame*)

    *(.gcc_except_table*)
    *(.note*)
  }
}
