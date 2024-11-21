# Configuration

By default, the user configuration file for MFS-16 desktop is found in one of the following locations:

- **Linux:** `~/.config/mfs16desktop/config.toml`
- **macOS:** `/Users/<USER>/Library/Caches/com.maxgmr.mfs16desktop/config.toml`
- **Windows:**`C:\Users\<USER>\AppData\Roaming\maxgmr\mfs16desktop\config\config.toml`

However, this path can be overridden by setting the `MFS16DESKTOP_CONFIG` environment variable.

`config.toml` can be edited by opening the file in any text editor. Any missing options will be overwritten by `default.toml` in the same directory.

## Configuration Options

### Path Settings

- **data_path:** The path to the data directory where files are stored.

```toml
[path_settings]
data_path = "~/path/to/data/directory"
```

### Key Bindings

- **exit:** The key which, when pressed, immediately exits the program. Must be a valid [SDL2 Scancode](https://wiki.libsdl.org/SDL3/SDL_Scancode).

```toml
[key_bindings]
exit = "Escape"

```

### Debugger Settings

- **history_size:** The number of cycles to record before the breakpoint is reached.

- **cycles_after_break:** The number of cycles to record after the breakpoint is reached.

- **mem_ranges:** The ranges of memory to record when keeping track of computer state.

```toml
[debugger_settings]
history_size = 128
cycles_after_break = 32

[[debugger_settings.mem_ranges]]
start = 0xFFFFCF
end = 0xFFFFFF

[[debugger_settings.mem_ranges]]
...

```

Note that _all_ non-empty break criteria must be satisfied for the debugger to break.

- **break_criteria.pc_list:** Break if the program counter is any one of the values in the list.

- **break_criteria.ei:** Break if interrupts get enabled.

- **break_criteria.pc_upper_bound:** Break if the program counter is greater than this value.

- **break_criteria.pc_lower_bound:** Break if the program counter is lesser than this value.

- **break_criteria.instr_list:** Break if the current instruction matches any instruction in the list.

- **break_criteria.reg_upper_bounds:** Break if any register in this list is greater than its corresponding value.

- **break_criteria.reg_lower_bounds:** Break if any register in this list is lesser than its corresponding value.

```toml
[debugger_settings.break_criteria]
pc_list = [0x144, 0xABC]
ei = true
pc_upper_bound = 0xFFFFFF
pc_lower_bound = 0x000000

[[debugger_settings.break_criteria.instr_list]]
CmpVraImm8 = "E0"

[[debugger_settings.break_criteria.instr_list]]
LdBraRb = ["DE", "C"]

[[debugger_settings.break_criteria.instr_list]]
BitRaB = ["A", 0]

# OR...
[debugger_settings.break_criteria]
instr_list = ["EI"]
...

[[debugger_settings.break_criteria.reg_upper_bounds]]
reg = "E"
val = 0xFFF0

[[debugger_settings.break_criteria.reg_upper_bounds]]
reg = "L"
val = 0x1234
...

[[debugger_settings.break_criteria.reg_lower_bounds]]
reg = "E"
val = 0x0200
...
```
