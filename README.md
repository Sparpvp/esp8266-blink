# esp8266-blink
Blink program on RISC L106 80Mhz 32bit CPU

# Flashing
Running rust on ESP* is sort of hard...  
We won't cover the installation process, instead we'll cover the flashing method.  
First, install espflash through cargo.
```
cargo install espflash
```
Then, flash into the ESP.
```
cargo +xtensa espflash --release <usbport> --features="xtensa-lx-rt/lx106 xtensa-lx/lx106 esp8266-hal"
```
Where **usbport** placeholder is usually **/dev/ttyUSB0**

# Todo
- Write in Register Level Programming
