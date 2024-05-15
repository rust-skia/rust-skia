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

.PHONY: rm-externals
rm-externals:
	rm -rf skia/third_party/externals/*

.PHONY: rm-locks
rm-locks:
	find . -type f -name 'index.lock' -print0 | xargs -0 rm -f
	find . -type f -name 'HEAD.lock' -print0 | xargs -0 rm -f
	find . -type f -name 'shallow.lock' -print0 | xargs -0 rm -f
