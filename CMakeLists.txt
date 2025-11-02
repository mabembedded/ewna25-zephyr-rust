cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(rust_ble_app)

target_sources(app PRIVATE src/ble_wrapper.c)

rust_cargo_application()
