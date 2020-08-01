.PHONY: skia-short-hash
skia-short-hash:
	cd skia && git rev-parse --short HEAD

unused_dirs=\
	animations\
	bench\
	build\
	dm\
	docker\
	docs\
	example\
	experimental\
	fuzz\
	gm\
	infra\
	platform_tools\
	resources\
	samplecode\
	site\
	specs\
	tests

.PHONY: rm-unused
rm-unused:
	cd skia && git rm -r ${unused_dirs}
