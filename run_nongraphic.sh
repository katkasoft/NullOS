#!/bin/bash

qemu-system-x86_64 \
    -kernel vmlinuz \
    -initrd initramfs.cpio.gz \
    -append "init=/sbin/init rdinit=/sbin/init quiet console=ttyS0" \
    -m 2G \
    -nographic