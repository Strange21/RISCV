

/*************************************************************************************************
1.Text an readonly sections are placed in FLASH.
2.sdata ,data ,bss ,sbss are placed in RAM(vma) which are copied from FLASH(lma) in startup code.
3.Stack is started from end of RAM uptil 1KB.
4.After Stack end,Heap occupies memory region between end of tbss section and stack section. 
***************************************************************************************************/
OUTPUT_ARCH( "riscv" )
ENTRY(_start)

_STACK_SIZE = 1K;

MEMORY
{
  FLASH (rx)                 : ORIGIN = 0x90001000, LENGTH = 128M
  RAM (rwx)                  : ORIGIN = 0x80000000, LENGTH = 128K
  PSRAM(rwx)                 : ORIGIN = 0xB0000000, LENGTH = 512M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

