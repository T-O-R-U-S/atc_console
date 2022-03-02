## ATC Fantasy Console

### .atc File Format

#### File header

The header contains metadata for the application. It's at the start of every app, and its end is marked by `0x00` (the `noop` instruction). Regular instructions are not parsed in the header.

If an input asks for multiple bytes, assume that it is in Little Endian order.

| Byte | Description |
|--    | --                                           |
|`0x01`| Marks the start/end of the game title        |
|`0x02`| Include this byte to tell the interpreter to keep looping instead of closing down the program once it has finished executing |
| `0x03` (TODO) | Include this byte to tell the interpreter to use the alternative colour pallette. Currently, this does nothing though! |

### Instruction invocation format

The format goes as follows:

`Instruction Parameters`.

If I wanted to write a pixel to a specific spot on the screen, the hex for that would be `f1 00 00 00`. The three parameters correspond to X location, Y location, and colour code (Despite the input byte being up to 255, only 16 codes are supported. Check the [colour reference](colour_code.md) for more info).

### Instruction set:

The instructions are focused more on ergonomics over being easier to implement an interpreter for.

| Name | Code | Parameters
| -- | -- | -- |
| No Op | `0x00` | |
| Write pixel | `0x01` | X pos, Y pos, Colour |
| Write pixel using mem | `0x02` | X pos addr, Y pos addr, Colour |
| Float Div | `0xf0` | LHS Addr, RHS Addr, Addr Num |
| Float Sub | `0xf1` | LHS Addr, RHS Addr, Addr Num |
| Float Add | `0xf2` | LHS Addr, RHS Addr, Addr Num |
| Float Mul | `0xf3` | LHS Addr, RHS Addr, Addr Num |
| Div | `0xf4` | LHS Addr, RHS Addr, Addr Num |
| Sub | `0xf5` | LHS Addr, RHS Addr, Addr Num |
| Add | `0xf6` | LHS Addr, RHS Addr, Addr Num |
| Mul | `0xf7` | LHS Addr, RHS Addr, Addr Num |
| Not | `0xb0` | Addr Num, Out Addr |
| Create/replace variable | `0xa1` | [Type](type_code.md), **Eight** data bytes, Addr Num; If the type is a bool, it consumes only one byte instead; either `0x01` (true) or `0x00` (false) |
| Create variable, avoid replacing pre-existing variable | `0xa2` | [Type](type_code.md), **Eight** data bytes, Addr Num; If the type is a bool, it consumes only one byte instead; either `0x01` (true) or `0x00` (false) |
| Write array item | `0xa3` | Array Addr, Arr Idx (0-7), Byte to write |
| TJump | `0xe1` | Condition (Addr), Byte to jump to |
| Check input | `0xd0` | [Keycode](key_code.md), Addr Num |