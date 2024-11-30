CHIP:=STM32F411CEUx

flash:
	cargo flash --chip $(CHIP) --release

rtt:
	cargo run -r

gdb_server:
	$(MAKE) flash
	probe-rs gdb --chip $(CHIP)

gdb:
	arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/mseq_hardware

