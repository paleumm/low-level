# Environment Setup for kernel development in RUST

This section is referenced from [ Mentorship Session: Setting Up an Environment for Writing Linux Kernel Modules in Rust ](https://www.youtube.com/watch?v=tPs1uRqOnlk) and [Linux kernel development](https://www.jackos.io/rust-kernel/rust-for-linux.html) from JackOS. To follow the setup I recommended using Linux Virtual Machine, I use the Ubuntu server 22.04.
 
## Instruction

[bridge the vm](https://getlabsdone.com/how-to-connect-kvm-vm-to-host-network/)

Update & Upgrade the VM.
```
sudo apt update 
sudo apt upgrade -y
```

Install the packages that are required.
```
sudo apt install bc bison curl clang fish flex git gcc libclang-dev libelf-dev lld llvm-dev libncurses-dev make neovim qemu-system-x86
``` 

Add this to .bashrc file.
```
export PATH="/root/.cargo/bin:${PATH}"
export MAKEFLAGS="-j4"
export LLVM="1"
```
> -j4 means use 4 core

I use `linux` source code  from [Rust-for-linux](https://github.com/Rust-for-Linux/linux).
```
git clone --depth=1 https://github.com/Rust-for-Linux/linux.git
```


## Config the kernel

```
cd linux
```

Run this to check if there any missing requirements for building kernel with rust.
```
make rustavailable
```

Install Rust using the command from https://www.rust-lang.org/tools/install.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then run,
```
source "$HOME/.cargo/env"
```
if still not work, try re-open the terminal.

The kernel require the specific version of rustc.
```
rustup override set $(scripts/min-tool-version.sh rustc)
```

Install rust bindgen.
```
cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

```
rustup component add rust-src
```

To make sure that all rust environment are configured, run this command again.
```
make rustavailable
```
The output should be `Rust is available!`

Config kernel
```
make allnoconfig qemu-busybox-min.config rust.config
```

## Build and run the kernel

Configure via TUI.
```
make menuconfig
```

You need `initrd.img`. Download from [JackOS](https://github.com/jackos/linux/blob/tutorial-start/initrd.img) to your `linux` directory.
```
make
qemu-system-x86_64 -nographic -kernel vmlinux -initrd initrd.img -nic user,model=rtl8139,hostfwd=tcp::5555-:23
```

For convinience, we add this to `linux/Makefile`
```
PHONY += rustvm
rustvm:
	$(Q) make && qemu-system-x86_64 -nographic -kernel vmlinux -initrd initrd.img -nic user,model=rtl8139,hostfwd=tcp::5555-:23
```
so that we can run build and run the kernel in a single command.
```
make rustvm
```
