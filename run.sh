#!/bin/bash

qemu-system-x86_64 \
    -kernel vmlinuz \
    -initrd initramfs.cpio.gz \
    -nographic \
    -append "console=ttyS0 init=/sbin/init rdinit=/sbin/init quiet"