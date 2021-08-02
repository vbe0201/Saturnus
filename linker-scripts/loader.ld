OUTPUT_ARCH(aarch64)
ENTRY(_start)

PHDRS
{
  loader  PT_LOAD FLAGS(7);
  dynamic PT_DYNAMIC;
}

SECTIONS
{
  PROVIDE(__start__ = 0);

  . = __start__;

  .text : {
    HIDDEN(__text_start__ = .);
    KEEP(*(.text.crt0))
    *(.text .text.*)
    HIDDEN(__text_end__ = .);
  } :loader

  /* Trampolines and alike. */
  .plt : {
    *(.plt .plt.*)
  } :loader

  .vectors ALIGN(4K) : {
    __vectors_start__ = .;
    KEEP(*(.vectors))
    __vectors_end__ = .;
  } :loader

  . = ALIGN(8);
  __rodata_start__ = .;

  .rodata : {
    *(.rodata .rodata.*)
  } :loader

  /* Symbols that are needed for relocation lookup. */
  .hash     : { *(.hash) } :loader
  .gnu.hash : { *(.gnu.hash) } :loader
  .dynsym   : { *(.dynsym .dynsym.*) } :loader
  .dynstr   : { *(.dynstr .dynstr.*) } :loader
  .rela.dyn : { *(.rela.*) } :loader

  .dynamic : {
    HIDDEN(__dynamic_start__ = .);
    *(.dynamic)
  } :loader :dynamic

  /* All exception handling sections. */
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

  .data : {
    *(.data .data.*)
    SORT(CONSTRUCTORS)
  } :loader

  __got_start__ = .;

  .got : {
    *(.got)
    *(.igot)
  } :loader

  .got.plt : {
    *(.got.plt)
    *(.igot.plt)
  } :loader

  __got_end__ = .;

  .init_array ALIGN(8) : {
    PROVIDE(__init_array_start__ = .);
    KEEP(*(SORT_BY_INIT_PRIORITY(.init_array.*)))
    KEEP(*(.init_array))
    PROVIDE(__init_array_end__ = .);
  } :loader

  .fini_array ALIGN(8) : {
    PROVIDE(__fini_array_start__ = .);
    KEEP(*(SORT_BY_INIT_PRIORITY(.fini_array.*)))
    KEEP(*(.fini_array))
    PROVIDE(__fini_array_end__ = .);
  } :loader

  __data_end__ = .;

  .bss : {
    . = ALIGN(8);
    HIDDEN(__bss_start__ = .);
    *(.bss .bss.*)
    *(COMMON)
    *(.dynbss)

    . = ALIGN(16);
    __stack_bottom__ = .;
    . += 0x1000; /* Reserve 4kB of stack memory. */
    __stack_top__ = .;
    HIDDEN(__bss_end__   = .);
  } :loader

  PROVIDE(__end__ = ABSOLUTE(.));

  /DISCARD/ : {
    *(.group)
    *(.comment)
    *(.note)
    *(.interp)
  }
}
