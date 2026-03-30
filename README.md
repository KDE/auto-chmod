<!-- SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL -->
<!-- SPDX-FileCopyrightText: 2026 Hadi Chokr <hadichokr@icloud.com> -->

# auto-chmod

Replaces the cryptic "permission denied" error when running a non-executable file with a
prompt asking if you want to make it executable and run it.

## Building
```sh
cargo build --release
```

## Setup

Add to your `~/.zshrc`:
```zsh
function auto_chmod() { /usr/lib/auto-chmod "$1" }
autoload -Uz add-zsh-hook
add-zsh-hook preexec auto_chmod
```

## Translation

Run `Messages.sh` to generate the translation template.
