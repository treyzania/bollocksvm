# Definition of BollocksISA

## Registers

There are 4 registers.  Registers are 64 bits.  The capitalization of the
letters specifies the register "name", as such:

* qq - Register A

* qQ - Register B

* Qq - Register C

* QQ - Register D

## Opcodes

### Data manipulation

* im?? - IMmediate load of hex ?? in register L

* move - Move register R to register L

* swap - Swap the values of the registers

* summ - Add register R to register L

* subt - Subtract register R from register L, store in L

* mult - Multiply register R by register L, store in L

* andd - Bitwise AND L and R into L

* orrr - Bitwise OR L and R into L

* xorr - Bitwise XOR L and R into L

* sf?? - Bitshift L by 2's-complement hex ??

* rt?? - Bitrotate L by 2's-complement hex ??

### Memory and IO

* load - Copy value from memory at R to register L

* stor - Copy value from register L to memory at R

* disp - Write to port L value of register R

* read - Read from port L to register R

### Flow control

* go?? - Short relative jump 2's-complement hex ??
