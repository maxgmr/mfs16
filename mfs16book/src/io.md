# I/O

## Error Register (0xFFFF_FFBA)

Bits in the error register get set if a non-fatal error occurs during program execution. This register is read-only, and reading the register resets all the bits back to 0.

This register is typically used for error handling and user-friendly error reports.

### Error Types:

-**IllegalRead:** This bit is set when a program attempts to read from write-only memory or an unused memory address.

-**IllegalWrite:** This bit is set when a program attempts to write to read-only memory or an unused memory address.

Each bit corresponds to a different error:

| 7 6 5 4 3 2 | 1            | 0           |
| ----------- | ------------ | ----------- |
|             | IllegalWrite | IllegalRead |

## Manual Frame Updates (0xFFFF_FFBB-0xFFFF_FFBD)

Some programs may want to slow down the frame rate until the frame is ready to be displayed. Do to this, programs can write to write-only address `0xFFFF_FFBD` to enable manual frame updates.

When manual frame updates are enabled, the screen will not display the new VRAM state until the program writes to write-only address `0xFFFF_FFBB`. This triggers a manual frame update, updating the screen to reflect the new VRAM state.

While manual frame updates prevent issues with flickering or invisible graphics, they can lead to choppy or inconsistent frames, so manual frame updates should only be enabled when needed. Ideally, programs should be aware of their timings and/or update the screen before the next frame.

Manual frame updates can be disabled again by writing to the write-only address `0xFFFF_FFBC`.

## Keyboard Input (0xFFFF_FFBE - 0xFFFF_FFFD)

The keyboard register consists of 64 bytes of memory located at the range \[`0xFFFF_FFBE`-`0xFFFF_FFFD`\]. Each byte corresponds to a physical key on the computer keyboard. When a key's bit is set, that key is currently being pressed. When a key's bit is reset, that key is not currently being pressed.

The order of bits within the keyboard register matches the order of [SDL2 Scancodes](https://wiki.libsdl.org/SDL2/SDL_Scancode). As stated above, the key name represents the physical key on the keyboard- the resulting value is handled entirely by the given program.

| Key Name           | Bit Index |
| ------------------ | --------- |
| A                  | 4         |
| B                  | 5         |
| C                  | 6         |
| D                  | 7         |
| E                  | 8         |
| F                  | 9         |
| G                  | 10        |
| H                  | 11        |
| I                  | 12        |
| J                  | 13        |
| K                  | 14        |
| L                  | 15        |
| M                  | 16        |
| N                  | 17        |
| O                  | 18        |
| P                  | 19        |
| Q                  | 20        |
| R                  | 21        |
| S                  | 22        |
| T                  | 23        |
| U                  | 24        |
| V                  | 25        |
| W                  | 26        |
| X                  | 27        |
| Y                  | 28        |
| Z                  | 29        |
| 1                  | 30        |
| 2                  | 31        |
| 3                  | 32        |
| 4                  | 33        |
| 5                  | 34        |
| 6                  | 35        |
| 7                  | 36        |
| 8                  | 37        |
| 9                  | 38        |
| 0                  | 39        |
| RETURN             | 40        |
| ESCAPE             | 41        |
| BACKSPACE          | 42        |
| TAB                | 43        |
| SPACE              | 44        |
| MINUS              | 45        |
| EQUALS             | 46        |
| LEFTBRACKET        | 47        |
| RIGHTBRACKET       | 48        |
| BACKSLASH          | 49        |
| NONUSHASH          | 50        |
| SEMICOLON          | 51        |
| APOSTROPHE         | 52        |
| GRAVE              | 53        |
| COMMA              | 54        |
| PERIOD             | 55        |
| SLASH              | 56        |
| CAPSLOCK           | 57        |
| F1                 | 58        |
| F2                 | 59        |
| F3                 | 60        |
| F4                 | 61        |
| F5                 | 62        |
| F6                 | 63        |
| F7                 | 64        |
| F8                 | 65        |
| F9                 | 66        |
| F10                | 67        |
| F11                | 68        |
| F12                | 69        |
| PRINTSCREEN        | 70        |
| SCROLLLOCK         | 71        |
| PAUSE              | 72        |
| INSERT             | 73        |
| HOME               | 74        |
| PAGEUP             | 75        |
| DELETE             | 76        |
| END                | 77        |
| PAGEDOWN           | 78        |
| RIGHT              | 79        |
| LEFT               | 80        |
| DOWN               | 81        |
| UP                 | 82        |
| NUMLOCKCLEAR       | 83        |
| KP_DIVIDE          | 84        |
| KP_MULTIPLY        | 85        |
| KP_PLUS            | 86        |
| KP_MINUS           | 87        |
| KP_ENTER           | 88        |
| KP_1               | 89        |
| KP_2               | 90        |
| KP_3               | 91        |
| KP_4               | 92        |
| KP_5               | 93        |
| KP_6               | 94        |
| KP_7               | 95        |
| KP_8               | 96        |
| KP_9               | 97        |
| KP_0               | 98        |
| KP_PERIOD          | 99        |
| NONUSBACKSLASH     | 100       |
| APPLICATION        | 101       |
| POWER              | 102       |
| KP_EQUALS          | 103       |
| F13                | 104       |
| F14                | 105       |
| F15                | 106       |
| F16                | 107       |
| F17                | 108       |
| F18                | 109       |
| F19                | 110       |
| F20                | 111       |
| F21                | 112       |
| F22                | 113       |
| F23                | 114       |
| F24                | 115       |
| EXECUTE            | 116       |
| HELP               | 117       |
| MENU               | 118       |
| SELECT             | 119       |
| STOP               | 120       |
| AGAIN              | 121       |
| UNDO               | 122       |
| CUT                | 123       |
| COPY               | 124       |
| PASTE              | 125       |
| FIND               | 126       |
| MUTE               | 127       |
| VOLUMEUP           | 128       |
| VOLUMEDOWN         | 129       |
| KP_COMMA           | 133       |
| KP_EQUALSAS400     | 134       |
| INTERNATIONAL1     | 135       |
| INTERNATIONAL2     | 136       |
| INTERNATIONAL3     | 137       |
| INTERNATIONAL4     | 138       |
| INTERNATIONAL5     | 139       |
| INTERNATIONAL6     | 140       |
| INTERNATIONAL7     | 141       |
| INTERNATIONAL8     | 142       |
| INTERNATIONAL9     | 143       |
| LANG1              | 144       |
| LANG2              | 145       |
| LANG3              | 146       |
| LANG4              | 147       |
| LANG5              | 148       |
| LANG6              | 149       |
| LANG7              | 150       |
| LANG8              | 151       |
| LANG9              | 152       |
| ALTERASE           | 153       |
| SYSREQ             | 154       |
| CANCEL             | 155       |
| CLEAR              | 156       |
| PRIOR              | 157       |
| RETURN2            | 158       |
| SEPARATOR          | 159       |
| OUT                | 160       |
| OPER               | 161       |
| CLEARAGAIN         | 162       |
| CRSEL              | 163       |
| EXSEL              | 164       |
| KP_00              | 176       |
| KP_000             | 177       |
| THOUSANDSSEPARATOR | 178       |
| DECIMALSSEPARATOR  | 179       |
| CURRENCYUNIT       | 180       |
| CURRENCYSUBUNIT    | 181       |
| KP_LEFTPAREN       | 182       |
| KP_RIGHTPAREN      | 183       |
| KP_LEFTBRACE       | 184       |
| KP_RIGHTBRACE      | 185       |
| KP_TAB             | 186       |
| KP_BACKSPACE       | 187       |
| KP_A               | 188       |
| KP_B               | 189       |
| KP_C               | 190       |
| KP_D               | 191       |
| KP_E               | 192       |
| KP_F               | 193       |
| KP_XOR             | 194       |
| KP_POWER           | 195       |
| KP_PERCENT         | 196       |
| KP_LESS            | 197       |
| KP_GREATER         | 198       |
| KP_AMPERSAND       | 199       |
| KP_DBLAMPERSAND    | 200       |
| KP_VERTICALBAR     | 201       |
| KP_DBLVERTICALBAR  | 202       |
| KP_COLON           | 203       |
| KP_HASH            | 204       |
| KP_SPACE           | 205       |
| KP_AT              | 206       |
| KP_EXCLAM          | 207       |
| KP_MEMSTORE        | 208       |
| KP_MEMRECALL       | 209       |
| KP_MEMCLEAR        | 210       |
| KP_MEMADD          | 211       |
| KP_MEMSUBTRACT     | 212       |
| KP_MEMMULTIPLY     | 213       |
| KP_MEMDIVIDE       | 214       |
| KP_PLUSMINUS       | 215       |
| KP_CLEAR           | 216       |
| KP_CLEARENTRY      | 217       |
| KP_BINARY          | 218       |
| KP_OCTAL           | 219       |
| KP_DECIMAL         | 220       |
| KP_HEXADECIMAL     | 221       |
| LCTRL              | 224       |
| LSHIFT             | 225       |
| LALT               | 226       |
| LGUI               | 227       |
| RCTRL              | 228       |
| RSHIFT             | 229       |
| RALT               | 230       |
| RGUI               | 231       |
| MODE               | 257       |
| AUDIONEXT          | 258       |
| AUDIOPREV          | 259       |
| AUDIOSTOP          | 260       |
| AUDIOPLAY          | 261       |
| AUDIOMUTE          | 262       |
| MEDIASELECT        | 263       |
| WWW                | 264       |
| MAIL               | 265       |
| CALCULATOR         | 266       |
| COMPUTER           | 267       |
| AC_SEARCH          | 268       |
| AC_HOME            | 269       |
| AC_BACK            | 270       |
| AC_FORWARD         | 271       |
| AC_STOP            | 272       |
| AC_REFRESH         | 273       |
| AC_BOOKMARKS       | 274       |
| BRIGHTNESSDOWN     | 275       |
| BRIGHTNESSUP       | 276       |
| DISPLAYSWITCH      | 277       |
| KBDILLUMTOGGLE     | 278       |
| KBDILLUMDOWN       | 279       |
| KBDILLUMUP         | 280       |
| EJECT              | 281       |
| SLEEP              | 282       |
| APP1               | 283       |
| APP2               | 284       |
| AUDIOREWIND        | 285       |
| AUDIOFASTFORWARD   | 286       |
| SOFTLEFT           | 287       |
| SOFTRIGHT          | 288       |
| CALL               | 289       |
| ENDCALL            | 290       |
