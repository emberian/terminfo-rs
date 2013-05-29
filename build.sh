#!/usr/bin/env bash
rustc --lib terminfo.rc || exit
rustc --test terminfo.rc || exit

rustc -L . tools/rcmp.rs || exit
rustc -L . tools/dumpcaps.rs || exit

if [ "$1" = "test" ]; then
	./terminfo
	for term in $(find /usr/share/terminfo -type f | awk 'FS="/" { print $6 }'); do echo $term; ./tools/rcmp $term; done
fi
