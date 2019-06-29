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

# We assume that the first (and only) device in the chain is our MCU
monitor swdp_scan
attach 1

load
compare-sections
run

