# BollocksVM

This is a virtual machine written in Rust implementing the Bollocks instruction
set.  The ISA is probably not fast enough compared to x86 or ARM, but it
*consists entirely of printable ASCII* characters.  See `ARCH.md` for details.

## Modes of operation

Ultimately we want to be able to support execution of `.bollock` binaries both
directly from the command line, as well as by spinning up a server to host
requests using WebSockets from a browser extension.

