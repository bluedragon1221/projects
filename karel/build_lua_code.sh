#!/bin/bash
LD_LIBRARY_PATH=./target/debug RUST_LUA_FFI_TYPE_PREFIX=karel luajit rust_lua_ffi/lua/bootstrap.lua karel > api.lua
