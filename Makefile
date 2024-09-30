gen:
	cargo +nightly build  --release --target riscv64gc-unknown-none-elf
	riscv64-unknown-elf-objcopy ./target/riscv64gc-unknown-none-elf/release/tomos_bootloader -O binary loader.efi
clean:
	cargo clean
	rm loader.efi