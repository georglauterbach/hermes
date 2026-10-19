---
ubuntu: 26.10
---

# `systemd-boot`

[`systemd-boot`](https://www.freedesktop.org/software/systemd/man/latest/systemd-boot.html) is a simple UEFI-only boot manager. It is simpler, leaner and more secure than [`grub`](https://en.wikipedia.org/wiki/GNU_GRUB).

## Installation

```bash
sudo apt install systemd-boot systemd-boot-tools
sudo bootctl install
sudo kernel-install add "$(uname -r)" "/boot/vmlinuz-$(uname -r)"

systemctl reboot

sudo efibootmgr
bootctl status
bootctl status |& grep -F systemd-boot
cat /boot/efi/loader/entries/*.conf

sudo apt --allow-remove-essential autoremove --purge grub-* os-prober shim-signed
sudo rm -rf /etc/default/grub /etc/grub.d /boot/grub /boot/efi/EFI/ubuntu

# Find the number next to the ubuntu entry (e.g., Boot0001* ubuntu).
# Delete that specific entry (replace 0001 with your actual number):
sudo efibootmgr
sudo efibootmgr -b 0001 -B
sudo bootctl update
```
