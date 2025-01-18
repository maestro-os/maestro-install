<p align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/maestro-os/maestro-lnf/master/logo-light.svg">
    <img src="https://raw.githubusercontent.com/maestro-os/maestro-lnf/master/logo.svg" alt="logo" width="50%" />
  </picture>
</p>

[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=book)](./LICENSE)
![Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fmaestro-os%2Fmaestro-install%2Fmaster%2FCargo.toml&query=%24.package.version&style=for-the-badge&label=version)



# About

The installer for the Maestro operating system is shipped under the ISO format, allowing to use it on a USB stick for example.



# Build the installer

> Required only if cross compiling (building for a different target than your current system):
> 
> First, specify the `TARGET` environment variable with the triplet of the target system. Example:
>
> ```sh
> export TARGET="i686-unknown-linux-musl"
> ```
> 
> This following targets are available:
> - `x86_64-unknown-linux-musl`
> - `i686-unknown-linux-musl`
> 
> Then, you need to [build a toolchain](https://github.com/maestro-os/blimp/tree/master/cross).



## Build packages


The installer requires a minimum set of packages. The list of those packages is in the `base_packages.txt` file.

To build those packages, you first need to install [Maestro's package manager](https://github.com/maestro-os/blimp) on your local computer.

Package descriptors can be found [here](https://github.com/maestro-os/blimp-packages). Clone the repository:

```sh
git clone https://github.com/maestro-os/blimp-packages
```

Create a local repository for built packages:

```sh
mkdir local_repo/
export LOCAL_REPO="local_repo/" # Required later when building the ISO image
```


### Temporary fixes

Since the build system is not yet working entirely, dependencies that are required for building packages are not installed automatically. Thus, the following fixes are currently required:

> Manually build and install the package `maestro-build` to be able to build kernel modules:
>
> ```sh
> blimp-builder blimp-packages/maestro-build local_repo/ # Build
> sudo LOCAL_REPO="$LOCAL_REPO" blimp install maestro-build                       # Install from local repository
> ```

> Patch `blimp` to disable network support (because it requires `libssl` as a dependency, which is not yet supported):
>
> ```sh
> sed -i 's/--features network//' blimp-packages/blimp/build-hook
> ```



### Build required packages

> Now, if you are cross compiling, [setup the package manager for cross compilation](https://github.com/maestro-os/blimp#cross-compilation).

Compile packages required by the installer (excluding the kernel):

```sh
for pkg in $(cat base_packages.txt); do
    blimp-builder blimp-packages/$pkg $LOCAL_REPO
done
```



## Build the ISO

The following command builds the ISO:

```sh
./build.sh
```

The resulting ISO is then named `maestro.iso`



# Usage

## Flash the ISO

This step is required only if you want to install the OS on a physical machine.

If installing on a virtual machine, just insert the ISO as a CD-ROM. The VM is required to have a disk on which the system will be installed.

First, make sure no filesystem belonging to the device is mounted. Flashing the ISO while a filesystem is mounted might corrupt the image.

> **Warning**: the following action is destructive. Make sure you have no important data on the device you are flashing.
> 
> Make also sure that you select the right device too. Flashing the wrong disk might erase your system.

The following command to flashes the ISO (where `XXX` is the device on which you want to flash):

```sh
dd if=<path to the ISO> of=/dev/XXX status=progress
```

Then, eject the device. It is now ready to be used as a bootable device to install the OS!



## Installation

First, plug the installation device on the computer. Then, you can just follow the instructions to install the system.

> **Note**: Do not install the system on a computer with important data. This OS and its installer are still work-in-progress softwares.