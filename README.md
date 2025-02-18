# stm32-temp-sensor-rust-can
STM32 Temperature Reader with RUST and CAN Communication

In this project, the implementation includes reading the temperature from the internal temperature sensor of the STM32 microcontroller. The temperature is sent via a CAN message to the Raspberry Pi for processing. Based on the temperature, the Raspberry Pi sends a signal CAN message to the microcontroller to determine whether to turn the LED on or off

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

## Running programm on Raspberry Pi 3 b+
To execute our code on raspberry, we will run the following command
```
sudo ip link set can0 up type can bitrate 50000
```
With this command, we set bitrate on 50kbit/s
Next, we run our executable file with command
```
./nameOfExecutableFile
```

> [!NOTE]
> **Note**: If we used SSH to connect to the Raspberry Pi, we will need to cross-compile our code for the ARM architecture. Once it is cross-compiled, we will copy it to the Raspberry Pi. The scp command can be used for copying.

> [!NOTE]
> **Note**:To successfully integrate the MCP2515 with the Raspberry Pi, we need to add the following lines to the boot/config.txt file. This file can be on the path boot/firmware/config.txt

```
dtparam=spi=on
dtoverlay=mcp2515-can0,oscillator=16000000,interrupt=25
```
