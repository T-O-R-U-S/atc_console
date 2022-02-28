## ATC Fantasy Console

### .atc File Format

#### File header

The header contains metadata for the application. It's at the start of every app, and its end is marked by `0x00` (the `noop` instruction). Regular instructions are not parsed in the header.

| Byte | Description |
|--    | --                                           |
|`0x01`| Marks the start/end of the game title        |
|`0x02`| Include this byte to tell the interpreter to keep looping instead of closing down the program once it has finished executing |
| `0x03` | Include this byte to tell the interpreter to use the alternative colour pallette. |

### Instruction invocation format

The format goes as follows:

`Instruction Parameters Instruction`

If I wanted to write a pixel to a specific spot on the screen, the hex for that would be `f1 00 00 00 f1`. The three parameters correspond to X location, Y location, and colour code (Despite the input byte being up to 255, only 16 codes are supported. Check the [colour reference](colour_code.md) for more info).

### Possible instructions:

The instructions are focused more on ergonomics over being easier to implement an interpreter for.

| Name | Code | Parameters
| -- | -- | -- |
| No Op | `0x00` | |
| Write pixel | `0x01` | X pos, Y pos, Colour |
| Div | `0xf0` | LHS, RHS, Addr Num |
| Sub | `0xf1` | LHS, RHS, Addr Num |
| Add | `0xf2` | LHS, RHS, Addr Num |
| Mul | `0xf3` | LHS, RHS, Addr Num |
| Create/replace variable | `0xa1` | **Four** data bytes, Addr Num |
| While | `0xe0` | Addr Num, **Four** bytes to indicate which byte to jump to |
| If | `0xe1` | Addr Num, **Four** bytes to indicate which byte to jump to. |
| Check input | `0xd0` | [Keycode](key_code.md), Addr Num |