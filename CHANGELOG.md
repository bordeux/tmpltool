## <small>1.4.3 (2026-01-08)</small>

* Merge pull request #35 from bordeux/feature/app-analyse ([c4579c1](https://github.com/bordeux/tmpltool/commit/c4579c1)), closes [#35](https://github.com/bordeux/tmpltool/issues/35)
* docs: reduce README.md ([c3d4360](https://github.com/bordeux/tmpltool/commit/c3d4360))
* docs: reduce README.md ([ad2e825](https://github.com/bordeux/tmpltool/commit/ad2e825))
* perf: optimize release binary size with LTO and symbol stripping ([a8f05ef](https://github.com/bordeux/tmpltool/commit/a8f05ef))

## <small>1.4.2 (2026-01-04)</small>

* Merge pull request #34 from bordeux/fix/fix-release-semantic ([fb2e518](https://github.com/bordeux/tmpltool/commit/fb2e518)), closes [#34](https://github.com/bordeux/tmpltool/issues/34)
* fix: fix semantic release process ([8ce01e7](https://github.com/bordeux/tmpltool/commit/8ce01e7))
* fix: use workflow_dispatch to trigger publish from release ([29bbab3](https://github.com/bordeux/tmpltool/commit/29bbab3))

## <small>1.4.1 (2026-01-04)</small>

* Merge pull request #33 from bordeux/hotfix/fix-after-1-4-release ([d412a40](https://github.com/bordeux/tmpltool/commit/d412a40)), closes [#33](https://github.com/bordeux/tmpltool/issues/33)
* fix: restructure release workflow for GitHub release immutability ([81489db](https://github.com/bordeux/tmpltool/commit/81489db))
* fix: update lock file ([57391be](https://github.com/bordeux/tmpltool/commit/57391be))

## 1.4.0 (2026-01-04)

* Merge pull request #32 from bordeux/feature/ide-support ([f5025fb](https://github.com/bordeux/tmpltool/commit/f5025fb)), closes [#32](https://github.com/bordeux/tmpltool/issues/32)
* fix: add Windows .exe suffix support in CLI tests ([4787e1d](https://github.com/bordeux/tmpltool/commit/4787e1d))
* fix: make DNS empty hostname test platform-tolerant ([7c5a7eb](https://github.com/bordeux/tmpltool/commit/7c5a7eb))
* fix: use assert_cmd for CLI integration tests ([e64e312](https://github.com/bordeux/tmpltool/commit/e64e312))
* docs: add package manager installation instructions ([115ec12](https://github.com/bordeux/tmpltool/commit/115ec12))
* feat: add --ide flag for IDE metadata export ([17f6a79](https://github.com/bordeux/tmpltool/commit/17f6a79))

## <small>1.3.1 (2026-01-03)</small>

* Merge pull request #31 from bordeux/fix/fix-workflow-version-app ([900b11a](https://github.com/bordeux/tmpltool/commit/900b11a)), closes [#31](https://github.com/bordeux/tmpltool/issues/31)
* chore: skip QA checks in pre-commit when no Rust files changed ([588a834](https://github.com/bordeux/tmpltool/commit/588a834))
* fix: ensure builds use correct git ref for version ([816b7e1](https://github.com/bordeux/tmpltool/commit/816b7e1))

## 1.3.0 (2026-01-03)

* Merge pull request #30 from bordeux/feature/new-packages-build ([ba1808b](https://github.com/bordeux/tmpltool/commit/ba1808b)), closes [#30](https://github.com/bordeux/tmpltool/issues/30)
* feat: add dual syntax support for all is-functions ([2d71887](https://github.com/bordeux/tmpltool/commit/2d71887))
* feat: unified filter-functions architecture and packaging improvements ([db27729](https://github.com/bordeux/tmpltool/commit/db27729))
* docs: add big picture overview and update future enhancements in CLAUDE.md ([8b63c98](https://github.com/bordeux/tmpltool/commit/8b63c98))

## <small>1.2.5 (2026-01-02)</small>

* Merge pull request #29 from bordeux/fix/fix-triggers ([4bee433](https://github.com/bordeux/tmpltool/commit/4bee433)), closes [#29](https://github.com/bordeux/tmpltool/issues/29)
* Merge remote-tracking branch 'origin/master' into fix/fix-triggers ([4584654](https://github.com/bordeux/tmpltool/commit/4584654))
* ci: update workflow names during repo update ([18cf9ab](https://github.com/bordeux/tmpltool/commit/18cf9ab))
* fix(ci): use ubuntu-22.04 for glibc builds for Debian 12 compatibility ([6670c6b](https://github.com/bordeux/tmpltool/commit/6670c6b))

## <small>1.2.4 (2026-01-02)</small>

* Merge pull request #28 from bordeux/fix/fix-triggers ([5b1c5b7](https://github.com/bordeux/tmpltool/commit/5b1c5b7)), closes [#28](https://github.com/bordeux/tmpltool/issues/28)
* fix(ci): correct bash indirect variable expansion for token lookup ([cafb472](https://github.com/bordeux/tmpltool/commit/cafb472))

## <small>1.2.3 (2026-01-02)</small>

* Merge pull request #26 from bordeux/hotfix/fix-branch-for-repos ([37981e5](https://github.com/bordeux/tmpltool/commit/37981e5)), closes [#26](https://github.com/bordeux/tmpltool/issues/26)
* Merge pull request #27 from bordeux/feature/distribution-v2 ([d4dd748](https://github.com/bordeux/tmpltool/commit/d4dd748)), closes [#27](https://github.com/bordeux/tmpltool/issues/27)
* fix: Add arch repo ([c8def93](https://github.com/bordeux/tmpltool/commit/c8def93))
* ci: add Alpine APK package support ([c9c3607](https://github.com/bordeux/tmpltool/commit/c9c3607))
* ci: add APK package building to CI workflow ([6cf676f](https://github.com/bordeux/tmpltool/commit/6cf676f))
* ci: consolidate package repo triggers into matrix job ([e58eda3](https://github.com/bordeux/tmpltool/commit/e58eda3))
* ci: fix branch ref for publishing repos ([235e1cc](https://github.com/bordeux/tmpltool/commit/235e1cc))

## <small>1.2.2 (2026-01-01)</small>

* Merge pull request #25 from bordeux/fix/fix-building-debian-package ([8247089](https://github.com/bordeux/tmpltool/commit/8247089)), closes [#25](https://github.com/bordeux/tmpltool/issues/25)
* fix: fix ARM64 DEB package build in cross-compilation ([5356105](https://github.com/bordeux/tmpltool/commit/5356105))

## <small>1.2.1 (2026-01-01)</small>

* Merge pull request #21 from bordeux/dependabot/npm_and_yarn/multi-289c097fca ([5bf7a2a](https://github.com/bordeux/tmpltool/commit/5bf7a2a)), closes [#21](https://github.com/bordeux/tmpltool/issues/21)
* Merge pull request #22 from bordeux/feature/distribution ([e2538fe](https://github.com/bordeux/tmpltool/commit/e2538fe)), closes [#22](https://github.com/bordeux/tmpltool/issues/22)
* Merge pull request #23 from bordeux/feature/distribution ([0b2150a](https://github.com/bordeux/tmpltool/commit/0b2150a)), closes [#23](https://github.com/bordeux/tmpltool/issues/23)
* Merge pull request #24 from bordeux/fix/upgrade-node-version ([9f77263](https://github.com/bordeux/tmpltool/commit/9f77263)), closes [#24](https://github.com/bordeux/tmpltool/issues/24)
* Merge remote-tracking branch 'origin/master' into feature/distribution ([60cc65f](https://github.com/bordeux/tmpltool/commit/60cc65f))
* fix: Force redeployment for new packages ([d3fdf2b](https://github.com/bordeux/tmpltool/commit/d3fdf2b))
* fix: upgrade Node.js version from 20 to 24 ([1aef904](https://github.com/bordeux/tmpltool/commit/1aef904))
* ci: add APT repository update trigger on release ([b01f26e](https://github.com/bordeux/tmpltool/commit/b01f26e))
* ci: add DEB package generation for Debian/Ubuntu ([9b2e338](https://github.com/bordeux/tmpltool/commit/9b2e338))
* ci: add DEB/RPM package building to CI workflow ([ff30cfc](https://github.com/bordeux/tmpltool/commit/ff30cfc))
* ci: add Homebrew formula auto-update on release ([586b164](https://github.com/bordeux/tmpltool/commit/586b164))
* ci: add RPM package generation for Fedora/RHEL ([cba83a3](https://github.com/bordeux/tmpltool/commit/cba83a3))
* ci: add versioned Homebrew formulas support ([cd5b383](https://github.com/bordeux/tmpltool/commit/cd5b383))
* ci: simplify Homebrew update to trigger external workflow ([0b1afb5](https://github.com/bordeux/tmpltool/commit/0b1afb5))
* chore(deps): bump glob and semantic-release ([21639a5](https://github.com/bordeux/tmpltool/commit/21639a5))

## [1.2.0](https://github.com/bordeux/tmpltool/compare/v1.1.2...v1.2.0) (2026-01-01)


### Features

* add 20 new template functions for string, array, and set operations ([49b2b9e](https://github.com/bordeux/tmpltool/commit/49b2b9e02fd095e052ac6967ba77abfe0dfa2c52))
* add 6 JSON/Object operation functions ([a6c282a](https://github.com/bordeux/tmpltool/commit/a6c282a107b3e54bc12b19f8301a1f120e4701a6))
* add 8 Kubernetes extended functions ([6d18b54](https://github.com/bordeux/tmpltool/commit/6d18b5413b8e1d3b8381d061484b700a9bc279a0))
* add 8 new string manipulation functions ([9a3ace3](https://github.com/bordeux/tmpltool/commit/9a3ace33dffd4b5d446af9aa79ca304e235414a2))
* add 9 network and system functions ([7af3f9a](https://github.com/bordeux/tmpltool/commit/7af3f9a8fb21f52b397f91aa65b53cceed63a60e))
* add UUID version support (v4, v7) to uuid() function ([792fc89](https://github.com/bordeux/tmpltool/commit/792fc895a56f3b2e827983f747738e1b35ef17fe))


### Bug Fixes

* support Windows paths in get_cwd() integration test ([9c06d74](https://github.com/bordeux/tmpltool/commit/9c06d7427d0f8dbef34359d0b8911d85de21350a))

## [1.1.2](https://github.com/bordeux/tmpltool/compare/v1.1.1...v1.1.2) (2026-01-01)


### Bug Fixes

* upgrade docker rust version ([cc38acb](https://github.com/bordeux/tmpltool/commit/cc38acb4f6fd48d86dbf476fbbc73074543a3232))

## [1.1.1](https://github.com/bordeux/tmpltool/compare/v1.1.0...v1.1.1) (2026-01-01)


### Bug Fixes

* Append missing files ([7fa3e69](https://github.com/bordeux/tmpltool/commit/7fa3e696fa32eb639dcd1c9985ad4bc27adf7430))

## [1.1.0](https://github.com/bordeux/tmpltool/compare/v1.0.1...v1.1.0) (2026-01-01)


### Features

* add --validate option for output format validation ([11a7416](https://github.com/bordeux/tmpltool/commit/11a7416b19a4b9c606be9df91e5b92854af4e8ae))
* add 12 string manipulation filters ([ee287c0](https://github.com/bordeux/tmpltool/commit/ee287c08e2ee6394b904ce779fdb0463364e759e))
* add advanced array manipulation functions ([30e4c21](https://github.com/bordeux/tmpltool/commit/30e4c218f9290713620c4e385501d8f5cc75904e))
* add data serialization functions and enhance read_lines ([b8d19c9](https://github.com/bordeux/tmpltool/commit/b8d19c993b0092ddc2076afc294472183e6d991a))
* add debugging and development functions ([24c496b](https://github.com/bordeux/tmpltool/commit/24c496b33b71d71afc958d27852a2fab53e4d0ca))
* add encoding and security functions ([c425e6c](https://github.com/bordeux/tmpltool/commit/c425e6cc7b4f4254c1d037cbb895a607983a9ffe))
* add exec() and exec_raw() command execution functions ([f524604](https://github.com/bordeux/tmpltool/commit/f524604d6dc348623f5fe2c7c918947f80f2e1cf))
* add Kubernetes helper functions with k8s_ prefix ([e74595a](https://github.com/bordeux/tmpltool/commit/e74595a44ddec6b8c33b510cfd8ca3a857f47b13))
* add Kubernetes reference functions and update documentation ([2c02663](https://github.com/bordeux/tmpltool/commit/2c026634d1f13556c010948822ac7431ba592f1c))
* add logic functions (default, coalesce, ternary, in_range) ([2035a51](https://github.com/bordeux/tmpltool/commit/2035a513a702cc75f67af7655d9d762b2e428ae5))
* add math functions (min, max, abs, round, ceil, floor, percentage) ([434481f](https://github.com/bordeux/tmpltool/commit/434481f592e5fbfd809f47d011bc8b5dea9cf622))
* add object manipulation functions ([91ca1eb](https://github.com/bordeux/tmpltool/commit/91ca1ebedf6e72b2a15f9db3787591a3f6f358dd))
* add path manipulation and filesystem checking functions ([14d4046](https://github.com/bordeux/tmpltool/commit/14d4046e6c30acd6e8169517f02b3226e49453a6))
* add predicate functions for array and string checking ([d9039f4](https://github.com/bordeux/tmpltool/commit/d9039f4e199c05737d436a82e00111d6d3647850))
* add statistical and array manipulation functions ([6d05bf7](https://github.com/bordeux/tmpltool/commit/6d05bf7a7e3429d7b469c86fca43717e9f4d09df))
* add system and network information functions ([96bea9c](https://github.com/bordeux/tmpltool/commit/96bea9c24776bf7b75c1fcee2a23cff5e7d1acd6))
* add URL and HTTP utility functions ([13a455a](https://github.com/bordeux/tmpltool/commit/13a455a3da1d797a54fd963820fc069a047938e1))
* enhance build_url with default scheme and object query support ([6c0ae42](https://github.com/bordeux/tmpltool/commit/6c0ae42fc846905e623993357a11d255dcaf4892))
* implement comprehensive date/time manipulation functions ([858a554](https://github.com/bordeux/tmpltool/commit/858a55430f44df31828291d57685a4ee1e83a024))


### Bug Fixes

* add renderer test ([91bdbef](https://github.com/bordeux/tmpltool/commit/91bdbefcf33638a7cc24015f42e5949bb2c7b440))
* correct Jinja2 template syntax for array functions ([71bded7](https://github.com/bordeux/tmpltool/commit/71bded765a2f835cd1dc5c1e65dac3b580afa449))
* **k8s:** collapse consecutive dashes in k8s_label_safe ([d4d5120](https://github.com/bordeux/tmpltool/commit/d4d5120991046bb524a22273cf7218ae0ed44ee5))
* make exec test cross-platform compatible with Windows ([e347fe3](https://github.com/bordeux/tmpltool/commit/e347fe335f2e5cd8fbace2a2132f0bd4792c9ed5))
* make renderer tests cross-platform compatible for Windows ([0c126be](https://github.com/bordeux/tmpltool/commit/0c126beeb9645ba25df3bdccf9a04b06609571fe))
* normalize path separators to forward slashes for cross-platform consistency ([dfe5e92](https://github.com/bordeux/tmpltool/commit/dfe5e925fd22392f878f4d451a0cad31a3ac54c0))
* replace undefined assert_true with proper pass/fail logic ([effbd09](https://github.com/bordeux/tmpltool/commit/effbd090a64c5f214c98c72cd4017ec60007e09d))
* resolve clippy warnings in test files ([33bf75f](https://github.com/bordeux/tmpltool/commit/33bf75ff88be83733d9c37d5b256111308e1e5d3))
* resolve integration test failures in CI/CD ([76402e7](https://github.com/bordeux/tmpltool/commit/76402e752e829a42cac6c907c6ad7afe6b8a360f))
* resolve remaining clippy unnecessary_unwrap warning in CI ([ace6cfc](https://github.com/bordeux/tmpltool/commit/ace6cfc3e49a92677df54779a7c016c961f274ef))
* use div_ceil instead of manual ceiling division ([81a938f](https://github.com/bordeux/tmpltool/commit/81a938f0f549768a299760581d34e699c8673e7f))


### Documentation

* add claude.md file ([e4c5a62](https://github.com/bordeux/tmpltool/commit/e4c5a6203f202a0f51377f719d2a6b91f69698c0))
* add command execution functions to README ([cdff5b4](https://github.com/bordeux/tmpltool/commit/cdff5b43c34928f25785c40183d29d30f0b1aacf))
* add debugging & development functions documentation ([60e0953](https://github.com/bordeux/tmpltool/commit/60e095382064d397a03f37cf0dc22997eeb263a9))
* improve function naming conventions in TODO ([8cd5831](https://github.com/bordeux/tmpltool/commit/8cd58319d3b94f5bbd7baff8b0e5ced45d10db8e))
* mark Network/System and String Manipulation sections as complete in TODO ([ec73f8f](https://github.com/bordeux/tmpltool/commit/ec73f8fcaddefe8698b5e3044893b27fbf069208))
* Remove TODO file ([ac242ef](https://github.com/bordeux/tmpltool/commit/ac242efb2d5b3200686e0128eef15cbe02c235e7))
* update Docker usage to binary extraction pattern ([ea693fa](https://github.com/bordeux/tmpltool/commit/ea693fa4bbdefe4e4204d4d9777667baad101a1d))
* update README and TODO with encoding and path functions ([280d24c](https://github.com/bordeux/tmpltool/commit/280d24cf8eb94c44cac17fa76538054fec998580))


### Code Refactoring

* eliminate code duplication in URL functions ([f950ad3](https://github.com/bordeux/tmpltool/commit/f950ad3089ddfc01ff0ec420546a99aa2e3f59bd))
* modularize binary integration tests ([61929b5](https://github.com/bordeux/tmpltool/commit/61929b5eadaf241716693d8db54139109aca8322))
* move unit tests from src/ to tests/ folder ([af9f07e](https://github.com/bordeux/tmpltool/commit/af9f07ec34bc3b817d56a98eea890627adfa6db3))
* move validator tests to tests directory ([5e87a91](https://github.com/bordeux/tmpltool/commit/5e87a91c390e93b9a561cd0ccb95b4b1fc702d03))

## [1.0.1](https://github.com/bordeux/tmpltool/compare/v1.0.0...v1.0.1) (2025-12-31)


### Documentation

* add comprehensive TODO with function ideas ([1c3dbff](https://github.com/bordeux/tmpltool/commit/1c3dbffb8a22db178d90057b0bf46d5f5d75211d))
* remove template caching from TODO ([4d0a533](https://github.com/bordeux/tmpltool/commit/4d0a5330504b3640524c9ddd4d240f0138863820))

## 1.0.0 (2025-12-31)


### Features

* initial release of tmpltool v0.1.0 ([29d3910](https://github.com/bordeux/tmpltool/commit/29d3910d494470920550cd35cd98e938f6e5f0d6))


### Bug Fixes

* make public release ([92e7b90](https://github.com/bordeux/tmpltool/commit/92e7b9075362b9e79b519f7c68eb513f02226dc1))

## 1.0.0 (2025-12-31)


### Features

* initial release of tmpltool v0.1.0 ([29d3910](https://github.com/bordeux/tmpltool/commit/29d3910d494470920550cd35cd98e938f6e5f0d6))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-31

### Added
- Initial release of tmpltool
- Environment variable access with `get_env()` and `filter_env()`
- Hash & crypto functions (MD5, SHA1, SHA256, SHA512, UUID, random strings)
- Filesystem operations (read, exists, list, glob, size, modified)
- Data parsing for JSON, YAML, and TOML
- Validation functions (email, URL, IP, UUID, regex)
- Template include functionality with security controls
- Trust mode for advanced filesystem access
- Full Jinja2 syntax support via MiniJinja
- Comprehensive test suite (267 tests)
- Docker support with multi-arch images
- CLI with flexible I/O (file or stdin/stdout)
- Prepare first public release
