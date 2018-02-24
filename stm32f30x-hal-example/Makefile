debug:
	xargo build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/`basename $$PWD` upload/`basename $$PWD`.debug.bin

release:
	xargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/release/`basename $$PWD` upload/`basename $$PWD`.release.bin


flash_release:
	st-flash write upload/`basename $$PWD`.release.bin 0x8000000  

clean:
	xargo clean
	rm upload/*
