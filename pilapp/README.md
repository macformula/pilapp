# MAC Formula PIL
Processor-in-the-loop tester to verify vehicle firmware after SIL tests

## Running the PIL
The PIL runs on a raspberry pi. Eventually, a microcontroller that the pi can use to talk to the DUT will be nice, but for 100Hz the pi works pretty well. (If we can get the pi and DUT 180 degrees out of sync then jitter will not matter given our current system).

The raspberry pi is provisioned by the following: https://github.com/macformula/nixos-images/. Make sure you read the instructions carefully, and check what system you are building on to avoid virtualization.

You do not need nix to build the pilapp itself, but it is useful.

## Further Development
Right now, the PIL is a pretty barebones gpio blinker. Eventually, we want to:
 * Read gpio
 * Pass in tasks with state, that describe test sequences
 * Drive CAN and other non-gpio peripherals
 * Measure output jitter. While we nominally achieve ~100ns jitter, that's only the onboard clock, and doesn't mean much
    * If jitter is high, we can move from gpiocdev to bit-banging the registers
    * If that still doesn't work, the second controller to actually drive gpio will be useful. Bonuses for a second controller include onboard CAN control instead of the spi-to-CAN that we will use for the pi
 * Flash the firmware automatically