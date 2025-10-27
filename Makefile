.default: help

.PHONY: help
help:
	@echo "usage:"
	@echo ""
	@echo "bumpversion [part=]"
	@echo "   bumps the given part of the version of the project. (Default: part='patch')"
	@echo ""
	@echo "bumpversion-show"
	@echo "   shows the bump path that is possible"
	@echo ""
	@echo "publish [branch=]"
	@echo "   pushes the given branch including tags to origin, for CI to publish based on tags. (Default: branch='main')"
	@echo "   Typically used after 'make bumpversion'"
	@echo ""
	@echo "prepare-release"
	@echo "   run tasks to prepare a release"
	@echo ""
part := "patch"
bumpversion:
	${INVENV} bump-my-version bump ${part}

bumpversion-show:
	${INVENV} bump-my-version show-bump

branch := "main"
publish:
	git push origin ${branch} --tags

.PHONY: prepare-release
prepare-release: update-changelog

.PHONY: update-changelog
update-changelog: CHANGELOG.md

.PHONY: CHANGELOG.md
CHANGELOG.md:
	git cliff --unreleased --prepend $@

.PHONY: download-models
download-models: download-sense-model download-context-model

.PHONY: download-context-model
download-context-model: assets/models/lem_cbow0_s512_w10_NEW2_ctx.bin

assets/models/lem_cbow0_s512_w10_NEW2_ctx.bin:
	curl -sL --output $@ https://github.com/spraakbanken/sparv-wsd/raw/refs/heads/master/models/scouse/lem_cbow0_s512_w10_NEW2_ctx.bin

.PHONY: download-sense-model
download-sense-model: assets/models/ALL_512_128_w10_A2_140403_ctx1.bin

assets/models/ALL_512_128_w10_A2_140403_ctx1.bin:
	curl -sL --output $@ https://github.com/spraakbanken/sparv-wsd/raw/refs/heads/master/models/scouse/ALL_512_128_w10_A2_140403_ctx1.bin
