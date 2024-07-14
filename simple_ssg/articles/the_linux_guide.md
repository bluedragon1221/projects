---
title: The Linux Guide
author: Collin Williams
slug: linux-guide
tags: ["Linux", "Tutorial"]
---

When building a linux distro, you are faces with many options. Everything from the kernel, to the init system, to the display manager is a choice! The goal of this guide is to help you pick those choices, so that you can build your own Linux distro

## Kernel

The kernel is the most important, but also one of the least important parts of a linux distro. It is the most important, because, well, your entire system wouldn't exist without it. But it is the least important, becasue in reality, you really can't tell whether a system is using the `linux` kernel, or `linux-hardened`, or whatever. That said, here are some different kernels to choose from:

-   `linux`, duh
-   `linux-hardened`, more secure than `linux`
-   `linux-lts`, make for "long-term use"
-   `linux-zen`, opinionated for desktop use
-   `linux-libre`, without proprietary blobs in drivers

Like I said, the kernel that you choose doesn't matter _that much_, so if you're unsure, just go with the `linux` kernel.

## Init System

_This will start a blood feud..._

Your init system matters, like _a lot_. You'll be messing around with it often, and you want something that does what you want it do, without a fuss. There are two main options:

-   SystemD
-   Anything else

If you do not choose SystemD, you _will_ have problems later on, because most things assume SystemD. To paraphrase [this reddit comment](https://www.reddit.com/r/linuxquestions/comments/ghquxc/comment/fqajmk7), The only reason you wouldn't use SystemD is philosophical, as in, you don't like the way that it works, and you wish to oppose it. If that's not you, then just use SystemD.

That said, if you choose to not use SystemD, here are some other options:

-   OpenRC
-   Runit
-   `s6`

Most distros, such as Arch, Debian, Fedora, etc all use SystemD. You will need to find Anti-SystemD distros if you wish to not use SystemD.

## Boot Loader

Your bootloader is the thing that boots your system. Here is where you need to choose your target audience: UEFI, or Legacy BIOS? These days, most computers run on UEFI, and it might be considered foolish to limit your userbase to Legacy BIOS. As for UEFI, there are many bootloaders to choose from:

-   `grub`, the gold standard of bootloaders, but more feature heavy than other options
-   `refind`, prettier, more customizable than other options
-   `systemd-boot`, maintained by SystemD, lighter than `grub`
-   None + UKI, see below

If you are already using SystemD, I don't see why you wouldn't use `systemd-boot`, as it is lighter than `grub` or `refind`

### Unified Kernel Image

A Unified Kernel Image is a way to roll the kernel, initramfs, and further resources into a single file that the computer can boot directly. You can build one with Arch's `mkinitcpio`. `systemd-boot` has first-class support for UKIs, but if you are using a UKI, you don't even need a bootloader. Using a tool like `efibootmgr`, you can create your own UEFI boot entry.

More info about [Unified Kernel Images](https://wiki.archlinux.org/title/Unified_kernel_image) on the Arch Wiki

## Display Manager

After your system boots, this will show first. It presents a login screen, and either drops you into a shell, or starts your graphical environment. Here are some options:

-   GDM, used in GNOME
-   SDDM, used in KDE
-   LightDM, exists
-   `ly`, runs in terminal
-   `agetty`, drop you into terminal

Aside from `ly`, I find display managers bloated, but necessary if you really want to hide the user from the terminal.

## Display Server

_Well, I've done it again, haven't I..._

Your Display Server is the server running in the background, helping your graphical environment run correctly. There are only two options:

-   Wayland, or
-   Xorg

In general, just use Wayland. However, if you have an Nvidia graphics card, then you might be forced to use Xorg. At this point, Xorg is [almost dead](https://y.yarn.co/c8ae3bf5-901c-46e2-a2f4-58f7d46e8925_text.gif), anyway.

## Graphical Environment

The graphical envrionment is what you interact with _every day_. If you want a full-blown desktop solution, here are your options:

-   GNOME (Wayland or Xorg)
-   KDE (Wayland or Xorg)
-   Cinnamon (Xorg)
-   Mate (Xorg)
-   And a bunch of others... (mostly based on GNOME or KDE)

Really, just try them all, and see which one suits you best. In terms of bloat, they are all at the same level: very.

But, there is another option! If you wish, you can run a standalone compositor, which is only in charge of manging windows on the screen. This options requires more work, but you can have a less bloated experience. (In Xorg, they're called Window Managers, but it's the same thing). Here are some options:

-   Sway (Wayland) / i3 (Xorg)
-   Hyprland (Wayland)
-   LabWC (Wayland) / Openbox (Xorg)
-   Some prolific Xorg-only ones:
    -   BSPWM
    -   AwesomeWM
    -   HerbsluftWM
    -   XMonad
    -   Literally too many to list...