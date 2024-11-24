# Interrupts

Interrupts can be handled by the CPU in order to deal with events in a timely manner.

## Interrupt Types

The interrupt types are listed from highest to lowest priority.

- **Frame:** This interrupt is triggered periodically at each frame. This will always happen after a set number of cycles.

- **Keyboard:** When _any_ bit in the keyboard register changes from 0 to 1, the Keyboard interrupt is triggered.

- **Error:** When _any_ bit in the error register changes from 0 to 1, (i.e., an error occurs), the Error interrupt is triggered.

## Interrupt Handling Logic

The CPU performs the following actions every cycle:

1. First, the CPU checks to see if it is halted or the master interrupt flag is set. If the CPU is not halted AND the master interrupt flag is reset, then it does nothing.

2. After that, the CPU uses the interrupt enable register as a bitmask for the interrupt register to check to see if any enabled interrupts have been triggered.

3. If no enabled interrupts have been triggered, then nothing happens and the CPU leaves its interrupt handling logic.

4. Otherwise, if the CPU is in a halted state, _regardless of the state of the master interrupt flag_, then the CPU will be taken out of its halted state.

5. The CPU then looks at the lowest bit of the interrupts register that has been activated or enabled. In other words, the CPU prioritises lower-bit interrupts first.

6. Finally, the CPU disables the master interrupt flag and jumps to the static ROM address of its respective interrupt handler. The address is `0x100 + (0x100 * interrupt bit number)`. For example, the `Frame` interrupt is bit 0 of the interrupt enable and interrupt registers. This means that the CPU jumps to address `0x100` when a `Frame` interrupt is triggered. The `Keyboard` interrupt is bit 1, so the CPU jumps to address `0x200` when a `Keyboard` interrupt is triggered.

## Master Interrupt Flag

This internal CPU flag (i.e., inaccessible by memory bus) can globally enable or disable any interrupt handling whatsoever. If this flag is reset, then any triggered interrupts can only take the CPU out of the halted state. It cannot be read directly, and is only modified in the following ways:

- **Reset:** `DI` instruction, execution of interrupt handler
- **Set:** `EI` instrution, `RETI` instruction

The master interrupt flag is reset when the computer boots.

## Interrupt Enable Register

Each bit of this 1-byte register corresponds to a different type of interrupt. A set bit means that the corresponding interrupt is enabled and can therefore be handled by the CPU, and vice versa.

The interrupt at bit 0 is handled with the highest priority, and the interrupt at bit 7 is handled with the lowest priority.

This register must be set explicitly by writing to address `0xFFFF_FFFE`.

| 7 6 5 4 3 | 2     | 1        | 0     |
| --------- | ----- | -------- | ----- |
|           | Error | Keyboard | Frame |

## Interrupt Register

The bits of this 1-byte register correspond to the same interrupts as the interrupt enable register. A set bit means that the corresponding interrupt has been triggered, but the interrupt will only be handled if the same bit in the interrupt enable register and the master interrupt flag are both set.

The interrupt at bit 0 is handled with the highest priority, and the interrupt at bit 7 is handled with the lowest priority.

Bits in this register is usually set naturally when their respective events occur, but interrupts can be "force-triggered" by manually writing to the register at `0xFFFF_FFFF`.

| 7 6 5 4 3 2 | 1        | 0     |
| ----------- | -------- | ----- |
|             | Keyboard | Frame |
