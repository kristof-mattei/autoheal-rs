<!-- header goes here -->
## [1.13.6](https://github.com/kristof-mattei/autoheal-rs/compare/v1.13.5..v1.13.6) - 2026-09-03

### 🐛 Bug Fixes

- *(config)* Require `--client-key` and `--client-cert` together by [@kristof-mattei](https://github.com/kristof-mattei) ([`5b9322c`](https://github.com/kristof-mattei/autoheal-rs/commit/5b9322c4356031aa4635b7e910135229bf5502c6))

### 💼 Other

- Write `/etc/passwd` and `/etc/group` as heredocs instead of creating the user in alpine by [@kristof-mattei](https://github.com/kristof-mattei) ([`ebb45c8`](https://github.com/kristof-mattei/autoheal-rs/commit/ebb45c85db6d33778dbbe097414fe3fb97edd7bc))
## [1.13.5](https://github.com/kristof-mattei/autoheal-rs/compare/v1.13.3..v1.13.5) - 2026-08-01

### 🐛 Bug Fixes

- *(ci)* Poll for the release source tag instead of racing the push retag by [@kristof-mattei](https://github.com/kristof-mattei) ([`c0be1d5`](https://github.com/kristof-mattei/autoheal-rs/commit/c0be1d5486cbb92bf8772e4aff991b4da9861f99))
- *(ci)* Skip Docker Hub tags that already exist with the expected digest by [@kristof-mattei](https://github.com/kristof-mattei) ([`784a607`](https://github.com/kristof-mattei/autoheal-rs/commit/784a607d54f6872e85ed0fc069377ae347530858))
## [1.13.3](https://github.com/kristof-mattei/autoheal-rs/compare/v1.13.2..v1.13.3) - 2026-07-31

### 🐛 Bug Fixes

- *(ci)* Disable asking for funds when installing pnpm by [@kristof-mattei](https://github.com/kristof-mattei) ([`b661b33`](https://github.com/kristof-mattei/autoheal-rs/commit/b661b336aeed70395e1a0a25484b48d88f6d3cb4))
- *(ci)* Merge the PR based on the branch name, not the url by [@kristof-mattei](https://github.com/kristof-mattei) ([`b0af60a`](https://github.com/kristof-mattei/autoheal-rs/commit/b0af60a01b2f10665e3d2ffaf658fc5e9f4c1cff))
- *(ci)* Address actionlint by [@kristof-mattei](https://github.com/kristof-mattei) ([`3d8e464`](https://github.com/kristof-mattei/autoheal-rs/commit/3d8e4648ec20159a31b930c0a6a6f39fa5b0eb6e))
- *(deps)* Update rust crate http-body-util to v0.1.4 by [@renovate[bot]](https://github.com/renovate[bot]) ([`280ab51`](https://github.com/kristof-mattei/autoheal-rs/commit/280ab51e062a654f4560a72c5fea60b07aa0a9d1))
- *(deps)* Update rust crate tokio to v1.53.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`ef562a9`](https://github.com/kristof-mattei/autoheal-rs/commit/ef562a94e421e6496bad43539c4dc4cf17ecb6a4))
- *(deps)* Update rust crate hyper to v1.11.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b45ee01`](https://github.com/kristof-mattei/autoheal-rs/commit/b45ee010e94453b023e9495f3b832b2610d01515))
- *(deps)* Update rust crate tokio-util to v0.7.19 by [@renovate[bot]](https://github.com/renovate[bot]) ([`0d07bfd`](https://github.com/kristof-mattei/autoheal-rs/commit/0d07bfdd4427e1fc2afc2bdc5ef6953fabed70bf))
- *(deps)* Update rust crate libc to v0.2.189 by [@renovate[bot]](https://github.com/renovate[bot]) ([`1419634`](https://github.com/kristof-mattei/autoheal-rs/commit/141963478847fe0ebddbb2eb525dc22853f7c904))
- *(deps)* Update rust crate http to v1.5.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e71012d`](https://github.com/kristof-mattei/autoheal-rs/commit/e71012dc0574f5e2f2b3024340abc12475c97ed1))
- *(deps)* Update rust crate clap to v4.6.5 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e0e14b0`](https://github.com/kristof-mattei/autoheal-rs/commit/e0e14b01c9e8f6471daabd86fbbc6fa94fa20bf2))
- Use tokio interval by [@kristof-mattei](https://github.com/kristof-mattei) ([`ae791fa`](https://github.com/kristof-mattei/autoheal-rs/commit/ae791faff6675a62cd15ce90ae10bec06d136fa6))
- Prettier config typescript version by [@kristof-mattei](https://github.com/kristof-mattei) ([`9a04bc8`](https://github.com/kristof-mattei/autoheal-rs/commit/9a04bc814c428efb9b1af82b6395e8a08fb69caa))
- For markdown, decrease tabwidth by [@kristof-mattei](https://github.com/kristof-mattei) ([`775286c`](https://github.com/kristof-mattei/autoheal-rs/commit/775286c57e10452aa3580f168399f48f50066e40))

### 💼 Other

- Generate SBOM & provenance attestations, embedding the actual crate list via cargo-auditable by [@kristof-mattei](https://github.com/kristof-mattei) ([`7069ac7`](https://github.com/kristof-mattei/autoheal-rs/commit/7069ac7c50b8a9c3eab63e1b31ebf77164141d7c))

### ⚙️ Miscellaneous Tasks

- *(ci)* Long-form options by [@kristof-mattei](https://github.com/kristof-mattei) ([`8cc89ce`](https://github.com/kristof-mattei/autoheal-rs/commit/8cc89ce310fd951216ebb9bf5dad46a9eb5b4cab))
- *(ci)* Create release commits with the api so they're signed by [@kristof-mattei](https://github.com/kristof-mattei) ([`0cacc4c`](https://github.com/kristof-mattei/autoheal-rs/commit/0cacc4cfff7e3e44eaaa27a9fcdf0d7006720c8b))
- *(ci)* Let release runs finish and clean up stranded branches by [@kristof-mattei](https://github.com/kristof-mattei) ([`09c302f`](https://github.com/kristof-mattei/autoheal-rs/commit/09c302fef81815f523c7a8fa2b849b02222d3bdc))
- *(ci)* Pass the github token to git-cliff via the environment by [@kristof-mattei](https://github.com/kristof-mattei) ([`73fe1c0`](https://github.com/kristof-mattei/autoheal-rs/commit/73fe1c0d32e7de5d97b90a7ee7433ef7edb75959))
- *(ci)* Only mark stable releases as latest by [@kristof-mattei](https://github.com/kristof-mattei) ([`357e974`](https://github.com/kristof-mattei/autoheal-rs/commit/357e974a1e321345048fc53d805da083c191149e))
- *(ci)* Update the release PR body via the rest api by [@kristof-mattei](https://github.com/kristof-mattei) ([`0358506`](https://github.com/kristof-mattei/autoheal-rs/commit/035850687f39ce852128a059f83f596cba9fab40))
- *(ci)* Upload images sequentially by [@kristof-mattei](https://github.com/kristof-mattei) ([`128f6d4`](https://github.com/kristof-mattei/autoheal-rs/commit/128f6d4a0436cec8aeba4754262307ff0274b9df))
- Ban more macros by [@kristof-mattei](https://github.com/kristof-mattei) ([`602457a`](https://github.com/kristof-mattei/autoheal-rs/commit/602457a4d5bcd65f5214d48e5601836041bebc0f))
- Cspell by [@kristof-mattei](https://github.com/kristof-mattei) ([`9f5797a`](https://github.com/kristof-mattei/autoheal-rs/commit/9f5797a16adf5f2880b5bbdea2827ad7cf465688))
- Remove copilot instructions as copilot is now behind a paywall by [@kristof-mattei](https://github.com/kristof-mattei) ([`8a34ae2`](https://github.com/kristof-mattei/autoheal-rs/commit/8a34ae290dfdb5ddf5d2b73eeca90a14146f3e9b))
## [1.13.2](https://github.com/kristof-mattei/autoheal-rs/compare/v1.13.1..v1.13.2) - 2026-07-07

### 🐛 Bug Fixes

- *(ci)* Parallel upload by [@kristof-mattei](https://github.com/kristof-mattei) ([`c690c06`](https://github.com/kristof-mattei/autoheal-rs/commit/c690c0622b43647b20f2f31e5a2b8fcab6418b3c))
- *(ci)* Disable caching ./target by [@kristof-mattei](https://github.com/kristof-mattei) ([`b9452ad`](https://github.com/kristof-mattei/autoheal-rs/commit/b9452adde2041cfb056b250b9dc1fe33ff81347b))
- *(ci)* `warm-up-cache` does not need mold by [@kristof-mattei](https://github.com/kristof-mattei) ([`b1f29a5`](https://github.com/kristof-mattei/autoheal-rs/commit/b1f29a5cf0d51fb9b9d5c86f3a437548832b04e9))
- *(ci)* Don't cache cargo registry by [@kristof-mattei](https://github.com/kristof-mattei) ([`4122327`](https://github.com/kristof-mattei/autoheal-rs/commit/41223274ed701eafd121660d73c9d77bb073167d))
- *(ci)* Disable registry caching in crate release by [@kristof-mattei](https://github.com/kristof-mattei) ([`227ac02`](https://github.com/kristof-mattei/autoheal-rs/commit/227ac0201fe4cf8ad8a6f8d5df08c1c369b11df5))
- *(ci)* Add missing read permissions by [@kristof-mattei](https://github.com/kristof-mattei) ([`0272238`](https://github.com/kristof-mattei/autoheal-rs/commit/027223880c30bf615630839cf7913ba63549b4fd))
- *(ci)* Retry installing spellcheck, it's sometimes flaky, also disable compile when the HTTP request fails by [@kristof-mattei](https://github.com/kristof-mattei) ([`1dd49a7`](https://github.com/kristof-mattei/autoheal-rs/commit/1dd49a7b7cda4f75ec810e29fdea0c7e7357abd5))
- *(deps)* Pin dependencies by [@renovate[bot]](https://github.com/renovate[bot]) ([`6e3faf5`](https://github.com/kristof-mattei/autoheal-rs/commit/6e3faf5afa52deaeb3d8539dc6e2f79268559d11))
- `cargo-fmt` does not need cache, and shouldn't try to restore it, worse, it could save a broken cache should `warm-up-cache` fail by [@kristof-mattei](https://github.com/kristof-mattei) ([`f30035a`](https://github.com/kristof-mattei/autoheal-rs/commit/f30035a937be4e0f0ff705beb239c300b7d7c3a4))
- Remove the `-build` in the cache name by [@kristof-mattei](https://github.com/kristof-mattei) ([`ec0649f`](https://github.com/kristof-mattei/autoheal-rs/commit/ec0649fb55579105c509cadd533d83e0143eb150))
- Only restore by [@kristof-mattei](https://github.com/kristof-mattei) ([`cb8f178`](https://github.com/kristof-mattei/autoheal-rs/commit/cb8f1786ae3b72f919f13b8a73410ba63c0b2b86))

### ⚙️ Miscellaneous Tasks

- Missing codeql.yml permission by [@kristof-mattei](https://github.com/kristof-mattei) ([`760bd59`](https://github.com/kristof-mattei/autoheal-rs/commit/760bd592b94e4b01b2220f40a96ff8430a312015))
- Missing attestation permission by [@kristof-mattei](https://github.com/kristof-mattei) ([`babe2a3`](https://github.com/kristof-mattei/autoheal-rs/commit/babe2a3c30a87c76a47a9429c24330ab8fb09859))
## [1.13.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.13.0..v1.13.1) - 2026-07-01

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.52.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`ebd0884`](https://github.com/kristof-mattei/autoheal-rs/commit/ebd08844c3ac11bdbe3b59d3b8183e87f2a81bca))
- *(deps)* Update rust crate hashbrown to 0.17.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`a6e34f5`](https://github.com/kristof-mattei/autoheal-rs/commit/a6e34f5ada188244855bbbf5eb3926796f18a4b2))
- *(deps)* Update rust crate mimalloc to 0.1.52 by [@renovate[bot]](https://github.com/renovate[bot]) ([`1890f97`](https://github.com/kristof-mattei/autoheal-rs/commit/1890f97240cf108d41c370179666282ebe343759))
- *(deps)* Update rust crate hyper to 1.10.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d85406d`](https://github.com/kristof-mattei/autoheal-rs/commit/d85406da9a12cbef7c29e47dee220b77b73ad23a))
- *(deps)* Update rust crate http to 1.4.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d6cd50b`](https://github.com/kristof-mattei/autoheal-rs/commit/d6cd50bb8952db9f69bde9286b624dff96f01949))
- *(deps)* Pin dependencies by [@renovate[bot]](https://github.com/renovate[bot]) ([`915be69`](https://github.com/kristof-mattei/autoheal-rs/commit/915be6932f7662f7aba54f1864d9fd08280d9f6f))

### ⚙️ Miscellaneous Tasks

- Node v26 and pnpm 11.2.2 by [@kristof-mattei](https://github.com/kristof-mattei) ([`16139f7`](https://github.com/kristof-mattei/autoheal-rs/commit/16139f7f5ad75af955e9cb7cce9a5968d478ee04))
- Enable `as_conversions` lint by [@kristof-mattei](https://github.com/kristof-mattei) ([`3bfb7bb`](https://github.com/kristof-mattei/autoheal-rs/commit/3bfb7bb6d1a60f11ecfa7c46b7b6f3d5f4e795a0))
- Re-establish link by [@kristof-mattei](https://github.com/kristof-mattei) ([`9b943e2`](https://github.com/kristof-mattei/autoheal-rs/commit/9b943e236a77d3936379035b9371ca44f851c714))
- Re-establish contents by [@kristof-mattei](https://github.com/kristof-mattei) ([`e6d92ca`](https://github.com/kristof-mattei/autoheal-rs/commit/e6d92ca95f4526a0f232f4adbf8219b1e3ee17c2))
- Address new lint by [@kristof-mattei](https://github.com/kristof-mattei) ([`7877146`](https://github.com/kristof-mattei/autoheal-rs/commit/7877146579c030004c80455fb3da6d91bad43c68))
- Mandate `pretty_assertions::assert_matches` over `std` version by [@kristof-mattei](https://github.com/kristof-mattei) ([`d5e030c`](https://github.com/kristof-mattei/autoheal-rs/commit/d5e030cf9a767a50b2c10ad5eac80bf2257c49ab))
## [1.13.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.12.4..v1.13.0) - 2026-04-30

### 🚀 Features

- Checkbox to allow for auto-merging a release PR by [@kristof-mattei](https://github.com/kristof-mattei) ([`8b67c89`](https://github.com/kristof-mattei/autoheal-rs/commit/8b67c89b30a8d6b3d4822110734fa4497caabe28))
## [1.12.4](https://github.com/kristof-mattei/autoheal-rs/compare/v1.12.3..v1.12.4) - 2026-04-25

### 🐛 Bug Fixes

- *(deps)* Update rust crate mimalloc to 0.1.50 by [@renovate[bot]](https://github.com/renovate[bot]) ([`f488e9c`](https://github.com/kristof-mattei/autoheal-rs/commit/f488e9c023782304221631029603e77be7779648))
- *(deps)* Update rust crate libc to 0.2.186 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d4dd213`](https://github.com/kristof-mattei/autoheal-rs/commit/d4dd213db02c0aa43ed7642b3cefe161e2b1df95))
- *(deps)* Update rust crate twistlock to 0.2.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`c5f8b92`](https://github.com/kristof-mattei/autoheal-rs/commit/c5f8b92901751d698ed0744b143b61c9028791fa))

### ⚙️ Miscellaneous Tasks

- Move to keep track by [@kristof-mattei](https://github.com/kristof-mattei) ([`caaabc6`](https://github.com/kristof-mattei/autoheal-rs/commit/caaabc689c02f78d1c464cc173c9a931255876c0))
- Restore by [@kristof-mattei](https://github.com/kristof-mattei) ([`8a075af`](https://github.com/kristof-mattei/autoheal-rs/commit/8a075afc397960177ca8b3818ac5cdc84c63292e))
## [1.12.3](https://github.com/kristof-mattei/autoheal-rs/compare/v1.12.2..v1.12.3) - 2026-04-23

### 🐛 Bug Fixes

- *(deps)* `mimalloc` by default now is v3, removing flag by [@kristof-mattei](https://github.com/kristof-mattei) ([`207ec6c`](https://github.com/kristof-mattei/autoheal-rs/commit/207ec6c5965ed8f5139c6912a604f7e3b6a0afaa))
- *(deps)* `mimalloc` by default now is v3, removing flag by [@kristof-mattei](https://github.com/kristof-mattei) ([`c0fdb8a`](https://github.com/kristof-mattei/autoheal-rs/commit/c0fdb8aa5fae0dd08bcfb2441388bd458dd23a0c))
- *(deps)* Update rust crate mimalloc to 0.1.50 by [@renovate[bot]](https://github.com/renovate[bot]) ([`a960433`](https://github.com/kristof-mattei/autoheal-rs/commit/a960433b7c26fb80b6e8afbb28b39faecc70dc16))
- Signals now exit with + 128 by [@kristof-mattei](https://github.com/kristof-mattei) ([`c0a3afa`](https://github.com/kristof-mattei/autoheal-rs/commit/c0a3afaf1436f6394fe3b23e909260c26768f6d1))
- Tokio_unstable by [@kristof-mattei](https://github.com/kristof-mattei) ([`ecab2c0`](https://github.com/kristof-mattei/autoheal-rs/commit/ecab2c052e9acde309fb6335b7d6b2a50da75e8a))
## [1.12.2](https://github.com/kristof-mattei/autoheal-rs/compare/v1.12.1..v1.12.2) - 2026-04-18

### 🐛 Bug Fixes

- `get_name()` returns a single name by [@kristof-mattei](https://github.com/kristof-mattei) ([`8477c41`](https://github.com/kristof-mattei/autoheal-rs/commit/8477c4124703213751a9cab7dbdef11a1704732b))
- Certificates in Docker container so that webhooks over https work by [@kristof-mattei](https://github.com/kristof-mattei) ([`6bf2763`](https://github.com/kristof-mattei/autoheal-rs/commit/6bf27638c6e45d1377c8bbb8b7e97de6e5fa5fdc))
## [1.12.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.12.0..v1.12.1) - 2026-04-18

### 🐛 Bug Fixes

- *(deps)* Update rust crate twistlock to 0.2.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6ceef19`](https://github.com/kristof-mattei/autoheal-rs/commit/6ceef1996a22671d8b6f6786e8834716b93801f1))
- Properly track history, as before we never recorded how often something was restarted by [@kristof-mattei](https://github.com/kristof-mattei) ([`7311c6a`](https://github.com/kristof-mattei/autoheal-rs/commit/7311c6af911670aa9716316d19d6f228bc1cc564))
- Twistlock updates by [@kristof-mattei](https://github.com/kristof-mattei) ([`cf5f1d3`](https://github.com/kristof-mattei/autoheal-rs/commit/cf5f1d3b874342341958fddc7740b34f85ad0839))

### ⚙️ Miscellaneous Tasks

- *(release)* Link in release proposal now points to sha as tag doesn't exist yet by [@kristof-mattei](https://github.com/kristof-mattei) ([`f2bcfc8`](https://github.com/kristof-mattei/autoheal-rs/commit/f2bcfc8ed61488c9863b26533432becd6e7267df))
## [1.12.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.11.1..v1.12.0) - 2026-04-17

### 🚀 Features

- Use clap to parse early by [@kristof-mattei](https://github.com/kristof-mattei) ([`648bd76`](https://github.com/kristof-mattei/autoheal-rs/commit/648bd76221a31be967a5b0fe657445d1b7a94627))
- Twistlock defers timeout reading to client by [@kristof-mattei](https://github.com/kristof-mattei) ([`090e396`](https://github.com/kristof-mattei/autoheal-rs/commit/090e3961b48b7017000f99a80c88795078f6fccc))

### 🐛 Bug Fixes

- Don't inline in message, clearer for when we do json logging by [@kristof-mattei](https://github.com/kristof-mattei) ([`f6a25b4`](https://github.com/kristof-mattei/autoheal-rs/commit/f6a25b4cd93aecad29663fbaf0b7e84454d24833))

### ⚙️ Miscellaneous Tasks

- Twistlock update by [@kristof-mattei](https://github.com/kristof-mattei) ([`5510440`](https://github.com/kristof-mattei/autoheal-rs/commit/5510440d91342af87fdb3d74184332dc146b08f3))
## [1.11.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.11.0..v1.11.1) - 2026-04-17

### 🐛 Bug Fixes

- Twistlock is now on crates.io by [@kristof-mattei](https://github.com/kristof-mattei) ([`c1819c2`](https://github.com/kristof-mattei/autoheal-rs/commit/c1819c263bcd0b494c4c8726c00065a2324b362b))
## [1.11.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.10.2..v1.11.0) - 2026-04-17

### 🚀 Features

- Use twistlock for shared docker interactions by [@kristof-mattei](https://github.com/kristof-mattei) ([`393f732`](https://github.com/kristof-mattei/autoheal-rs/commit/393f732b1e69ec48a66084cdcb5263469c478d21))

### 🐛 Bug Fixes

- *(deps)* Update rust crate hashbrown to 0.17.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`7199657`](https://github.com/kristof-mattei/autoheal-rs/commit/7199657056958e3f52dc5c274b5fc925c8e5e3f8))
- *(deps)* Update rust crate rustls to 0.23.38 by [@renovate[bot]](https://github.com/renovate[bot]) ([`7bc7780`](https://github.com/kristof-mattei/autoheal-rs/commit/7bc77809ee18ac1b4c7d43894fb121e786f2c09b))
- *(deps)* Update rust crate hyper-unix-socket to 0.6.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`fb02fed`](https://github.com/kristof-mattei/autoheal-rs/commit/fb02feda8bed16b0ddb1c53a4b866faaea1a9ee9))
- *(deps)* Update rust crate libc to 0.2.185 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6b16078`](https://github.com/kristof-mattei/autoheal-rs/commit/6b1607845e615531bc14483db6c06abc98028ef9))
- *(deps)* Update rust crate hyper-rustls to 0.27.9 by [@renovate[bot]](https://github.com/renovate[bot]) ([`fb44459`](https://github.com/kristof-mattei/autoheal-rs/commit/fb44459cd72536685dc59e4eab73635c654ed40c))
- *(deps)* Update rust crate clap to 4.6.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d44015f`](https://github.com/kristof-mattei/autoheal-rs/commit/d44015f1f5aa6cebf2121580c67dcf41c9ee2f73))
- *(deps)* Update rust crate tokio to 1.52.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`47610d9`](https://github.com/kristof-mattei/autoheal-rs/commit/47610d999de971b113b8206b62bae2d5a7cc4eeb))
- Disable plugins by [@kristof-mattei](https://github.com/kristof-mattei) ([`3725751`](https://github.com/kristof-mattei/autoheal-rs/commit/37257510cce405e0426a57c6e76ec53d73454c91))

### ⚙️ Miscellaneous Tasks

- *(ci)* Restore ability to do prerelease versions by [@kristof-mattei](https://github.com/kristof-mattei) ([`7c5b0ea`](https://github.com/kristof-mattei/autoheal-rs/commit/7c5b0ea895c5e92aa3c69fec62e3a7156f2ca183))
- *(ci)* Allow re-release pre-release without having to add features by [@kristof-mattei](https://github.com/kristof-mattei) ([`657a7db`](https://github.com/kristof-mattei/autoheal-rs/commit/657a7db145169f1dd3cd17b587324fae32ec4d04))
- Start of tearing it apart by [@kristof-mattei](https://github.com/kristof-mattei) ([`7e197c6`](https://github.com/kristof-mattei/autoheal-rs/commit/7e197c6ea54cbecc859cb2be928484e015492628))
## [1.10.2](https://github.com/kristof-mattei/autoheal-rs/compare/v1.10.1..v1.10.2) - 2026-04-05

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.51.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`37781dd`](https://github.com/kristof-mattei/autoheal-rs/commit/37781dd9559d8d09d419777364c1caf13646dddb))
- We can now release without having ANY tags on the repo by [@kristof-mattei](https://github.com/kristof-mattei) ([`80cbc44`](https://github.com/kristof-mattei/autoheal-rs/commit/80cbc44189bf721637640dbf7257f852ca82f877))

### ⚙️ Miscellaneous Tasks

- *(ci)* Upload edge to docker.io by [@kristof-mattei](https://github.com/kristof-mattei) ([`f235bac`](https://github.com/kristof-mattei/autoheal-rs/commit/f235bac7e7bf720a4c42711f12fa57cb361c6adf))
- *(ci)* Reattest every stage by [@kristof-mattei](https://github.com/kristof-mattei) ([`19c7f04`](https://github.com/kristof-mattei/autoheal-rs/commit/19c7f048fad6474f798c9d810dbe1d2278731d6d))
- *(ci)* Use skopeo wherever possible by [@kristof-mattei](https://github.com/kristof-mattei) ([`50a6786`](https://github.com/kristof-mattei/autoheal-rs/commit/50a678665fd200021293e938c0b493891077a5ef))
- *(ci)* Scout for visualizing cves etc by [@kristof-mattei](https://github.com/kristof-mattei) ([`2fe6e06`](https://github.com/kristof-mattei/autoheal-rs/commit/2fe6e06ada42423eeacec95be6a29ccbd3806068))
- *(ci)* Username is not secret by [@kristof-mattei](https://github.com/kristof-mattei) ([`7d168f4`](https://github.com/kristof-mattei/autoheal-rs/commit/7d168f446d7bbdabaf2bd58c2818af2a65b07c41))
- *(release)* Release v1.10.2 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`34a50c7`](https://github.com/kristof-mattei/autoheal-rs/commit/34a50c72a4df42b1efcca870f5524ef7a10d5575))
- Fix fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`1c63af5`](https://github.com/kristof-mattei/autoheal-rs/commit/1c63af585079ee98e0b241fc6683c31982159106))
## [1.10.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.10.0..v1.10.1) - 2026-04-01

### 🐛 Bug Fixes

- *(deps)* Update rust crate hyper to 1.9.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`cdf8f50`](https://github.com/kristof-mattei/autoheal-rs/commit/cdf8f50c8b57c6cb5759f34ca79cd6fcee1438ef))
- *(deps)* Update rust crate libc to 0.2.184 by [@renovate[bot]](https://github.com/renovate[bot]) ([`8cb5fd8`](https://github.com/kristof-mattei/autoheal-rs/commit/8cb5fd8570d134839e6ab6096c0695c1b51dd01c))
- Allow recover from failed publish by [@kristof-mattei](https://github.com/kristof-mattei) ([`4a6232d`](https://github.com/kristof-mattei/autoheal-rs/commit/4a6232df706f61d1ea6bd7e74262ed468bf8d2aa))

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.10.1 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`8473ba2`](https://github.com/kristof-mattei/autoheal-rs/commit/8473ba27d93835d3a0eaa9428976e076f4428b29))
- Prepare doesn't need the cache & toolchain by [@kristof-mattei](https://github.com/kristof-mattei) ([`a255daf`](https://github.com/kristof-mattei/autoheal-rs/commit/a255daf9066d8a79c834e73fee97a89c7d32b1a2))
- Render username link by [@kristof-mattei](https://github.com/kristof-mattei) ([`19debe7`](https://github.com/kristof-mattei/autoheal-rs/commit/19debe713ec00dc8f6534a65bab8f1f9ef5dccec))
- Sort tags by creation time by [@kristof-mattei](https://github.com/kristof-mattei) ([`39ce04b`](https://github.com/kristof-mattei/autoheal-rs/commit/39ce04bd2b02ef17d869c2828bf3e641854a25b1))
## [1.10.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.9.1..v1.10.0) - 2026-03-29

### 🚀 Features

- Remove crane by [@kristof-mattei](https://github.com/kristof-mattei) ([`073ba68`](https://github.com/kristof-mattei/autoheal-rs/commit/073ba68f70d54c2adebaa489958e9527f20b6c9f))

### 🐛 Bug Fixes

- *(deps)* Update rust crate libc to 0.2.183 by [@renovate[bot]](https://github.com/renovate[bot]) ([`26a0536`](https://github.com/kristof-mattei/autoheal-rs/commit/26a053628058aea327d2479fd98746a0a1864960))
- *(deps)* Update rust crate clap to 4.6.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d4f5e45`](https://github.com/kristof-mattei/autoheal-rs/commit/d4f5e458a107bd5502e17a8845c560edd34c0c19))
- *(deps)* Update rust crate tracing-subscriber to 0.3.23 by [@renovate[bot]](https://github.com/renovate[bot]) ([`62fce7d`](https://github.com/kristof-mattei/autoheal-rs/commit/62fce7d2dc21bc06903db9ece87275637cde4e96))
- Use wildcard by [@kristof-mattei](https://github.com/kristof-mattei) ([`54c7292`](https://github.com/kristof-mattei/autoheal-rs/commit/54c7292ccd2141ef3ee576b988e32a9e214fa035))
- Embed the calculated version in the image label, not the PR it came from by [@kristof-mattei](https://github.com/kristof-mattei) ([`654a96c`](https://github.com/kristof-mattei/autoheal-rs/commit/654a96c34a4b33e89f2b6eecc443d8097b5cd1c2))
- `std::mem::zeroed()` asserts that all zeroes is valid for the struct by [@kristof-mattei](https://github.com/kristof-mattei) ([`91dc697`](https://github.com/kristof-mattei/autoheal-rs/commit/91dc6971ebb36491e0e4292cc922e6c448cd1091))
- Use skopeo to copy the image, as buildx wrapped the image in an OCI image index by [@kristof-mattei](https://github.com/kristof-mattei) ([`7c179a2`](https://github.com/kristof-mattei/autoheal-rs/commit/7c179a2ac1aeda46e08cb54383d904aed5dc5c8b))

### ⚙️ Miscellaneous Tasks

- *(build)* Switch to annotations by [@kristof-mattei](https://github.com/kristof-mattei) ([`c041c69`](https://github.com/kristof-mattei/autoheal-rs/commit/c041c6948bd7f7318ad320659d8233d416bcbd6a))
- *(ci)* Allow empty by [@kristof-mattei](https://github.com/kristof-mattei) ([`e6ce7fb`](https://github.com/kristof-mattei/autoheal-rs/commit/e6ce7fbb2a3ceb423c1c822dca92049f70f43ea2))
- *(ci)* Comment cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`4fa99d9`](https://github.com/kristof-mattei/autoheal-rs/commit/4fa99d9cace57656b0c725fa995cbf85cedd07b4))
- *(release)* Release v1.10.0 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`071b71b`](https://github.com/kristof-mattei/autoheal-rs/commit/071b71b92c1f0650e2931d184b85c565323df2b6))
- Clean up ignore by [@kristof-mattei](https://github.com/kristof-mattei) ([`404a01e`](https://github.com/kristof-mattei/autoheal-rs/commit/404a01e12c18e72240ce1f85a49ee6aa8fc9e403))
- Rust 1.94.0 Docker by [@kristof-mattei](https://github.com/kristof-mattei) ([`9427eb1`](https://github.com/kristof-mattei/autoheal-rs/commit/9427eb18b46bf01da1357e3d83312f0846aa42ae))
- Pin sha for Rust image by [@kristof-mattei](https://github.com/kristof-mattei) ([`e2986db`](https://github.com/kristof-mattei/autoheal-rs/commit/e2986db43914999be69f279056fd5be0908f7216))
- Skip more when releasing by [@kristof-mattei](https://github.com/kristof-mattei) ([`762a3bc`](https://github.com/kristof-mattei/autoheal-rs/commit/762a3bcfe9b2c625e41b5020939e9bcdb3fb8815))
- Lowercase variables by [@kristof-mattei](https://github.com/kristof-mattei) ([`dff26ac`](https://github.com/kristof-mattei/autoheal-rs/commit/dff26ac37a1826cf3b57c9c848aba1a9de9e5d67))
- Lowercase variables & path hygiene by [@kristof-mattei](https://github.com/kristof-mattei) ([`668015d`](https://github.com/kristof-mattei/autoheal-rs/commit/668015dbabaf3cf2f2480896ed7737e4b8f7490c))
- Disable formatting on push to main, different array syntax by [@kristof-mattei](https://github.com/kristof-mattei) ([`c1d61b0`](https://github.com/kristof-mattei/autoheal-rs/commit/c1d61b06948c3bde697272a21bd27c66d0cb5fd8))
- Use same [] syntax everywhere for yaml by [@kristof-mattei](https://github.com/kristof-mattei) ([`a90eeff`](https://github.com/kristof-mattei/autoheal-rs/commit/a90eefffd10960f09c14efb2200161ef2663f6d2))
- Move / add mold by [@kristof-mattei](https://github.com/kristof-mattei) ([`639f3a5`](https://github.com/kristof-mattei/autoheal-rs/commit/639f3a50a697763fc25b8e0c4934a16ad8e83cd6))
- Improve caching by [@kristof-mattei](https://github.com/kristof-mattei) ([`6841817`](https://github.com/kristof-mattei/autoheal-rs/commit/6841817ca0e5927b21745c8608c9e2a972ea92fc))
- Test by [@kristof-mattei](https://github.com/kristof-mattei) ([`222103f`](https://github.com/kristof-mattei/autoheal-rs/commit/222103f074c48ae8645cb5ae37c4fed4fcea447d))
- Remove test by [@kristof-mattei](https://github.com/kristof-mattei) ([`c73b6d6`](https://github.com/kristof-mattei/autoheal-rs/commit/c73b6d6d7181b52e65b4e94f37bbde8ddeabac74))
- Not needed by [@kristof-mattei](https://github.com/kristof-mattei) ([`e92df85`](https://github.com/kristof-mattei/autoheal-rs/commit/e92df85de4a37e42e9774e6f983c27ebdf09b3c2))
- Remove suprious newline by [@kristof-mattei](https://github.com/kristof-mattei) ([`be7faa7`](https://github.com/kristof-mattei/autoheal-rs/commit/be7faa7228268d8f0b97fdb7348ac2a552e3c6c6))
- Move build cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`0ab1328`](https://github.com/kristof-mattei/autoheal-rs/commit/0ab1328687c12bf15ae3e21bb031a2d38632c4c8))
- Rename by [@kristof-mattei](https://github.com/kristof-mattei) ([`6544b67`](https://github.com/kristof-mattei/autoheal-rs/commit/6544b67f89b757a1a86986291fd1efef42f07445))
- Stop skopeo from trying to infer type by [@kristof-mattei](https://github.com/kristof-mattei) ([`66ca282`](https://github.com/kristof-mattei/autoheal-rs/commit/66ca282f0a4adcd2a4ee63f8ad69c9c822ac88ac))
- Ensure we copy all layers by [@kristof-mattei](https://github.com/kristof-mattei) ([`6428eb8`](https://github.com/kristof-mattei/autoheal-rs/commit/6428eb84930a25d7c8d4fda31d0f32dc4ad55f9f))
- Also run build & spellcheck once merged, this warms up the cache for subsequent builds by [@kristof-mattei](https://github.com/kristof-mattei) ([`e6db1ca`](https://github.com/kristof-mattei/autoheal-rs/commit/e6db1ca9fa9b6353c81e9c3870a31fdde32303ce))
- Speed up spellcheck compilation with sccache by [@kristof-mattei](https://github.com/kristof-mattei) ([`5e53dcd`](https://github.com/kristof-mattei/autoheal-rs/commit/5e53dcd877309fafb5cf123c5a39697ccd2f47d7))
- Enable sccache for spellcheck by [@kristof-mattei](https://github.com/kristof-mattei) ([`d99fa01`](https://github.com/kristof-mattei/autoheal-rs/commit/d99fa01ce7d28befefea7882be13557a9d9ffc24))
- Remove cache cargo for spellcheck, as it doesn't do anything by [@kristof-mattei](https://github.com/kristof-mattei) ([`565017e`](https://github.com/kristof-mattei/autoheal-rs/commit/565017e04b68ad7de0efad32f98ba111041d773a))
- Use a spellcheck fork for quicker installation by [@kristof-mattei](https://github.com/kristof-mattei) ([`93bfd69`](https://github.com/kristof-mattei/autoheal-rs/commit/93bfd695bd0a871fbdb10a269b4d266194303729))
- No need to do spellcheck on push anymore, as we use pre-built binaries by [@kristof-mattei](https://github.com/kristof-mattei) ([`afa2005`](https://github.com/kristof-mattei/autoheal-rs/commit/afa2005227a8ac9fb078efe5abf6f9cad8f52c37))
- Convert manifest to index annotations by [@kristof-mattei](https://github.com/kristof-mattei) ([`affcb0a`](https://github.com/kristof-mattei/autoheal-rs/commit/affcb0a26c6f809ad3d355a997a62075ed22b827))
- Don't run machete on push by [@kristof-mattei](https://github.com/kristof-mattei) ([`f6bdbd8`](https://github.com/kristof-mattei/autoheal-rs/commit/f6bdbd887606edbc9ed3a52615afe69de39ed220))
- If no component bumps, bump patch by [@kristof-mattei](https://github.com/kristof-mattei) ([`fd4eca7`](https://github.com/kristof-mattei/autoheal-rs/commit/fd4eca7deceb7d644a16cb69879411090ed9e599))
- Add labels to the image too by [@kristof-mattei](https://github.com/kristof-mattei) ([`ba1cd05`](https://github.com/kristof-mattei/autoheal-rs/commit/ba1cd0507337529bdf19dbd8da2bd78014f79ced))
- Ability to publish to docker hub as well by [@kristof-mattei](https://github.com/kristof-mattei) ([`99103ff`](https://github.com/kristof-mattei/autoheal-rs/commit/99103ffedc5d4a2322609d3ce9f7e11af58bb1cd))
- Clarify name by [@kristof-mattei](https://github.com/kristof-mattei) ([`4ea320e`](https://github.com/kristof-mattei/autoheal-rs/commit/4ea320e830a0c69560e9ebc67fcdcca0f4687605))
## [1.9.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.9.0..v1.9.1) - 2026-03-03

### 🚀 Features

- Prerelease types by [@kristof-mattei](https://github.com/kristof-mattei) ([`c27ba7c`](https://github.com/kristof-mattei/autoheal-rs/commit/c27ba7c358bec30b3332520cd8aca88b73dafebf))

### 🐛 Bug Fixes

- *(deps)* Update rust crate clap to 4.5.60 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b499147`](https://github.com/kristof-mattei/autoheal-rs/commit/b4991478a2dc68deaad631a2cdf18d9e80d4c747))
- *(deps)* Update rust crate rustls to 0.23.37 by [@renovate[bot]](https://github.com/renovate[bot]) ([`84ed7e2`](https://github.com/kristof-mattei/autoheal-rs/commit/84ed7e2aa9f9340ad87ae0704206d8e7713ad8ef))
- *(deps)* Update rust crate tokio to 1.50.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`021bb7d`](https://github.com/kristof-mattei/autoheal-rs/commit/021bb7da13bd2aa7c8a9c98283684cea2df8ed3d))
- Truncate changelog when too long by [@kristof-mattei](https://github.com/kristof-mattei) ([`d06dc1c`](https://github.com/kristof-mattei/autoheal-rs/commit/d06dc1c8f9e10be96f3bf88aca729b2886c5ca31))
- Also truncate the release notes in the release by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c05c13`](https://github.com/kristof-mattei/autoheal-rs/commit/5c05c13dfee4524b931f1879804cdff9e80c6a33))
- Long form options by [@kristof-mattei](https://github.com/kristof-mattei) ([`55c52d6`](https://github.com/kristof-mattei/autoheal-rs/commit/55c52d64284913d31a02cbc831c82b0b94465176))
- Truncation on total size, not characters per line by [@kristof-mattei](https://github.com/kristof-mattei) ([`62df253`](https://github.com/kristof-mattei/autoheal-rs/commit/62df253e9217379e282005a81983159343359b1e))
- Add , for readability by [@kristof-mattei](https://github.com/kristof-mattei) ([`910b62f`](https://github.com/kristof-mattei/autoheal-rs/commit/910b62fbdf9c3ba6895f9d2945d3be90f33fb9c3))
- Ensure pre-release and release tags on containers are emitted with the 'v' prefix by [@kristof-mattei](https://github.com/kristof-mattei) ([`6edac00`](https://github.com/kristof-mattei/autoheal-rs/commit/6edac007d710b502d0db3c6e95a644eaf6a2044a))
- Skip spellcheck on release/ branches by [@kristof-mattei](https://github.com/kristof-mattei) ([`736f013`](https://github.com/kristof-mattei/autoheal-rs/commit/736f01396b30052e9f01927fd93f321f49ffeb8a))
- Don't render commit id if it doesn't exist yet by [@kristof-mattei](https://github.com/kristof-mattei) ([`3528d32`](https://github.com/kristof-mattei/autoheal-rs/commit/3528d32c7ccb599a9611741fb36736298b36ffde))
- Disable pre-release by [@kristof-mattei](https://github.com/kristof-mattei) ([`79d7dd0`](https://github.com/kristof-mattei/autoheal-rs/commit/79d7dd0d0b7dea0e7e5fa1a737e845dea6caa14a))
- Set default to `none` by [@kristof-mattei](https://github.com/kristof-mattei) ([`8f01df9`](https://github.com/kristof-mattei/autoheal-rs/commit/8f01df921bd091f5230f2721a078de8981ee8265))
- Fix trailing spaces by [@kristof-mattei](https://github.com/kristof-mattei) ([`9341634`](https://github.com/kristof-mattei/autoheal-rs/commit/934163416392000dddfdea80a3cdb1e7156acb47))

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.9.1 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`f190dc6`](https://github.com/kristof-mattei/autoheal-rs/commit/f190dc6d123ee245d0b9b1d5579cbc616406cd8d))
- Remove colon by [@kristof-mattei](https://github.com/kristof-mattei) ([`635bd83`](https://github.com/kristof-mattei/autoheal-rs/commit/635bd8386a4c2e47f7cace63f54341234c80218f))
- Decrease verbosity by [@kristof-mattei](https://github.com/kristof-mattei) ([`64c0473`](https://github.com/kristof-mattei/autoheal-rs/commit/64c04739ca3f7bb5711c2adc38a690f6413f48e7))
- `attest-build-provenance` is deprecated by [@kristof-mattei](https://github.com/kristof-mattei) ([`26fa46a`](https://github.com/kristof-mattei/autoheal-rs/commit/26fa46a6ed449dddd256e1817aa81300cd374346))
- Typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`59e98b7`](https://github.com/kristof-mattei/autoheal-rs/commit/59e98b7382ca6c0a220af8fdbe468ef70b21128c))
## [1.9.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.9.0-rc.1..v1.9.0) - 2026-02-18

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.9.0 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`ff70df9`](https://github.com/kristof-mattei/autoheal-rs/commit/ff70df9b3ec4f824a98b676640efa351358afcee))
## [1.9.0-rc.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.8.2..v1.9.0-rc.1) - 2026-02-18

### 🚀 Features

- Make clippy more strict by [@kristof-mattei](https://github.com/kristof-mattei) ([`17f9b68`](https://github.com/kristof-mattei/autoheal-rs/commit/17f9b68bafb0b8b550fac341f8a429e8480cdf69))
- Remove cargo-get, use cargo metadata by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c3ba9b`](https://github.com/kristof-mattei/autoheal-rs/commit/5c3ba9b2d608d3bf1917c0a61f8e3e00c3645b53))
- Add spell-checking by [@kristof-mattei](https://github.com/kristof-mattei) ([`aafed8a`](https://github.com/kristof-mattei/autoheal-rs/commit/aafed8a7e54b88066c1c5feb66b2b003873ecd1c))
- Clippy 1.93 lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`f13cef9`](https://github.com/kristof-mattei/autoheal-rs/commit/f13cef988195cc51a3fc02715bca40f5a117cdf1))
- Move to crates by [@kristof-mattei](https://github.com/kristof-mattei) ([`51ff0f0`](https://github.com/kristof-mattei/autoheal-rs/commit/51ff0f0005f659b594dca4a7e007e7b11e00dc9a))
- Modernize build by [@kristof-mattei](https://github.com/kristof-mattei) ([`80a409e`](https://github.com/kristof-mattei/autoheal-rs/commit/80a409e515740565941b92c6694d7ef4470f6ba9))
- Use `CARGO_TARGET_DIR` to separate caches by [@kristof-mattei](https://github.com/kristof-mattei) ([`d6a7766`](https://github.com/kristof-mattei/autoheal-rs/commit/d6a776643631728c2ca8e02fd87eaf46328f6d45))
- Use PRs to control releases by [@kristof-mattei](https://github.com/kristof-mattei) ([`6025b82`](https://github.com/kristof-mattei/autoheal-rs/commit/6025b8250b0e659104ea0a86a46bc48ba025cb15))
- Add username & PR by [@kristof-mattei](https://github.com/kristof-mattei) ([`e20ac88`](https://github.com/kristof-mattei/autoheal-rs/commit/e20ac88c0fa5c191e6f82a3e292060074a7acbb6))

### 🐛 Bug Fixes

- *(deps)* Update rust crate hashbrown to 0.16.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`8ea681a`](https://github.com/kristof-mattei/autoheal-rs/commit/8ea681ab468cbbcb4913034728e8ae6ae1b752cc))
- *(deps)* Update rust crate http to 1.4.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`3643d7c`](https://github.com/kristof-mattei/autoheal-rs/commit/3643d7c8463118f2974feda5e9176d58d5b5b416))
- *(deps)* Update tokio-tracing monorepo by [@renovate[bot]](https://github.com/renovate[bot]) ([`3a844c2`](https://github.com/kristof-mattei/autoheal-rs/commit/3a844c2728e45553753a061f461f1196e368a086))
- *(deps)* Update rust crate tracing to 0.1.44 by [@renovate[bot]](https://github.com/renovate[bot]) ([`50f01bf`](https://github.com/kristof-mattei/autoheal-rs/commit/50f01bf605c2c7a8c0e3998296670d0638f1582b))
- *(deps)* Update rust crate rustls-native-certs to 0.8.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b16fd23`](https://github.com/kristof-mattei/autoheal-rs/commit/b16fd237a549d46d32f2e8e9ae9cf3179838225a))
- *(deps)* Update rust crate tokio to 1.49.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`40aeac0`](https://github.com/kristof-mattei/autoheal-rs/commit/40aeac0d9ac122d83de7c2248a11b3d1c6242bc0))
- *(deps)* Update rust crate rustls to 0.23.36 by [@renovate[bot]](https://github.com/renovate[bot]) ([`67d8b86`](https://github.com/kristof-mattei/autoheal-rs/commit/67d8b86effbbe1dc5febe3739f88e7491ba657be))
- *(deps)* Update rust crate serde_json to 1.0.149 by [@renovate[bot]](https://github.com/renovate[bot]) ([`92509cd`](https://github.com/kristof-mattei/autoheal-rs/commit/92509cd0a729e6c3cb4a257be8e981feb7f1dc5b))
- *(deps)* Update rust crate hyper-util to 0.1.20 by [@renovate[bot]](https://github.com/renovate[bot]) ([`4f77e2e`](https://github.com/kristof-mattei/autoheal-rs/commit/4f77e2eb0551421b16019505badaeda6a7acc7b3))
- *(deps)* Update rust crate libc to 0.2.182 by [@renovate[bot]](https://github.com/renovate[bot]) ([`7f2c4f5`](https://github.com/kristof-mattei/autoheal-rs/commit/7f2c4f50c565fa995ac4516fe4c1d99306aecec8))
- *(deps)* Update rust crate clap to 4.5.59 by [@renovate[bot]](https://github.com/renovate[bot]) ([`ec5f84e`](https://github.com/kristof-mattei/autoheal-rs/commit/ec5f84ebcd4b8197c30583f480271056374569ac))
- Pin alpine image used in testing too by [@kristof-mattei](https://github.com/kristof-mattei) ([`57cc99b`](https://github.com/kristof-mattei/autoheal-rs/commit/57cc99bd738c4f90429e4afeaf86fcd09e78a807))
- Make spellcheck mandatory by [@kristof-mattei](https://github.com/kristof-mattei) ([`1eae461`](https://github.com/kristof-mattei/autoheal-rs/commit/1eae4611b34bcf58630fbe8457e9ebc1cee26e0f))
- More " by [@kristof-mattei](https://github.com/kristof-mattei) ([`e9630d4`](https://github.com/kristof-mattei/autoheal-rs/commit/e9630d4e66b7212a52ce156218ac06ca56b2f85d))
- Simplify installs by [@kristof-mattei](https://github.com/kristof-mattei) ([`9444a11`](https://github.com/kristof-mattei/autoheal-rs/commit/9444a1152476575d51e514c3166da394c4fd30ef))
- Remove trailing newline by [@kristof-mattei](https://github.com/kristof-mattei) ([`8d7ce8b`](https://github.com/kristof-mattei/autoheal-rs/commit/8d7ce8bb2d448f92db4f58a665faa69517ab41da))
- Make spellcheck mandatory by [@kristof-mattei](https://github.com/kristof-mattei) ([`c88f4d1`](https://github.com/kristof-mattei/autoheal-rs/commit/c88f4d121ed0d18202dcafaa5c06b23e92d250b0))
- Be explicit about casting the fn by [@kristof-mattei](https://github.com/kristof-mattei) ([`f4959f1`](https://github.com/kristof-mattei/autoheal-rs/commit/f4959f1b0e2640d8b100296473b228d0f429e521))
- Use slim by [@kristof-mattei](https://github.com/kristof-mattei) ([`d592167`](https://github.com/kristof-mattei/autoheal-rs/commit/d5921679e2109283e49fb99be46c0ebd21013f66))
- Use apt-get instaed of apt, as we're not supposed to use apt in scripts by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e73341`](https://github.com/kristof-mattei/autoheal-rs/commit/4e73341deee8d0e1f7134b2fbcba1c2075b011fd))
- Copying a directory copies the contents into the destination, so we have to repeat the name, meaning we cannot mix it with files by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e5cf88`](https://github.com/kristof-mattei/autoheal-rs/commit/4e5cf882676b9d2382754fa70f4e081d64d992a2))
- Add . by [@kristof-mattei](https://github.com/kristof-mattei) ([`bfa3e2b`](https://github.com/kristof-mattei/autoheal-rs/commit/bfa3e2b4fedd8868d07a67c99fd918af43bec31e))
- Move pretty_assertions to dev deps by [@kristof-mattei](https://github.com/kristof-mattei) ([`53756e3`](https://github.com/kristof-mattei/autoheal-rs/commit/53756e35b7275204607d9d3235f39ecfd2d2706c))
- Don't wait for codecov, it sometimes blocks by [@kristof-mattei](https://github.com/kristof-mattei) ([`62f2501`](https://github.com/kristof-mattei/autoheal-rs/commit/62f250176d914b20cc8ddc1f9ac45dd1835b48f3))
- `TARGETARCH` is already the arch, the `//\//-` was for `TARGETPLATFORM` by [@kristof-mattei](https://github.com/kristof-mattei) ([`3b8ca47`](https://github.com/kristof-mattei/autoheal-rs/commit/3b8ca4789fa67ba8503dd0837f48378be974d3c9))
- Reorganize crates by [@kristof-mattei](https://github.com/kristof-mattei) ([`db451c3`](https://github.com/kristof-mattei/autoheal-rs/commit/db451c334731fa8931f104bf4baad02643a27519))
- Quotes for safety by [@kristof-mattei](https://github.com/kristof-mattei) ([`f623589`](https://github.com/kristof-mattei/autoheal-rs/commit/f62358982e5c848e077d9527bc9a12390d3523e5))
- Version casing by [@kristof-mattei](https://github.com/kristof-mattei) ([`ea0d1c8`](https://github.com/kristof-mattei/autoheal-rs/commit/ea0d1c8bf7171c98e68aeeb5311e3c0cd3973fc7))
- Spellcheck builds from source, so cache cargo by [@kristof-mattei](https://github.com/kristof-mattei) ([`0d5f4da`](https://github.com/kristof-mattei/autoheal-rs/commit/0d5f4da8842d65c1d3f93d02f9fc9955b43f6893))
- Be explicit about copying as a file by [@kristof-mattei](https://github.com/kristof-mattei) ([`fbf153f`](https://github.com/kristof-mattei/autoheal-rs/commit/fbf153fa9467a304906144d0c8e7e9bf05b5c3b0))
- Missing mold by [@kristof-mattei](https://github.com/kristof-mattei) ([`24d2d64`](https://github.com/kristof-mattei/autoheal-rs/commit/24d2d64bacda3238c932f1d080504ea2785cb4a8))
- Generically touch files by [@kristof-mattei](https://github.com/kristof-mattei) ([`087abda`](https://github.com/kristof-mattei/autoheal-rs/commit/087abdaad3bd02f274f25c6b7a9b417b93fafd71))
- Don't hang should there be a question by [@kristof-mattei](https://github.com/kristof-mattei) ([`61daade`](https://github.com/kristof-mattei/autoheal-rs/commit/61daade12a64cdfd76ff4455a70e85be94204ef1))
- Fix touch not relying on /bin/bash by [@kristof-mattei](https://github.com/kristof-mattei) ([`ca5f315`](https://github.com/kristof-mattei/autoheal-rs/commit/ca5f3154fa175d6f179cb60d7c225db0a0014925))
- Success cannot be cancelled by [@kristof-mattei](https://github.com/kristof-mattei) ([`ff32f1f`](https://github.com/kristof-mattei/autoheal-rs/commit/ff32f1f2a5249a9fd1fa9f4c99d8edef8127f737))
- Re-enable container cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`3681a0b`](https://github.com/kristof-mattei/autoheal-rs/commit/3681a0bca2c2ad0057c77917856e91c1ade74587))
- Use frozen by [@kristof-mattei](https://github.com/kristof-mattei) ([`b83181e`](https://github.com/kristof-mattei/autoheal-rs/commit/b83181e3a58ca79f8ffd24d1de3361d4325b27e9))
- Quotes & reorder by [@kristof-mattei](https://github.com/kristof-mattei) ([`4d3399d`](https://github.com/kristof-mattei/autoheal-rs/commit/4d3399dfbaec8bcf300acca43fdd72e9ad27dbd8))
- Name by [@kristof-mattei](https://github.com/kristof-mattei) ([`dc479a1`](https://github.com/kristof-mattei/autoheal-rs/commit/dc479a1ba61a84416c7e6c31c6ff297b474c9f68))
- Copy into cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`3de174a`](https://github.com/kristof-mattei/autoheal-rs/commit/3de174a45d3275b82def24db1eecc8d2d36ac52a))
- The subsequent rust builds need to come from the warmed up cache, and nothing else by [@kristof-mattei](https://github.com/kristof-mattei) ([`2236525`](https://github.com/kristof-mattei/autoheal-rs/commit/2236525cd347dfd60455884374e0781256aab014))
- More caches by [@kristof-mattei](https://github.com/kristof-mattei) ([`df84e19`](https://github.com/kristof-mattei/autoheal-rs/commit/df84e1901142303569f4360152934c207ea90bfc))
- Ensure the build fails when the detect changes task fails by [@kristof-mattei](https://github.com/kristof-mattei) ([`6a6d4c9`](https://github.com/kristof-mattei/autoheal-rs/commit/6a6d4c959d983c819c2d75037b57e70da3ac6c7f))
- Also depend on calculate-version by [@kristof-mattei](https://github.com/kristof-mattei) ([`1e633b6`](https://github.com/kristof-mattei/autoheal-rs/commit/1e633b6795d6f4f5ce99d6de5c146151ddaf9d63))
- Add `env_vars`, set name by [@kristof-mattei](https://github.com/kristof-mattei) ([`712dfd2`](https://github.com/kristof-mattei/autoheal-rs/commit/712dfd2bbc1b0d0c9ffd613a6a9174e05b590c46))
- Disable flags when not needed by [@kristof-mattei](https://github.com/kristof-mattei) ([`6d2faf0`](https://github.com/kristof-mattei/autoheal-rs/commit/6d2faf0a5ce7dddc927dcf37d26b1935e6e8e297))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`62ee9cf`](https://github.com/kristof-mattei/autoheal-rs/commit/62ee9cf47b5b4754efa0bf6b6fca92c0e6b5ca9f))
- Remove semgrep, it's useless by [@kristof-mattei](https://github.com/kristof-mattei) ([`ab83edc`](https://github.com/kristof-mattei/autoheal-rs/commit/ab83edcd3f5b0ed0aaa87a74c863ca4f45bfb2cc))
- Versions have v-prefix by [@kristof-mattei](https://github.com/kristof-mattei) ([`75f32fe`](https://github.com/kristof-mattei/autoheal-rs/commit/75f32fef1fc511587187c93475915de1266351a4))
- Prevent injection by [@kristof-mattei](https://github.com/kristof-mattei) ([`59671e9`](https://github.com/kristof-mattei/autoheal-rs/commit/59671e96be74e797298077dca7e0f43c170b4174))
- Exclude more, fix path, output by [@kristof-mattei](https://github.com/kristof-mattei) ([`960f0b9`](https://github.com/kristof-mattei/autoheal-rs/commit/960f0b9c1cdd36cba6821a5a23fe3f9b849866e9))
- Spacing by [@kristof-mattei](https://github.com/kristof-mattei) ([`b9e5c3c`](https://github.com/kristof-mattei/autoheal-rs/commit/b9e5c3cf40a3abca5eee0d36ee1c0eeb74041ab6))
- Added comments by [@kristof-mattei](https://github.com/kristof-mattei) ([`a967b76`](https://github.com/kristof-mattei/autoheal-rs/commit/a967b76d0648c9348938d08b5eb85f6eb78e8183))
- Temp file by [@kristof-mattei](https://github.com/kristof-mattei) ([`98d9e13`](https://github.com/kristof-mattei/autoheal-rs/commit/98d9e1349616df02e11404b6501bd5844253eec2))
- Delete test-release.yml workflow by [@kristof-mattei](https://github.com/kristof-mattei) ([`5473330`](https://github.com/kristof-mattei/autoheal-rs/commit/5473330ead5d036a76efea837e0e5b56ef40173f))
- Backticks by [@kristof-mattei](https://github.com/kristof-mattei) ([`fbe5412`](https://github.com/kristof-mattei/autoheal-rs/commit/fbe5412a6da2f25cb5d06320d8b16ffa65b7bbcd))
- Push with token that can trigger workflows, otherwise force pushes don't trigger the PR by [@kristof-mattei](https://github.com/kristof-mattei) ([`2384e4e`](https://github.com/kristof-mattei/autoheal-rs/commit/2384e4e356e2f516f343409b5a311fb9a4b7cd08))
- Remove `-` by [@kristof-mattei](https://github.com/kristof-mattei) ([`b8df04c`](https://github.com/kristof-mattei/autoheal-rs/commit/b8df04cf2d1f4f1abe9f8c670882cdbf4837f1d6))

### ⚙️ Miscellaneous Tasks

- *(ci)* Make coverage optional by [@kristof-mattei](https://github.com/kristof-mattei) ([`3400180`](https://github.com/kristof-mattei/autoheal-rs/commit/34001803940dedc56564b839d4abf55ab6d94299))
- *(release)* Release v1.9.0-rc.1 by [@github-actions[bot]](https://github.com/github-actions[bot]) ([`2d215d3`](https://github.com/kristof-mattei/autoheal-rs/commit/2d215d3132594cceb9e1a8d1b047458fb9b625a3))
- Use full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`2eb8f41`](https://github.com/kristof-mattei/autoheal-rs/commit/2eb8f41bae2b9c8738b851b6361957c63a8b8486))
- Use full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`8c4c444`](https://github.com/kristof-mattei/autoheal-rs/commit/8c4c444ffac801dd72db7804dd15b53ef09f49fd))
- Update from upstream by [@kristof-mattei](https://github.com/kristof-mattei) ([`7e9670c`](https://github.com/kristof-mattei/autoheal-rs/commit/7e9670c1a64557337d0560a427c1976320433d18))
- Reorder by [@kristof-mattei](https://github.com/kristof-mattei) ([`1894a52`](https://github.com/kristof-mattei/autoheal-rs/commit/1894a5229b3d1bb7b37109623eb48ba18d9731c0))
- Squash incoming commits, don't pollute history by [@kristof-mattei](https://github.com/kristof-mattei) ([`a884a35`](https://github.com/kristof-mattei/autoheal-rs/commit/a884a353507118f35d9184117ababfbf8937cd94))
- Back to the good old merge by [@kristof-mattei](https://github.com/kristof-mattei) ([`c0cbd14`](https://github.com/kristof-mattei/autoheal-rs/commit/c0cbd14804d02d9b17a22849a3e4398e1f88a294))
- Restore script by [@kristof-mattei](https://github.com/kristof-mattei) ([`62ca5d5`](https://github.com/kristof-mattei/autoheal-rs/commit/62ca5d5d386f9d0e20e0af879c0315d246b57d6f))
- Fix redundant_closure_for_method_calls by [@kristof-mattei](https://github.com/kristof-mattei) ([`0ab7e46`](https://github.com/kristof-mattei/autoheal-rs/commit/0ab7e46e411844975e90f42f9ce86930b89906a9))
- Long version by [@kristof-mattei](https://github.com/kristof-mattei) ([`ef85f83`](https://github.com/kristof-mattei/autoheal-rs/commit/ef85f8391dae73caa9deb5c8e101fe3308db7162))
- Fix title by [@kristof-mattei](https://github.com/kristof-mattei) ([`0b10ac2`](https://github.com/kristof-mattei/autoheal-rs/commit/0b10ac21b7cae91c02076e5ec460824000647429))
- Write in full semver by [@kristof-mattei](https://github.com/kristof-mattei) ([`56b70c0`](https://github.com/kristof-mattei/autoheal-rs/commit/56b70c089d3ed211e71a3fba8ad00bc4b12f30e5))
- Add exception for non-semver github actions, allowing full-semver all, and non-semver for packages like `mold` by [@kristof-mattei](https://github.com/kristof-mattei) ([`edb13d4`](https://github.com/kristof-mattei/autoheal-rs/commit/edb13d461d4eff96d4aa18a1d477a918b873dedd))
- Bump packages by [@kristof-mattei](https://github.com/kristof-mattei) ([`a497c97`](https://github.com/kristof-mattei/autoheal-rs/commit/a497c9717ad38bb2d8ea3e2863f40533d782a72e))
- Codecov/test-results-action is deprecated by [@kristof-mattei](https://github.com/kristof-mattei) ([`70c85b7`](https://github.com/kristof-mattei/autoheal-rs/commit/70c85b746c742287c76ef854155617e9f71dab39))
- Require `pre-build-cargo-edit` success or skip, but not failure by [@kristof-mattei](https://github.com/kristof-mattei) ([`bf0e7a3`](https://github.com/kristof-mattei/autoheal-rs/commit/bf0e7a3456b37d17418708a8d07a4a991a3156a0))
- Attestation needs `artifact-metadata: write` by [@kristof-mattei](https://github.com/kristof-mattei) ([`978f466`](https://github.com/kristof-mattei/autoheal-rs/commit/978f466c545b5bede1ebf9ded92dcd7921f1b7c1))
- Clean up dictionary by [@kristof-mattei](https://github.com/kristof-mattei) ([`b06dadd`](https://github.com/kristof-mattei/autoheal-rs/commit/b06dadd72115acaf06d7b5a076433ece3a5ea949))
- Remove debug line by [@kristof-mattei](https://github.com/kristof-mattei) ([`6a52ed9`](https://github.com/kristof-mattei/autoheal-rs/commit/6a52ed9a45931dd5e4f7b8222607cdcdd2a20596))
## [1.8.2](https://github.com/kristof-mattei/autoheal-rs/compare/v1.8.1..v1.8.2) - 2025-11-19

### 🐛 Bug Fixes

- Untagged containers is broken, they are actually multiplatform sources, but not detected as such by [@kristof-mattei](https://github.com/kristof-mattei) ([`8138918`](https://github.com/kristof-mattei/autoheal-rs/commit/8138918a0acae18f6c62ff126b5ae89c482d72cf))
- Re-enable attestation by [@kristof-mattei](https://github.com/kristof-mattei) ([`b0bd3e1`](https://github.com/kristof-mattei/autoheal-rs/commit/b0bd3e1d07a88c229cace8f93fb7cc0f6f0ba534))

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.8.2 ([`7b79a47`](https://github.com/kristof-mattei/autoheal-rs/commit/7b79a4723b7db095898ed531263a621d9678b770))
- Bump packages by [@kristof-mattei](https://github.com/kristof-mattei) ([`8faeb89`](https://github.com/kristof-mattei/autoheal-rs/commit/8faeb89e4a25e22f479a560e12ebadbb2b450061))
## [1.8.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.8.0..v1.8.1) - 2025-11-19

### 🐛 Bug Fixes

- *(deps)* Update rust crate clap to 4.5.52 by [@renovate[bot]](https://github.com/renovate[bot]) ([`9cc315c`](https://github.com/kristof-mattei/autoheal-rs/commit/9cc315c0de1994b47516080d4b8774679b901378))
- Push image with imagetools create by [@kristof-mattei](https://github.com/kristof-mattei) ([`9e58649`](https://github.com/kristof-mattei/autoheal-rs/commit/9e586493dfd7fac05e70787914d50cdc43fd95a2))

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.8.1 ([`d255358`](https://github.com/kristof-mattei/autoheal-rs/commit/d255358946eca55784156639845f09f254be4ac5))
## [1.8.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.7.0..v1.8.0) - 2025-11-14

### 🚀 Features

- *(ci)* Multi level by [@kristof-mattei](https://github.com/kristof-mattei) ([`eb36e7a`](https://github.com/kristof-mattei/autoheal-rs/commit/eb36e7acf6ab747376cddd1d67e917262d6627dd))
- Use target cpu for optimal builds by [@kristof-mattei](https://github.com/kristof-mattei) ([`0cf1732`](https://github.com/kristof-mattei/autoheal-rs/commit/0cf17329e505ee5fd266a1777147321d6618a3ec))
- Mimalloc by [@kristof-mattei](https://github.com/kristof-mattei) ([`20ef31e`](https://github.com/kristof-mattei/autoheal-rs/commit/20ef31e5fb4015f96f1df19c7107c10ed2afcf69))
- Buildscript to embed targetted platform by [@kristof-mattei](https://github.com/kristof-mattei) ([`40464d9`](https://github.com/kristof-mattei/autoheal-rs/commit/40464d910ef834a24b9e16f1007b742fbc496595))

### 🐛 Bug Fixes

- *(ci)* Only pre-build cargo-edit when we actually build a container by [@kristof-mattei](https://github.com/kristof-mattei) ([`9bfd2a5`](https://github.com/kristof-mattei/autoheal-rs/commit/9bfd2a517d7b9a9081211c94e124de3ddcf19fc3))
- *(ci)* Surpress "warning: be sure to add `/output/bin` to your PATH to be able to run the installed binaries" by [@kristof-mattei](https://github.com/kristof-mattei) ([`77f6810`](https://github.com/kristof-mattei/autoheal-rs/commit/77f681080fb7d2b399ba3944c63db8d1bbfaa169))
- *(ci)* Use --list-different to actually list the files different by [@kristof-mattei](https://github.com/kristof-mattei) ([`37e5d53`](https://github.com/kristof-mattei/autoheal-rs/commit/37e5d53fe82cf449cc43fed4a8ff33bd1e7635b4))
- *(deps)* Update rust crate libc to 0.2.177 by [@renovate[bot]](https://github.com/renovate[bot]) ([`93bfb62`](https://github.com/kristof-mattei/autoheal-rs/commit/93bfb6220d1ef06f10f7efe3410acedb45f2631d))
- *(deps)* Update rust crate tokio to 1.48.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`bc4588f`](https://github.com/kristof-mattei/autoheal-rs/commit/bc4588fdb5db139c8dd08a41809c06de17ab8f6b))
- *(deps)* Update rust crate rustls-native-certs to 0.8.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6d44469`](https://github.com/kristof-mattei/autoheal-rs/commit/6d444699b881b65d003ffcd7dde5985a7ef03816))
- *(deps)* Update rust crate clap to 4.5.51 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d832bed`](https://github.com/kristof-mattei/autoheal-rs/commit/d832beda8f8a99a06b50a82bc8aec6232bca6b77))
- *(deps)* Update rust crate console-subscriber to 0.5.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e521213`](https://github.com/kristof-mattei/autoheal-rs/commit/e521213be8c4b282b89e48ec5ad6125358727c00))
- *(deps)* Update rust crate rustls to 0.23.35 by [@renovate[bot]](https://github.com/renovate[bot]) ([`78232e7`](https://github.com/kristof-mattei/autoheal-rs/commit/78232e7cf700b55939234e34f004598c8edb2c15))
- *(deps)* Update rust crate hyper-util to 0.1.18 by [@renovate[bot]](https://github.com/renovate[bot]) ([`be5ee70`](https://github.com/kristof-mattei/autoheal-rs/commit/be5ee709a4464c6f8d0d9690c3070283b5d4a486))
- *(deps)* Update rust crate hyper to 1.8.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`46005aa`](https://github.com/kristof-mattei/autoheal-rs/commit/46005aaed8ed90c5f5c7a3253cc00fbabd364960))
- Breaking doesn't make us go from 0->1 by default by [@kristof-mattei](https://github.com/kristof-mattei) ([`b6c3182`](https://github.com/kristof-mattei/autoheal-rs/commit/b6c3182ca96730a0d6adb3cea886d4dabc39b8d4))
- Run test from local single-platform image (we don't have a multiplatform one yet here) by [@kristof-mattei](https://github.com/kristof-mattei) ([`396c3c5`](https://github.com/kristof-mattei/autoheal-rs/commit/396c3c55ff35128d6d8e69b39e3b97897b98ad5e))
- Use correct image for testing by [@kristof-mattei](https://github.com/kristof-mattei) ([`6fc9368`](https://github.com/kristof-mattei/autoheal-rs/commit/6fc936894ea65609d7d8b8b3be842b0c206f8e2c))
- Cleanup script by [@kristof-mattei](https://github.com/kristof-mattei) ([`9b6dc8b`](https://github.com/kristof-mattei/autoheal-rs/commit/9b6dc8bf460824cb8510719e191a5d5cb03b9f93))
- Removed erroneous space, added name by [@kristof-mattei](https://github.com/kristof-mattei) ([`fbed64f`](https://github.com/kristof-mattei/autoheal-rs/commit/fbed64f4e620ab45862d7ed38b1b8cc5477662eb))
- Clean up build-cache as well, and run every day by [@kristof-mattei](https://github.com/kristof-mattei) ([`1d6a0a4`](https://github.com/kristof-mattei/autoheal-rs/commit/1d6a0a4181be25b94c08af55da3d69f4c4846e69))
- Also delete from build cache if expired (right now 30 days) by [@kristof-mattei](https://github.com/kristof-mattei) ([`70b2624`](https://github.com/kristof-mattei/autoheal-rs/commit/70b2624ed0eae2e920c0a0733faa8b4531c85557))
- Launch main code also as a task, and moved code from Rc<str> to Box<str>, as we only clone the `id` once, so no point in having the `Arc<str>` overhead there by [@kristof-mattei](https://github.com/kristof-mattei) ([`e32a598`](https://github.com/kristof-mattei/autoheal-rs/commit/e32a5983a8ae46ccc06ea1d61bf4a2bb7c3ba589))
- Full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`95902e1`](https://github.com/kristof-mattei/autoheal-rs/commit/95902e1100aca1c5fc03d4c4b73e4fb6d12d7751))
- Full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`801bbfa`](https://github.com/kristof-mattei/autoheal-rs/commit/801bbfa4b2da8cd3e824be51ea31f989a00cab1b))
- String_to_string is deprecated and fails in 1.91.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`ebfba18`](https://github.com/kristof-mattei/autoheal-rs/commit/ebfba184701e819f7b0700d0e86b55c06f20c55c))
- Update build_env by [@kristof-mattei](https://github.com/kristof-mattei) ([`75149fe`](https://github.com/kristof-mattei/autoheal-rs/commit/75149fea98ba64e5679877bf4669c25ebdfac96a))
- Limit amount of concurrent builds to reduce pressure by [@kristof-mattei](https://github.com/kristof-mattei) ([`8f67f94`](https://github.com/kristof-mattei/autoheal-rs/commit/8f67f94909e191e8ba75a2d21f8d415d779ef5a5))
- Retry final docker push by [@kristof-mattei](https://github.com/kristof-mattei) ([`c49e870`](https://github.com/kristof-mattei/autoheal-rs/commit/c49e870692a536f7f261fb47ef14b1921aac16ab))
- More timeout defense by [@kristof-mattei](https://github.com/kristof-mattei) ([`06820af`](https://github.com/kristof-mattei/autoheal-rs/commit/06820afdccce9bdc8cac5aa8e1c7e6178f647f43))
- Disable collapsible-if lint, it decreases legibility by [@kristof-mattei](https://github.com/kristof-mattei) ([`2274bcd`](https://github.com/kristof-mattei/autoheal-rs/commit/2274bcd883877479f663fe526ed3732d91ad606e))
- Move the attestation to a separate job for easier retries by [@kristof-mattei](https://github.com/kristof-mattei) ([`11bb234`](https://github.com/kristof-mattei/autoheal-rs/commit/11bb2345f62fcfdac4946c53581ce05e423bfdb5))
- Separate push and inspection by [@kristof-mattei](https://github.com/kristof-mattei) ([`91903f9`](https://github.com/kristof-mattei/autoheal-rs/commit/91903f95b8d84a6726caffef9aadbd7ba2773697))
- No attestation for building initial images by [@kristof-mattei](https://github.com/kristof-mattei) ([`f174a10`](https://github.com/kristof-mattei/autoheal-rs/commit/f174a10ad48b6d4ea02b685854966ffc18d08f3f))

### ⚙️ Miscellaneous Tasks

- *(ci)* Add step that builds cargo-edit and cargo-get before the docker build. Docker build will then pick up cached version by [@kristof-mattei](https://github.com/kristof-mattei) ([`a0c20d6`](https://github.com/kristof-mattei/autoheal-rs/commit/a0c20d6543ef13d301721a66f75250cfeacc491a))
- *(ci)* Add comment ensuring the non-standard action gets updated by [@kristof-mattei](https://github.com/kristof-mattei) ([`af60f14`](https://github.com/kristof-mattei/autoheal-rs/commit/af60f143907c0fc3a3e1ddcc37ccc02784e0394e))
- *(ci)* Split builds again for speed by [@kristof-mattei](https://github.com/kristof-mattei) ([`a33e1ab`](https://github.com/kristof-mattei/autoheal-rs/commit/a33e1ab61073a555dd707ca7cfc0d1efa275ef6f))
- *(ci)* Reduce platforms for qemu by [@kristof-mattei](https://github.com/kristof-mattei) ([`e67fe57`](https://github.com/kristof-mattei/autoheal-rs/commit/e67fe57673bcdb67648141144b6fb077d3df3e0e))
- *(ci)* Pin action to full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`6fb96b1`](https://github.com/kristof-mattei/autoheal-rs/commit/6fb96b162a9098884338b7da32cba77c4addda33))
- *(ci)* Use github-scripts@v8 by [@kristof-mattei](https://github.com/kristof-mattei) ([`7495be9`](https://github.com/kristof-mattei/autoheal-rs/commit/7495be94fc8a75dfcfef52a5deb5dd76f096825a))
- *(ci)* Add machete by [@kristof-mattei](https://github.com/kristof-mattei) ([`08bd0ef`](https://github.com/kristof-mattei/autoheal-rs/commit/08bd0efd204b4b7ba7994ce4f9bfb985e0692cc0))
- *(ci)* Enforce machete by [@kristof-mattei](https://github.com/kristof-mattei) ([`38c2d3b`](https://github.com/kristof-mattei/autoheal-rs/commit/38c2d3b8949c898688c4edeca898da735d6f90cd))
- *(ci)* Rename format to machete by [@kristof-mattei](https://github.com/kristof-mattei) ([`d8e6d9f`](https://github.com/kristof-mattei/autoheal-rs/commit/d8e6d9f0ef133cee5f38e5c6641e79a26d99cfaf))
- *(ci)* Enforce nightly fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`3ec2ad1`](https://github.com/kristof-mattei/autoheal-rs/commit/3ec2ad14ee4b4dd97b161ce62d37871f42f78dd7))
- *(ci)* Kill enforcement of nightly fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`ba7dfda`](https://github.com/kristof-mattei/autoheal-rs/commit/ba7dfdaedabe7840657c288075ee4796fd31faf4))
- *(fmt)* Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`f7b0a11`](https://github.com/kristof-mattei/autoheal-rs/commit/f7b0a1189c2e2b834199e2fc876b95aa63c8a246))
- *(release)* Release 0.0.1 ([`a075d2d`](https://github.com/kristof-mattei/autoheal-rs/commit/a075d2d69cd0202de3d42c8a888ca14b49e0051f))
- *(release)* Release v1.8.0 ([`c7fa166`](https://github.com/kristof-mattei/autoheal-rs/commit/c7fa1666653505fe92c424232b2e9db1ef88e8a0))
- Move root store creation to separate fn by [@kristof-mattei](https://github.com/kristof-mattei) ([`34cfaf6`](https://github.com/kristof-mattei/autoheal-rs/commit/34cfaf66c1e2fd33b9b8f1d9b1c6c44d21258402))
- Bump... again by [@kristof-mattei](https://github.com/kristof-mattei) ([`d4aa088`](https://github.com/kristof-mattei/autoheal-rs/commit/d4aa088566c4a806669f49f40830260a3ea95870))
- Use slim-trixie instead of trixie by [@kristof-mattei](https://github.com/kristof-mattei) ([`ecd2b0e`](https://github.com/kristof-mattei/autoheal-rs/commit/ecd2b0e978c392b1ed12726877de4e78f19980e7))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`fb181c3`](https://github.com/kristof-mattei/autoheal-rs/commit/fb181c3e899c70d21a4d0081ebf2ddf44192012e))
- Don't show progress by [@kristof-mattei](https://github.com/kristof-mattei) ([`e08f1a3`](https://github.com/kristof-mattei/autoheal-rs/commit/e08f1a3f42403d0c564f2672cffea0c22709897a))
- Prebuild -> pre-build by [@kristof-mattei](https://github.com/kristof-mattei) ([`9cd7f51`](https://github.com/kristof-mattei/autoheal-rs/commit/9cd7f51513126a751427be4f4e5cfa2fe9a2a875))
- Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`54866c7`](https://github.com/kristof-mattei/autoheal-rs/commit/54866c7e83be6676c455beba5ff83f92985c25ac))
- Test script for local coverage display by [@kristof-mattei](https://github.com/kristof-mattei) ([`37ee7be`](https://github.com/kristof-mattei/autoheal-rs/commit/37ee7bed77437a8f2ce2d3a7f1d72b9cbbd77bbf))
- Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`dc64562`](https://github.com/kristof-mattei/autoheal-rs/commit/dc6456255c2f810edf5d774400065f5e9edcd1a5))
- Full (semantic) version by [@kristof-mattei](https://github.com/kristof-mattei) ([`57c3c2b`](https://github.com/kristof-mattei/autoheal-rs/commit/57c3c2b3941327f56dd19f67d44bfdb784084103))
- Update lints, keep the ones we allow to allow for tracking new ones by [@kristof-mattei](https://github.com/kristof-mattei) ([`357ce57`](https://github.com/kristof-mattei/autoheal-rs/commit/357ce572bc03bd7e2c9810b8216e8d36006b47c9))
- Update lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`df8ba12`](https://github.com/kristof-mattei/autoheal-rs/commit/df8ba122d9772796479bfaf7b1ef1440219f20e8))
- Ensure both jobs have a setup docker and setup buildx by [@kristof-mattei](https://github.com/kristof-mattei) ([`b780ecc`](https://github.com/kristof-mattei/autoheal-rs/commit/b780eccdf762e5996ab2ee2a96a733345b4b9f2a))
- Allow parallel by [@kristof-mattei](https://github.com/kristof-mattei) ([`ebf9517`](https://github.com/kristof-mattei/autoheal-rs/commit/ebf95176c678741fa9387c70a82d2b7441f6054d))
## [1.7.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.6.1..v1.7.0) - 2025-09-21

### 🚀 Features

- Restrict ALL by [@kristof-mattei](https://github.com/kristof-mattei) ([`4dbfae3`](https://github.com/kristof-mattei/autoheal-rs/commit/4dbfae3b6f9aeda309bd3c788d1f2c18317d1da8))
- Restrict ALL by [@kristof-mattei](https://github.com/kristof-mattei) ([`77113a9`](https://github.com/kristof-mattei/autoheal-rs/commit/77113a9eb79c398ab5b71fea9ad97ff6b913d955))
- Restrict ALL by [@kristof-mattei](https://github.com/kristof-mattei) ([`28544d3`](https://github.com/kristof-mattei/autoheal-rs/commit/28544d39a362f8b2e9066f1a4ba89e8f9a6e3941))
- Implemented tls connection by [@kristof-mattei](https://github.com/kristof-mattei) ([`a3ec7d9`](https://github.com/kristof-mattei/autoheal-rs/commit/a3ec7d982371ba9f6941fb821673e2b2557fdc33))
- I rule(set) by [@kristof-mattei](https://github.com/kristof-mattei) ([`37253e2`](https://github.com/kristof-mattei/autoheal-rs/commit/37253e288eefd5b1e8f7c40184fcf7b9acd90ec4))
- Use anchors to dedup build by [@kristof-mattei](https://github.com/kristof-mattei) ([`f6cafa4`](https://github.com/kristof-mattei/autoheal-rs/commit/f6cafa45b7e0bfd4c29dc2f7f0e57ba258a96922))
- Pin to trixie, use gcc-14 from trixie by [@kristof-mattei](https://github.com/kristof-mattei) ([`e0ea2a2`](https://github.com/kristof-mattei/autoheal-rs/commit/e0ea2a212118b3a24d52bb86ae8bb5ed8e6582bd))
- Separate cache based on target to allow for more efficient caching by [@kristof-mattei](https://github.com/kristof-mattei) ([`89bcb9b`](https://github.com/kristof-mattei/autoheal-rs/commit/89bcb9be9680fa5d0df610d93fb9b978a3362202))
- Write output to per-target folder, otherwise caches overwrite each other causing recompilation in the install step by [@kristof-mattei](https://github.com/kristof-mattei) ([`aabbed1`](https://github.com/kristof-mattei/autoheal-rs/commit/aabbed1fd86be610b46a911d2d1cf7231b0ec5ac))
- Use git-cliff for better changelogs by [@kristof-mattei](https://github.com/kristof-mattei) ([`cb13014`](https://github.com/kristof-mattei/autoheal-rs/commit/cb130144b31681f7bc6d70dd9843d4d2110463bd))
- Add the self-referencing commit too by [@kristof-mattei](https://github.com/kristof-mattei) ([`3b18cba`](https://github.com/kristof-mattei/autoheal-rs/commit/3b18cba9bfb761c67d844e26b0c189f654c68d9b))
- Diff is generated after the tag, so we don't need to add the as-if message by [@kristof-mattei](https://github.com/kristof-mattei) ([`7b9b76e`](https://github.com/kristof-mattei/autoheal-rs/commit/7b9b76e224972b4063357ec818ca403457ea90e9))

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.47.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`22ec986`](https://github.com/kristof-mattei/autoheal-rs/commit/22ec986b338b66f750cfb8b61346f2384fdd2649))
- *(deps)* Update rust crate hashbrown to 0.15.5 by [@renovate[bot]](https://github.com/renovate[bot]) ([`60be830`](https://github.com/kristof-mattei/autoheal-rs/commit/60be8308bb195832b1fdbf07f7371c43ed356c2d))
- *(deps)* Update rust crate libc to 0.2.175 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d088844`](https://github.com/kristof-mattei/autoheal-rs/commit/d0888440b0d75461d361da82917a7d0b21542c0a))
- *(deps)* Update rust crate hyper to 1.7.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`5f44d5e`](https://github.com/kristof-mattei/autoheal-rs/commit/5f44d5ec2a4c0cbae5c714cc8b1fa91e4553e21d))
- *(deps)* Update rust crate percent-encoding to 2.3.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`fb3deb4`](https://github.com/kristof-mattei/autoheal-rs/commit/fb3deb4be445eb379007c48d822691f790330dc4))
- *(deps)* Update rust crate url to 2.5.7 by [@renovate[bot]](https://github.com/renovate[bot]) ([`64a502b`](https://github.com/kristof-mattei/autoheal-rs/commit/64a502bd8107364bce41e2d4f41b1f0f3fcc7e9c))
- *(deps)* Update rust crate tracing-subscriber to v0.3.20 [security] by [@renovate[bot]](https://github.com/renovate[bot]) ([`efa7900`](https://github.com/kristof-mattei/autoheal-rs/commit/efa79003ef5ae2ac6c4794ab211bbb830c37944f))
- *(deps)* Update rust crate tracing-subscriber to 0.3.20 by [@renovate[bot]](https://github.com/renovate[bot]) ([`bed882a`](https://github.com/kristof-mattei/autoheal-rs/commit/bed882a7336d06cdaef943120151042785f3f97a))
- *(deps)* Update rust crate serde_json to 1.0.145 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d3e00cc`](https://github.com/kristof-mattei/autoheal-rs/commit/d3e00ccac77f653e42735c2b7ff1d4ffbc3f6068))
- *(deps)* Update rust crate hyper-util to 0.1.17 by [@renovate[bot]](https://github.com/renovate[bot]) ([`c034ddb`](https://github.com/kristof-mattei/autoheal-rs/commit/c034ddb720b8ac7c27ca967af88ad6f14813db9c))
- *(deps)* Update rust crate rustls to 0.23.32 by [@renovate[bot]](https://github.com/renovate[bot]) ([`7cb43a7`](https://github.com/kristof-mattei/autoheal-rs/commit/7cb43a7c4b1369896437107964ed35034fbe538c))
- *(deps)* Update rust crate clap to 4.5.48 by [@renovate[bot]](https://github.com/renovate[bot]) ([`f83042f`](https://github.com/kristof-mattei/autoheal-rs/commit/f83042f8738ff0b779d2dcd4d7dc84959e0794ee))
- *(deps)* Update rust crate serde to 1.0.226 by [@renovate[bot]](https://github.com/renovate[bot]) ([`36c3152`](https://github.com/kristof-mattei/autoheal-rs/commit/36c31522a48956c5884dadb014abffdeda8e6e54))
- Cleanup unused lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`5a38041`](https://github.com/kristof-mattei/autoheal-rs/commit/5a38041fe40daa98087753ea9c3a2a4cab2a842e))
- We know that sort order when iterating over hash-type isn't guaranteed by [@kristof-mattei](https://github.com/kristof-mattei) ([`82a22ac`](https://github.com/kristof-mattei/autoheal-rs/commit/82a22acb0bf4caf05e96785c69f69db94a7f07fa))
- Default is 30 seconds, not 30 milliseconds by [@kristof-mattei](https://github.com/kristof-mattei) ([`f233bb3`](https://github.com/kristof-mattei/autoheal-rs/commit/f233bb33c41c2d2b1636c5ec02633352daf62695))
- Set defaults by [@kristof-mattei](https://github.com/kristof-mattei) ([`b747dc6`](https://github.com/kristof-mattei/autoheal-rs/commit/b747dc619a18351fb0edba5b37c2489c276dbf48))
- Allow cargo features selection by [@kristof-mattei](https://github.com/kristof-mattei) ([`bb52f2f`](https://github.com/kristof-mattei/autoheal-rs/commit/bb52f2fe772f0cbed2ae63ebcd109f7b10d08597))
- FeaTures by [@kristof-mattei](https://github.com/kristof-mattei) ([`6372af0`](https://github.com/kristof-mattei/autoheal-rs/commit/6372af04da9b9ce338e2ec2d12e00f3fedbb2ec5))
- Missing read permissions in test and report by [@kristof-mattei](https://github.com/kristof-mattei) ([`3f8f6a5`](https://github.com/kristof-mattei/autoheal-rs/commit/3f8f6a51b3ce1be82b2c165f75353c4e9b222ae4))
- Use the github token to ensure we can download by [@kristof-mattei](https://github.com/kristof-mattei) ([`f6ecce7`](https://github.com/kristof-mattei/autoheal-rs/commit/f6ecce72871eba2bd3d8386da8afc525f803942d))
- Set full version by [@kristof-mattei](https://github.com/kristof-mattei) ([`5535b81`](https://github.com/kristof-mattei/autoheal-rs/commit/5535b81f32546a474b3efcc5a96158bf47311133))
- Build multi-platform docker images by [@kristof-mattei](https://github.com/kristof-mattei) ([`5342da1`](https://github.com/kristof-mattei/autoheal-rs/commit/5342da175081a5dd495b3039e06cea73011ee475))
- Pre-cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`e35eb20`](https://github.com/kristof-mattei/autoheal-rs/commit/e35eb2015728b1f7e1c210e89bacbbc18ff96c2e))
- Copilot instructions by [@kristof-mattei](https://github.com/kristof-mattei) ([`a494929`](https://github.com/kristof-mattei/autoheal-rs/commit/a4949293d72a56254fd14699807855ee539a787e))
- Fetch per arch, locked, and explicit import by [@kristof-mattei](https://github.com/kristof-mattei) ([`399084a`](https://github.com/kristof-mattei/autoheal-rs/commit/399084afecc27cbc36fe204c6804d036d4a98834))
- Src in registry should not be cached by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c4d96c`](https://github.com/kristof-mattei/autoheal-rs/commit/5c4d96cd47b47d95fce5e7bdf614c73ccb193f05))
- Lock fetch by [@kristof-mattei](https://github.com/kristof-mattei) ([`d7ac9da`](https://github.com/kristof-mattei/autoheal-rs/commit/d7ac9da6eba1e8c34be075a95a107496f24f7181))
- Bring ARG together by [@kristof-mattei](https://github.com/kristof-mattei) ([`cc62162`](https://github.com/kristof-mattei/autoheal-rs/commit/cc621629fd8c447546f8bb2391bda7cf1ff7e858))
- Disable cache dependencies for docker build, the downloading of ./target takes up too much space, and we're not building anyway by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6b9b6f`](https://github.com/kristof-mattei/autoheal-rs/commit/a6b9b6fd4afc687635b0c2dd4f2be81c19476344))
- Hashbrown 0.16.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`f5a0d9b`](https://github.com/kristof-mattei/autoheal-rs/commit/f5a0d9b07a42224ba045d72d73852d9ddbd59577))
- Shrink what we cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`a71972a`](https://github.com/kristof-mattei/autoheal-rs/commit/a71972a001a23b800efb383affc9bc0c5356da7c))
- Fmt doesn't need target by [@kristof-mattei](https://github.com/kristof-mattei) ([`0d9005c`](https://github.com/kristof-mattei/autoheal-rs/commit/0d9005c5f324e604141694007df0b24bfc2cff61))
- Download binstall based on runner arch by [@kristof-mattei](https://github.com/kristof-mattei) ([`1307db8`](https://github.com/kristof-mattei/autoheal-rs/commit/1307db8f053679f68b0ace029d056730980eedda))
- We lost the pr-<number>-latest tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`78eb251`](https://github.com/kristof-mattei/autoheal-rs/commit/78eb2517abc67f51409de808583dcacf75544dd8))
- Restore missed raw tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`cc2ea20`](https://github.com/kristof-mattei/autoheal-rs/commit/cc2ea20087e8ec53794c650a0c330b23f69fb589))
- Separate cache package by [@kristof-mattei](https://github.com/kristof-mattei) ([`b14d54a`](https://github.com/kristof-mattei/autoheal-rs/commit/b14d54ac88e7ff177857235b4d0067b86aa7b202))
- Use semver version. by [@kristof-mattei](https://github.com/kristof-mattei) ([`f7fa495`](https://github.com/kristof-mattei/autoheal-rs/commit/f7fa495682e4910b99bf39850988cecd6eb6e7c2))
- Set sha for pnpm by [@kristof-mattei](https://github.com/kristof-mattei) ([`eb3e312`](https://github.com/kristof-mattei/autoheal-rs/commit/eb3e3126b017db2a4482f00cb5097f53cd852a1e))
- Admin can bypass by [@kristof-mattei](https://github.com/kristof-mattei) ([`872a8bf`](https://github.com/kristof-mattei/autoheal-rs/commit/872a8bf3a220c13f61cfaed5fc72862363e033ce))
- Image as well because renovate is slow by [@kristof-mattei](https://github.com/kristof-mattei) ([`b2a2c2d`](https://github.com/kristof-mattei/autoheal-rs/commit/b2a2c2d675d902775baa61a96ee7c47e9ab0106c))
- Use git-cliff to get next version by [@kristof-mattei](https://github.com/kristof-mattei) ([`0be0460`](https://github.com/kristof-mattei/autoheal-rs/commit/0be04606cccbb69b08ba47233de310b0fb290469))
- Ignore .git by [@kristof-mattei](https://github.com/kristof-mattei) ([`8293ae3`](https://github.com/kristof-mattei/autoheal-rs/commit/8293ae364ac6a820b9745564ba83b206b2f969a5))

### ⚙️ Miscellaneous Tasks

- *(release)* Release v1.5.0 ([`5504d10`](https://github.com/kristof-mattei/autoheal-rs/commit/5504d10df91b765fac320a1ae903bc761017728c))
- *(release)* Release v1.7.0 ([`433cb4d`](https://github.com/kristof-mattei/autoheal-rs/commit/433cb4dca8a1077a53800866530def13f5a8dec4))
- *(version)* V1.4.0 by [@invalid-email-address](https://github.com/invalid-email-address) ([`d2b999b`](https://github.com/kristof-mattei/autoheal-rs/commit/d2b999b17175f8686884477736fc1412eb32928f))
- Enable as_conversions lint by [@kristof-mattei](https://github.com/kristof-mattei) ([`b60eb10`](https://github.com/kristof-mattei/autoheal-rs/commit/b60eb1002c74159fb6a80cdf8b3d39a7f2dbfde8))
- Move deps, disable as_conversions, too broad by [@kristof-mattei](https://github.com/kristof-mattei) ([`4a48efe`](https://github.com/kristof-mattei/autoheal-rs/commit/4a48efeafbb4224b6ef154e50a203e5696263c6d))
- Remove redundant excepts by [@kristof-mattei](https://github.com/kristof-mattei) ([`90a50f2`](https://github.com/kristof-mattei/autoheal-rs/commit/90a50f29f6ca210dbe480b063da16d2af0a68266))
- Restructure by [@kristof-mattei](https://github.com/kristof-mattei) ([`1c21c8f`](https://github.com/kristof-mattei/autoheal-rs/commit/1c21c8f6c67aae11af31df8924b83502adc7b959))
- Simplify by [@kristof-mattei](https://github.com/kristof-mattei) ([`2853307`](https://github.com/kristof-mattei/autoheal-rs/commit/2853307e95544be0514a150be33f8b5299ccb043))
- Rename components by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e83515`](https://github.com/kristof-mattei/autoheal-rs/commit/4e83515ed40b55ec552f38c2a12a99d7987adb0c))
- Rename components by [@kristof-mattei](https://github.com/kristof-mattei) ([`50a4456`](https://github.com/kristof-mattei/autoheal-rs/commit/50a4456ac60460ed472e773351b92ec7680dbd5a))
- Don't cache in lint-configs by [@kristof-mattei](https://github.com/kristof-mattei) ([`ab37d2f`](https://github.com/kristof-mattei/autoheal-rs/commit/ab37d2fb61fef6b4a462c1686faa77fbdcccb6f9))
- Disable codecov, fails too often by [@kristof-mattei](https://github.com/kristof-mattei) ([`252a089`](https://github.com/kristof-mattei/autoheal-rs/commit/252a089765ee5872923b20f2d3b26f8e6b5dc41c))
- Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`e9239d2`](https://github.com/kristof-mattei/autoheal-rs/commit/e9239d2809368cb2f242e5b7573b8589b15b7362))
- Remove unneeded newline by [@kristof-mattei](https://github.com/kristof-mattei) ([`66e19b3`](https://github.com/kristof-mattei/autoheal-rs/commit/66e19b3bfb7a4371b072ac7c8dc51db797bfa730))
- More robust downloading of crane by [@kristof-mattei](https://github.com/kristof-mattei) ([`c306637`](https://github.com/kristof-mattei/autoheal-rs/commit/c3066375bd8abf949146bc69e873acd4c4ca99a5))
- Pass download format to binstall for cocogitto by [@kristof-mattei](https://github.com/kristof-mattei) ([`2e173f6`](https://github.com/kristof-mattei/autoheal-rs/commit/2e173f61f09b1b338bff01edee10e1fed0f63540))
- Disable oldstyle branch protection by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6a851a`](https://github.com/kristof-mattei/autoheal-rs/commit/a6a851a1f84c80ee58e3294c132601bb4f330892))
- Delete old-style protection by [@kristof-mattei](https://github.com/kristof-mattei) ([`a754f7d`](https://github.com/kristof-mattei/autoheal-rs/commit/a754f7dce43b3d534a364df25fcec53df3196f96))
- Dedupe checkout in the integration test by [@kristof-mattei](https://github.com/kristof-mattei) ([`e0a6f69`](https://github.com/kristof-mattei/autoheal-rs/commit/e0a6f69ff116542d3fb5c3f54ad873dcc467b3c1))
- Use musl all the way by [@kristof-mattei](https://github.com/kristof-mattei) ([`828d679`](https://github.com/kristof-mattei/autoheal-rs/commit/828d67953e2be358e3565197feb91ec127cb2da2))
- Remove lldb-prettifier built as part of repo, use shared config by [@kristof-mattei](https://github.com/kristof-mattei) ([`8a7e110`](https://github.com/kristof-mattei/autoheal-rs/commit/8a7e110376c9a57b26a0a3ab9540b466394a8290))
- Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`3d4e946`](https://github.com/kristof-mattei/autoheal-rs/commit/3d4e9462ada1800d8c22b210ba56ce28758770ec))
- Remove arch from image as we now always deal with a multi-platform image by [@kristof-mattei](https://github.com/kristof-mattei) ([`1958005`](https://github.com/kristof-mattei/autoheal-rs/commit/19580050ce6cef2f354c3e3ce4732d18cc167245))
- Typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`cded012`](https://github.com/kristof-mattei/autoheal-rs/commit/cded012f14f816d0e1878e4abf0225bbc22b5925))
## [1.6.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.6.0..v1.6.1) - 2025-07-14

### 🐛 Bug Fixes

- Remove unused env that comes in via variables by [@kristof-mattei](https://github.com/kristof-mattei) ([`adf0cd7`](https://github.com/kristof-mattei/autoheal-rs/commit/adf0cd7117034c106eac21c2384bfa282d4271b8))
- Set correct guard name by [@kristof-mattei](https://github.com/kristof-mattei) ([`3131d66`](https://github.com/kristof-mattei/autoheal-rs/commit/3131d663eac696ac0d1ed2aa522051f1218e8067))

### ⚙️ Miscellaneous Tasks

- *(version)* V1.6.1 by [@invalid-email-address](https://github.com/invalid-email-address) ([`a4a70dd`](https://github.com/kristof-mattei/autoheal-rs/commit/a4a70dd2f2ab1818636d70c99a6bf7a3fce8cbb8))
## [1.6.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.5.0..v1.6.0) - 2025-07-13

### 🚀 Features

- Docker multiplatform by [@kristof-mattei](https://github.com/kristof-mattei) ([`c1f9221`](https://github.com/kristof-mattei/autoheal-rs/commit/c1f922116dd2e78d149247667a514f3bb4724aa1))
- Support for releasing crates by [@kristof-mattei](https://github.com/kristof-mattei) ([`6da4854`](https://github.com/kristof-mattei/autoheal-rs/commit/6da485494a7d16767c01b0255eae49fff6cb6dcf))
- Get rid of semantic release, use cocogitto by [@kristof-mattei](https://github.com/kristof-mattei) ([`a4b5e8c`](https://github.com/kristof-mattei/autoheal-rs/commit/a4b5e8cf58786d438cccee5678dcad0bc1fdcdb2))
- Add cross building by [@kristof-mattei](https://github.com/kristof-mattei) ([`46dedc2`](https://github.com/kristof-mattei/autoheal-rs/commit/46dedc27e9c8fc8af3110a95ac803c08f6a82aa8))
- Multiplatform with caching by [@kristof-mattei](https://github.com/kristof-mattei) ([`6c5188b`](https://github.com/kristof-mattei/autoheal-rs/commit/6c5188b32d43e0f8ae0bd1d9082871b23e244116))
- Enable codeql by [@kristof-mattei](https://github.com/kristof-mattei) ([`9d22fa5`](https://github.com/kristof-mattei/autoheal-rs/commit/9d22fa5261061d03a3c63a9fa5f5599e374b36ed))
- Update publish defaults by [@kristof-mattei](https://github.com/kristof-mattei) ([`406e06c`](https://github.com/kristof-mattei/autoheal-rs/commit/406e06cae013094a6e2995e2c49158b1677b814e))
- Ensure formatting works by [@kristof-mattei](https://github.com/kristof-mattei) ([`a7ae4c6`](https://github.com/kristof-mattei/autoheal-rs/commit/a7ae4c6870fcd124a76741003d4dce3773f7a056))
- Attest individual and multiplatform images by [@kristof-mattei](https://github.com/kristof-mattei) ([`b2393c1`](https://github.com/kristof-mattei/autoheal-rs/commit/b2393c1478174e162e13e33ac06c4d3ccf028e84))

### 🐛 Bug Fixes

- *(deps)* Update rust crate backtrace to 0.3.74 by [@renovate[bot]](https://github.com/renovate[bot]) ([`88601cf`](https://github.com/kristof-mattei/autoheal-rs/commit/88601cf14dbea3feeda3b81e7e4d6c8eb754dfca))
- *(deps)* Update rust crate tracing to 0.1.41 by [@renovate[bot]](https://github.com/renovate[bot]) ([`8b8de58`](https://github.com/kristof-mattei/autoheal-rs/commit/8b8de58e6eef5736f66af1f90de6b0a0be745327))
- *(deps)* Update rust crate tracing-subscriber to 0.3.19 by [@renovate[bot]](https://github.com/renovate[bot]) ([`2101c76`](https://github.com/kristof-mattei/autoheal-rs/commit/2101c766173596988113f9ceb9be48f1c1553484))
- *(deps)* Update rust crate hyper-unix-socket to 0.3.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d3ac57a`](https://github.com/kristof-mattei/autoheal-rs/commit/d3ac57a85dda87e169ca85a0a52ba7ebd40f164c))
- *(deps)* Update rust crate hyper to 1.6.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`be42873`](https://github.com/kristof-mattei/autoheal-rs/commit/be42873859d153c97833e9c9f23eb7d35504921f))
- *(deps)* Update rust crate serde_json to 1.0.140 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b409630`](https://github.com/kristof-mattei/autoheal-rs/commit/b4096309843eaabd874436b3fd2a74a508b0a033))
- *(deps)* Update rust crate serde to 1.0.219 by [@renovate[bot]](https://github.com/renovate[bot]) ([`fa4afa3`](https://github.com/kristof-mattei/autoheal-rs/commit/fa4afa34da6ab966c03e7c8e636a2ba0eb18e2c5))
- *(deps)* Update rust crate http-body-util to 0.1.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`3f23c15`](https://github.com/kristof-mattei/autoheal-rs/commit/3f23c15a6a5226c7839c5b517294f7b3ee94f375))
- *(deps)* Update rust crate http to 1.3.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`fbfb24e`](https://github.com/kristof-mattei/autoheal-rs/commit/fbfb24e3d0df0ecb0f1dd43e409b51b50896d14b))
- *(deps)* Update rust crate url to 2.5.4 by [@renovate[bot]](https://github.com/renovate[bot]) ([`867a4a4`](https://github.com/kristof-mattei/autoheal-rs/commit/867a4a4b804372df3c5145b9755bb5271ec1ec7a))
- *(deps)* Update rust crate hashbrown to 0.15.4 by [@renovate[bot]](https://github.com/renovate[bot]) ([`5d3c383`](https://github.com/kristof-mattei/autoheal-rs/commit/5d3c3836aec7a7bb70f41750f8158d1181f93393))
- *(deps)* Update rust crate openssl to 0.10.73 by [@renovate[bot]](https://github.com/renovate[bot]) ([`818d5d7`](https://github.com/kristof-mattei/autoheal-rs/commit/818d5d763e04fc8e3f0638a9264e9c8f1aecc4cb))
- *(deps)* Update rust crate color-eyre to 0.6.5 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6cbb98d`](https://github.com/kristof-mattei/autoheal-rs/commit/6cbb98dc877f5d2785ceb938f3fca30172c21bbe))
- *(deps)* Update rust crate libc to 0.2.174 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b94400c`](https://github.com/kristof-mattei/autoheal-rs/commit/b94400c6e1061fbefce7ab2df7bfbdc2f412b3f7))
- *(deps)* Update rust crate tokio to 1.46.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`da95a8f`](https://github.com/kristof-mattei/autoheal-rs/commit/da95a8f1b874a5a8e55e6b596d9d984a18510794))
- *(deps)* Update rust crate hyper-util to 0.1.15 by [@renovate[bot]](https://github.com/renovate[bot]) ([`0986d07`](https://github.com/kristof-mattei/autoheal-rs/commit/0986d076cfb47b7cda8728e288f7ea7086293fd2))
- Try something else by [@kristof-mattei](https://github.com/kristof-mattei) ([`0efe974`](https://github.com/kristof-mattei/autoheal-rs/commit/0efe9741d77cf9f7ca93f38255aa91e7dcf670af))
- Temp use custom version of eyre that depends on an updated of backtrace so we can update backtrace itself to its latest version by [@kristof-mattei](https://github.com/kristof-mattei) ([`899f442`](https://github.com/kristof-mattei/autoheal-rs/commit/899f44227e870e8706e94ab813d3e839b8813428))
- Report tests to codecov for tracking by [@kristof-mattei](https://github.com/kristof-mattei) ([`cc1ccdd`](https://github.com/kristof-mattei/autoheal-rs/commit/cc1ccdd729170ab4f8fddc1660911caf587fc3ad))
- Docker-compose has been deprecated for docker compose by [@kristof-mattei](https://github.com/kristof-mattei) ([`164dd98`](https://github.com/kristof-mattei/autoheal-rs/commit/164dd986987a326254cbbd0170903da31cb689fb))
- Always run integration tests by [@kristof-mattei](https://github.com/kristof-mattei) ([`db8728b`](https://github.com/kristof-mattei/autoheal-rs/commit/db8728b9c3c02cf96b8ce6235546a07d6df70726))
- Ensure docker_integration_test needs to pass by [@kristof-mattei](https://github.com/kristof-mattei) ([`ab3b2d7`](https://github.com/kristof-mattei/autoheal-rs/commit/ab3b2d7bdcdbd8964142bb37346aeb88e0863044))
- Remove obsolete props by [@kristof-mattei](https://github.com/kristof-mattei) ([`9e14174`](https://github.com/kristof-mattei/autoheal-rs/commit/9e14174ff0c98f57d27fe539a887225b9203eda9))
- Docker compose uses '-' instead of '_' by [@kristof-mattei](https://github.com/kristof-mattei) ([`f463dba`](https://github.com/kristof-mattei/autoheal-rs/commit/f463dba3a3e495881d2a1654f136871784a85c63))
- Prettier 3.41.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`64aa76c`](https://github.com/kristof-mattei/autoheal-rs/commit/64aa76cc522d7ee28a38bfffee53dc789abe4c12))
- Restore needed workflows by [@kristof-mattei](https://github.com/kristof-mattei) ([`72180e8`](https://github.com/kristof-mattei/autoheal-rs/commit/72180e8239e1a85a892f7713d8986bb9ad34f55c))
- Build container from scratch by [@kristof-mattei](https://github.com/kristof-mattei) ([`250217a`](https://github.com/kristof-mattei/autoheal-rs/commit/250217a80fa10d152f21f31c6031ce45039b32ce))
- Remove @actions/tool-cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`3889143`](https://github.com/kristof-mattei/autoheal-rs/commit/3889143b5e053e5d63835407138decb395314a13))
- Start tracking lldb debug helper by [@kristof-mattei](https://github.com/kristof-mattei) ([`b662ffa`](https://github.com/kristof-mattei/autoheal-rs/commit/b662ffa8c38b08c5f3d276e6e58646013a051efd))
- Rust 1.87.0, use .display() by [@kristof-mattei](https://github.com/kristof-mattei) ([`041042b`](https://github.com/kristof-mattei/autoheal-rs/commit/041042b6ea3f7b110ec39f35584b9c2ff657f0c7))
- Disable clippy 1.87.0 let_and_return by [@kristof-mattei](https://github.com/kristof-mattei) ([`9246d86`](https://github.com/kristof-mattei/autoheal-rs/commit/9246d861995c87085c04618a8619320afdbd771d))
- Add runner.arch to the cache keys by [@kristof-mattei](https://github.com/kristof-mattei) ([`64b63e2`](https://github.com/kristof-mattei/autoheal-rs/commit/64b63e2f99501f0208e54f3d1d35e19990751cec))
- Set correct cache key for the docker step by [@kristof-mattei](https://github.com/kristof-mattei) ([`bb875b8`](https://github.com/kristof-mattei/autoheal-rs/commit/bb875b8d038162d657082014a7070edb573cae92))
- Don't install binstall, cargo-edit doesn't have a package anyway by [@kristof-mattei](https://github.com/kristof-mattei) ([`377a21e`](https://github.com/kristof-mattei/autoheal-rs/commit/377a21ec73fd822d22099945fb196dcb89b75cc4))
- Ensure we enable tokio_unstable in the container as well by [@kristof-mattei](https://github.com/kristof-mattei) ([`d87f4d5`](https://github.com/kristof-mattei/autoheal-rs/commit/d87f4d5ae3279f35abaaaf257c53741d751564c5))
- Remove incorrec exit by [@kristof-mattei](https://github.com/kristof-mattei) ([`3791382`](https://github.com/kristof-mattei/autoheal-rs/commit/379138240224b860cf69d1249f93cacc8c847b6e))
- Switch to prettier's mjs setup, widen limit for non-json files by [@kristof-mattei](https://github.com/kristof-mattei) ([`15954b2`](https://github.com/kristof-mattei/autoheal-rs/commit/15954b23a004049eb88cbce34cc69bd231dce81e))
- Correct dpkg-architecture architecture check by [@kristof-mattei](https://github.com/kristof-mattei) ([`83055c9`](https://github.com/kristof-mattei/autoheal-rs/commit/83055c9bba144255202d4d12658323903be69d53))
- Install with locked to prevent cargo from updating deps during cargo install by [@kristof-mattei](https://github.com/kristof-mattei) ([`bdb7e91`](https://github.com/kristof-mattei/autoheal-rs/commit/bdb7e91b3054f6ef902f1c87870118ff035638e3))
- Cache per arch, as these overwrite each other by [@kristof-mattei](https://github.com/kristof-mattei) ([`90bb69a`](https://github.com/kristof-mattei/autoheal-rs/commit/90bb69af3c0d082b4cea18a161b99e34d61cd5fc))
- Dir, path, I don't know anymore by [@kristof-mattei](https://github.com/kristof-mattei) ([`4bfa22d`](https://github.com/kristof-mattei/autoheal-rs/commit/4bfa22d121e114bb8fbb8435ec90735f4f30466c))
- Line continuation by [@kristof-mattei](https://github.com/kristof-mattei) ([`8239c41`](https://github.com/kristof-mattei/autoheal-rs/commit/8239c4108c487acea05461ebe09f69887a3bd597))
- Uppercase by [@kristof-mattei](https://github.com/kristof-mattei) ([`e70a0ca`](https://github.com/kristof-mattei/autoheal-rs/commit/e70a0ca8dda3bc0126f539a945a0477ab6c8c875))
- Test against amd64 by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6da33f`](https://github.com/kristof-mattei/autoheal-rs/commit/a6da33f82bedfc04d9ad5cdc1b09d0ee375975f2))
- Grab pointer without intermediate reference by [@kristof-mattei](https://github.com/kristof-mattei) ([`e52bbbd`](https://github.com/kristof-mattei/autoheal-rs/commit/e52bbbd7ecc504d953bdd7b6b7157640022f9368))
- Remove no-deps by [@kristof-mattei](https://github.com/kristof-mattei) ([`80518c0`](https://github.com/kristof-mattei/autoheal-rs/commit/80518c0b1875c34158846e16e7de58d7b422b6e9))
- Reduce permissions by [@kristof-mattei](https://github.com/kristof-mattei) ([`54b4398`](https://github.com/kristof-mattei/autoheal-rs/commit/54b4398e66ee00c3e8f2254b9479d7ff79b29026))
- Manually build Rust for codeql as per our standard build by [@kristof-mattei](https://github.com/kristof-mattei) ([`af9390e`](https://github.com/kristof-mattei/autoheal-rs/commit/af9390ee873ba6968417c2749569cc630214ca7e))
- Build-mode `manual` is not supported for Rust by [@kristof-mattei](https://github.com/kristof-mattei) ([`7659499`](https://github.com/kristof-mattei/autoheal-rs/commit/76594994e616ebef734f5a7c275c5a19249d5d52))
- Pnpm by [@kristof-mattei](https://github.com/kristof-mattei) ([`229ffad`](https://github.com/kristof-mattei/autoheal-rs/commit/229ffad2a4bb0deb7f0c2d4fa954c84691262494))
- Ignore pnpm-lock.yaml format by [@kristof-mattei](https://github.com/kristof-mattei) ([`34ee806`](https://github.com/kristof-mattei/autoheal-rs/commit/34ee80670cf28b511be17ca61fdb52d0eb463ce4))
- Ensure cargo.lock is up to date by [@kristof-mattei](https://github.com/kristof-mattei) ([`75f241b`](https://github.com/kristof-mattei/autoheal-rs/commit/75f241bf01868d419fa383572650b891c06cacc6))
- Remove unused script by [@kristof-mattei](https://github.com/kristof-mattei) ([`f8aa057`](https://github.com/kristof-mattei/autoheal-rs/commit/f8aa05727ba3a31d557bb9287eccf147edc8827d))
- Also set style_edition by [@kristof-mattei](https://github.com/kristof-mattei) ([`172d6ab`](https://github.com/kristof-mattei/autoheal-rs/commit/172d6ab04a50b2f19415ec0b64726b6d6d773db5))
- Infer edition from Cargo.toml by [@kristof-mattei](https://github.com/kristof-mattei) ([`f1d0742`](https://github.com/kristof-mattei/autoheal-rs/commit/f1d0742fa46c850b4c983d5c6e378108bd22e942))
- Schema by [@kristof-mattei](https://github.com/kristof-mattei) ([`df22c46`](https://github.com/kristof-mattei/autoheal-rs/commit/df22c4616102dd6af00cd2f8993cf6d955467c99))
- Also ignore samply output by [@kristof-mattei](https://github.com/kristof-mattei) ([`87357c9`](https://github.com/kristof-mattei/autoheal-rs/commit/87357c9d0439fd731cb818c830898947a3735025))
- Simplify license, use MIT license instead of BSD, simplify package.json by [@kristof-mattei](https://github.com/kristof-mattei) ([`da64573`](https://github.com/kristof-mattei/autoheal-rs/commit/da64573e286744b1e812856b7d8cb8e27d600447))
- Validate toml & sh & ... formatting as part of PR process by [@kristof-mattei](https://github.com/kristof-mattei) ([`69f84c6`](https://github.com/kristof-mattei/autoheal-rs/commit/69f84c6436c864b272611f8629b1dbda46560e45))
- Ignore Cargo.lock from being formatted by [@kristof-mattei](https://github.com/kristof-mattei) ([`e721512`](https://github.com/kristof-mattei/autoheal-rs/commit/e72151217ff0eb6b92d4a15dfd11aeba8ee2687d))
- Restore package.json 2 tab width by [@kristof-mattei](https://github.com/kristof-mattei) ([`f8851e3`](https://github.com/kristof-mattei/autoheal-rs/commit/f8851e36b4f0f8795b4a945211e673f3aa8dcb55))
- Reorder by [@kristof-mattei](https://github.com/kristof-mattei) ([`d0080cd`](https://github.com/kristof-mattei/autoheal-rs/commit/d0080cde013c86eab0087d897024be4ab18df1ae))
- Update integration test to use local registry by [@kristof-mattei](https://github.com/kristof-mattei) ([`d28ae41`](https://github.com/kristof-mattei/autoheal-rs/commit/d28ae41481fa36e82d0c85b867d1170e70a564a1))

### ⚙️ Miscellaneous Tasks

- *(version)* V1.2.0 by [@invalid-email-address](https://github.com/invalid-email-address) ([`1dbbec6`](https://github.com/kristof-mattei/autoheal-rs/commit/1dbbec608864c5cb559a90d8904996011daef52c))
- *(version)* V1.3.0 by [@invalid-email-address](https://github.com/invalid-email-address) ([`bee1b83`](https://github.com/kristof-mattei/autoheal-rs/commit/bee1b83e6cfe1afd6286073fc1bb38787c0ba48e))
- *(version)* V1.3.1 by [@invalid-email-address](https://github.com/invalid-email-address) ([`096c422`](https://github.com/kristof-mattei/autoheal-rs/commit/096c4229799cecace867a1de699a7f65eefe59bb))
- *(version)* V1.6.0 by [@invalid-email-address](https://github.com/invalid-email-address) ([`717d540`](https://github.com/kristof-mattei/autoheal-rs/commit/717d540e2e933f9c4c9609a31d6d1e11f247d916))
- Minor build changes, formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`8b1e9f8`](https://github.com/kristof-mattei/autoheal-rs/commit/8b1e9f8aa2e6a37c268dd7587d0912d251afdea4))
- Fix typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`3a3e968`](https://github.com/kristof-mattei/autoheal-rs/commit/3a3e96855b5eadc81a2d0704a1c4de6569d721a5))
- Also rebuild when .nvmrc changes by [@kristof-mattei](https://github.com/kristof-mattei) ([`6155455`](https://github.com/kristof-mattei/autoheal-rs/commit/61554551d015d07fb230e09d8d5e33da2cfd1e54))
- Readme by [@kristof-mattei](https://github.com/kristof-mattei) ([`5d5d935`](https://github.com/kristof-mattei/autoheal-rs/commit/5d5d93557bf83ec841b8abd310f7e67cc7c5d086))
- Always run reporting, even when no changes as reports are mandatory by [@kristof-mattei](https://github.com/kristof-mattei) ([`c86f535`](https://github.com/kristof-mattei/autoheal-rs/commit/c86f5353d6813262e0bf327dbe2db641f87d363c))
- Disable codecov running plugins, disable codecov searching by [@kristof-mattei](https://github.com/kristof-mattei) ([`c946f90`](https://github.com/kristof-mattei/autoheal-rs/commit/c946f90a1a119f7f97f1e2830c2c55eef3050c6a))
- Add linebreaks in the if statements, otherwise the vscode parser gets upset by [@kristof-mattei](https://github.com/kristof-mattei) ([`79ecc87`](https://github.com/kristof-mattei/autoheal-rs/commit/79ecc8745e7629e60c75b1990aae2850543eb4d0))
- Remove unneeded id by [@kristof-mattei](https://github.com/kristof-mattei) ([`81b3536`](https://github.com/kristof-mattei/autoheal-rs/commit/81b35367789ed5722e391766ed52dd73b02ac259))
- Change name by [@kristof-mattei](https://github.com/kristof-mattei) ([`7bcbcb6`](https://github.com/kristof-mattei/autoheal-rs/commit/7bcbcb6b9a667327f8b239b4f49743efc6e55130))
- Separate the name so the rename script doesn't update it by [@kristof-mattei](https://github.com/kristof-mattei) ([`db3de07`](https://github.com/kristof-mattei/autoheal-rs/commit/db3de077fe7c68dfd00b94332eb776c889abc19b))
- Enforce_admins should be null if you want to disable it... by [@kristof-mattei](https://github.com/kristof-mattei) ([`97a3c84`](https://github.com/kristof-mattei/autoheal-rs/commit/97a3c846ebfaa2489bb7af87f4149ec7b9276efc))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`53a0e5b`](https://github.com/kristof-mattei/autoheal-rs/commit/53a0e5b88ad79efebc8f790df0cb253df0f3ff99))
- Try Rc<str> instead of String by [@kristof-mattei](https://github.com/kristof-mattei) ([`7d3c8ac`](https://github.com/kristof-mattei/autoheal-rs/commit/7d3c8ac03fe016e643727adc5e119dc445bc1a1c))
- Use rev as branch is gone by [@kristof-mattei](https://github.com/kristof-mattei) ([`11a117f`](https://github.com/kristof-mattei/autoheal-rs/commit/11a117fbb9e8898bdf88f506d068d2c3f2c7646e))
- Syntax consistency, as -> AS by [@kristof-mattei](https://github.com/kristof-mattei) ([`a28deb7`](https://github.com/kristof-mattei/autoheal-rs/commit/a28deb75b3a8ae060d7ca1f4459ad074ab7a276d))
- Syntax consistency, as -> AS by [@kristof-mattei](https://github.com/kristof-mattei) ([`ba8d334`](https://github.com/kristof-mattei/autoheal-rs/commit/ba8d3344466d98c4ddd2260291cecbe946c7c9ec))
- Fix merge conflict by [@kristof-mattei](https://github.com/kristof-mattei) ([`e14bfb5`](https://github.com/kristof-mattei/autoheal-rs/commit/e14bfb53d2071860925471e906cb7e458bf14f56))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`773e45a`](https://github.com/kristof-mattei/autoheal-rs/commit/773e45a7839624fef2056ed1e7f4e37339860f23))
- Allow multi wasi versions by [@kristof-mattei](https://github.com/kristof-mattei) ([`7953ddc`](https://github.com/kristof-mattei/autoheal-rs/commit/7953ddc71690bed3cbd3da8b64c40ed7430b32c9))
- Reduce allowed duplicate crates by [@kristof-mattei](https://github.com/kristof-mattei) ([`578dd3d`](https://github.com/kristof-mattei/autoheal-rs/commit/578dd3dd585db7233e5d132a0c99c84a036c1e52))
- Rust 1.85.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`e790939`](https://github.com/kristof-mattei/autoheal-rs/commit/e790939e5162a5b1f5bac286dac41d51f4969cfc))
- Use eyre::Report instead of the too-specialized color_eyre::eyre::Report by [@kristof-mattei](https://github.com/kristof-mattei) ([`02b08be`](https://github.com/kristof-mattei/autoheal-rs/commit/02b08beaf2315701e7f5a254facff26b68f90793))
- Correct logging with console subscriber by [@kristof-mattei](https://github.com/kristof-mattei) ([`d4e096d`](https://github.com/kristof-mattei/autoheal-rs/commit/d4e096d5c58b010f55aa68fe3116bf0540b66c88))
- Allow custom docker socket by [@kristof-mattei](https://github.com/kristof-mattei) ([`1072e05`](https://github.com/kristof-mattei/autoheal-rs/commit/1072e056884eb704f5e228ed1af42b2504d4ad22))
- Be more specific by [@kristof-mattei](https://github.com/kristof-mattei) ([`0caaab7`](https://github.com/kristof-mattei/autoheal-rs/commit/0caaab7de808ef8e50ce96fc3887fe24542d0fae))
- Remove unneeded .ci by [@kristof-mattei](https://github.com/kristof-mattei) ([`3be9c19`](https://github.com/kristof-mattei/autoheal-rs/commit/3be9c19e347578b89a0c2e9b3448e5df3e9aeaee))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`978f4b2`](https://github.com/kristof-mattei/autoheal-rs/commit/978f4b2a669b1d2f6ab3d9e7d738ab2b87516be4))
- Enable more lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`4cb6bb7`](https://github.com/kristof-mattei/autoheal-rs/commit/4cb6bb7967f130408917be3a75e19421c5fedb86))
- Ensure we have oras by [@kristof-mattei](https://github.com/kristof-mattei) ([`adb17dd`](https://github.com/kristof-mattei/autoheal-rs/commit/adb17dd3071a0f100b7f9b25d5ee79b2af0594da))
- Fix title by [@kristof-mattei](https://github.com/kristof-mattei) ([`1ecfb70`](https://github.com/kristof-mattei/autoheal-rs/commit/1ecfb70f8778f65098b117403bbc1574a2f6a017))
- Install cargo-binstall from updated url by [@kristof-mattei](https://github.com/kristof-mattei) ([`8172e7f`](https://github.com/kristof-mattei/autoheal-rs/commit/8172e7fe5a8996eb4b499ccd4ee443f7e5ee2cca))
- Format dockerfile by [@kristof-mattei](https://github.com/kristof-mattei) ([`edb1f30`](https://github.com/kristof-mattei/autoheal-rs/commit/edb1f309e3326acbad14b3b83aea65b53e12e506))
- Fmt also 1.85.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`c9488e3`](https://github.com/kristof-mattei/autoheal-rs/commit/c9488e384c6ac0816b2a02574f18e0e54eff3b12))
- Remove oras by [@kristof-mattei](https://github.com/kristof-mattei) ([`8eceae9`](https://github.com/kristof-mattei/autoheal-rs/commit/8eceae99297f4f5900549ef08f6105df530069cb))
- Forgot `push` by [@kristof-mattei](https://github.com/kristof-mattei) ([`6842ab4`](https://github.com/kristof-mattei/autoheal-rs/commit/6842ab4e26a3417520d3334da6fd542d1d240871))
- Push by tag, not filepath... by [@kristof-mattei](https://github.com/kristof-mattei) ([`cb03a00`](https://github.com/kristof-mattei/autoheal-rs/commit/cb03a00f208e2e3116f048bc6dcd42615933d90d))
- Add logging, try remove unneeded (?) buildx by [@kristof-mattei](https://github.com/kristof-mattei) ([`baafce6`](https://github.com/kristof-mattei/autoheal-rs/commit/baafce66428bb303013f3dbc2760556cbbeca150))
- Fix for rustup 1.28.0 not installing needed toolchain by default by [@kristof-mattei](https://github.com/kristof-mattei) ([`9f283b7`](https://github.com/kristof-mattei/autoheal-rs/commit/9f283b736f6e0f03ac8c2afaa2fda5cbbb86896d))
- Install rust-fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`234465e`](https://github.com/kristof-mattei/autoheal-rs/commit/234465e358cc52c0fead88af8f2116bac0f632b7))
- Use working-directory by [@kristof-mattei](https://github.com/kristof-mattei) ([`e7e90d5`](https://github.com/kristof-mattei/autoheal-rs/commit/e7e90d51a297e56a3fabc95ee2ade32c4150bb57))
- Set working-directory by [@kristof-mattei](https://github.com/kristof-mattei) ([`f657766`](https://github.com/kristof-mattei/autoheal-rs/commit/f657766fd5b75716fa316bd1ad90c5d7c32e975b))
- Ensure we restore symlinks by [@kristof-mattei](https://github.com/kristof-mattei) ([`ceede20`](https://github.com/kristof-mattei/autoheal-rs/commit/ceede20c91e783b56d32c109e7e989834ea38cfe))
- Clippy 1.86 fixes by [@kristof-mattei](https://github.com/kristof-mattei) ([`883a211`](https://github.com/kristof-mattei/autoheal-rs/commit/883a2118ff3600b8adb5117011e412c2faf49f7e))
- Add template clippy.toml by [@kristof-mattei](https://github.com/kristof-mattei) ([`307cbdb`](https://github.com/kristof-mattei/autoheal-rs/commit/307cbdb563c89194cf50119410509e4f4030659b))
- Remove incorrect comment by [@kristof-mattei](https://github.com/kristof-mattei) ([`82a8d6d`](https://github.com/kristof-mattei/autoheal-rs/commit/82a8d6d47279919c074137571d5901367e9826d3))
- Ignore generated changelog by [@kristof-mattei](https://github.com/kristof-mattei) ([`9084f3f`](https://github.com/kristof-mattei/autoheal-rs/commit/9084f3fe60d2794d98ca6fff47428c169a8f3f80))
- Clippy 1.86 fixes by [@kristof-mattei](https://github.com/kristof-mattei) ([`1ba9e82`](https://github.com/kristof-mattei/autoheal-rs/commit/1ba9e82c8de2d8fd2ba93018639ec3683e95c9e6))
- Disable required signatures by [@kristof-mattei](https://github.com/kristof-mattei) ([`445e425`](https://github.com/kristof-mattei/autoheal-rs/commit/445e4253a7dcbe8fac577cdcaff328466590abe1))
- Update debug setup by [@kristof-mattei](https://github.com/kristof-mattei) ([`59756b7`](https://github.com/kristof-mattei/autoheal-rs/commit/59756b712762276d2032290b7d24f3a9e900d059))
- Update texts by [@kristof-mattei](https://github.com/kristof-mattei) ([`8d09808`](https://github.com/kristof-mattei/autoheal-rs/commit/8d09808eccdb55a2c4def5d4732a932ca0741745))
- Change wording by [@kristof-mattei](https://github.com/kristof-mattei) ([`34fb4cf`](https://github.com/kristof-mattei/autoheal-rs/commit/34fb4cf3eda6724b074e3ce877a733dfaf8780cf))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`f3dde64`](https://github.com/kristof-mattei/autoheal-rs/commit/f3dde6447ec7fc4eadd646da761b872e21fdd6a2))
- I686 is 32-bit, we need 64-bit by [@kristof-mattei](https://github.com/kristof-mattei) ([`e57dbf8`](https://github.com/kristof-mattei/autoheal-rs/commit/e57dbf819b37fe82bab231af7c693a8bb32cc04c))
- Remove need for build & targetplatform in scripts by [@kristof-mattei](https://github.com/kristof-mattei) ([`13b8275`](https://github.com/kristof-mattei/autoheal-rs/commit/13b827509a357046309726f116cdfb611753d266))
- More precise coverage, don't include test/** by [@kristof-mattei](https://github.com/kristof-mattei) ([`83002c7`](https://github.com/kristof-mattei/autoheal-rs/commit/83002c72807cf5ffe0b77fcc16f7353eeefe46fe))
- Make wget more robust by [@kristof-mattei](https://github.com/kristof-mattei) ([`a7e1e60`](https://github.com/kristof-mattei/autoheal-rs/commit/a7e1e60645a975d9787f1681273af3b8b22326fc))
- Remove customization, packages now work oob with binstall by [@kristof-mattei](https://github.com/kristof-mattei) ([`c85dd07`](https://github.com/kristof-mattei/autoheal-rs/commit/c85dd0741ff01084504eb2d18c53efcb57a5f37b))
- Convention: bash variable names are lowercase by [@kristof-mattei](https://github.com/kristof-mattei) ([`dc27ba1`](https://github.com/kristof-mattei/autoheal-rs/commit/dc27ba1dc0d30acc3765a6369749891823bf5a70))
- Consolidation of scripts by [@kristof-mattei](https://github.com/kristof-mattei) ([`0a075f6`](https://github.com/kristof-mattei/autoheal-rs/commit/0a075f6204c8e468fa8027314dc89ed552b5d839))
- Move away from env, use output by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6a7b2c`](https://github.com/kristof-mattei/autoheal-rs/commit/a6a7b2c240dfc8f005740d3b15b9a1e5473f7a4d))
- Even more variables by [@kristof-mattei](https://github.com/kristof-mattei) ([`64866e5`](https://github.com/kristof-mattei/autoheal-rs/commit/64866e5bc5547d3b4ef13329f07708f59a0c1c6b))
- Fix output by [@kristof-mattei](https://github.com/kristof-mattei) ([`39aecb7`](https://github.com/kristof-mattei/autoheal-rs/commit/39aecb7f88547541cb73a0daea054cba7bafc37f))
- Group variables in single step by [@kristof-mattei](https://github.com/kristof-mattei) ([`793b007`](https://github.com/kristof-mattei/autoheal-rs/commit/793b0079596c261547767c8e939b31d12345e336))
- Set revision explicitely by [@kristof-mattei](https://github.com/kristof-mattei) ([`1ca21b9`](https://github.com/kristof-mattei/autoheal-rs/commit/1ca21b9c350223869f42f51e85cf6311b2ca7307))
- Fix strip-components, it caused nothing to be placed by [@kristof-mattei](https://github.com/kristof-mattei) ([`b88f13c`](https://github.com/kristof-mattei/autoheal-rs/commit/b88f13c48b7a68493819786d1c7e3ddf70e81526))
- Move scripts by [@kristof-mattei](https://github.com/kristof-mattei) ([`242ce77`](https://github.com/kristof-mattei/autoheal-rs/commit/242ce77b6b07062f09cecc116df82720af27fb2f))
- Fix typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`0ac7ffb`](https://github.com/kristof-mattei/autoheal-rs/commit/0ac7ffb096832c33b15a2369f9a101849c342ffb))
- Use `CARGO_CRATE_NAME` which always has `-` replaced by `_` by [@kristof-mattei](https://github.com/kristof-mattei) ([`8873e26`](https://github.com/kristof-mattei/autoheal-rs/commit/8873e26a1d228d9bad7252d3b7bdf57b59c2f065))
- Upgrade before installing by [@kristof-mattei](https://github.com/kristof-mattei) ([`8887b23`](https://github.com/kristof-mattei/autoheal-rs/commit/8887b2380320aaf84268ac9f199b6b453b47c893))
- Fix deprecation warning by [@kristof-mattei](https://github.com/kristof-mattei) ([`837157b`](https://github.com/kristof-mattei/autoheal-rs/commit/837157b57a19dfefa416576f38b7d597030d9d83))
- Don't prompt to accept commit when no conflicts by [@kristof-mattei](https://github.com/kristof-mattei) ([`e0677f8`](https://github.com/kristof-mattei/autoheal-rs/commit/e0677f83c9c49b189b96f5136b327f722de0e85c))
- Caching doesn't need the runner's OS by [@kristof-mattei](https://github.com/kristof-mattei) ([`f6c6e07`](https://github.com/kristof-mattei/autoheal-rs/commit/f6c6e07f96ef39a9927b6959976115a69d1a4c88))
- Disable telemetry, use oidc by [@kristof-mattei](https://github.com/kristof-mattei) ([`0b862de`](https://github.com/kristof-mattei/autoheal-rs/commit/0b862dea6cb59aa90c377cdf2a622c68a4a1e57a))
- Add coveralls by [@kristof-mattei](https://github.com/kristof-mattei) ([`e9fe37b`](https://github.com/kristof-mattei/autoheal-rs/commit/e9fe37b857938501c790abd7c63eb99633a73b8e))
- Split command, remove prefix by [@kristof-mattei](https://github.com/kristof-mattei) ([`61fb123`](https://github.com/kristof-mattei/autoheal-rs/commit/61fb1236d3451146507548f0efab8e5a4eb590c5))
- Testing by [@kristof-mattei](https://github.com/kristof-mattei) ([`3de7372`](https://github.com/kristof-mattei/autoheal-rs/commit/3de73727c81184cd5f2f492d27a6664d4519a314))
- Cargo binstall defaults to cargo install when not found by [@kristof-mattei](https://github.com/kristof-mattei) ([`61cf619`](https://github.com/kristof-mattei/autoheal-rs/commit/61cf6192a81156d3e81026058d4f4e8c0cdf54b8))
- Alphabet by [@kristof-mattei](https://github.com/kristof-mattei) ([`e6d1bef`](https://github.com/kristof-mattei/autoheal-rs/commit/e6d1beffdac232a3f25358804bd22539db56cca5))
- Disable raw-entry & allocator-api2 (they were enabled by default by hashbrown) by [@kristof-mattei](https://github.com/kristof-mattei) ([`8daf122`](https://github.com/kristof-mattei/autoheal-rs/commit/8daf122cdfe66d7b094fdd8e5a66bdcf27396078))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`6fb033f`](https://github.com/kristof-mattei/autoheal-rs/commit/6fb033fbb4458100673639f29835a6af0496b5a0))
- Shuffle stuff around by [@kristof-mattei](https://github.com/kristof-mattei) ([`cd283f4`](https://github.com/kristof-mattei/autoheal-rs/commit/cd283f4470b10eb7509b6c4634e20548b4d74f43))
- Disable multiple_crate_versions, it's just noise by [@kristof-mattei](https://github.com/kristof-mattei) ([`c9dee4b`](https://github.com/kristof-mattei/autoheal-rs/commit/c9dee4bc1e7e8d99750843b3883ed8b327062655))
- Update comment by [@kristof-mattei](https://github.com/kristof-mattei) ([`0ddb54f`](https://github.com/kristof-mattei/autoheal-rs/commit/0ddb54f6ec28a11d5839f9d47f7cb0732b4b16b7))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`f113c99`](https://github.com/kristof-mattei/autoheal-rs/commit/f113c99f22a429bf6f743289129a5d0a3ea69f15))
- Don't push image cargo build/test/... failed by [@kristof-mattei](https://github.com/kristof-mattei) ([`bbc21d0`](https://github.com/kristof-mattei/autoheal-rs/commit/bbc21d076d767fe5fa393f782b7dcc43b63d831d))
- Fix dockerfile instruction order by [@kristof-mattei](https://github.com/kristof-mattei) ([`6013577`](https://github.com/kristof-mattei/autoheal-rs/commit/6013577f1f0615e0cf8d1c861c5b3bcc0d421df9))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`1135c86`](https://github.com/kristof-mattei/autoheal-rs/commit/1135c86230b38095743a9cb189064872fcb3a51a))
- Rust doesn't support manual mode, no need to pre-build by [@kristof-mattei](https://github.com/kristof-mattei) ([`e932a61`](https://github.com/kristof-mattei/autoheal-rs/commit/e932a6182bc42113faf8ff9aecc9a3c551897236))
- Remove glob from path by [@kristof-mattei](https://github.com/kristof-mattei) ([`faf30f8`](https://github.com/kristof-mattei/autoheal-rs/commit/faf30f8f10ecc0d43c0d07f1fed5883694f5ef2d))
- Settings update by [@kristof-mattei](https://github.com/kristof-mattei) ([`be03d72`](https://github.com/kristof-mattei/autoheal-rs/commit/be03d72b4def3a601986845cd063edd915e71daa))
- Enable clone_on_ref_ptr by [@kristof-mattei](https://github.com/kristof-mattei) ([`1afa75c`](https://github.com/kristof-mattei/autoheal-rs/commit/1afa75c15ffd8409cacab55724fbdeb171bef55e))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`146c27d`](https://github.com/kristof-mattei/autoheal-rs/commit/146c27dc94d80bb1a3dfc3fa2a3459c3ea29d7cd))
- Fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`e28e2d9`](https://github.com/kristof-mattei/autoheal-rs/commit/e28e2d9467de13ed7431bbcd0bed16f960057b69))
- Also format toml by [@kristof-mattei](https://github.com/kristof-mattei) ([`aebaeae`](https://github.com/kristof-mattei/autoheal-rs/commit/aebaeae7a9cd79ead5a4bd64e7c5f5633bca43a8))
- Simplify prettierconfig by [@kristof-mattei](https://github.com/kristof-mattei) ([`15da060`](https://github.com/kristof-mattei/autoheal-rs/commit/15da060278add228c90f65f822279a0f93e43ed2))
## [1.5.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.4.1..v1.5.0) - 2024-04-13

### 🚀 Features

- Count unhealthy times by [@kristof-mattei](https://github.com/kristof-mattei) ([`c866ebc`](https://github.com/kristof-mattei/autoheal-rs/commit/c866ebcd7b2f34631ae26c55b5c9d6549e4a9f0e))
- Multi-platform images by [@kristof-mattei](https://github.com/kristof-mattei) ([`45196bd`](https://github.com/kristof-mattei/autoheal-rs/commit/45196bd6bd8801472a6db0b76278fbeaa54c1ca1))
- Codecov by [@kristof-mattei](https://github.com/kristof-mattei) ([`fac48b6`](https://github.com/kristof-mattei/autoheal-rs/commit/fac48b684db19cf74aae4afbf77783a9d31cacc2))

### 🐛 Bug Fixes

- *(deps)* Update rust crate http-body-util to 0.1.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`d951ffd`](https://github.com/kristof-mattei/autoheal-rs/commit/d951ffdd4e775b87038c4543330d81c3ecc685dc))
- *(deps)* Update rust crate color-eyre to 0.6.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`baf1dc7`](https://github.com/kristof-mattei/autoheal-rs/commit/baf1dc7b4352679914415662a2b16ec8f036d409))
- *(deps)* Update rust crate backtrace to 0.3.70 by [@renovate[bot]](https://github.com/renovate[bot]) ([`538cf6e`](https://github.com/kristof-mattei/autoheal-rs/commit/538cf6edaa42b46eb4cfec1cc80a609c33c2dc41))
- *(deps)* Update rust crate serde_json to 1.0.115 by [@renovate[bot]](https://github.com/renovate[bot]) ([`b0e8894`](https://github.com/kristof-mattei/autoheal-rs/commit/b0e8894e98b3af25283b8e36831abec04d3c13e6))
- *(deps)* Update rust crate tokio to 1.37.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`f5847db`](https://github.com/kristof-mattei/autoheal-rs/commit/f5847dbaea422ccea1066513b3d9a0d7861e66d6))
- Don't set shell, not needed in semgrep container by [@kristof-mattei](https://github.com/kristof-mattei) ([`de4ab4c`](https://github.com/kristof-mattei/autoheal-rs/commit/de4ab4c42e490e7e5a5ce2a6af342659f9ef3a49))
- Separate scan and fixup, as the scan container doesn't have bash / jq anymore by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e59581`](https://github.com/kristof-mattei/autoheal-rs/commit/4e595812651951b3e762d8abe0ef7dac61ff6156))
- Only upload sarif file itself by [@kristof-mattei](https://github.com/kristof-mattei) ([`0629d45`](https://github.com/kristof-mattei/autoheal-rs/commit/0629d45d07730960d087feed26c941ee1d7fa0a6))
- Set unpack folder, not filepath by [@kristof-mattei](https://github.com/kristof-mattei) ([`3612966`](https://github.com/kristof-mattei/autoheal-rs/commit/36129662df205055eedb8fd73e62a4dbeb08e323))
- Also wait on integration test by [@kristof-mattei](https://github.com/kristof-mattei) ([`9788d0d`](https://github.com/kristof-mattei/autoheal-rs/commit/9788d0dd0913acc1a11f6cee5cb8d352ef071ce4))
- Platform name by [@kristof-mattei](https://github.com/kristof-mattei) ([`d8cf974`](https://github.com/kristof-mattei/autoheal-rs/commit/d8cf9747ae7b1c14fa7f70b1f7f612d342d1e31a))
- Filepaths were wrong by [@kristof-mattei](https://github.com/kristof-mattei) ([`6d1f131`](https://github.com/kristof-mattei/autoheal-rs/commit/6d1f131d50f29b4e9f58a3ab29cc83da7dd3c0e2))
- Use artifact v4 settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`d8c091d`](https://github.com/kristof-mattei/autoheal-rs/commit/d8c091de62781eac1789ad4a11889e3fa4559414))
- Correctly build musl by [@kristof-mattei](https://github.com/kristof-mattei) ([`c712945`](https://github.com/kristof-mattei/autoheal-rs/commit/c7129451d3a7618b0304bcf78471f5fd14811bf2))

### ⚙️ Miscellaneous Tasks

- Revert use of hashset for names by [@kristof-mattei](https://github.com/kristof-mattei) ([`b8a60ba`](https://github.com/kristof-mattei/autoheal-rs/commit/b8a60ba2616a8f5a17efbb8f189137032937611a))
- Align title by [@kristof-mattei](https://github.com/kristof-mattei) ([`0a560bf`](https://github.com/kristof-mattei/autoheal-rs/commit/0a560bf62205abac716b75abe3a54d1776fbca81))
- Checkout to satisfy the codeql tool by [@kristof-mattei](https://github.com/kristof-mattei) ([`d6f858a`](https://github.com/kristof-mattei/autoheal-rs/commit/d6f858a3a1f026bb0c219e0dfdc199f3b3676af6))
- Ignore warning certain packages pulling in same crate with different version by [@kristof-mattei](https://github.com/kristof-mattei) ([`ba42c01`](https://github.com/kristof-mattei/autoheal-rs/commit/ba42c01790c622adfcde37248338be4bd00abd42))
- Rename semgrep job to make it register with semgrep by [@kristof-mattei](https://github.com/kristof-mattei) ([`9720764`](https://github.com/kristof-mattei/autoheal-rs/commit/9720764f82f17f202c8886bd638cf0faa36c4897))
- Use semgrep action, not container by [@kristof-mattei](https://github.com/kristof-mattei) ([`738b8de`](https://github.com/kristof-mattei/autoheal-rs/commit/738b8deb1606266e1106577b3f61231efe5b2d4c))
- Back to container, the action is outdated by [@kristof-mattei](https://github.com/kristof-mattei) ([`dc2bd52`](https://github.com/kristof-mattei/autoheal-rs/commit/dc2bd5258b484556d1600145067aa2a1fdb20d56))
- Add category by [@kristof-mattei](https://github.com/kristof-mattei) ([`df3df77`](https://github.com/kristof-mattei/autoheal-rs/commit/df3df77ba61c070a904f3d2f721cb245a0ac6ad9))
- Semgrep 1 job by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c6c2ee`](https://github.com/kristof-mattei/autoheal-rs/commit/5c6c2ee8527a899ec058e6194fea015c957e23a9))
- Fix filename by [@kristof-mattei](https://github.com/kristof-mattei) ([`3802280`](https://github.com/kristof-mattei/autoheal-rs/commit/3802280c2239d9a4174a315ccd172cfe8fc7f181))
- Allow warnings in test by [@kristof-mattei](https://github.com/kristof-mattei) ([`83d3bb2`](https://github.com/kristof-mattei/autoheal-rs/commit/83d3bb205342485cf9f3ff8f87c20cd1a9df616e))
- Allow warnings in test by [@kristof-mattei](https://github.com/kristof-mattei) ([`e49075f`](https://github.com/kristof-mattei/autoheal-rs/commit/e49075f2c6d6ab5b98e0d81304a2a6acad966c8a))
- Set checks with new API by [@kristof-mattei](https://github.com/kristof-mattei) ([`8361952`](https://github.com/kristof-mattei/autoheal-rs/commit/83619524f171939a235a921255eea4f52c4e9e07))
- Fix ] typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`3648333`](https://github.com/kristof-mattei/autoheal-rs/commit/3648333faa3377507bef44aa89fdbc06882293d3))
- Try codecov by [@kristof-mattei](https://github.com/kristof-mattei) ([`e1dd293`](https://github.com/kristof-mattei/autoheal-rs/commit/e1dd29358d55d597f95fa4053b76a7a99b1154f7))
- Support for ARM64 by [@kristof-mattei](https://github.com/kristof-mattei) ([`8658748`](https://github.com/kristof-mattei/autoheal-rs/commit/865874837be48194e825e92178951bdd7859d744))
- Try OCI by [@kristof-mattei](https://github.com/kristof-mattei) ([`d40e4f7`](https://github.com/kristof-mattei/autoheal-rs/commit/d40e4f7901c299a5429579c484e5fbc3c24716dc))
- Build with matrix by [@kristof-mattei](https://github.com/kristof-mattei) ([`87566ad`](https://github.com/kristof-mattei/autoheal-rs/commit/87566adcc016b050b52b7749ce47183f7d78e791))
- Also add rust target to name by [@kristof-mattei](https://github.com/kristof-mattei) ([`fa11cb5`](https://github.com/kristof-mattei/autoheal-rs/commit/fa11cb5b76646809cd5de6451a07818bec922b93))
- Correct params by [@kristof-mattei](https://github.com/kristof-mattei) ([`3b4513c`](https://github.com/kristof-mattei/autoheal-rs/commit/3b4513c6dfd7d46a21b41594173de731f9139ece))
- Debugging by [@kristof-mattei](https://github.com/kristof-mattei) ([`89153e2`](https://github.com/kristof-mattei/autoheal-rs/commit/89153e203b481aa34552ffdb093e608138b5bc10))
- Export docker by [@kristof-mattei](https://github.com/kristof-mattei) ([`5cff099`](https://github.com/kristof-mattei/autoheal-rs/commit/5cff0993fca0e1a278b63e91059474efe6853486))
- Prettier by [@kristof-mattei](https://github.com/kristof-mattei) ([`abe1135`](https://github.com/kristof-mattei/autoheal-rs/commit/abe113575b1383bb35be5849d65ced7b7bf7b240))
- Disable arm64 - musl by [@kristof-mattei](https://github.com/kristof-mattei) ([`828077d`](https://github.com/kristof-mattei/autoheal-rs/commit/828077da7fdf2772d50a55eb89067504e4c5a209))
- Use correct tar name by [@kristof-mattei](https://github.com/kristof-mattei) ([`8e3a561`](https://github.com/kristof-mattei/autoheal-rs/commit/8e3a5611108a2a62f0475104d2a27fda0369843f))
- Prettier by [@kristof-mattei](https://github.com/kristof-mattei) ([`1ae2986`](https://github.com/kristof-mattei/autoheal-rs/commit/1ae2986e3828aa0949d6319146f544c545a4c9d8))
- Linker for aarch64 by [@kristof-mattei](https://github.com/kristof-mattei) ([`112a197`](https://github.com/kristof-mattei/autoheal-rs/commit/112a19734f542c4ce61a93e1e1d365dbc4b0619d))
- Copy in linker into docker container by [@kristof-mattei](https://github.com/kristof-mattei) ([`57ff932`](https://github.com/kristof-mattei/autoheal-rs/commit/57ff9320ee1aa210641ea66041351a3d308cd9a0))
- Re-enable aarch64 by [@kristof-mattei](https://github.com/kristof-mattei) ([`d629f99`](https://github.com/kristof-mattei/autoheal-rs/commit/d629f99b3e09fc0df2d64d0fbc5d4186011bd4e1))
- Fix build error by [@kristof-mattei](https://github.com/kristof-mattei) ([`4dae08f`](https://github.com/kristof-mattei/autoheal-rs/commit/4dae08f1665344c945f3d95663fbf10b58203bf4))
- Arm64 by [@kristof-mattei](https://github.com/kristof-mattei) ([`9510d6d`](https://github.com/kristof-mattei/autoheal-rs/commit/9510d6d00bbb8b67a8317f3e452fc5fda93b0036))
## [1.4.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.4.0..v1.4.1) - 2024-03-09

### 🐛 Bug Fixes

- Ensure we start the to-be-ignored-container, otherwise our tests aren't testing anything by [@kristof-mattei](https://github.com/kristof-mattei) ([`1ef9d6e`](https://github.com/kristof-mattei/autoheal-rs/commit/1ef9d6e17ff3002f744b6d27e73c867c83aa9752))
- Trim slash, fix ignore test by [@kristof-mattei](https://github.com/kristof-mattei) ([`b1513a7`](https://github.com/kristof-mattei/autoheal-rs/commit/b1513a7b743e3f78ffc41f8109211a41485b3cdb))

### ⚙️ Miscellaneous Tasks

- Log when we're ignorning an unhealthy container by [@kristof-mattei](https://github.com/kristof-mattei) ([`38b1adc`](https://github.com/kristof-mattei/autoheal-rs/commit/38b1adc343706a725401c8720ba2f6e81875c8df))
- Better message by [@kristof-mattei](https://github.com/kristof-mattei) ([`d972d43`](https://github.com/kristof-mattei/autoheal-rs/commit/d972d43c21b7513540d7cc6a87b02182952ed357))
- Fix tests by [@kristof-mattei](https://github.com/kristof-mattei) ([`94fa5ec`](https://github.com/kristof-mattei/autoheal-rs/commit/94fa5ec7273143bd60d668110b7705c4db5f337b))
- Fix unneeded use by [@kristof-mattei](https://github.com/kristof-mattei) ([`413eae3`](https://github.com/kristof-mattei/autoheal-rs/commit/413eae33b533813ab016c7cf9b7eac5ff3bacf82))
- Fix docker compose container naming scheme by [@kristof-mattei](https://github.com/kristof-mattei) ([`dae11f1`](https://github.com/kristof-mattei/autoheal-rs/commit/dae11f1933484c8bc52b8612fb93327b44f56dbe))
## [1.3.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.2.0..v1.3.0) - 2024-03-07

### 🚀 Features

- Webhooks support ntfy by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6f110b`](https://github.com/kristof-mattei/autoheal-rs/commit/a6f110be9c728b97e39577e78c661341469992c3))
- Start migrating away from =true auto labels by [@kristof-mattei](https://github.com/kristof-mattei) ([`2f6960b`](https://github.com/kristof-mattei/autoheal-rs/commit/2f6960b18e81ef2975ba371e0d61028f6678fdee))
- Allow for excluding containers by name by [@kristof-mattei](https://github.com/kristof-mattei) ([`023b97d`](https://github.com/kristof-mattei/autoheal-rs/commit/023b97db6e934e4f3d602983299b3db604dca35f))
- Update env var name by [@kristof-mattei](https://github.com/kristof-mattei) ([`9d24b6f`](https://github.com/kristof-mattei/autoheal-rs/commit/9d24b6f3f73d87d4daa7159dbe2f791f723328a1))

### 🐛 Bug Fixes

- *(deps)* Update rust crate tracing-subscriber to 0.3.18 by [@renovate[bot]](https://github.com/renovate[bot]) ([`a253173`](https://github.com/kristof-mattei/autoheal-rs/commit/a2531730cbec708180e6f446445d1fbe87653ea0))
- *(deps)* Update rust crate percent-encoding to 2.3.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6dc5187`](https://github.com/kristof-mattei/autoheal-rs/commit/6dc5187efe3aff95201ebb2d3588216890b97663))
- *(deps)* Update rust crate url to 2.5.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`f150348`](https://github.com/kristof-mattei/autoheal-rs/commit/f15034848a6e43d534f0f61c0421153996ddf54a))
- *(deps)* Update rust crate libc to 0.2.153 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e2d2350`](https://github.com/kristof-mattei/autoheal-rs/commit/e2d23501f25aaddf0ad0f916cd434064b8b28399))
- *(deps)* Update rust crate hyper-util to 0.1.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`95bbb6b`](https://github.com/kristof-mattei/autoheal-rs/commit/95bbb6b1f5ccd7ffbf0917c2f653f43f92fc3fbd))
- *(deps)* Update rust crate tokio to 1.36.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`87cfec2`](https://github.com/kristof-mattei/autoheal-rs/commit/87cfec270f01dd7a894125e480287e7a3f8d39e3))
- *(deps)* Update rust crate serde to 1.0.197 by [@renovate[bot]](https://github.com/renovate[bot]) ([`85d629c`](https://github.com/kristof-mattei/autoheal-rs/commit/85d629c4addda006a7265430e763556449bad20f))
- *(deps)* Update rust crate serde_json to 1.0.114 by [@renovate[bot]](https://github.com/renovate[bot]) ([`17ebab2`](https://github.com/kristof-mattei/autoheal-rs/commit/17ebab2ab12b8e09cb5e600586738c0c88a0d063))
- *(deps)* Update rust crate hyper to 1.2.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e692907`](https://github.com/kristof-mattei/autoheal-rs/commit/e692907cd58ed01e0501a0062168d43de560ce89))
- *(deps)* Update rust crate http to 1.1.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`91c23ce`](https://github.com/kristof-mattei/autoheal-rs/commit/91c23ceac3ddf6094037957579256b379e5f0852))
- Don't share IDE settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`d096f8b`](https://github.com/kristof-mattei/autoheal-rs/commit/d096f8b70dacea9cf3deb5258209951e1eacb0c6))
- Workflow_dispatch does not take a branch by [@kristof-mattei](https://github.com/kristof-mattei) ([`7cb9b28`](https://github.com/kristof-mattei/autoheal-rs/commit/7cb9b2833bc904c936e96cb57d091262d3246933))
- Fix new version by [@kristof-mattei](https://github.com/kristof-mattei) ([`ef697ab`](https://github.com/kristof-mattei/autoheal-rs/commit/ef697abfef681fa7d69f7baa93c9c76b77b3548c))
- Fixup botched merge by [@kristof-mattei](https://github.com/kristof-mattei) ([`21d26c8`](https://github.com/kristof-mattei/autoheal-rs/commit/21d26c8e7d4ce1239d01a870c4b439b751636003))
- Add placeholder for env variable by [@kristof-mattei](https://github.com/kristof-mattei) ([`4bb5340`](https://github.com/kristof-mattei/autoheal-rs/commit/4bb534066cf1c3471e352cc5c8eb80ea3af113d5))
- Move semantic-release config file as per https://github.com/semantic-release/semantic-release/releases/tag/v23.0.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`e77b9d1`](https://github.com/kristof-mattei/autoheal-rs/commit/e77b9d10a3019ff3c00f28ed49eb6cf4faeb5f1c))
- Mixed up config name order by [@kristof-mattei](https://github.com/kristof-mattei) ([`6976f89`](https://github.com/kristof-mattei/autoheal-rs/commit/6976f89fbd6f5774047fa42c1dff717e2a37e0e5))
- Simplified tags by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c2e4f9`](https://github.com/kristof-mattei/autoheal-rs/commit/5c2e4f911c20a994d2c70cbb9105e1ebb156c6ae))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`80a9dda`](https://github.com/kristof-mattei/autoheal-rs/commit/80a9dda3495b1f1142304906a9246b8f6072c0ec))

### ⚙️ Miscellaneous Tasks

- Move to node 20, make backtrace always compile release as we don't care about their internals by [@kristof-mattei](https://github.com/kristof-mattei) ([`47ce350`](https://github.com/kristof-mattei/autoheal-rs/commit/47ce350ee2b385ddb05e0f04271950fa344b11a1))
- Just retain all names by [@kristof-mattei](https://github.com/kristof-mattei) ([`bbba821`](https://github.com/kristof-mattei/autoheal-rs/commit/bbba8212f5098f636c7fa876a6e2e1b7999b7e7d))
- Use lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`3f9d4ef`](https://github.com/kristof-mattei/autoheal-rs/commit/3f9d4ef710834eb82812c2cba60986df58494117))
- DENY uninlined format args by [@kristof-mattei](https://github.com/kristof-mattei) ([`9a8e100`](https://github.com/kristof-mattei/autoheal-rs/commit/9a8e1007eba0101c9aa5cbefb1e1edcfb2bb5f2c))
- ALLOW uninlined format args by [@kristof-mattei](https://github.com/kristof-mattei) ([`73e964d`](https://github.com/kristof-mattei/autoheal-rs/commit/73e964d65776a2eed8c71af0ab734b9ef517f2ea))
- Remove redundant quotes by [@kristof-mattei](https://github.com/kristof-mattei) ([`50c419b`](https://github.com/kristof-mattei/autoheal-rs/commit/50c419b558e160a602b69911a6356fb2f875ced9))
- Bump uninlined format args priority by [@kristof-mattei](https://github.com/kristof-mattei) ([`61e818b`](https://github.com/kristof-mattei/autoheal-rs/commit/61e818b5e0f6f209e8f28d601e99ff98d3b922ea))
- Add mold, use lints by [@kristof-mattei](https://github.com/kristof-mattei) ([`7a06c47`](https://github.com/kristof-mattei/autoheal-rs/commit/7a06c474c0b4fcb9df8099bd46a933872d017b8c))
- Restore backtrace always optimize by [@kristof-mattei](https://github.com/kristof-mattei) ([`9f2d778`](https://github.com/kristof-mattei/autoheal-rs/commit/9f2d77895397e064a3b86e0e75624318181d3f27))
- Fix typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`3bc4758`](https://github.com/kristof-mattei/autoheal-rs/commit/3bc4758d422559eab53724774bda44684c585eb3))
- Pin mold by [@kristof-mattei](https://github.com/kristof-mattei) ([`6222e7a`](https://github.com/kristof-mattei/autoheal-rs/commit/6222e7a5d096509d7b714b152c1dbfc4a5f32692))
- Run from scratch by [@kristof-mattei](https://github.com/kristof-mattei) ([`a296dd1`](https://github.com/kristof-mattei/autoheal-rs/commit/a296dd1b4307d9c81f36fd2e0ca77eb20665ed42))
- Comments by [@kristof-mattei](https://github.com/kristof-mattei) ([`f2701db`](https://github.com/kristof-mattei/autoheal-rs/commit/f2701db16e2019fa31bd72b069e6425ae6eec715))
- Use hyper-tls from hyperium by [@kristof-mattei](https://github.com/kristof-mattei) ([`f3a5dea`](https://github.com/kristof-mattei/autoheal-rs/commit/f3a5dea88d6e9d7a03027f63acbf9004bcc884c2))
- Disable function-next-line formatting, it looks weird by [@kristof-mattei](https://github.com/kristof-mattei) ([`1eb4dd2`](https://github.com/kristof-mattei/autoheal-rs/commit/1eb4dd2f28f2e6955ba36f09e464e6891b6d4c2c))
- Rename nextversion to next_version by [@kristof-mattei](https://github.com/kristof-mattei) ([`4821da4`](https://github.com/kristof-mattei/autoheal-rs/commit/4821da4b016aa974cffaf974c43165b504c1f125))
- No trailing commas in json by [@kristof-mattei](https://github.com/kristof-mattei) ([`a7b01be`](https://github.com/kristof-mattei/autoheal-rs/commit/a7b01be89742c69a3a671fba6f674d7895cec524))
- Fix startColumn/endColumn being 0. Is invalid. Normalize json file for diffing, ignore output. Diff is expected by [@kristof-mattei](https://github.com/kristof-mattei) ([`d37020f`](https://github.com/kristof-mattei/autoheal-rs/commit/d37020fdd19bd9f2d7348fd7b9553dd0921afa6f))
- Tests by [@kristof-mattei](https://github.com/kristof-mattei) ([`01451a8`](https://github.com/kristof-mattei/autoheal-rs/commit/01451a8724ec741b6f78a296f0fb1a3d309c258a))
## [1.2.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.1.0..v1.2.0) - 2023-11-02

### 🚀 Features

- Generalize dockerfile by [@kristof-mattei](https://github.com/kristof-mattei) ([`be4e9a5`](https://github.com/kristof-mattei/autoheal-rs/commit/be4e9a5fe9cc34bcd239080845ace4c6a631ba97))
- Allow for https webhooks by [@kristof-mattei](https://github.com/kristof-mattei) ([`d7dc9d8`](https://github.com/kristof-mattei/autoheal-rs/commit/d7dc9d82f0f906ca6fa14d465be15c3ffb2c4d66))

### 🐛 Bug Fixes

- *(deps)* Update rust crate tracing-subscriber to 0.3.17 by [@renovate[bot]](https://github.com/renovate[bot]) ([`11fde92`](https://github.com/kristof-mattei/autoheal-rs/commit/11fde92371d908d80ea008afd18001d1cfd74e2b))
- *(deps)* Update rust crate anyhow to 1.0.71 by [@renovate[bot]](https://github.com/renovate[bot]) ([`a1a51dc`](https://github.com/kristof-mattei/autoheal-rs/commit/a1a51dc83841ef978f5d6dad3fc7a2c21030d2c9))
- *(deps)* Update rust crate percent-encoding to 2.3.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`6a07766`](https://github.com/kristof-mattei/autoheal-rs/commit/6a0776658607dd079806849135be3e1ed98b0295))
- *(deps)* Update rust crate http-body-util to 0.1.0-rc.3 by [@renovate[bot]](https://github.com/renovate[bot]) ([`936801f`](https://github.com/kristof-mattei/autoheal-rs/commit/936801f7c1883634355caa384cdce40c1f1797b1))
- *(deps)* Update rust crate hyper to 1.0.0-rc.4 by [@renovate[bot]](https://github.com/renovate[bot]) ([`110022f`](https://github.com/kristof-mattei/autoheal-rs/commit/110022f7ef76fa5963d53f23aec99187ed871c76))
- *(deps)* Update rust crate url to 2.4.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`7c2ad80`](https://github.com/kristof-mattei/autoheal-rs/commit/7c2ad8057f7b047975bb7719542b7a4cf8cd840d))
- *(deps)* Update rust crate libc to 0.2.149 by [@renovate[bot]](https://github.com/renovate[bot]) ([`5001b0c`](https://github.com/kristof-mattei/autoheal-rs/commit/5001b0cfddc4d2d5da4fecf9d203facd0beac7f5))
- *(deps)* Update rust crate tokio to 1.33.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`605859a`](https://github.com/kristof-mattei/autoheal-rs/commit/605859a02f586298c1d4a45d2b90f5623841d46c))
- *(deps)* Update rust crate tracing to 0.1.40 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e4349ce`](https://github.com/kristof-mattei/autoheal-rs/commit/e4349ce264c3cc6a8b7d1a0a2b93dfc58ecc0ac4))
- *(deps)* Update rust crate serde to 1.0.190 by [@renovate[bot]](https://github.com/renovate[bot]) ([`bc4c06e`](https://github.com/kristof-mattei/autoheal-rs/commit/bc4c06eb922b4b16e6efc099dfcbacd810ba990b))
- *(deps)* Update rust crate serde_json to 1.0.108 by [@renovate[bot]](https://github.com/renovate[bot]) ([`486eede`](https://github.com/kristof-mattei/autoheal-rs/commit/486eedeb7f5475af4502c54d0f2d06d39a255066))
- Dump version number on boot by [@kristof-mattei](https://github.com/kristof-mattei) ([`9bb0132`](https://github.com/kristof-mattei/autoheal-rs/commit/9bb0132d3dc87987f5fa9acd233a509344c371fb))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`bc850da`](https://github.com/kristof-mattei/autoheal-rs/commit/bc850daa182501dbe2c7e7ff6280e0f84e433652))
- Clippy 1.67 by [@kristof-mattei](https://github.com/kristof-mattei) ([`bc72883`](https://github.com/kristof-mattei/autoheal-rs/commit/bc72883c3b8ac54956fd16bfedb462d0951a4ef7))
- Switch to editorconfig by [@kristof-mattei](https://github.com/kristof-mattei) ([`02402b7`](https://github.com/kristof-mattei/autoheal-rs/commit/02402b75b91fc52f36a1140d67f95460670a5f11))
- Allow uninlined format args by [@kristof-mattei](https://github.com/kristof-mattei) ([`203df05`](https://github.com/kristof-mattei/autoheal-rs/commit/203df05f519b8fa8d6f0f89044320194726ed5ca))
- Add update script by [@kristof-mattei](https://github.com/kristof-mattei) ([`bb08a31`](https://github.com/kristof-mattei/autoheal-rs/commit/bb08a315ab8b2269cc5d848c821a75b1ad79e97d))
- Make clippy more annoying by [@kristof-mattei](https://github.com/kristof-mattei) ([`af9bc12`](https://github.com/kristof-mattei/autoheal-rs/commit/af9bc12597c1236c3a642570c3c4fb526f5668b1))
- Hack version (?) by [@kristof-mattei](https://github.com/kristof-mattei) ([`58ff338`](https://github.com/kristof-mattei/autoheal-rs/commit/58ff338a043dc581440936114e8e0005a00a938c))
- Remove version, doesn't work for container > image by [@kristof-mattei](https://github.com/kristof-mattei) ([`2ef1e88`](https://github.com/kristof-mattei/autoheal-rs/commit/2ef1e883d0411aeaad4d0ea9e4a925bcd73a4ac1))
- Set rangeStrategy by [@kristof-mattei](https://github.com/kristof-mattei) ([`752da2c`](https://github.com/kristof-mattei/autoheal-rs/commit/752da2c3898fddaa537083430da3d0fd31d5a5cd))
- Unset rangeStrategy, move to the renovate base config by [@kristof-mattei](https://github.com/kristof-mattei) ([`2dae343`](https://github.com/kristof-mattei/autoheal-rs/commit/2dae3431cfeda8a8ef5c6b78a69f0e32b02db894))
- Editorconfig settings for shell files by [@kristof-mattei](https://github.com/kristof-mattei) ([`72d74b7`](https://github.com/kristof-mattei/autoheal-rs/commit/72d74b755522d30b07a00202ddf6e23138a9b1cb))
- Pin clippy by [@kristof-mattei](https://github.com/kristof-mattei) ([`bd33a24`](https://github.com/kristof-mattei/autoheal-rs/commit/bd33a2469072c2430b02694ebfe406f956de3862))
- Uninlined args, it's not well recognized and actually doesn't improve readability by [@kristof-mattei](https://github.com/kristof-mattei) ([`7da6808`](https://github.com/kristof-mattei/autoheal-rs/commit/7da68084ee59be4060297107e9cb8b2b99a06596))
- Yeet code climate by [@kristof-mattei](https://github.com/kristof-mattei) ([`0ff8087`](https://github.com/kristof-mattei/autoheal-rs/commit/0ff8087a8aca74832292d787329cb2c30d3cddb7))
- Lock down with version and digest by [@kristof-mattei](https://github.com/kristof-mattei) ([`ea6cda5`](https://github.com/kristof-mattei/autoheal-rs/commit/ea6cda5aff2e89af491654668c95edef6231898c))
- More formatting enforcement by [@kristof-mattei](https://github.com/kristof-mattei) ([`4825d45`](https://github.com/kristof-mattei/autoheal-rs/commit/4825d45eac04d98c8b20870657fb8346ef5636a3))
- Allow for building / not building docker container by [@kristof-mattei](https://github.com/kristof-mattei) ([`9a91217`](https://github.com/kristof-mattei/autoheal-rs/commit/9a91217a67620dcae3d4a9dbe5f71712c97aa03c))
- Ascii idents only to prevent idents starting with characters my keyboard can't handle by [@kristof-mattei](https://github.com/kristof-mattei) ([`0b7f646`](https://github.com/kristof-mattei/autoheal-rs/commit/0b7f6469dbd30ad44508f836277282860cad6cbd))
- Allow disable container retagging by [@kristof-mattei](https://github.com/kristof-mattei) ([`8e39b39`](https://github.com/kristof-mattei/autoheal-rs/commit/8e39b39694931df1716d2b79dafcfd7348de04f0))
- Updated cache ids by [@kristof-mattei](https://github.com/kristof-mattei) ([`75f6e51`](https://github.com/kristof-mattei/autoheal-rs/commit/75f6e51d56f491eabd2038688a4348f421eaad34))
- Comment indent by [@kristof-mattei](https://github.com/kristof-mattei) ([`aff7168`](https://github.com/kristof-mattei/autoheal-rs/commit/aff7168b45600424fed73fb582001cf55c2e2cd3))
- Don't retag when we don't build a container by [@kristof-mattei](https://github.com/kristof-mattei) ([`fe1a006`](https://github.com/kristof-mattei/autoheal-rs/commit/fe1a006c3d9e9d95f0d8cef01555d3a4c303e92a))
- Set maximum backtrace by [@kristof-mattei](https://github.com/kristof-mattei) ([`cd44fe2`](https://github.com/kristof-mattei/autoheal-rs/commit/cd44fe2ab070d930dea9b1040a5e57ec9276ed34))
- Hyper v1.0.0-rc.4 fixes by [@kristof-mattei](https://github.com/kristof-mattei) ([`417b954`](https://github.com/kristof-mattei/autoheal-rs/commit/417b954e3c59a3a3605f52f7c0b643a477189b89))
- Default is to use color-eyre by [@kristof-mattei](https://github.com/kristof-mattei) ([`9ec7951`](https://github.com/kristof-mattei/autoheal-rs/commit/9ec795182352fc168b18cd5a9fd623b1930494f3))
- Add update-name script by [@kristof-mattei](https://github.com/kristof-mattei) ([`57e5023`](https://github.com/kristof-mattei/autoheal-rs/commit/57e502335d9e4f409054ed5ecfb7d1ad8ab1d2d9))
- Coveralls as CodeCov keeps on failing by [@kristof-mattei](https://github.com/kristof-mattei) ([`fab1710`](https://github.com/kristof-mattei/autoheal-rs/commit/fab171080b3de6c51c4bc7531699841534366f8a))
- Specify version, Renovate will pin it by [@kristof-mattei](https://github.com/kristof-mattei) ([`2a44ecb`](https://github.com/kristof-mattei/autoheal-rs/commit/2a44ecbc3cbca50f1fddaf65ec4633b762384ef1))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`e4de56e`](https://github.com/kristof-mattei/autoheal-rs/commit/e4de56e38ddd799c5a76a534a14deeba7346f8ab))
- Use hyper-util by [@kristof-mattei](https://github.com/kristof-mattei) ([`89f5859`](https://github.com/kristof-mattei/autoheal-rs/commit/89f585990b1dc07796acdcfaaae861c14cf28884))
- Make BUILD_DOCKER_CONTAINER configurable from variables by [@kristof-mattei](https://github.com/kristof-mattei) ([`5ab39f5`](https://github.com/kristof-mattei/autoheal-rs/commit/5ab39f5f95c94aec0291d15c83e10f466fa659e4))
- Env -> vars by [@kristof-mattei](https://github.com/kristof-mattei) ([`a4ef4d5`](https://github.com/kristof-mattei/autoheal-rs/commit/a4ef4d59898bea126c2531a76799c0ba557ff2c7))
- Get application name automatically by [@kristof-mattei](https://github.com/kristof-mattei) ([`5f71149`](https://github.com/kristof-mattei/autoheal-rs/commit/5f71149c9c86a79155fe7180c8fc7e154febbca3))
- Remove unneeded newline by [@kristof-mattei](https://github.com/kristof-mattei) ([`4a3fe57`](https://github.com/kristof-mattei/autoheal-rs/commit/4a3fe575e0eb6d3a254a5041ccba662ddcbcbdcd))
- Flatten match, easier to read by [@kristof-mattei](https://github.com/kristof-mattei) ([`c54faf4`](https://github.com/kristof-mattei/autoheal-rs/commit/c54faf46bb142040d4ad528a8a1e3e85c1e54990))
- We don't use .idea config by [@kristof-mattei](https://github.com/kristof-mattei) ([`4d22e96`](https://github.com/kristof-mattei/autoheal-rs/commit/4d22e96241230cf0406dfc2cb3199b38acd2bad3))
- Consolidate clippy & rust config on top of main, all the rest causes duplication by [@kristof-mattei](https://github.com/kristof-mattei) ([`e158e41`](https://github.com/kristof-mattei/autoheal-rs/commit/e158e41a05171aadc4a36bed4f40685e9f15a8fc))
- Don't show progress by [@kristof-mattei](https://github.com/kristof-mattei) ([`2be0887`](https://github.com/kristof-mattei/autoheal-rs/commit/2be088747fc874dad816ed60d0e49dbf3390e0bb))
- Use static url by [@kristof-mattei](https://github.com/kristof-mattei) ([`586cbd1`](https://github.com/kristof-mattei/autoheal-rs/commit/586cbd15494baa03098ede86b0f11a4807d6671d))
- Use native-tls via hyper-tls by [@kristof-mattei](https://github.com/kristof-mattei) ([`98b8008`](https://github.com/kristof-mattei/autoheal-rs/commit/98b80081f1cebb3b94fee5e01dd2197ee0a2519e))

### ⚙️ Miscellaneous Tasks

- Ensure we take the longest tag, v1.0.0 instead of v1 by [@kristof-mattei](https://github.com/kristof-mattei) ([`f9d106a`](https://github.com/kristof-mattei/autoheal-rs/commit/f9d106a78779f14e61abea3416341a0801268a3b))
- Updated devcontainer config by [@kristof-mattei](https://github.com/kristof-mattei) ([`fd84775`](https://github.com/kristof-mattei/autoheal-rs/commit/fd84775429513a4be51795a717b780eb303fcfdc))
- Rename by [@kristof-mattei](https://github.com/kristof-mattei) ([`a8dc23f`](https://github.com/kristof-mattei/autoheal-rs/commit/a8dc23fde7791f5fb7bff45888a0286151a64682))

### ◀️ Revert

- *(deps)* Update rust crate tracing to 0.1.38 by [@kristof-mattei](https://github.com/kristof-mattei) ([`a0d13db`](https://github.com/kristof-mattei/autoheal-rs/commit/a0d13db03ecd4897d212694fb8c9c275b4bd742b))
## [1.1.0](https://github.com/kristof-mattei/autoheal-rs/compare/v1.0.2..v1.1.0) - 2023-01-22

### 🚀 Features

- Set latest tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`f024dd6`](https://github.com/kristof-mattei/autoheal-rs/commit/f024dd60e4eb831a2dc0bd4011701e0c44b9e318))
- Always run as root by [@kristof-mattei](https://github.com/kristof-mattei) ([`984cc68`](https://github.com/kristof-mattei/autoheal-rs/commit/984cc6833311263026d015e37b61afdc46fb3fbf))

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.24.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`f96f994`](https://github.com/kristof-mattei/autoheal-rs/commit/f96f994fff83c4cbbd60fc017b0fc8a219f6d285))
- Prevent cog throwing an error which causes script termination by [@kristof-mattei](https://github.com/kristof-mattei) ([`a0e6468`](https://github.com/kristof-mattei/autoheal-rs/commit/a0e6468e9491beccaed2ac227f9f6c93bec69bc4))
- Set latest tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`1870e1d`](https://github.com/kristof-mattei/autoheal-rs/commit/1870e1da3545c8fee6f6026ba6cece373316c1c4))

### ⚙️ Miscellaneous Tasks

- Bump package-lock.json by [@kristof-mattei](https://github.com/kristof-mattei) ([`1e0d6eb`](https://github.com/kristof-mattei/autoheal-rs/commit/1e0d6eb569c0928fc17eba447bf64a4fe6e4f809))
## [1.0.2](https://github.com/kristof-mattei/autoheal-rs/compare/v1.0.1..v1.0.2) - 2023-01-11

### 🐛 Bug Fixes

- Settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`769ba7d`](https://github.com/kristof-mattei/autoheal-rs/commit/769ba7d6fab0e5833e503b669531a1c9018c15a6))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`a6c47ee`](https://github.com/kristof-mattei/autoheal-rs/commit/a6c47ee700414a12bb6ee60a8a3d9bcdd28b7194))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`f71fbed`](https://github.com/kristof-mattei/autoheal-rs/commit/f71fbed5f1ac288c4d25462fbcd013dcca7a9ba0))
- Snake in comments is causing parsing failures by [@kristof-mattei](https://github.com/kristof-mattei) ([`69d278d`](https://github.com/kristof-mattei/autoheal-rs/commit/69d278dee797be71e67582b6ac0275853d1f5898))
- Use cocogitto bump, and support no new version generated by [@kristof-mattei](https://github.com/kristof-mattei) ([`68a37df`](https://github.com/kristof-mattei/autoheal-rs/commit/68a37dfdadd533d41497cdfed2cdff70716df06e))

### ⚙️ Miscellaneous Tasks

- Node v18 by [@kristof-mattei](https://github.com/kristof-mattei) ([`33b78ea`](https://github.com/kristof-mattei/autoheal-rs/commit/33b78ea423dedd7d25bfc96395aa1f532347afa3))
## [1.0.1](https://github.com/kristof-mattei/autoheal-rs/compare/v1.0.0..v1.0.1) - 2023-01-11

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.24.1 by [@renovate[bot]](https://github.com/renovate[bot]) ([`4b4a956`](https://github.com/kristof-mattei/autoheal-rs/commit/4b4a956578e1c4033573b59b8f1590c76e0252ca))
## [1.0.0] - 2023-01-03

### 🚀 Features

- Initial commit by [@kristof-mattei](https://github.com/kristof-mattei) ([`7af0684`](https://github.com/kristof-mattei/autoheal-rs/commit/7af0684c86b197699123cc67d0d87ec66fbd1e2e))
- Added Quz and test by [@kristof-mattei](https://github.com/kristof-mattei) ([`07ee173`](https://github.com/kristof-mattei/autoheal-rs/commit/07ee173edd9983669abfac91aa60245c7347d911))
- Use crane as tool instead of elaborate docker setup by [@kristof-mattei](https://github.com/kristof-mattei) ([`a5bc70b`](https://github.com/kristof-mattei/autoheal-rs/commit/a5bc70b9231c9fe16b4594e6349d75f5c0986932))
- Allowed for concurrent building of docker container by [@kristof-mattei](https://github.com/kristof-mattei) ([`7a99b3b`](https://github.com/kristof-mattei/autoheal-rs/commit/7a99b3b303846cb1a98e80f54f5950faee7fc28a))
- Grcov -> tarpaulin by [@kristof-mattei](https://github.com/kristof-mattei) ([`a37fa30`](https://github.com/kristof-mattei/autoheal-rs/commit/a37fa307247860f4071f31a569506b1bc8544045))
- Use cog by [@kristof-mattei](https://github.com/kristof-mattei) ([`146fdca`](https://github.com/kristof-mattei/autoheal-rs/commit/146fdca905e0a1d268eb9750934e3204803be68b))
- Use shell by [@kristof-mattei](https://github.com/kristof-mattei) ([`2a266e9`](https://github.com/kristof-mattei/autoheal-rs/commit/2a266e93167b8a39c482a8a615e12ecd5ddc4c21))
- Rust 1.58.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`3ac03e8`](https://github.com/kristof-mattei/autoheal-rs/commit/3ac03e829ae931369967728de3326294446a53e5))
- Rust 1.60.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`0180532`](https://github.com/kristof-mattei/autoheal-rs/commit/0180532b1e432662ccb7908ebc98d39ad501a227))
- Rust 1.61.0 by [@kristof-mattei](https://github.com/kristof-mattei) ([`c130478`](https://github.com/kristof-mattei/autoheal-rs/commit/c130478ad91eaf6b1cea04a2cfe57f6e926efa92))
- Upgrade semgrep to latest version by [@kristof-mattei](https://github.com/kristof-mattei) ([`c7dc4f5`](https://github.com/kristof-mattei/autoheal-rs/commit/c7dc4f5bf96c15784a6b6b5c7ff260a15f244dcd))
- More rust 1.62 by [@kristof-mattei](https://github.com/kristof-mattei) ([`628756b`](https://github.com/kristof-mattei/autoheal-rs/commit/628756bf8790f9ffe84ac5cb3ff37baac70c7fcc))
- Initial commit by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e1dc05`](https://github.com/kristof-mattei/autoheal-rs/commit/4e1dc056165cd10542f105be9e11d5116e4bbeb5))
- Sync-repo-settings first pass by [@kristof-mattei](https://github.com/kristof-mattei) ([`0d88baa`](https://github.com/kristof-mattei/autoheal-rs/commit/0d88baa34a3d7b167028941aafdf32e5c464b9c2))

### 🐛 Bug Fixes

- *(deps)* Update rust crate tokio to 1.23.0 by [@renovate[bot]](https://github.com/renovate[bot]) ([`c5400de`](https://github.com/kristof-mattei/autoheal-rs/commit/c5400de23697c8c5799e5d10714b84a2f664e003))
- *(deps)* Update rust crate serde_json to 1.0.91 by [@renovate[bot]](https://github.com/renovate[bot]) ([`3c150f4`](https://github.com/kristof-mattei/autoheal-rs/commit/3c150f491bf91d64d3b3a8b73c3a095366afa9fc))
- *(deps)* Update rust crate anyhow to 1.0.68 by [@renovate[bot]](https://github.com/renovate[bot]) ([`ef78200`](https://github.com/kristof-mattei/autoheal-rs/commit/ef78200f3fe362783bc145b60961a0bf75243c89))
- *(deps)* Update rust crate libc to 0.2.139 by [@renovate[bot]](https://github.com/renovate[bot]) ([`8a1ac1f`](https://github.com/kristof-mattei/autoheal-rs/commit/8a1ac1fc59650fdde4747aecc6656cfdd08a47af))
- *(deps)* Update rust crate serde to 1.0.152 by [@renovate[bot]](https://github.com/renovate[bot]) ([`e9349a4`](https://github.com/kristof-mattei/autoheal-rs/commit/e9349a40317d1e9683f595a9d4a6c3e7f7729770))
- *(deps)* Update rust crate http-body-util to 0.1.0-rc.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`c3f112c`](https://github.com/kristof-mattei/autoheal-rs/commit/c3f112ca0d74dad02a96a4699059b66437e44c82))
- *(deps)* Update rust crate hyper to 1.0.0-rc.2 by [@renovate[bot]](https://github.com/renovate[bot]) ([`5d83f65`](https://github.com/kristof-mattei/autoheal-rs/commit/5d83f6562fd10cb064eb46ad45c86763125cdf18))
- Enabled codecov by [@kristof-mattei](https://github.com/kristof-mattei) ([`70a7b60`](https://github.com/kristof-mattei/autoheal-rs/commit/70a7b60de32c66a3c0315f67d0f8278d91fc797a))
- Bumped cargo version by [@kristof-mattei](https://github.com/kristof-mattei) ([`f47f7d0`](https://github.com/kristof-mattei/autoheal-rs/commit/f47f7d0d5f7c6ba227002ba705a58f2fdf05aa2b))
- Username.toLowerCase() by [@kristof-mattei](https://github.com/kristof-mattei) ([`3c41052`](https://github.com/kristof-mattei/autoheal-rs/commit/3c410523e2c154652a3564511bbc90fde0710bb3))
- Make sure husky doesn't install on CI servers by [@kristof-mattei](https://github.com/kristof-mattei) ([`6908d84`](https://github.com/kristof-mattei/autoheal-rs/commit/6908d84e175ffab99bd18227bcbd515e0020c778))
- Download grcov from releases, WAY faster by [@kristof-mattei](https://github.com/kristof-mattei) ([`9a8c19b`](https://github.com/kristof-mattei/autoheal-rs/commit/9a8c19b538ae5c3cff200aa2460d03055ee1d14b))
- Don't try extract bz2 as gzip by [@kristof-mattei](https://github.com/kristof-mattei) ([`edd13c8`](https://github.com/kristof-mattei/autoheal-rs/commit/edd13c85f1f11c38e7bd1be2a1e8a4210346797a))
- Arguments go into an array by [@kristof-mattei](https://github.com/kristof-mattei) ([`9c6ff93`](https://github.com/kristof-mattei/autoheal-rs/commit/9c6ff93cb599504352c601e3e9f34790859a86d5))
- 2nd param is dest, not flags, null to infer destination though by [@kristof-mattei](https://github.com/kristof-mattei) ([`6db1ca8`](https://github.com/kristof-mattei/autoheal-rs/commit/6db1ca87785155c6e124dd99782acd5d9967778f))
- Remove duplicated login by [@kristof-mattei](https://github.com/kristof-mattei) ([`03eeddf`](https://github.com/kristof-mattei/autoheal-rs/commit/03eeddf662f5ebafb049c945d052340c8ee0d7e6))
- Split build and push for faster overall times by [@kristof-mattei](https://github.com/kristof-mattei) ([`83d504e`](https://github.com/kristof-mattei/autoheal-rs/commit/83d504eb164a50cdd52f850b144a71b467c3cf04))
- Reduced parent^2 detection complexity by [@kristof-mattei](https://github.com/kristof-mattei) ([`2fab698`](https://github.com/kristof-mattei/autoheal-rs/commit/2fab698158059e4ad8f2c9cc44a553beba68f826))
- Forgot to put back line to add SHA to $env by [@kristof-mattei](https://github.com/kristof-mattei) ([`5e2a12a`](https://github.com/kristof-mattei/autoheal-rs/commit/5e2a12a1eea0baa55a445077396f9468a1a16e3e))
- --quiet isn't quiet by [@kristof-mattei](https://github.com/kristof-mattei) ([`764ef0a`](https://github.com/kristof-mattei/autoheal-rs/commit/764ef0ab2140b0b80cf53b476ba6525b3a873b4e))
- Initialize otherwise EXITCODE is not set on success, and then it still fails 😅 by [@kristof-mattei](https://github.com/kristof-mattei) ([`6658ea7`](https://github.com/kristof-mattei/autoheal-rs/commit/6658ea7799a523b54b122ff6c8d5a33a18e2c7c6))
- Centralized names by [@kristof-mattei](https://github.com/kristof-mattei) ([`2e2345b`](https://github.com/kristof-mattei/autoheal-rs/commit/2e2345b2f1337ed5c99dea090d94de2b4b932949))
- Removed submodules, going direct by [@kristof-mattei](https://github.com/kristof-mattei) ([`2e2a71c`](https://github.com/kristof-mattei/autoheal-rs/commit/2e2a71c68a23764369cf67dfc485c9a76372dad2))
- Switched to fixed commits for actions by [@kristof-mattei](https://github.com/kristof-mattei) ([`2bd59a8`](https://github.com/kristof-mattei/autoheal-rs/commit/2bd59a85d847c6cbe5ce39038e25666c99086674))
- Fix for too much action... by [@kristof-mattei](https://github.com/kristof-mattei) ([`8256387`](https://github.com/kristof-mattei/autoheal-rs/commit/8256387796424f2cd5066452cf17a32f0cbe8f35))
- Removed needs for docker-build to allow parallel operation by [@kristof-mattei](https://github.com/kristof-mattei) ([`64986f6`](https://github.com/kristof-mattei/autoheal-rs/commit/64986f67e074439c1433011d5b27eafc725c35e4))
- Remove be by [@kristof-mattei](https://github.com/kristof-mattei) ([`21d027b`](https://github.com/kristof-mattei/autoheal-rs/commit/21d027bdf904697f96c9fe4adfe91fd2cf081d4f))
- Reduce docker tag complexity by [@kristof-mattei](https://github.com/kristof-mattei) ([`d187184`](https://github.com/kristof-mattei/autoheal-rs/commit/d187184277fadc66b6c7027624ecb9340c1e2fd9))
- Aspiring comments by [@kristof-mattei](https://github.com/kristof-mattei) ([`791bbed`](https://github.com/kristof-mattei/autoheal-rs/commit/791bbedad194c189fb9e35389f1c265c94df29da))
- ToUpper() by [@kristof-mattei](https://github.com/kristof-mattei) ([`81ba43c`](https://github.com/kristof-mattei/autoheal-rs/commit/81ba43cd7ba41aad25165e5e2232dad00476b3a7))
- Added skip-tags to prevent it from pushing tags by [@kristof-mattei](https://github.com/kristof-mattei) ([`3806f32`](https://github.com/kristof-mattei/autoheal-rs/commit/3806f32e8b0e56462a525744feb93954a15e562e))
- Dry run still verifies push permissions by [@kristof-mattei](https://github.com/kristof-mattei) ([`57b2cde`](https://github.com/kristof-mattei/autoheal-rs/commit/57b2cde87e49ce10c0e9b9b082eb0174409a2b0f))
- Even dry-run wants a token by [@kristof-mattei](https://github.com/kristof-mattei) ([`a1349fc`](https://github.com/kristof-mattei/autoheal-rs/commit/a1349fc215d87c4f5220496ae841c56776c9067f))
- Handle script dependency by [@kristof-mattei](https://github.com/kristof-mattei) ([`54be1b9`](https://github.com/kristof-mattei/autoheal-rs/commit/54be1b982120c880256a061895f15784d11752d9))
- Corrected library usage by [@kristof-mattei](https://github.com/kristof-mattei) ([`ed8e289`](https://github.com/kristof-mattei/autoheal-rs/commit/ed8e289589a490b61dc229021fc860d1d52fbb89))
- Forgot .rest by [@kristof-mattei](https://github.com/kristof-mattei) ([`2cb78e7`](https://github.com/kristof-mattei/autoheal-rs/commit/2cb78e791ee76ef95925e61a4f7988fc427bdde2))
- Variable correction by [@kristof-mattei](https://github.com/kristof-mattei) ([`7e0baaa`](https://github.com/kristof-mattei/autoheal-rs/commit/7e0baaa911c814c430ee69d43edfe3433c189aec))
- Create reports directory, otherwise tool complains by [@kristof-mattei](https://github.com/kristof-mattei) ([`15ef2e8`](https://github.com/kristof-mattei/autoheal-rs/commit/15ef2e83b9aff72e8986656d457e9439c2d1b7ba))
- Capture the rest, not the value called 'rest' by [@kristof-mattei](https://github.com/kristof-mattei) ([`222ef7c`](https://github.com/kristof-mattei/autoheal-rs/commit/222ef7c6edd6c3011d8d70cb0af0ae6319899842))
- Renamed for clarity by [@kristof-mattei](https://github.com/kristof-mattei) ([`319817f`](https://github.com/kristof-mattei/autoheal-rs/commit/319817fac4290114976eab8c87102278c8b87440))
- Locked node version by [@kristof-mattei](https://github.com/kristof-mattei) ([`5a7acea`](https://github.com/kristof-mattei/autoheal-rs/commit/5a7aceafe7488f93450bbeb494f400e5ef60193f))
- Reran npm install with npm 8 to update package-lock by [@kristof-mattei](https://github.com/kristof-mattei) ([`2f51f25`](https://github.com/kristof-mattei/autoheal-rs/commit/2f51f25cf9aec79943511b87b12d57e40224b6c3))
- Added npm to dependabot by [@kristof-mattei](https://github.com/kristof-mattei) ([`f99a1cb`](https://github.com/kristof-mattei/autoheal-rs/commit/f99a1cbce03151e38279c081b2becd4478a5f87b))
- A change in build scripts should cause a full rebuild by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e3da47`](https://github.com/kristof-mattei/autoheal-rs/commit/4e3da4743be7b4043d46cd10c2f9a05c60c4fc1b))
- Use environment variable to get cargo location by [@kristof-mattei](https://github.com/kristof-mattei) ([`c57b6db`](https://github.com/kristof-mattei/autoheal-rs/commit/c57b6dbcf20fabf1cbbc3cb946e656855acf992d))
- Corrected environment variable by [@kristof-mattei](https://github.com/kristof-mattei) ([`40527e4`](https://github.com/kristof-mattei/autoheal-rs/commit/40527e48765c4a99008d3207173de249c1de0b81))
- Updated cargo cache to use tilde again by [@kristof-mattei](https://github.com/kristof-mattei) ([`e650509`](https://github.com/kristof-mattei/autoheal-rs/commit/e650509c93198c1ca654b7ad46bf0f672b6f101a))
- See if we can cache the whole .cargo directory by [@kristof-mattei](https://github.com/kristof-mattei) ([`54d824f`](https://github.com/kristof-mattei/autoheal-rs/commit/54d824f82fd2e12457feaeb08ed259d15960f33f))
- Updated concurrency key by [@kristof-mattei](https://github.com/kristof-mattei) ([`eda182b`](https://github.com/kristof-mattei/autoheal-rs/commit/eda182b9fd074b2f515ad546d2ac15ef450c9141))
- Leverage setup-node's ability to read from .nvmrc by [@kristof-mattei](https://github.com/kristof-mattei) ([`f78dc4c`](https://github.com/kristof-mattei/autoheal-rs/commit/f78dc4ce7a4008e34e43a367be283327926f8581))
- Added missing data by [@kristof-mattei](https://github.com/kristof-mattei) ([`230c651`](https://github.com/kristof-mattei/autoheal-rs/commit/230c651afffa59e575e6d231d3334314df9179f3))
- Alltargets for tarpaulin by [@kristof-mattei](https://github.com/kristof-mattei) ([`3ba7006`](https://github.com/kristof-mattei/autoheal-rs/commit/3ba7006cd3bf3d027a0485aced51810837f41333))
- Install tarpaulin from source by [@kristof-mattei](https://github.com/kristof-mattei) ([`9f1966c`](https://github.com/kristof-mattei/autoheal-rs/commit/9f1966ce22f42e508d223a35ee4b651768fc2786))
- Synced tool invocation parameters by [@kristof-mattei](https://github.com/kristof-mattei) ([`8417c23`](https://github.com/kristof-mattei/autoheal-rs/commit/8417c23b540cd56be8123b85a0796b12ada21740))
- Bumped rust version by [@kristof-mattei](https://github.com/kristof-mattei) ([`c84b022`](https://github.com/kristof-mattei/autoheal-rs/commit/c84b02218ea3256f0138944b6939860a48236d7f))
- Added rust-toolchain, symlinked to rust-toolchain.toml by [@kristof-mattei](https://github.com/kristof-mattei) ([`cac0b9a`](https://github.com/kristof-mattei/autoheal-rs/commit/cac0b9aac7d25396a442e45dc3279ad9dd183eee))
- Getting toolchain file to work by [@kristof-mattei](https://github.com/kristof-mattei) ([`15ae1ab`](https://github.com/kristof-mattei/autoheal-rs/commit/15ae1abfbcc6472cd29039c4ef13c720966b5520))
- Try with profile and toolchain specified here by [@kristof-mattei](https://github.com/kristof-mattei) ([`05991b7`](https://github.com/kristof-mattei/autoheal-rs/commit/05991b716bed23ab70dfc1c6d8cdbfc83489e408))
- Infer components from rust-toolchain.toml by [@kristof-mattei](https://github.com/kristof-mattei) ([`e8aa97d`](https://github.com/kristof-mattei/autoheal-rs/commit/e8aa97da616b81fc8fbdfff829f6b820cac90513))
- Corrected build dependencies, all-done work be a success if docker-build failed by [@kristof-mattei](https://github.com/kristof-mattei) ([`ee3789d`](https://github.com/kristof-mattei/autoheal-rs/commit/ee3789db3225903b09255799f34df000d4b79462))
- The world is ok by [@kristof-mattei](https://github.com/kristof-mattei) ([`b9c1196`](https://github.com/kristof-mattei/autoheal-rs/commit/b9c119625d8f140e0e249ff5fcdafa55db969c0b))
- Fixed wrong needs name by [@kristof-mattei](https://github.com/kristof-mattei) ([`977e159`](https://github.com/kristof-mattei/autoheal-rs/commit/977e1592730e0ac018d1d6e1c7dfcc3a41ff4bfb))
- Set up toolchain manually by [@kristof-mattei](https://github.com/kristof-mattei) ([`f5f92d6`](https://github.com/kristof-mattei/autoheal-rs/commit/f5f92d6794df30cf70f7ad9963cd00c2367f1591))
- -y to accept defaults by [@kristof-mattei](https://github.com/kristof-mattei) ([`9280bfc`](https://github.com/kristof-mattei/autoheal-rs/commit/9280bfc01863ebac2baa024c28a447e2cc52098d))
- Don't fail tarpaulin when a test fails by [@kristof-mattei](https://github.com/kristof-mattei) ([`b065efa`](https://github.com/kristof-mattei/autoheal-rs/commit/b065efadf1e005dcc2c4c4ed8a7ddc74627bcf1b))
- Rebuild when NPM packages change by [@kristof-mattei](https://github.com/kristof-mattei) ([`43b4a7f`](https://github.com/kristof-mattei/autoheal-rs/commit/43b4a7fcce073b3ab19ea9d44d2cc8a3189dc285))
- Add commit linting by [@kristof-mattei](https://github.com/kristof-mattei) ([`141820a`](https://github.com/kristof-mattei/autoheal-rs/commit/141820aabf5b6a5823bb6c1080304797884187c5))
- Npm cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`12b30b3`](https://github.com/kristof-mattei/autoheal-rs/commit/12b30b3877504b61ec1157e9fc3350ad1f219830))
- Reduced complexity tied to semantic release because cog doesn't need it by [@kristof-mattei](https://github.com/kristof-mattei) ([`21ed988`](https://github.com/kristof-mattei/autoheal-rs/commit/21ed98844785b0addf90a083e624e26aa92e08d8))
- Set git user and email so that cog bump works by [@kristof-mattei](https://github.com/kristof-mattei) ([`01efb04`](https://github.com/kristof-mattei/autoheal-rs/commit/01efb04af74d945bd368152b9281f4ab2d551209))
- Set the right variable by [@kristof-mattei](https://github.com/kristof-mattei) ([`18978f8`](https://github.com/kristof-mattei/autoheal-rs/commit/18978f8e875aec0a31553f3ab5dd6d6d998b44f8))
- Ensure we're logged in to use the registry cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`994a73c`](https://github.com/kristof-mattei/autoheal-rs/commit/994a73c7c3d49280a7f5b7f6110859bafb7ba1c2))
- Reordered cache restore and toolchain setup by [@kristof-mattei](https://github.com/kristof-mattei) ([`5f08e37`](https://github.com/kristof-mattei/autoheal-rs/commit/5f08e374bcb75cc7d1eccd519884f069d05ce758))
- Try coveralls by [@kristof-mattei](https://github.com/kristof-mattei) ([`2e06476`](https://github.com/kristof-mattei/autoheal-rs/commit/2e06476d20cb459d068c9b6daf1f4e2725559835))
- Try coveralls by [@kristof-mattei](https://github.com/kristof-mattei) ([`d7196c5`](https://github.com/kristof-mattei/autoheal-rs/commit/d7196c57c7fa793db927c58dadf8cf6c28bd7a67))
- Renamed step name by [@kristof-mattei](https://github.com/kristof-mattei) ([`a7044ff`](https://github.com/kristof-mattei/autoheal-rs/commit/a7044ff77cf15d48660acb46621237e2aad81c82))
- Remove cache exclusion by [@kristof-mattei](https://github.com/kristof-mattei) ([`baee94e`](https://github.com/kristof-mattei/autoheal-rs/commit/baee94e4f6a90a8696c1e62b43c3d187083ab477))
- Use built-in rustup by [@kristof-mattei](https://github.com/kristof-mattei) ([`1b2d13b`](https://github.com/kristof-mattei/autoheal-rs/commit/1b2d13b9889b833b3616e825e49714bdfcde2cc8))
- Removed unneeded linting file by [@kristof-mattei](https://github.com/kristof-mattei) ([`7a6b3ed`](https://github.com/kristof-mattei/autoheal-rs/commit/7a6b3ed5d44dad4c4eb1360a73016200e4b10834))
- Merged configuration into 1 by [@kristof-mattei](https://github.com/kristof-mattei) ([`db6004a`](https://github.com/kristof-mattei/autoheal-rs/commit/db6004ac753af69521dbb529916b5c9f0fdc293e))
- Updated to 2021 edition by [@kristof-mattei](https://github.com/kristof-mattei) ([`cee0bd0`](https://github.com/kristof-mattei/autoheal-rs/commit/cee0bd0a17664f00947cc0d4bfe56fc178bcf9fe))
- Restored packages configuration by [@kristof-mattei](https://github.com/kristof-mattei) ([`506eac8`](https://github.com/kristof-mattei/autoheal-rs/commit/506eac86ff59359bf45758ca4182eeea3069fdbf))
- Missing package by [@kristof-mattei](https://github.com/kristof-mattei) ([`a20bc10`](https://github.com/kristof-mattei/autoheal-rs/commit/a20bc107500b531f2e1b23d45ab86063ab125225))
- Lock down sha256 of docker images by [@kristof-mattei](https://github.com/kristof-mattei) ([`659aadd`](https://github.com/kristof-mattei/autoheal-rs/commit/659aadd292af52bade19aeebc00747c8ee3c808d))
- Grcov intstead of tarpaulin by [@kristof-mattei](https://github.com/kristof-mattei) ([`d8f1fe1`](https://github.com/kristof-mattei/autoheal-rs/commit/d8f1fe173d6fe3efaa5aa2bb77ae0d988296c51b))
- Filename was wrong + comment update + rust-version bump by [@kristof-mattei](https://github.com/kristof-mattei) ([`4371418`](https://github.com/kristof-mattei/autoheal-rs/commit/4371418d7fd70cd8faaee6d77b98c932905339b5))
- Corrected title by [@kristof-mattei](https://github.com/kristof-mattei) ([`b3b0cb6`](https://github.com/kristof-mattei/autoheal-rs/commit/b3b0cb62e4b96f95daef20bc0d1ec6143e5d2e97))
- Merge steps for testbased coverage by [@kristof-mattei](https://github.com/kristof-mattei) ([`a7b74e1`](https://github.com/kristof-mattei/autoheal-rs/commit/a7b74e1637ac50f4165507f491278386cb986244))
- Install llvm tools by [@kristof-mattei](https://github.com/kristof-mattei) ([`3d5b63b`](https://github.com/kristof-mattei/autoheal-rs/commit/3d5b63b77c7c0b431c4f547c7b429a5266703637))
- Typo in env variable by [@kristof-mattei](https://github.com/kristof-mattei) ([`d4e47fa`](https://github.com/kristof-mattei/autoheal-rs/commit/d4e47faa41b598137d3e60418a7f4d9d8f8a0921))
- Code climate test reporter by [@kristof-mattei](https://github.com/kristof-mattei) ([`9c58b72`](https://github.com/kristof-mattei/autoheal-rs/commit/9c58b72f8b04698b8ef901e4e19ae642f3c99e8e))
- Use correct subcommand by [@kristof-mattei](https://github.com/kristof-mattei) ([`96fa7aa`](https://github.com/kristof-mattei/autoheal-rs/commit/96fa7aa0d712a2d8feb188ad28587b5f0e697de1))
- Remove dependency on build to run on main by [@kristof-mattei](https://github.com/kristof-mattei) ([`0a6b0c6`](https://github.com/kristof-mattei/autoheal-rs/commit/0a6b0c66405b44a17ba512cb8f56f64ee4420c9f))
- Tag -> sha by [@kristof-mattei](https://github.com/kristof-mattei) ([`7be092d`](https://github.com/kristof-mattei/autoheal-rs/commit/7be092dfdf25244113b8695cf94e7edae3a4ec23))
- Filter out lcov by [@kristof-mattei](https://github.com/kristof-mattei) ([`60e9e4e`](https://github.com/kristof-mattei/autoheal-rs/commit/60e9e4eb97e40080a4c4ec563c27675404e370a9))
- Also keep tests by [@kristof-mattei](https://github.com/kristof-mattei) ([`a746b08`](https://github.com/kristof-mattei/autoheal-rs/commit/a746b08a460aafa5b984303dc4527ef0046dcbe8))
- Add linebreak by [@kristof-mattei](https://github.com/kristof-mattei) ([`4712a1f`](https://github.com/kristof-mattei/autoheal-rs/commit/4712a1f6df2e3005b70e49cc4d6a097d0645a455))
- Set code climate filter by [@kristof-mattei](https://github.com/kristof-mattei) ([`e171505`](https://github.com/kristof-mattei/autoheal-rs/commit/e17150556571145f9295d3f6ed46eb372485b262))
- No need to bring in action just for this by [@kristof-mattei](https://github.com/kristof-mattei) ([`faa92dc`](https://github.com/kristof-mattei/autoheal-rs/commit/faa92dc31731b248339b30e9c2c583164f37afce))
- Removed spaces by [@kristof-mattei](https://github.com/kristof-mattei) ([`15a0f18`](https://github.com/kristof-mattei/autoheal-rs/commit/15a0f18752a64ce109f269bd8eb362334982b327))
- Auto dependabot merge by [@kristof-mattei](https://github.com/kristof-mattei) ([`cdc772a`](https://github.com/kristof-mattei/autoheal-rs/commit/cdc772a475f030e5135f22ced30af4a74129b8b2))
- Ignore merge commit linting by [@kristof-mattei](https://github.com/kristof-mattei) ([`314180a`](https://github.com/kristof-mattei/autoheal-rs/commit/314180a6a826335dfab4d4425fc29b474871ab33))
- Instrument coverage is stable! by [@kristof-mattei](https://github.com/kristof-mattei) ([`cd2a8e6`](https://github.com/kristof-mattei/autoheal-rs/commit/cd2a8e618131056b33df7711cac1da6d7e267e9c))
- Switch to auto config by [@kristof-mattei](https://github.com/kristof-mattei) ([`71ec2d9`](https://github.com/kristof-mattei/autoheal-rs/commit/71ec2d96c2778dc3c8c9359840b7c43f18323f44))
- Remove fixes by [@kristof-mattei](https://github.com/kristof-mattei) ([`e90953e`](https://github.com/kristof-mattei/autoheal-rs/commit/e90953e4eba7037818de69e8067faa244f6259a0))
- Add user by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e414de`](https://github.com/kristof-mattei/autoheal-rs/commit/4e414de196055df363eb355b100ba5d6e7cb4a06))
- Set the right user by [@kristof-mattei](https://github.com/kristof-mattei) ([`c82af71`](https://github.com/kristof-mattei/autoheal-rs/commit/c82af712a81c84e773764916c281274377d150e0))
- Set permissions by [@kristof-mattei](https://github.com/kristof-mattei) ([`d3c9dcf`](https://github.com/kristof-mattei/autoheal-rs/commit/d3c9dcfea41224b596e87672338c66bf5f98f78e))
- Try centralized renovate config by [@kristof-mattei](https://github.com/kristof-mattei) ([`973829b`](https://github.com/kristof-mattei/autoheal-rs/commit/973829b723a20cdd4b5c5f9d0bf3477cc8d421e2))
- Download binstall to /tmp to avoid additional untracked files by [@kristof-mattei](https://github.com/kristof-mattei) ([`eb3d325`](https://github.com/kristof-mattei/autoheal-rs/commit/eb3d3252dabeeb48c960995a46bb13b4df71796d))
- Binstall now wants stuff with a capital by [@kristof-mattei](https://github.com/kristof-mattei) ([`4e862db`](https://github.com/kristof-mattei/autoheal-rs/commit/4e862dbb4fcd5577e2ab36ebf2ba3ceb755eb122))
- Don't copy paste by [@kristof-mattei](https://github.com/kristof-mattei) ([`d9ed65e`](https://github.com/kristof-mattei/autoheal-rs/commit/d9ed65e9c1ac8e3ffd77a33f5f0956637ee90302))
- Fixed... the name! by [@kristof-mattei](https://github.com/kristof-mattei) ([`219147e`](https://github.com/kristof-mattei/autoheal-rs/commit/219147ea097f02f1c275036e4f4b9d638d237e88))
- Lowercase package fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`d464837`](https://github.com/kristof-mattei/autoheal-rs/commit/d4648375d819651c86216d8b2d3f511622f91d82))
- Fix schema by [@kristof-mattei](https://github.com/kristof-mattei) ([`2363092`](https://github.com/kristof-mattei/autoheal-rs/commit/23630924a373ca06ffed352f8574516afb7e0cda))
- Don't require reviews by [@kristof-mattei](https://github.com/kristof-mattei) ([`921c419`](https://github.com/kristof-mattei/autoheal-rs/commit/921c4193b6dd2b37af2487072117c7586bbc37c3))
- Force signed commits by [@kristof-mattei](https://github.com/kristof-mattei) ([`9c408fd`](https://github.com/kristof-mattei/autoheal-rs/commit/9c408fdef65a9dbe2599402b5654de69ed9cfbcc))
- Update rules by [@kristof-mattei](https://github.com/kristof-mattei) ([`db556a8`](https://github.com/kristof-mattei/autoheal-rs/commit/db556a800db25f534302a594a120632d709b8a2f))
- Set the correct tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`273291f`](https://github.com/kristof-mattei/autoheal-rs/commit/273291fe484addf894f8ddc3c13889b140389103))
- Deny stuff, nobody reads warnings by [@kristof-mattei](https://github.com/kristof-mattei) ([`20a9980`](https://github.com/kristof-mattei/autoheal-rs/commit/20a998015fee19324cf08f8893bb260c69aff3a3))
- Semgrep from container by [@kristof-mattei](https://github.com/kristof-mattei) ([`de3c82d`](https://github.com/kristof-mattei/autoheal-rs/commit/de3c82ddb3ec7fc91b63376ab3db0dfcbd6cfd48))
- Set tag & sha256 by [@kristof-mattei](https://github.com/kristof-mattei) ([`7b6e267`](https://github.com/kristof-mattei/autoheal-rs/commit/7b6e267179f85e2615185826d42f1ac74b1cb14d))
- Group imports by [@kristof-mattei](https://github.com/kristof-mattei) ([`a561eab`](https://github.com/kristof-mattei/autoheal-rs/commit/a561eabca180fbdebe301f830f2bcd3139887134))
- Quotes by [@kristof-mattei](https://github.com/kristof-mattei) ([`91c129c`](https://github.com/kristof-mattei/autoheal-rs/commit/91c129c1d09e29453c37024514cfdda0b3407c87))
- Comment out nightly function, add match | by [@kristof-mattei](https://github.com/kristof-mattei) ([`d0c9366`](https://github.com/kristof-mattei/autoheal-rs/commit/d0c936698945e3733e598e229ba117b44557d822))
- No leading pipes by [@kristof-mattei](https://github.com/kristof-mattei) ([`1010233`](https://github.com/kristof-mattei/autoheal-rs/commit/101023383a45998b4e0ed2a1dbd86f5681f7a73f))
- Correct cocogitto again with binstall by [@kristof-mattei](https://github.com/kristof-mattei) ([`6d58aa7`](https://github.com/kristof-mattei/autoheal-rs/commit/6d58aa766b57ca79248ddfffaa2e405efda60fb2))
- Google -> probot by [@kristof-mattei](https://github.com/kristof-mattei) ([`23ac95c`](https://github.com/kristof-mattei/autoheal-rs/commit/23ac95cd6c81f0b0f32accf434f4a50d5c48e65b))
- Allow overrides by [@kristof-mattei](https://github.com/kristof-mattei) ([`33769f0`](https://github.com/kristof-mattei/autoheal-rs/commit/33769f0d2f9451e5411470dc2dfc3b278b036229))
- Add other settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`f520994`](https://github.com/kristof-mattei/autoheal-rs/commit/f520994518d0f5e13e59ec7c593c76da65062b02))
- Updated script for easier integrations by [@kristof-mattei](https://github.com/kristof-mattei) ([`a89a35d`](https://github.com/kristof-mattei/autoheal-rs/commit/a89a35d6e763354f644708a2389a25851cee80de))
- Rewrite to rust by [@kristof-mattei](https://github.com/kristof-mattei) ([`e438f25`](https://github.com/kristof-mattei/autoheal-rs/commit/e438f25e6f2461313e76a93fbe41ee3ac4dc9229))
- Set-output is deprecated by [@kristof-mattei](https://github.com/kristof-mattei) ([`8b69dd4`](https://github.com/kristof-mattei/autoheal-rs/commit/8b69dd437a5cec1f3dd7e256be22658338bcbb03))
- Set correct nextest config by [@kristof-mattei](https://github.com/kristof-mattei) ([`2447503`](https://github.com/kristof-mattei/autoheal-rs/commit/2447503724f960fb66a723705e52810e169ae2aa))
- Formatting by [@kristof-mattei](https://github.com/kristof-mattei) ([`c5cd437`](https://github.com/kristof-mattei/autoheal-rs/commit/c5cd437d2876744a61201a6db494866c021510df))
- Move optimization by [@kristof-mattei](https://github.com/kristof-mattei) ([`5eb3d81`](https://github.com/kristof-mattei/autoheal-rs/commit/5eb3d81f0f168f7e56257e02e50cd97482863a6f))
- Allow macro to be called with both StdError and anyhow::Error by [@kristof-mattei](https://github.com/kristof-mattei) ([`fa3f522`](https://github.com/kristof-mattei/autoheal-rs/commit/fa3f522d5cc4f6b401a107eea4ed0aa7c70ab738))
- Use tracing by [@kristof-mattei](https://github.com/kristof-mattei) ([`04295d7`](https://github.com/kristof-mattei/autoheal-rs/commit/04295d75e4c4541a121f33ca271a24a50e618e0d))
- Add lines to set upstream properly by [@kristof-mattei](https://github.com/kristof-mattei) ([`8a03499`](https://github.com/kristof-mattei/autoheal-rs/commit/8a03499dc3a635d3cce3a7e3f1454c859cbb5db0))
- Comments by [@kristof-mattei](https://github.com/kristof-mattei) ([`2d2fe04`](https://github.com/kristof-mattei/autoheal-rs/commit/2d2fe04efc456c63fc1954fae5e64b9ba0a14e56))
- Testing by [@kristof-mattei](https://github.com/kristof-mattei) ([`8f62e62`](https://github.com/kristof-mattei/autoheal-rs/commit/8f62e6203a27454f67166a76478c4b6ea9490fc8))
- Integration testing by [@kristof-mattei](https://github.com/kristof-mattei) ([`67f4313`](https://github.com/kristof-mattei/autoheal-rs/commit/67f43132097874b5affd72e1d54650a7e82132d5))
- Write full name by [@kristof-mattei](https://github.com/kristof-mattei) ([`fc014b6`](https://github.com/kristof-mattei/autoheal-rs/commit/fc014b6260e21cd60b8145adb1a1a9f56a5bbd14))
- Inline label by [@kristof-mattei](https://github.com/kristof-mattei) ([`797a7ec`](https://github.com/kristof-mattei/autoheal-rs/commit/797a7ec0423156d5ecc2fbc7005eb819c8d0a3ce))
- Use buildkit for compose by [@kristof-mattei](https://github.com/kristof-mattei) ([`5c6b4b6`](https://github.com/kristof-mattei/autoheal-rs/commit/5c6b4b65f1beae36e493b50418e8c282ac81763f))
- Use up to date action by [@kristof-mattei](https://github.com/kristof-mattei) ([`47190c9`](https://github.com/kristof-mattei/autoheal-rs/commit/47190c9524d3029fe2a3dedc570e1b88d2f63105))
- Bump packages by [@kristof-mattei](https://github.com/kristof-mattei) ([`05fd117`](https://github.com/kristof-mattei/autoheal-rs/commit/05fd11757ab2ecf9ae9e4c48843876a9e09354a3))
- Separate before and after cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`82f9775`](https://github.com/kristof-mattei/autoheal-rs/commit/82f97756352773948e9a4572f2740831d5bf9f2c))
- Don't test docker until we've successfully built container and use digest by [@kristof-mattei](https://github.com/kristof-mattei) ([`db3a72b`](https://github.com/kristof-mattei/autoheal-rs/commit/db3a72b2ebee52995beb61150595c5ff43457e40))
- Settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`b114d41`](https://github.com/kristof-mattei/autoheal-rs/commit/b114d412cd21416f37b976b239ce30938fdc6c5d))
- Use cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`ad1886d`](https://github.com/kristof-mattei/autoheal-rs/commit/ad1886de48fcfec634f400b2ff961eb7a83a8fea))
- Cleanup by [@kristof-mattei](https://github.com/kristof-mattei) ([`0eecc55`](https://github.com/kristof-mattei/autoheal-rs/commit/0eecc55c44375cb00b6a662cda21e80854361398))
- Log webhook success by [@kristof-mattei](https://github.com/kristof-mattei) ([`5390013`](https://github.com/kristof-mattei/autoheal-rs/commit/53900133d8b3c9a64e2217ee85d05b91cad52564))

### 🧪 Testing

- Flow by [@kristof-mattei](https://github.com/kristof-mattei) ([`9e940fb`](https://github.com/kristof-mattei/autoheal-rs/commit/9e940fbe55fa07e3b24358b023720ba2d73d8c86))
- Print changelog.md by [@kristof-mattei](https://github.com/kristof-mattei) ([`ce90089`](https://github.com/kristof-mattei/autoheal-rs/commit/ce900894ee712c58c14955ec4bfa9b90b0c050e8))
- Failure test by [@kristof-mattei](https://github.com/kristof-mattei) ([`fc23c01`](https://github.com/kristof-mattei/autoheal-rs/commit/fc23c01aee19e787bf952f9ce9b2205e62ff15cc))

### ⚙️ Miscellaneous Tasks

- Copy tags with docker tags again by [@kristof-mattei](https://github.com/kristof-mattei) ([`8ffcf1e`](https://github.com/kristof-mattei/autoheal-rs/commit/8ffcf1ed1de7a080abcea97328b049bf909a1dba))
- Spelling correction by [@kristof-mattei](https://github.com/kristof-mattei) ([`78e52a8`](https://github.com/kristof-mattei/autoheal-rs/commit/78e52a882d3cc7e22e17e4457e870f9b3b7534d2))
- Removed unneeded print by [@kristof-mattei](https://github.com/kristof-mattei) ([`045ea6c`](https://github.com/kristof-mattei/autoheal-rs/commit/045ea6c8ecceed3918dd2c5307cbae9e56ff1864))
- Added comment for future me <insert 88mph joke> by [@kristof-mattei](https://github.com/kristof-mattei) ([`1c4c032`](https://github.com/kristof-mattei/autoheal-rs/commit/1c4c0321a161d6719796b0724a13eb754b2125c8))
- Renamed files by [@kristof-mattei](https://github.com/kristof-mattei) ([`e34981a`](https://github.com/kristof-mattei/autoheal-rs/commit/e34981a3a8506a261ad818f45ea726e3d2e20e7c))
- Debugging changelog printing issues by [@kristof-mattei](https://github.com/kristof-mattei) ([`cf9a3de`](https://github.com/kristof-mattei/autoheal-rs/commit/cf9a3de23bf33ea746cd56d0c50b15b3825aca52))
- Fix linebreaks by [@kristof-mattei](https://github.com/kristof-mattei) ([`25900e3`](https://github.com/kristof-mattei/autoheal-rs/commit/25900e3490df1800b89ed40ad727f64a74ef8252))
- Forgot dependency by [@kristof-mattei](https://github.com/kristof-mattei) ([`24064f8`](https://github.com/kristof-mattei/autoheal-rs/commit/24064f83d4cb48e7b908485d1a1c0104e6b07bc4))
- Fix space by [@kristof-mattei](https://github.com/kristof-mattei) ([`023334b`](https://github.com/kristof-mattei/autoheal-rs/commit/023334b405aabed437cbb00a6f9152f259f30830))
- Added final step by [@kristof-mattei](https://github.com/kristof-mattei) ([`ea08b74`](https://github.com/kristof-mattei/autoheal-rs/commit/ea08b74d52466ba5ae07af106737e036b27ce0b1))
- Update README.md by [@kristof-mattei](https://github.com/kristof-mattei) ([`f043d40`](https://github.com/kristof-mattei/autoheal-rs/commit/f043d40421a8e5836e2b18711e751965e67103ae))
- Updated name everywhere by [@kristof-mattei](https://github.com/kristof-mattei) ([`e0da1df`](https://github.com/kristof-mattei/autoheal-rs/commit/e0da1dfbaab56c5bfe15145abab04ddfbd05539c))
- Bumped rust numbers everywhere by [@kristof-mattei](https://github.com/kristof-mattei) ([`a0577b5`](https://github.com/kristof-mattei/autoheal-rs/commit/a0577b5c76c2c590f7ec371dab5669b648b31bf9))
- Fixed name by [@kristof-mattei](https://github.com/kristof-mattei) ([`eb13981`](https://github.com/kristof-mattei/autoheal-rs/commit/eb13981ac036fa28dfe7f8df5665d28ef071aab2))
- Consolidated npm usage by [@kristof-mattei](https://github.com/kristof-mattei) ([`146c18f`](https://github.com/kristof-mattei/autoheal-rs/commit/146c18fa3ddc6c63cd2fa1e32a9cfefaf346ffa2))
- Cleanup un-used script by [@kristof-mattei](https://github.com/kristof-mattei) ([`e8ab324`](https://github.com/kristof-mattei/autoheal-rs/commit/e8ab3245bc3ac74a7bd96844047c68ee789f18b4))
- Foundation for correct version numbers by [@kristof-mattei](https://github.com/kristof-mattei) ([`82fe6c5`](https://github.com/kristof-mattei/autoheal-rs/commit/82fe6c562db8fcc81eed9ff614346e1dcb6ca8e3))
- Set version number of Rust binary at build time by [@kristof-mattei](https://github.com/kristof-mattei) ([`5746431`](https://github.com/kristof-mattei/autoheal-rs/commit/57464314eadd600e6ceb395041d8ba6f33c3afb5))
- Reduce unneeded builds by [@kristof-mattei](https://github.com/kristof-mattei) ([`ed49297`](https://github.com/kristof-mattei/autoheal-rs/commit/ed49297faf6c0300cbfe42ae5b4b5d7fcfbbf091))
- Add dry-run to make sure we don't publish preemptively by [@kristof-mattei](https://github.com/kristof-mattei) ([`5285cee`](https://github.com/kristof-mattei/autoheal-rs/commit/5285cee32977ef1ada378dc306c20b9c5f08ccb5))
- Ensure checkout so that we have a package-lock by [@kristof-mattei](https://github.com/kristof-mattei) ([`4eeb52e`](https://github.com/kristof-mattei/autoheal-rs/commit/4eeb52e02e4eb3c18f39edadbdafe4ed3b47b2fb))
- Remove spurious ) by [@kristof-mattei](https://github.com/kristof-mattei) ([`5a81c7e`](https://github.com/kristof-mattei/autoheal-rs/commit/5a81c7e34b0dbe620524c8399eb6a4f56083084c))
- I used the wrong script by [@kristof-mattei](https://github.com/kristof-mattei) ([`7dcc44d`](https://github.com/kristof-mattei/autoheal-rs/commit/7dcc44d2c43e506d4b436c1fdf43bd9cabf3da33))
- Flip aroud switches until we find the right combination by [@kristof-mattei](https://github.com/kristof-mattei) ([`1c225f6`](https://github.com/kristof-mattei/autoheal-rs/commit/1c225f6c17d71594ab8fbb751d5de52df0d821d1))
- Hack the semantic-release by [@kristof-mattei](https://github.com/kristof-mattei) ([`65bd2eb`](https://github.com/kristof-mattei/autoheal-rs/commit/65bd2ebb3aeea525700fab234db4499581e11fa8))
- Semantic-release doesn't like refs/pulls/12/merge, surrounding with quotes to test by [@kristof-mattei](https://github.com/kristof-mattei) ([`c3acbf5`](https://github.com/kristof-mattei/autoheal-rs/commit/c3acbf55e23ecd55e4d280efdfc3bb1d7ccfa834))
- Just trying to get  this to work by [@kristof-mattei](https://github.com/kristof-mattei) ([`b5478dd`](https://github.com/kristof-mattei/autoheal-rs/commit/b5478dd32304377204c6d7b8960ac877f5af19f4))
- Can't go without --no-ci by [@kristof-mattei](https://github.com/kristof-mattei) ([`29fbe92`](https://github.com/kristof-mattei/autoheal-rs/commit/29fbe92afa714985de7f0c6fc8c779a8caaeca04))
- Try get last tag by [@kristof-mattei](https://github.com/kristof-mattei) ([`a37253e`](https://github.com/kristof-mattei/autoheal-rs/commit/a37253eb904612f60e0a6e4ea2a891cf7a76e051))
- Removed duplicate version id by [@kristof-mattei](https://github.com/kristof-mattei) ([`97ee09a`](https://github.com/kristof-mattei/autoheal-rs/commit/97ee09a29da6ccedb4591b9cd91d1520506cad58))
- Better way to check out the head? by [@kristof-mattei](https://github.com/kristof-mattei) ([`04c1587`](https://github.com/kristof-mattei/autoheal-rs/commit/04c1587aea423444d50b9b63ebd78a74b5ec895c))
- Work around semantic-release restrictions by [@kristof-mattei](https://github.com/kristof-mattei) ([`63a6315`](https://github.com/kristof-mattei/autoheal-rs/commit/63a63159018ab9b7806dd669768fcfdc06f6c6b1))
- Hack around semantic-release some more by [@kristof-mattei](https://github.com/kristof-mattei) ([`3bd1a9c`](https://github.com/kristof-mattei/autoheal-rs/commit/3bd1a9c0f70f33abdc0f51b68ddb6b02cfa298af))
- Wrong script name by [@kristof-mattei](https://github.com/kristof-mattei) ([`517cb82`](https://github.com/kristof-mattei/autoheal-rs/commit/517cb822538623679fcd57d5aaba79317038ee97))
- Skip tag, we don't want to give this one rights to push by [@kristof-mattei](https://github.com/kristof-mattei) ([`278546c`](https://github.com/kristof-mattei/autoheal-rs/commit/278546c990fc0937d60403b661ac555cee8a966d))
- Give more permissions for dry-run by [@kristof-mattei](https://github.com/kristof-mattei) ([`1d5cbf0`](https://github.com/kristof-mattei/autoheal-rs/commit/1d5cbf0ba44ead2afaa0e94bf967155758e8577d))
- Testing new flow by [@kristof-mattei](https://github.com/kristof-mattei) ([`8d9c25a`](https://github.com/kristof-mattei/autoheal-rs/commit/8d9c25a2ff7b97357fd5f385eaf5cf801995d2f4))
- Fix publish script by [@kristof-mattei](https://github.com/kristof-mattei) ([`68bb738`](https://github.com/kristof-mattei/autoheal-rs/commit/68bb738d6d2f86e71f91588e1c5fe2f4ccc45a62))
- Check for changes by [@kristof-mattei](https://github.com/kristof-mattei) ([`39f071c`](https://github.com/kristof-mattei/autoheal-rs/commit/39f071cd709181ee79f9e42869332af34a142da1))
- Restructure, remove unneeded submodule pull by [@kristof-mattei](https://github.com/kristof-mattei) ([`cfebd29`](https://github.com/kristof-mattei/autoheal-rs/commit/cfebd2921b7e87dec2003216c88e2e0efb58dcd7))
- Fix condition by [@kristof-mattei](https://github.com/kristof-mattei) ([`02df130`](https://github.com/kristof-mattei/autoheal-rs/commit/02df1304baa8a2b096b8e0f29b93eaf6faa552e6))
- Split steps by [@kristof-mattei](https://github.com/kristof-mattei) ([`95bf3b6`](https://github.com/kristof-mattei/autoheal-rs/commit/95bf3b6a5c85f11cfb08c9594369d58bd7a9188c))
- Reduced complexity of filter by [@kristof-mattei](https://github.com/kristof-mattei) ([`20f398c`](https://github.com/kristof-mattei/autoheal-rs/commit/20f398c8adbd47f93754d42298ca2a59c808bf6b))
- Beautified titles by [@kristof-mattei](https://github.com/kristof-mattei) ([`00b5d37`](https://github.com/kristof-mattei/autoheal-rs/commit/00b5d37f9dde952c90c8684fe7d2dec900d1e341))
- Updated task name by [@kristof-mattei](https://github.com/kristof-mattei) ([`10882f2`](https://github.com/kristof-mattei/autoheal-rs/commit/10882f2f181ae6d5e441dc8327c37ac729671261))
- Prevent clippy from running twice by [@kristof-mattei](https://github.com/kristof-mattei) ([`5db45b9`](https://github.com/kristof-mattei/autoheal-rs/commit/5db45b9964d66de7d9d5377748158bdd624b5bbf))
- Aligned commandline parameters by [@kristof-mattei](https://github.com/kristof-mattei) ([`fd9d518`](https://github.com/kristof-mattei/autoheal-rs/commit/fd9d51865bd14d729d49d15a6de25fb53a42c869))
- Expanded clippy warnings by [@kristof-mattei](https://github.com/kristof-mattei) ([`01272f6`](https://github.com/kristof-mattei/autoheal-rs/commit/01272f622b1a588f3fb064c82203222da3f29252))
- Also run pedantic and cargo test on push by [@kristof-mattei](https://github.com/kristof-mattei) ([`23ba6db`](https://github.com/kristof-mattei/autoheal-rs/commit/23ba6db09d220e17608aa43ab4ffe903507f2ada))
- Also run test-and-report on main to update 'main' coverage by [@kristof-mattei](https://github.com/kristof-mattei) ([`07cab02`](https://github.com/kristof-mattei/autoheal-rs/commit/07cab02c61478f168aa0fc7984a2dc5e79265ef1))
- Also run clippy when merged to main to track progress by [@kristof-mattei](https://github.com/kristof-mattei) ([`bccc597`](https://github.com/kristof-mattei/autoheal-rs/commit/bccc59739a8d274a06f093a2b625b09195d03ce0))
- Create LICENSE by [@kristof-mattei](https://github.com/kristof-mattei) ([`9662fd0`](https://github.com/kristof-mattei/autoheal-rs/commit/9662fd0bc59b52956cd6b30e26e6c5d4854ead9a))
- Respect cargo.lock when doing cargo install by [@kristof-mattei](https://github.com/kristof-mattei) ([`20b7f38`](https://github.com/kristof-mattei/autoheal-rs/commit/20b7f38541eb2641b99ede169a18b4323deae8b2))
- Fixed the name by [@kristof-mattei](https://github.com/kristof-mattei) ([`7166635`](https://github.com/kristof-mattei/autoheal-rs/commit/716663523d3c1e3d10b86a6b477298fa70196312))
- Remove outdated comment by [@kristof-mattei](https://github.com/kristof-mattei) ([`88369f7`](https://github.com/kristof-mattei/autoheal-rs/commit/88369f7ade36328e1b5156841a8127358c4a69a3))
- Switch to cog by [@kristof-mattei](https://github.com/kristof-mattei) ([`d9dc537`](https://github.com/kristof-mattei/autoheal-rs/commit/d9dc537936181daea34d14a6958064ff9c507eea))
- Switch to cog wip by [@kristof-mattei](https://github.com/kristof-mattei) ([`70bc0fe`](https://github.com/kristof-mattei/autoheal-rs/commit/70bc0fee3bec0b60d2eabd27999c80b43018bd92))
- Use registry cache by [@kristof-mattei](https://github.com/kristof-mattei) ([`6e66668`](https://github.com/kristof-mattei/autoheal-rs/commit/6e66668788e41a33e2efa36edf74297e83e58a13))
- Typo, docker -> Docker by [@kristof-mattei](https://github.com/kristof-mattei) ([`8571cd7`](https://github.com/kristof-mattei/autoheal-rs/commit/8571cd7fd8d6153eb75bef254aaa2e7ae8a1dc0f))
- Use built-in rustup by [@kristof-mattei](https://github.com/kristof-mattei) ([`e3a791a`](https://github.com/kristof-mattei/autoheal-rs/commit/e3a791a527a9f0e5b8a7696dd310d08f394ca4ce))
- Set the checkout name by [@kristof-mattei](https://github.com/kristof-mattei) ([`a8a45b4`](https://github.com/kristof-mattei/autoheal-rs/commit/a8a45b4f6ff2ef4783a60d0ddf99a519069e9f0a))
- Fixed rustup update warning about rustfmt and cargo-fmt by [@kristof-mattei](https://github.com/kristof-mattei) ([`bb187f3`](https://github.com/kristof-mattei/autoheal-rs/commit/bb187f384fb1042f363b9d756afc7aa64f5c852b))
- No need to run clippy on push to main by [@kristof-mattei](https://github.com/kristof-mattei) ([`44890f7`](https://github.com/kristof-mattei/autoheal-rs/commit/44890f764e814f9d99b92f43fd720cb2ab3998f3))
- Updated generated cache name by [@kristof-mattei](https://github.com/kristof-mattei) ([`4aa7084`](https://github.com/kristof-mattei/autoheal-rs/commit/4aa7084bddacb70518081f6a5872d4896f3bfd81))
- Removed todo that's not gonna happen by [@kristof-mattei](https://github.com/kristof-mattei) ([`5205794`](https://github.com/kristof-mattei/autoheal-rs/commit/5205794394c05e1f65333e4936def18162a8db0d))
- Set nice name by [@kristof-mattei](https://github.com/kristof-mattei) ([`22f0bca`](https://github.com/kristof-mattei/autoheal-rs/commit/22f0bcaf26b62359c2afd18cfad6fb280baf3d85))
- Removed unneeded comments by [@kristof-mattei](https://github.com/kristof-mattei) ([`dca69dc`](https://github.com/kristof-mattei/autoheal-rs/commit/dca69dc3ea2cbacb0a490e011c8c5a385ebf7c51))
- Exclude rustfmt and cargo-fmt from the cache as rustup doesn't like that by [@kristof-mattei](https://github.com/kristof-mattei) ([`d95f77f`](https://github.com/kristof-mattei/autoheal-rs/commit/d95f77ffd16ffa56db736e564928aa4726a7c224))
- Exclude rustfmt and cargo-fmt from the cache as rustup doesn't like that by [@kristof-mattei](https://github.com/kristof-mattei) ([`2c04592`](https://github.com/kristof-mattei/autoheal-rs/commit/2c04592da28a24dc92b3f1025345654ed51d84cb))
- Run prettier by [@kristof-mattei](https://github.com/kristof-mattei) ([`46ff0fb`](https://github.com/kristof-mattei/autoheal-rs/commit/46ff0fb4c6b3f3509f42798b1b9adbd59617f0b4))
- Don't consider it 'all-done' when anything is cancelled by [@kristof-mattei](https://github.com/kristof-mattei) ([`4fdc19d`](https://github.com/kristof-mattei/autoheal-rs/commit/4fdc19df0990801ebaf40f25fa955eb22f3f1946))
- Also rebuild on cargo.lock changes by [@kristof-mattei](https://github.com/kristof-mattei) ([`b196ad4`](https://github.com/kristof-mattei/autoheal-rs/commit/b196ad45b3b648f7588091e083df970f896b8185))
- Updated npm packages by [@kristof-mattei](https://github.com/kristof-mattei) ([`79e5129`](https://github.com/kristof-mattei/autoheal-rs/commit/79e5129e640c62201cc72d4807e3ff44a8ad2033))
- Also bump rust-toolchain to rust 1.58.1 by [@kristof-mattei](https://github.com/kristof-mattei) ([`1f91934`](https://github.com/kristof-mattei/autoheal-rs/commit/1f919341edc6d1538f96ae02ec832424ea0a9395))
- Cleaned up rustfmt, added 2 settings by [@kristof-mattei](https://github.com/kristof-mattei) ([`b484519`](https://github.com/kristof-mattei/autoheal-rs/commit/b484519b041b4336e93bbc556d202f70f68bf73e))
- Set test comment mode by [@kristof-mattei](https://github.com/kristof-mattei) ([`137affd`](https://github.com/kristof-mattei/autoheal-rs/commit/137affd7ddec571febf32f235fa26ac8702da2eb))
- Formatting! by [@kristof-mattei](https://github.com/kristof-mattei) ([`f2ab063`](https://github.com/kristof-mattei/autoheal-rs/commit/f2ab06359a1a980404bf2aebd013e43aef804662))
- Don't create new comment, recycle! by [@kristof-mattei](https://github.com/kristof-mattei) ([`c3a7cb4`](https://github.com/kristof-mattei/autoheal-rs/commit/c3a7cb4dd9fa46c893969e84e42c5b673e7adb8b))
- Fixed title by [@kristof-mattei](https://github.com/kristof-mattei) ([`14bdc49`](https://github.com/kristof-mattei/autoheal-rs/commit/14bdc49a2c8135972be93fef36a425bd9b094c80))
- Correctly report test failure by [@kristof-mattei](https://github.com/kristof-mattei) ([`6250142`](https://github.com/kristof-mattei/autoheal-rs/commit/6250142008a5f74fad09c6088fa64ec87d5c49a5))
- Also update cargo & toolchain by [@kristof-mattei](https://github.com/kristof-mattei) ([`291be2a`](https://github.com/kristof-mattei/autoheal-rs/commit/291be2aa3dd1d15c726686b0dd71ff2e35bff341))
- Fail done properly by [@kristof-mattei](https://github.com/kristof-mattei) ([`9a420bf`](https://github.com/kristof-mattei/autoheal-rs/commit/9a420bf23ae315b64a7a0d20734de30db6ac01df))
- Spacing and remove verbose by [@kristof-mattei](https://github.com/kristof-mattei) ([`abceeeb`](https://github.com/kristof-mattei/autoheal-rs/commit/abceeeb0e023cd1629df3540fe5f9d310b8dbd36))
- Consolidated extensions by [@kristof-mattei](https://github.com/kristof-mattei) ([`7ebc90c`](https://github.com/kristof-mattei/autoheal-rs/commit/7ebc90c506e2570c0ec92145c4b4a6d23270c2cf))
- Add title (name) to step by [@kristof-mattei](https://github.com/kristof-mattei) ([`8f4396d`](https://github.com/kristof-mattei/autoheal-rs/commit/8f4396d247c865897e0088c6af23a54a0affef7c))
- Docker images names should always be lowercase by [@kristof-mattei](https://github.com/kristof-mattei) ([`9932b72`](https://github.com/kristof-mattei/autoheal-rs/commit/9932b72ad08d8a5eeacc0946b1fc367320c9e19f))
- Add shell name and consolidated format by [@kristof-mattei](https://github.com/kristof-mattei) ([`00ca5d7`](https://github.com/kristof-mattei/autoheal-rs/commit/00ca5d7c67d31eed62556cdcf12b0a77318a0be4))
- Delete unneeded file by [@kristof-mattei](https://github.com/kristof-mattei) ([`4734616`](https://github.com/kristof-mattei/autoheal-rs/commit/4734616fc8be237c5ee89130cd942fa971ad767f))
- Update packages by [@kristof-mattei](https://github.com/kristof-mattei) ([`c39ff7e`](https://github.com/kristof-mattei/autoheal-rs/commit/c39ff7e8cc8032f91fa8d765d600a1acce21e3de))
- Clean up semgrep file, update package-lock by [@kristof-mattei](https://github.com/kristof-mattei) ([`df7e1e4`](https://github.com/kristof-mattei/autoheal-rs/commit/df7e1e4d3942aa0f3b703de1c14005093ef41dec))
- Crlf to lf by [@kristof-mattei](https://github.com/kristof-mattei) ([`fdca795`](https://github.com/kristof-mattei/autoheal-rs/commit/fdca795c627dbed981e6ed02fe4287c37b29d169))
- Install latest semgrep by [@kristof-mattei](https://github.com/kristof-mattei) ([`b720d30`](https://github.com/kristof-mattei/autoheal-rs/commit/b720d30449d174b74e1ed3fa184d4c1e9e69f255))
- Use token to get more rules by [@kristof-mattei](https://github.com/kristof-mattei) ([`342529b`](https://github.com/kristof-mattei/autoheal-rs/commit/342529ba015edca81583186343b7d93949f5ac76))
- Try to speed up by using binstall by [@kristof-mattei](https://github.com/kristof-mattei) ([`ddf6fc7`](https://github.com/kristof-mattei/autoheal-rs/commit/ddf6fc7bb52110b4da4d67c3e380ed915442d848))
- And more 1.62 by [@kristof-mattei](https://github.com/kristof-mattei) ([`f048ac0`](https://github.com/kristof-mattei/autoheal-rs/commit/f048ac05a5aaed0f6a39ae632d7595c352acfaf1))
- Fix the binary names by [@kristof-mattei](https://github.com/kristof-mattei) ([`e700aaa`](https://github.com/kristof-mattei/autoheal-rs/commit/e700aaaae4cd095ab196892fb19e86cac143b8bf))
- Spacing, made lint-commits also use binstall by [@kristof-mattei](https://github.com/kristof-mattei) ([`c4daf59`](https://github.com/kristof-mattei/autoheal-rs/commit/c4daf591677669b2603f25019f5176dc850bbbe8))
- Updated file property to junit_files as per https://github.com/EnricoMi/publish-unit-test-result-action/pull/285 by [@kristof-mattei](https://github.com/kristof-mattei) ([`73a0ad7`](https://github.com/kristof-mattei/autoheal-rs/commit/73a0ad75fdb21be7081eb628fc2d8da728355629))
- Remove duplicate `USER`, not needed by [@kristof-mattei](https://github.com/kristof-mattei) ([`2bc87f5`](https://github.com/kristof-mattei/autoheal-rs/commit/2bc87f5cdc946847eef2f215f9316a45ec840f3b))
- Enable renovate by [@kristof-mattei](https://github.com/kristof-mattei) ([`741b4bd`](https://github.com/kristof-mattei/autoheal-rs/commit/741b4bd48ce77da72268caddf460cba3fe7ea75e))
- Explicitly set token to avoid failed uploads by [@kristof-mattei](https://github.com/kristof-mattei) ([`45e6bcf`](https://github.com/kristof-mattei/autoheal-rs/commit/45e6bcfbb31f1e1219db97bf65da1c33fdc64961))
- Disable coveralls, it's acting up by [@kristof-mattei](https://github.com/kristof-mattei) ([`e8ff3d3`](https://github.com/kristof-mattei/autoheal-rs/commit/e8ff3d3eca52fc746ef5ad8120071d9619f0b17e))
- Also include rust-specific configs by [@kristof-mattei](https://github.com/kristof-mattei) ([`0888cb2`](https://github.com/kristof-mattei/autoheal-rs/commit/0888cb2abb68acd808d532186220ebabe986de36))
- Fixed double update typo by [@kristof-mattei](https://github.com/kristof-mattei) ([`bc15a92`](https://github.com/kristof-mattei/autoheal-rs/commit/bc15a92e9c4eb0910f44a9368bbae281c5a4eb91))
- Testing renovate's custom file updater by [@kristof-mattei](https://github.com/kristof-mattei) ([`7918b9c`](https://github.com/kristof-mattei/autoheal-rs/commit/7918b9c204a40bd5b5ce19c8d3ad6abb61f2e148))
- Restored codecov, remove executable modifier by [@kristof-mattei](https://github.com/kristof-mattei) ([`b45eb0f`](https://github.com/kristof-mattei/autoheal-rs/commit/b45eb0f353f1928990d23fd54996ed97b70e1c61))
- Disable coveralls & codecov by [@kristof-mattei](https://github.com/kristof-mattei) ([`c43c49a`](https://github.com/kristof-mattei/autoheal-rs/commit/c43c49a500d053d5c1d2a5861a8ffa8d44281093))
- Put versions so that renovate can tag correctly by [@kristof-mattei](https://github.com/kristof-mattei) ([`dce5fd5`](https://github.com/kristof-mattei/autoheal-rs/commit/dce5fd57af8aa1a1a7722f4bfedc42e7be29a306))
- Update from upstream util by [@kristof-mattei](https://github.com/kristof-mattei) ([`a5298cd`](https://github.com/kristof-mattei/autoheal-rs/commit/a5298cd749e13e1959c2a00eee2947de7e512aaa))
- Try codecov again by [@kristof-mattei](https://github.com/kristof-mattei) ([`f53b8db`](https://github.com/kristof-mattei/autoheal-rs/commit/f53b8dbce2882cfdbff917b4ee2261bc7fa47ad2))
<!-- generated by git-cliff -->
