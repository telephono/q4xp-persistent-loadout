.PHONY: clean
clean:
	cargo clean

.PHONY: build/linux
build/linux:
	cargo build --release

.PHONY: build/windows
build/windows:
	cross build --target x86_64-pc-windows-gnu --release

.PHONY: build/all
build/all: build/linux build/windows
	mkdir -p target/persistent-loadout/lin_64
	mv target/release/libpersistent_loadout.so target/persistent-loadout/lin_64/persistent-loadout.xpl
	mkdir -p target/persistent-loadout/win_64
	mv target/x86_64-pc-windows-gnu/release/persistent_loadout.dll target/persistent-loadout/win_64/persistent-loadout.xpl
