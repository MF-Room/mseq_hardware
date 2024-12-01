CHIP := STM32F411CEUx
GDB ?= arm-none-eabi-gdb

flash:
	cargo flash --chip $(CHIP) --release

rtt:
	cargo run -r

gdb_server:
	$(MAKE) flash
	probe-rs gdb --chip $(CHIP)

gdb:
	 $(GDB) -x init.gdb target/thumbv7em-none-eabihf/release/mseq_hardware

