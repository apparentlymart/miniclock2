# NOTE: This is assuming Linux with a udev rule to symlink the BMP's GDB port
target extended-remote /dev/blackmagicprobe-gdb

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

monitor arm semihosting enable
load

# start the process but immediately halt the processor
stepi
