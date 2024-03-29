# ATC Fantasy Console

## .atc File Format

### Fair warning; ATC is slow on some systems

~~There is nothing I can really do about this. The library I chose to do this, `minifb`, is rather slow on macOS with regular frametimes of >40 ms.~~

There are two rendering back-ends now that can be used. `minifb` is not fully supported, as it has been dropped in favour of `pixels` + `fltk`.

#### File header

The header contains metadata for the application. It's at the start of every app, and its end is marked by `0x00` (the `noop` instruction). Regular instructions are not parsed in the header.

If an input asks for multiple bytes, assume that it is in Little Endian order.

| Byte | Description |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `0x01` | Marks the start/end of the game title |
| `0x02` | Include this byte to tell the interpreter to keep looping instead of closing down the program once it has finished executing |
| `0x04` | Include this byte to tell the interpreter to keep the window open once the application has finished executing |
| `0x03` (TODO) | Include this byte to tell the interpreter to use the alternative colour pallette. Currently, this does nothing though! |
| `0xd5` | Include this byte to tell the interpreter to print all debugging info to the terminal. |

### Instruction invocation format

The format goes as follows:

`InstructionByte Parameters`.

If I wanted to write a pixel to a specific spot on the screen, the hex for that would be `f1 00 00 00`. The three parameters correspond to X location, Y location, and colour code (Despite the input byte being up to 255, only 16 codes are supported. Check the [colour reference](colour_code.md) for more info).

### Instruction set

The instruction set concedes space for ergonomics when working in a hex editor. The minimum size for all inputs is at least 8 bits.

- Length of an instruction: 8 bits / 1 byte
- Length of an address: 8 bits / 1 byte
- Length of a colour code: 8 bits / 1 byte
- Length of a key code: 8 bits / 1 byte
- Length of a type: 8 bites / 1 byte
- Length of a screen position (X/Y): 8 bits / 1 byte
- Length of a jump position: 64 bits / 8 bytes
- Size of a variable: 64 bits / 8 bytes

| Name | Code | Parameters |
| --------------------------- | ------ | ----------------------- |
| No Op | `0x00` | |
| Write pixel | `0x01` | X pos, Inverted Y pos, [Colour](colour_code.md) |
| Write pixel using mem | `0x02` | X pos addr, Inverted Y pos addr, [Colour](colour_code.md) |
| Draw Sprite | `0x03` | Eight **addresses** to arrays containing [colour](colour_code.md) codes, X pos addr, Y pos addr |
| Clear screen | `0xfc` | Input [colour](colour_code.md) code |
| Flush buffer (Render frame) | `0xfb` | |
| Float Div | `0xf0` | LHS Addr, RHS Addr, Addr Num|
| Float Sub | `0xf1` | LHS Addr, RHS Addr, Addr Num|
| Float Add | `0xf2` | LHS Addr, RHS Addr, Addr Num|
| Float Mul | `0xf3` | LHS Addr, RHS Addr, Addr Num|
| Div | `0xf4` | LHS Addr, RHS Addr, Addr Num|
| Sub | `0xf5` | LHS Addr, RHS Addr, Addr Num|
| Add | `0xf6` | LHS Addr, RHS Addr, Addr Num|
| Mul | `0xf7` | LHS Addr, RHS Addr, Addr Num|
| Not | `0xb0` | Addr Num, Out Addr |
| Greater than | `0xb1` | LHS Addr, RHS Addr, Out Addr|
| Less than | `0xb2` | LHS Addr, RHS Addr, Out Addr|
| Create/replace variable | `0xa1` | [Type](type_code.md), **Eight** data bytes, Addr Num |
| Create variable | `0xa2` | [Type](type_code.md), **Eight** data bytes, Addr Num. The distinction here is that this does not replace pre-existing variables |
| Write array item | `0xa3` | Array Addr, Arr Idx (0-7), Byte to write |
| TJump | `0xe1` | Condition (Addr), Byte to jump to if true (8bytes input) |
| FJump | `0xe2` | Condition (Addr), Byte to jump to if false (8bytes input) |
| Jump | `0xe3` | Byte to jump to |
| VJump | `0xe4` | Address of byte to jump to |
| Check input | `0xd0` | [Keycode](key_code.md), Addr Num |
