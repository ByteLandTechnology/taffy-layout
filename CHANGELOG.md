## [1.1.0](https://github.com/ByteLandTechnology/taffy-layout/compare/v1.0.3...v1.1.0) (2026-01-22)

### ✨ Features

- **style:** add batch get and set methods ([25f6dc4](https://github.com/ByteLandTechnology/taffy-layout/commit/25f6dc40db6dbeff84ae4b09b14dce68b7af5c26))
- **style:** add individual property getters/setters and flatten property paths ([1e4c648](https://github.com/ByteLandTechnology/taffy-layout/commit/1e4c6480439da4a9565a7bc96b56bd02b9ef222d))

## [1.0.3](https://github.com/ByteLandTechnology/taffy-layout/compare/v1.0.2...v1.0.3) (2026-01-21)

### 📚 Documentation

- add Taffy Layout logo and update READMEs ([a12bca1](https://github.com/ByteLandTechnology/taffy-layout/commit/a12bca12e65ffcaf46932ba7c9f139fdf0bd4458))

### 🔧 Chores

- add MIT LICENSE file ([95dbb9d](https://github.com/ByteLandTechnology/taffy-layout/commit/95dbb9d5f1d31e59ef09320696b3c1d0c82b2427))
- update wasm-pack to ^0.14.0 ([eb41a97](https://github.com/ByteLandTechnology/taffy-layout/commit/eb41a9728a355cd70f25edc54eb7b980b586dad7))

## [1.0.2](https://github.com/ByteLandTechnology/taffy-layout/compare/v1.0.1...v1.0.2) (2026-01-20)

### 📚 Documentation

- add localized READMEs ([2422e71](https://github.com/ByteLandTechnology/taffy-layout/commit/2422e718e1d415f89c82efa0cbfae6aa64fe90cc))

## [1.0.1](https://github.com/ByteLandTechnology/taffy-layout/compare/v1.0.0...v1.0.1) (2026-01-20)

### 🐛 Bug Fixes

- ignore node modules in webpack build ([9b0e70b](https://github.com/ByteLandTechnology/taffy-layout/commit/9b0e70b12042c5f585d609495189033a119ca231))

### ♻️ Refactoring

- use try-import pattern for isomorphic wasm loading ([510f3bf](https://github.com/ByteLandTechnology/taffy-layout/commit/510f3bf0d7809004b139dfff12cf616527c1d675))

## 1.0.0 (2026-01-20)

### ✨ Features

- add NodeContext, epoch tracking, Edge/Gutter enums, and debug logging ([7d85171](https://github.com/ByteLandTechnology/taffy-layout/commit/7d8517192bc9c0bd042cfd74e051b4a459536027))
- bind Rust Layout object to JS ([4f8e734](https://github.com/ByteLandTechnology/taffy-layout/commit/4f8e73493a59f0a3b405ebac8b64ed561f8347fb))
- comprehensive testing, API improvements, and documentation refactoring ([a18301f](https://github.com/ByteLandTechnology/taffy-layout/commit/a18301fc2155badd69f6d847d990347c569c4fe6))
- expose BoxSizing and update documentation ([be47c3e](https://github.com/ByteLandTechnology/taffy-layout/commit/be47c3e108ab8c578bbd0d33ced42ba4b89c9217))
- refactor source into modules and setup documentation generation ([8c285dc](https://github.com/ByteLandTechnology/taffy-layout/commit/8c285dccf1695e108dcd649a852f47f131a5c2a6))
- **style:** add Grid, Text and Table properties ([8b8619c](https://github.com/ByteLandTechnology/taffy-layout/commit/8b8619c8649ed2c7797d04fe0061683d3ec13598))
- update WASM bindings, add README and tsconfig, bump version to 0.1.2 ([99bcec3](https://github.com/ByteLandTechnology/taffy-layout/commit/99bcec38a4b6f01f9591e3d11037b22eb67fef14))

### 🐛 Bug Fixes

- correct percentage value conversion (0-100 range) ([c59f6ab](https://github.com/ByteLandTechnology/taffy-layout/commit/c59f6abb8f82ba419901760b4ba4f9cc71d23e89))
- include pkg directory in npm publish ([a75dc8d](https://github.com/ByteLandTechnology/taffy-layout/commit/a75dc8defd098e6eb993256b02a3494ca1597239))
- refactor AvailableSpace handling to use camelCase ([1cdfb7f](https://github.com/ByteLandTechnology/taffy-layout/commit/1cdfb7fe4c0c15fb8e8e8d0b35809698689f0a42))

### ♻️ Refactoring

- restructure lib.rs interface and update documentation ([12d0611](https://github.com/ByteLandTechnology/taffy-layout/commit/12d061183f01b5021565094cd33217d5cff34e97))
- split into taffy-wasm and taffy-js packages ([1dd4d06](https://github.com/ByteLandTechnology/taffy-layout/commit/1dd4d06046c37d085f98a19bf14f3b3b1650780f))
- update MeasureFunction knownDimensions type to use undefined ([ded4eea](https://github.com/ByteLandTechnology/taffy-layout/commit/ded4eea6d64e2b2ffa23877b0f5856441b0ac443))

### 📚 Documentation

- fix compute_layout_with_measure documentation to accurately describe behavior ([b1676d7](https://github.com/ByteLandTechnology/taffy-layout/commit/b1676d79856a5af3994d03b2dae0c4dfa4553307))
- refactor code comments to TSDoc format and update generated documentation ([1fff692](https://github.com/ByteLandTechnology/taffy-layout/commit/1fff6925b13c71a6dd8233a0a633ef61a9a64fa7))

### 👷 CI/CD

- add release workflow ([e493d56](https://github.com/ByteLandTechnology/taffy-layout/commit/e493d56095106d01997e3fd61cf9a444ebb7140b))

### 🔧 Chores

- add husky for commit hooks ([64bbb17](https://github.com/ByteLandTechnology/taffy-layout/commit/64bbb17d4f2894753b3e6f0a39c27e245d3a8535))
- bump version to 0.2.10 (wasm 0.9.8) ([9ee7b0c](https://github.com/ByteLandTechnology/taffy-layout/commit/9ee7b0c9980620c9257bb5a3def26ccabfb8b524))
- bump version to 0.2.11 (wasm 0.9.9) ([6ceedde](https://github.com/ByteLandTechnology/taffy-layout/commit/6ceedde6d7154f21e6bd04bb541239f5f8a33132))
- bump version to 0.2.12 (wasm 0.9.10) ([efbe395](https://github.com/ByteLandTechnology/taffy-layout/commit/efbe3953611e167200a382549a1d9c8b1057c7cc))
- bump version to 0.2.7 (wasm 0.9.5) ([f452879](https://github.com/ByteLandTechnology/taffy-layout/commit/f45287954fe645b7d72c6175d40312bcaf4751bf))
- bump version to 0.2.8 (wasm 0.9.6) ([86b26c0](https://github.com/ByteLandTechnology/taffy-layout/commit/86b26c0f6da9cf0fc0a4eceabf074a64b8eeb062))
- bump version to 0.2.9 (wasm 0.9.7) ([4acdbbb](https://github.com/ByteLandTechnology/taffy-layout/commit/4acdbbbccc162e407834a4502396014613d0093a))
- initialize project ([2840a43](https://github.com/ByteLandTechnology/taffy-layout/commit/2840a43840305e5367c4adf3ce382a9d74ab99b1))
- install lint-staged and prettier ([76605be](https://github.com/ByteLandTechnology/taffy-layout/commit/76605be964c94f002474c1fa232103297cf83de3))
- refactor bindings, fix docs and add tests ([dc9c5ac](https://github.com/ByteLandTechnology/taffy-layout/commit/dc9c5aca4c6384340aa64949dadb1e6b575a7fff))
- **release:** bump version to 0.1.3 ([20aa4da](https://github.com/ByteLandTechnology/taffy-layout/commit/20aa4da68f193d04171421efdbd6c2547d695d9a))
- **release:** bump version to 0.2.1 ([688de6d](https://github.com/ByteLandTechnology/taffy-layout/commit/688de6dc77bedb14b05ae52f3f0cdb585063e4b7))
- **release:** bump version to 0.2.2 ([746255f](https://github.com/ByteLandTechnology/taffy-layout/commit/746255fffb09574cea5d4ca4cd839e5420e71e5f))
- **release:** bump version to 0.2.3 ([ec4cc2f](https://github.com/ByteLandTechnology/taffy-layout/commit/ec4cc2f5d7c36fe464a2099d139906431e00752a))
- **release:** bump version to 0.2.4 ([739a371](https://github.com/ByteLandTechnology/taffy-layout/commit/739a371d59da37b0a433c2f0b948ba4caf631175))
- rename project to taffy-layout and bump version to 0.1.0 ([207f509](https://github.com/ByteLandTechnology/taffy-layout/commit/207f5098e8b8a975e28bcf5644cc7bdafd902ac7))
- setup cargo project ([3e7bab6](https://github.com/ByteLandTechnology/taffy-layout/commit/3e7bab6b49be2eea6f49e110d7f72461879d4735))
- setup commitlint ([c4d04e7](https://github.com/ByteLandTechnology/taffy-layout/commit/c4d04e7977672636e8aff625f83bc85da1d621f3))
- setup semantic-release and disable package-lock ([272b401](https://github.com/ByteLandTechnology/taffy-layout/commit/272b401276b001ea30381f081c0ad4ccc296ccfa))
