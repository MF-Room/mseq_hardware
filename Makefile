CHIP:=STM32F411CEUx

flash:
	cargo flash --chip $(CHIP) --release

rtt:
	cargo run -r

gdb_server:
	probe-rs gdb --chip STM32F411CEUx

gdb:
	cargo build -r
	arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/mseq_hardware

