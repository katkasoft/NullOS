#!/bin/bash
set -e

TARGET="x86_64-unknown-linux-musl"

echo "[*] Building NullOS"

rustup target add $TARGET >/dev/null 2>&1 || true

echo "[*] Cleaning rootfs..."
rm -rf rootfs
mkdir -p rootfs/{bin,sbin,etc,proc,sys,dev}

build_and_copy() {
    local dir=$1
    local out_dir=$2

    echo "[*] Building $dir"

    pushd "$dir" > /dev/null
    cargo build --release --target $TARGET
    BIN_NAME=$(basename "$dir")
    cp "target/$TARGET/release/$BIN_NAME" "../../../rootfs/$out_dir/$BIN_NAME"
    chmod +x "../../../rootfs/$out_dir/$BIN_NAME"
    popd > /dev/null
}

for d in src/sbin/*; do
    [ -d "$d" ] && build_and_copy "$d" "sbin"
done

for d in src/bin/*; do
    [ -d "$d" ] && build_and_copy "$d" "bin"
done

if [ ! -f rootfs/sbin/init ]; then
    echo "[!] ERROR: init not found!"
    exit 1
fi

echo "[*] Packing initramfs..."
pushd rootfs > /dev/null
find . | cpio -o -H newc | gzip > ../initramfs.cpio.gz
popd > /dev/null

echo "[✓] Build complete"