cargo build --release
cbindgen -d --lang c++ -o freeverb.hpp .
clang++ --std=c++1z --stdlib=libc++ -L../target/release -lfreeverb_clib test.cpp
