#!/usr/bin/env bash
set -eu

ROOT_DIR=$(dirname $(dirname $(readlink -f ${0:-})))
EFI_FILE=${1:-}

cd ${ROOT_DIR}

mkdir -p mnt/EFI/BOOT
cp -f ${EFI_FILE} mnt/EFI/BOOT/BOOTX64.EFI
qemu-system-x86_64 \
  -bios third_party/ovmf/RELEASEX64_OVMF.fd \
  -drive format=raw,file=fat:rw:mnt
