# stm32-temp-sensor-rust-can
STM32 Temperature Reader with RUST and CAN Communication

## Running a program on STM32
To execute our code on the STM32 microcontroller, we need the OpenOCD tool and the GDB debugger. 
We will run the following commands

```
openocd -f /usr/share/openocd/scripts/interface/stlink-v2-1.cfg -f /usr/share/openocd/scripts/target/stm32f4x.cfg
gdb-multiarch -q target/thumbv7em-none-eabihf/debug/stm32f4_project
```
After these commands, we enter the following commands in the GDB terminal

```
(gdb) target extended-remote :3333
(gdb) load
(gdb) monitor arm semihosting enable
(gdb) continue 
```
