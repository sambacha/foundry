VERSION?=
CHANNEL?=

VOLUME_MOUNTS=-v "$(CURDIR)":/v
SHELLCHECK_EXCLUSIONS=$(addprefix -e, SC1091 SC1117)
SHELLCHECK=docker run --rm $(VOLUME_MOUNTS) -w /v koalaman/shellcheck $(SHELLCHECK_EXCLUSIONS)

ENVSUBST_VARS=LOAD_SCRIPT_COMMIT_SHA

.PHONY: build
build: build/install

build/install: install
	mkdir -p $(@D)
	LOAD_SCRIPT_COMMIT_SHA='$(shell git rev-parse HEAD)' envsubst '$(addprefix $$,$(ENVSUBST_VARS))' < $< > $@

.PHONY: shellcheck
shellcheck: build/install
	$(SHELLCHECK) $<

.PHONY: test
test: build/install
	cat build/install
		sh "$<"

.PHONY: clean
clean:
	$(RM) -r build/

.PHONY: fmt-testdata

fmt-testdata:
	@FOLDER=$(shell dirname "$0")/fmt/testdata/prettier-plugin-solidity;\
	if [ ! -d $$FOLDER/.git ] ; then git clone --depth 1 --recursive https://github.com/prettier-solidity/prettier-plugin-solidity $$FOLDER;\
	else cd $$FOLDER; git pull --recurse-submodules; fi
