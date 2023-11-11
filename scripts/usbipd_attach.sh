#!/usr/bin/env bash
set -Eeux

main() {
	local -r filepath="$(dirname $0)/usbipd_attach.ps1"
	powershell.exe -ExecutionPolicy Bypass -File $(dirname $0)/usbipd_attach.ps1
}

main
