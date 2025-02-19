; Multiboot section tells grub how/what to boot
  ; As specified by https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Header-layout

MAGIC equ 0xE85250D6

section .multiboot
global multiboot_start
multiboot_start:
  align 8 ; Ensure alignment
  ; Magic value
  dd MAGIC
  ; Architecture
  dd 0 ; 32-bits (protected) mode 
  ; Header Length
  dd multiboot_end - multiboot_start
  ; Checksum
  dd -(MAGIC + 0 + (multiboot_end - multiboot_start))

  ; Console tag
  align 8 ; Ensure alignment
  dw 4 ; type
  dw 0 ; flags
  dd 12 ; size
  ; We require a console that must support text 
  dd 0b11 ; console_flags

  ; Sentinel tag
  align 8 ; Ensure alignment
  dw 0 ; type
  dw 0 ; flags
  dd 8 ; size
global multiboot_end
multiboot_end:

section .bss
  ; Create a basic stack
  ; This is aligned to 4K (see linker script)
  align 16
  resb 16384
stack_bottom:

section .text
; Kernel code entrypoint
global _start
_start:
  cli ; Deactivate interrupts

  ; Create a basic stack
  mov ebp, stack_bottom
  mov esp, stack_bottom

  ; Jump to 
extern entrypoint
  call entrypoint

  ; Security if entrypoint code returns
loop:
  pause ; Wait for interuption
  jmp loop

