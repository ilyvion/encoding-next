.PHONY: all
all:
	@echo 'Try `cargo build` instead.'

.PHONY: authors
authors:
	echo 'Encoding is mainly written by Kang Seonghoon <public+rust@mearie.org>,' > AUTHORS.txt
	echo 'and also the following people (in ascending order):' >> AUTHORS.txt
	echo >> AUTHORS.txt
	git log --format='%aN <%aE>' | grep -v 'Kang Seonghoon' | sort -u >> AUTHORS.txt

.PHONY: test
test:
	# `test_correct_table` tests with indices with non-BMP mappings tend to be
	# very slow without the optimization, so japanese and tradchinese got flags
	cargo +nightly test -v
	cargo +nightly test -v -p encoding-index-singlebyte
	cargo +nightly test -v -p encoding-index-korean
	RUSTFLAGS='-C opt-level=1' cargo +nightly test -v -p encoding-index-japanese
	cargo +nightly test -v -p encoding-index-simpchinese
	RUSTFLAGS='-C opt-level=1' cargo +nightly test -v -p encoding-index-tradchinese
	cargo +nightly test -v -p encoding-types
