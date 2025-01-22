
OUTPUT_ARCH( "riscv" )
ENTRY(_start)

_STACK_SIZE = 1K;

MEMORY
{
  FLASH (rx)                 : ORIGIN = 0x90001000, LENGTH = 128M
  RAM (rwx)                  : ORIGIN = 0x80000000, LENGTH = 128K
  PSRAM(rwx)                 : ORIGIN = 0xB0000000, LENGTH = 128M
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

#INCLUDE link.x

PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 0);

/** TRAP ENTRY POINTS **/

/* Default trap entry point. The riscv-rt crate provides a weak alias of this function,
   which saves caller saved registers, calls _start_trap_rust, restores caller saved registers
   and then returns. Users can override this alias by defining the symbol themselves */
EXTERN(_start_trap);

/* Default interrupt trap entry point. When vectored trap mode is enabled,
   the riscv-rt crate provides an implementation of this function, which saves caller saved
   registers, calls the the DefaultHandler ISR, restores caller saved registers and returns. */
PROVIDE(_start_DefaultHandler_trap = _start_trap);

/* When vectored trap mode is enabled, each interrupt source must implement its own
   trap entry point. By default, all interrupts start in _start_trap. However, users can
   override these alias by defining the symbol themselves */
PROVIDE(_start_SupervisorSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorExternal_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineExternal_trap = _start_DefaultHandler_trap);

/** EXCEPTION HANDLERS **/

/* Default exception handler. The riscv-rt crate provides a weak alias of this function,
   which is a busy loop. Users can override this alias by defining the symbol themselves */
EXTERN(ExceptionHandler);

/* It is possible to define a special handler for each exception type.
   By default, all exceptions are handled by ExceptionHandler. However, users can
   override these alias by defining the symbol themselves */
PROVIDE(InstructionMisaligned = ExceptionHandler);
PROVIDE(InstructionFault = ExceptionHandler);
PROVIDE(IllegalInstruction = ExceptionHandler);
PROVIDE(Breakpoint = ExceptionHandler);
PROVIDE(LoadMisaligned = ExceptionHandler);
PROVIDE(LoadFault = ExceptionHandler);
PROVIDE(StoreMisaligned = ExceptionHandler);
PROVIDE(StoreFault = ExceptionHandler);
PROVIDE(UserEnvCall = ExceptionHandler);
PROVIDE(SupervisorEnvCall = ExceptionHandler);
PROVIDE(MachineEnvCall = ExceptionHandler);
PROVIDE(InstructionPageFault = ExceptionHandler);
PROVIDE(LoadPageFault = ExceptionHandler);
PROVIDE(StorePageFault = ExceptionHandler);

/** INTERRUPT HANDLERS **/

/* Default interrupt handler. The riscv-rt crate provides a weak alias of this function,
   which is a busy loop. Users can override this alias by defining the symbol themselves */
EXTERN(DefaultHandler);

/* It is possible to define a special handler for each interrupt type.
   By default, all interrupts are handled by DefaultHandler. However, users can
   override these alias by defining the symbol themselves */
PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);


SECTIONS
{
  .text.init : 
  { 
    *(.text.init) 
  }>FLASH
  
  . = ALIGN(8);
  
  .text : 
  { 
    *(.text)
    . = ALIGN(8);
    *(.text.*)
  }>FLASH
  
  . = ALIGN(8);

  .rodata : 
  {
    __rodata_start = .;
    *(.rodata)
    *(.rodata.*)
    *(.gnu.linkonce.r.*)
    __rodata_end = .;
      . = ALIGN(8);
  }>FLASH

  _la_sdata =LOADADDR(.sdata);
  
  .sdata : 
  {
    __sdata_start = .;
    _sidata = LOADADDR(.data);
    _sdata = .;
    __global_pointer$ = . + 0x800;
    *(.srodata.cst16) *(.srodata.cst8) *(.srodata.cst4) *(.srodata.cst2) *(.srodata*)
    *(.sdata .sdata.* .gnu.linkonce.s.*)
    _edata = .;
    __sdata_end = .;
      . = ALIGN(8);
  }>RAM AT> FLASH
  
  _la_data =LOADADDR(.data);
  
  .data : 
  {
    __data_start = .;
    *(.data)
    *(.data.*)
    *(.gnu.linkonce.d.*)
        _tls_data = .;
    *(.tdata.begin)
    *(.tdata)
    *(.tdata.end)
          *(.tbss)
      *(.tbss.end)
    _tls_end = .;
    __data_end = .;
  }>RAM AT> FLASH
  
  .sbss : 
  {
    . = ALIGN(8);
    __sbss_start = .;
    *(.sbss)
    *(.sbss.*)
    *(.gnu.linkonce.sb.*)
    __sbss_end = .;
  }>RAM

  .bss : 
  {
     . = ALIGN(8);
     _sbss = .;
     __bss_start = .;
     *(.bss)
     *(.bss.*)
     *(.gnu.linkonce.b.*)
     *(COMMON)
     . = ALIGN(8);
      _end = .;
      __bss_end = .;
      _ebss = .;
  . = ORIGIN(RAM)+LENGTH(RAM) - 8;
  }>RAM
  
 
  .stack : 
  {
    . = ALIGN(8);
   _stack = . ; 
   __stack_pointer$ = . ;
   _stack_end = . - _STACK_SIZE;
  }>RAM
  
  .heap : 
  {
    _HEAP_SIZE  = _stack_end - _end - 8;
    _heap_end = _stack_end - 8;
   _heap = _heap_end - _HEAP_SIZE ;
  }>RAM
}
