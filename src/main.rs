// SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL
// SPDX-FileCopyrightText: 2026 Hadi Chokr <hadichokr@icloud.com>

use std::env;
use std::io::{self, BufRead, Write};

use gettextrs::{TextDomain, gettext};
use rustix::fs::{FileType, Mode, OFlags, fchmod, fstat, open};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let words = match shell_words::split(&args[1]) {
        Ok(w) => w,
        Err(_) => return,
    };

    let target = match words.iter().find(|w| w.contains('/')) {
        Some(t) => t.clone(),
        None => return,
    };

    let fd = match open(&*target, OFlags::RDONLY | OFlags::NOFOLLOW, Mode::empty()) {
        Ok(fd) => fd,
        Err(_) => return,
    };

    let st = match fstat(&fd) {
        Ok(s) => s,
        Err(_) => return,
    };

    if FileType::from_raw_mode(st.st_mode) != FileType::RegularFile {
        return;
    }

    let mode = Mode::from_bits_truncate(st.st_mode);
    if mode.intersects(Mode::XUSR) {
        return;
    }

    TextDomain::new("auto-chmod").init().ok();

    // i18n: Ask to make file executable. {} = filename.
    // Translate [y/N] to match your locale's yes/no keys.
    print!(
        "{}",
        gettext("{} is not executable. Make it executable and run? [y/N] ").replace("{}", &target)
    );
    io::stdout().flush().ok();

    let mut answer = String::new();
    io::stdin().lock().read_line(&mut answer).ok();

    // i18n: Single character for "yes", must match what you put in [y/N] above.
    let yes = gettext("y");
    if answer.trim().to_lowercase() == yes.to_lowercase() {
        fchmod(&fd, mode | Mode::XUSR).ok();
    }
}
