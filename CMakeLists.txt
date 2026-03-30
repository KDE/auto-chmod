# SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only OR LicenseRef-KDE-Accepted-GPL
# SPDX-FileCopyrightText: 2026 Hadi Chokr <hadichokr@icloud.com>

cmake_minimum_required(VERSION 3.25)
project(auto-chmod)

include(GNUInstallDirs)
find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_MODULE_PATH})

find_package(KF6I18n REQUIRED)
find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

ki18n_install(po)

install(PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/auto-chmod DESTINATION ${CMAKE_INSTALL_LIBDIR})
