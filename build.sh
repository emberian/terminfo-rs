#!/usr/bin/env bash
rustc --lib terminfo.rc
rustc --test terminfo.rc

rustc -L . rcmp.rs

./terminfo

for term in $(find /usr/share/terminfo -type f | awk 'FS="/" { print $6 }'); do echo $term; ./rcmp $term; done
