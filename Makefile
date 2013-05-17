terminfo:
	rustc --lib terminfo.rc
	rustc --test terminfo.rc

rcmp:
	rustc -L . rcmp.rs

.PHONY: terminfo rcmp
