OUTPUT_ARCH(aarch64)
ENTRY(__saturnus_start)

PHDRS
{
  loader  PT_LOAD FLAGS(7);
  dynamic PT_DYNAMIC;
}

SECTIONS
{
  PROVIDE(__start__ = 0);

  . = __start__;

  .r0 : {
    KEEP(*(.r0 .r0.*))
    . = ALIGN(8);
  } :loader

  /* Executable code section. */
  .text : {
    *(.text .text.*)
    . = ALIGN(8);
  } :loader

  /* Trampolines and alike. */
  .plt : {
    *(.plt .plt.*)
    . = ALIGN(8);
  } :loader

  /* Exception vector tables. */
  .vectors ALIGN(2K) : {
    __vectors_start__ = .;
    KEEP(*(.vectors))
    __vectors_end__ = .;
  } :loader

  . = ALIGN(8);
  __rodata_start__ = .;

  /* Read-only data section. */
  .rodata : {
    *(.rodata .rodata.*)
  } :loader

  /* Relocation sections and those needed for relocation lookup. */
  .hash     : { *(.hash)             } :loader
  .gnu.hash : { *(.gnu.hash)         } :loader
  .dynsym   : { *(.dynsym .dynsym.*) } :loader
  .dynstr   : { *(.dynstr .dynstr.*) } :loader
  .rela.dyn : { *(.rela.*)           } :loader

  .dynamic : {
    HIDDEN(__dynamic_start__ = .);
    *(.dynamic)
  } :loader :dynamic

  /* GNU exception handling sections. */
  .gcc_except_table : { *(.gcc_except_table .gcc_except_table.*) } :loader
  .eh_frame_hdr : {
    __eh_frame_hdr_start__ = .;
    *(.eh_frame_hdr)
    *(.eh_frame_entry .eh_frame_entry.*)
    __eh_frame_hdr_end__ = .;
  } :loader
  .eh_frame : { KEEP(*(.eh_frame)) } :loader

  __rodata_end__ = .;

  __data_start__ = .;

  /* Read-writable data section. */
  .data ALIGN(8) : {
    *(.data .data.*)
    SORT(CONSTRUCTORS)
  } :loader

  __got_start__ = .;

  /* Dynamic relocations for the code. */

  .got : {
    *(.got)
    *(.igot)
  } :loader

  .got.plt : {
    *(.got.plt)
    *(.igot.plt)
  } :loader

  __got_end__ = .;

  __data_end__ = .;

  /* Uninitialized locals section. */
  .bss ALIGN(16) : {
    HIDDEN(__bss_start__ = .);
    *(.bss .bss.*)
    *(COMMON)
    *(.dynbss)

    /* Reserve 4KiB of stack memory. */
    . = ALIGN(16);
    __stack_bottom__ = .;
    . += 0x1000;
    __stack_top__ = .;
    HIDDEN(__bss_end__ = .);
  } :loader

  /* Mark the end of the Kernel Loader binary. */
  PROVIDE(__end__ = ABSOLUTE(.));

  /* Discard sections that difficult post-processing. */
  /DISCARD/ : {
    *(.group)
    *(.comment)
    *(.note)
    *(.interp)
  }
}
