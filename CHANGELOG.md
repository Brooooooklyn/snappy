# [7.3.0](https://github.com/Brooooooklyn/snappy/compare/v7.2.2...v7.3.0) (2025-07-15)


### Bug Fixes

* **deps:** update rust crate napi to 3.0.0-alpha ([#198](https://github.com/Brooooooklyn/snappy/issues/198)) ([9f8c9b9](https://github.com/Brooooooklyn/snappy/commit/9f8c9b9acf8bc05c79b07d4db0fb05e699c423a1))
* **deps:** update rust crate napi-derive to 3.0.0-alpha ([#197](https://github.com/Brooooooklyn/snappy/issues/197)) ([41125e2](https://github.com/Brooooooklyn/snappy/commit/41125e23b0f621ce1ebc0b956b366c660bba151e))


### Features

* add more targets ([#265](https://github.com/Brooooooklyn/snappy/issues/265)) ([c0a3244](https://github.com/Brooooooklyn/snappy/commit/c0a3244488975cfe6e41466386d0658982fef5d7))



## [7.2.2](https://github.com/Brooooooklyn/snappy/compare/v7.2.1...v7.2.2) (2022-11-13)

### Bug Fixes

- unref buffer when compress/uncompress synchronously faild ([#105](https://github.com/Brooooooklyn/snappy/issues/105)) ([8cfc521](https://github.com/Brooooooklyn/snappy/commit/8cfc5215763b5f49d454aec342cb6602b4220de4))

## [7.2.1](https://github.com/Brooooooklyn/snappy/compare/v7.2.0...v7.2.1) (2022-10-23)

### Bug Fixes

- memory leak of sync api ([#100](https://github.com/Brooooooklyn/snappy/issues/100)) ([aaa1950](https://github.com/Brooooooklyn/snappy/commit/aaa19500a35582b633d956a32beadaaa9a769c14))

# [7.2.0](https://github.com/Brooooooklyn/snappy/compare/v7.1.2...v7.2.0) (2022-10-05)

### Features

- provide copyOutputData to compatible with electron >= 21 ([772abc9](https://github.com/Brooooooklyn/snappy/commit/772abc94264c212ea3abed5bea27891f0821f175))

## [7.1.2](https://github.com/Brooooooklyn/snappy/compare/v7.1.1...v7.1.2) (2022-08-09)

### Bug Fixes

- compatible issues with centos:7 and void Linux ([dbd61db](https://github.com/Brooooooklyn/snappy/commit/dbd61db3de57921bf6c6f6acd809093ba417ea26))

## [7.1.1](https://github.com/Brooooooklyn/snappy/compare/v7.1.0...v7.1.1) (2021-12-22)

### Bug Fixes

- override the package name in generated index.js ([42346fd](https://github.com/Brooooooklyn/snappy/commit/42346fdd78ad4aa91e65a5d0cd176ea716459f72))

# [7.1.0](https://github.com/Brooooooklyn/snappy/compare/v7.0.5...v7.1.0) (2021-12-22)

### Features

- upgrade to napi2 ([e6cc543](https://github.com/Brooooooklyn/snappy/commit/e6cc5433eb503987d7e6f09f4346c7d317a3fccf))

## [7.0.5](https://github.com/Brooooooklyn/snappy/compare/v7.0.4...v7.0.5) (2021-11-06)

### Bug Fixes

- **decompress:** cast input to buffer ([9bdf8f3](https://github.com/Brooooooklyn/snappy/commit/9bdf8f39d4a6792798a3641b6a5b4d2d5dfe6b45)), closes [#38](https://github.com/Brooooooklyn/snappy/issues/38)

## [7.0.4](https://github.com/Brooooooklyn/snappy/compare/v7.0.3...v7.0.4) (2021-10-28)

### Bug Fixes

- avoid copy input buffer ([00cdd39](https://github.com/Brooooooklyn/snappy/commit/00cdd39a9576567620216ad01c8d063bac32ac77))

## [7.0.3](https://github.com/Brooooooklyn/snappy/compare/v7.0.2...v7.0.3) (2021-08-27)

### Bug Fixes

- remove Ref usage to avoid memory leak ([c89bb2e](https://github.com/Brooooooklyn/snappy/commit/c89bb2e278a1f9ad3a2e14c790e069bc699fe492))

## [7.0.2](https://github.com/Brooooooklyn/snappy/compare/v7.0.1...v7.0.2) (2021-08-11)

### Bug Fixes

- missing asBuffer option in uncompress/uncompressSync ([ac573f8](https://github.com/Brooooooklyn/snappy/commit/ac573f8523abd7cd3642eb8557fc51f43acbc34c))

## [7.0.1](https://github.com/Brooooooklyn/snappy/compare/v7.0.0...v7.0.1) (2021-08-04)

### Bug Fixes

- native binding package name ([6dc09af](https://github.com/Brooooooklyn/snappy/commit/6dc09af844700875dd1594fb70ec3b3af37de78b))

# [7.0.0](https://github.com/Brooooooklyn/snappy/compare/v1.0.2...v7.0.0) (2021-08-04)

Change package name to `snappy` [#16](https://github.com/Brooooooklyn/snappy/issues/16) .

## [1.0.2](https://github.com/Brooooooklyn/snappy/compare/v1.0.1...v1.0.2) (2021-07-22)

### Bug Fixes

- linux aarch64 musl build ([1a9a475](https://github.com/Brooooooklyn/snappy/commit/1a9a475c2aef170abfd9e1e4d8eeb4d955384fa0))

## [1.0.1](https://github.com/Brooooooklyn/snappy/compare/v1.0.0...v1.0.1) (2021-06-10)

### Performance Improvements

- mimalloc as global allocator ([3fbab59](https://github.com/Brooooooklyn/snappy/commit/3fbab59ba2c095bb1b2a819eb3445ca06fc743c4))

# [1.0.0](https://github.com/Brooooooklyn/snappy/compare/df2ccd289ca2418504aff3a8fd65cc75c34ce6d8...v1.0.0) (2021-06-10)

### Features

- implement compress and uncompress ([df2ccd2](https://github.com/Brooooooklyn/snappy/commit/df2ccd289ca2418504aff3a8fd65cc75c34ce6d8))
