skparagraph.patch: skia/BUILD.gn skia/modules/skparagraph/BUILD.gn
	(cd skia && git diff) >$@

.PHONY: skia-short-hash
skia-short-hash:
	cd skia && git rev-parse --short HEAD
