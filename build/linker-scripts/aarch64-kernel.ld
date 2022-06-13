OUTPUT_ARCH(aarch64)
ENTRY(__saturnus_start)

/* The page size that is used by the Kernel. */
PAGE_SIZE = 4K;

PHDRS
{
  text    PT_LOAD FLAGS(5);
  rodata  PT_LOAD FLAGS(4);
  data    PT_LOAD FLAGS(6);
  dynamic PT_DYNAMIC;
}

SECTIONS
{
  PROVIDE(__start__ = 0);

  . = __start__;

  .r0 : {
    KEEP(*(.r0 .r0.*))
    . = ALIGN(8);
  } :text

  /* Executable code section. */
  .text : {
    *(.text .text.*)
    . = ALIGN(8);
  } :text

  /* Trampolines and alike. */
  .plt : {
    *(.plt .plt.*)
    . = ALIGN(8);
  } :text

  /* Exception vector tables. */
  .vectors ALIGN(2K) : {
    __vectors_start__ = .;
    KEEP(*(.vectors))
    __vectors_end__ = .;
  } :text

  . = ALIGN(PAGE_SIZE);
  __text_end__ = .;

  __rodata_start__ = .;

  /* Read-only data section. */
  .rodata : {
    *(.rodata .rodata.*)
    . = ALIGN(8);
  } :rodata

  /* Read-only globals after relocation. */
  .data.rel.ro : {
    *(.data.rel.ro .data.rel.ro.*)
    . = ALIGN(8);
  } :rodata

  /* Relocation sections and those needed for relocation lookup. */
  .hash     : { *(.hash)             } :rodata
  .gnu.hash : { *(.gnu.hash)         } :rodata
  .dynsym   : { *(.dynsym .dynsym.*) } :rodata
  .dynstr   : { *(.dynstr .dynstr.*) } :rodata
  .rela.dyn : { *(.rela.*)           } :rodata

  .dynamic : {
    HIDDEN(__dynamic_start__ = .);
    *(.dynamic)
  } :rodata :dynamic

  /* GNU exception handling sections. */
  .gcc_except_table : ONLY_IF_RO { *(.gcc_except_table .gcc_except_table.*) } :rodata
  .eh_frame_hdr : {
    __eh_frame_hdr_start__ = .;
    *(.eh_frame_hdr)
    *(.eh_frame_entry .eh_frame_entry.*)
    __eh_frame_hdr_end__ = .;
  } :rodata
  .eh_frame : ONLY_IF_RO { KEEP(*(.eh_frame)) } :rodata

  __got_start__ = .;

  .got : {
    *(.got)
    *(.igot)
  } :rodata

  .got.plt : {
    *(.got.plt)
    *(.igot.plt)
  } :rodata

  __got_end__ = .;

  . = ALIGN(PAGE_SIZE);
  __rodata_end__ = .;

  __data_start__ = .;

  /* GNU exception handling sections. */
  .gcc_except_table : ONLY_IF_RW { *(.gcc_except_table .gcc_except_table.*) } :data
  .eh_frame : ONLY_IF_RW { KEEP(*(.eh_frame)) } :data

  /* Read-writable data section. */
  .data ALIGN(8) : {
    *(.data .data.*)
    SORT(CONSTRUCTORS)
    . = ALIGN(8);
  } :data

  __data_end__ = .;

  __bss_start__ = .;

  /* Runtime dynamic relocation table. */
  .rela.dyn : { *(.rela.*) } :data

  /* Uninitialized locals section. */
  .bss ADDR(.rela.dyn) (NOLOAD) : {
    *(.bss .bss.*)
    *(COMMON)
    *(.dynbss)
  }

  . = ALIGN(PAGE_SIZE);
  __bss_end__ = .;

  /* Mark the end of the Kernel binary. */
  __end__ = ABSOLUTE(.);

  /* Discard sections that difficult post-processing. */
  /DISCARD/ : {
    *(.group)
    *(.comment)
    *(.note)
    *(.interp)
  }
}
