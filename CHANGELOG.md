* fix: convolution test deadcode (#177) [Mizar / みざー] 77d7849
* fix `Dsu::leader` to avoid multiple assert checks (#175) [Not_Leonian] 8935673
* Add assertation in `lcp_array_arbitrary` (#173) [Not_Leonian] eb19cd5
* Update KaTeX to version 0.16.22 (#169) [Mizar / みざー] 3085eda
* Fix import paths in examples for `convolution` module (#168) [Mizar / みざー] 032ef62
* Add document for `convolution` (#76) [Ryo Yamashita] bb05f6d
* Implement Clone for Segtree (#167) [Not_Leonian] 181c1a0
* Implement Clone for LazySegtree (#166) [Not_Leonian] b59764c
* Implement Clone for FenwickTree (#165) [Not_Leonian] 85a6e90
* Implement Clone and Debug for MinCostFlowEdge (#163) [Not_Leonian] d03074b
* Implement Clone and Debug for MinCostFlowGraph and _Edge (#162) [Not_Leonian] 4f48685
* Implement Clone and Debug for SccGraph (#161) [Not_Leonian] 2bb238f
* Implement Clone for Edge (#160) [Not_Leonian] f7d76f7
* Implement Clone and Debug for MfGraph and _Edge (#159) [Not_Leonian] 997a4e3
* Implement Clone and Debug for TwoSat (#158) [Not_Leonian] 5be42f0
* Merge pull request #157 from NotLeonian/feature/add-clone-debug-internal-scc [Hiroki Kobayashi] eea0b2a
* Merge pull request #156 from Ryoga-exe/fix-scc-docs [Hiroki Kobayashi] a13e68f
* Implement Clone and Debug for internal_scc::SccGraph and Debug for _Edge [NotLeonian] 6741b1a
* fix: Correct `n` to refer to vertices, not edges in `SccGraph::new` [Ryoga-exe] 71aa3c0
* Implement Clone and Debug for Dsu (#154) [Not_Leonian] 89568c5
* Fix FromIterator for Segtree (#138) [mousecrusher2] cf82412
* Merge pull request #142 from mizar/modint1_fix [Hiroki Kobayashi] 663c8fb
* Corner cases of "modint" when mod = 1 [Mizar] d815ff0
* Merge pull request #141 from rust-lang-ja/fix/clippy-20250127 [Ryo Yamashita] cfc44a3
* MacOS uses aarch64 [koba-e964] e409510
* Upgrade Python [koba-e964] 3857fec
* Upgrade OSs on GitHub Actions [koba-e964] 2f9efee
* Fix clippy [koba-e964] b726b5b
* Merge pull request #133 from TonalidadeHidrica/dev/floor_sum [Hiroki Kobayashi] 42c1384
* Merge pull request #140 from Ryoga-exe/refactor/FlowCalculator [Hiroki Kobayashi] 1954461
* refactor: que.pop in FlowCalculator [Ryoga-exe] 3936994
* Merge pull request #137 from togatoga/patch-1 [Hiroki Kobayashi] 5323fca
* Replace 1.42 with 1.70.0 [togatoga] 4d85d29
* Update rust-toolchain to 1.70.0 [togatoga] cdf3d0d
* Merge pull request #136 from rust-lang-ja/fix/convolution-tests [Ryo Yamashita] a629ff0
* Fix tests in src/convolution.rs [koba-e964] f8df397
* Merge pull request #115 from mizar/segtree [TonalidadeHidrica] 33dc514
* segtree: get_slice [Mizar] 4d0dc0b
* segtree: FromIterator [Mizar] 764c814
* segtree: monoid BitwiseAnd/BitwiseOr/BitwiseXor [Mizar] a04c1ef
* Fix clippy [TonalidadeHidrica] 46caad4
* In floor_sum, use Wrapping<u64> to handle overflows [TonalidadeHidrica] ece6abe
* Update docs of floor_sum [TonalidadeHidrica] 7703e17
* Merge pull request #132 from rust-lang-ja/release-v0.1.1 [Ryo Yamashita] 71c3474
* Start next development iteration 0.2.0-alpha.1 [Ryo Yamashita] 05a5bd4
* Release v0.1.1 [Ryo Yamashita] 8b315c0
* Fix `package.metadata.docs.rs.rustdoc-args` [Ryo Yamashita] 66795f2
* Merge pull request #131 from qryxip/set-package-metadata-docs-rs-rustdoc-args [Ryo Yamashita] 9954619
* Set `package.metadata.docs.rs.rustdoc-args` [Ryo Yamashita] c8e336f
* Merge pull request #129 from rust-lang-ja/release-v0.1.0 [Ryo Yamashita] 67c1486
* Start next development iteration 0.2.0-alpha.1 [Ryo Yamashita] 87708b4
* Release v0.1.0 [Ryo Yamashita] a9f4342
* Merge pull request #126 from qryxip/rename-crate-name-to-ac-library [Ryo Yamashita] 7e7757e
* Merge pull request #130 from qryxip/set-rust-version [Ryo Yamashita] 7ae1881
* Merge pull request #128 from qryxip/update-readme [Ryo Yamashita] bec9cf2
* Merge pull request #127 from qryxip/rm-package-authors [Ryo Yamashita] 4365273
* Merge pull request #124 from qryxip/use-katex-for-modint [Ryo Yamashita] 69adb3e
* Set `rust-version` [Ryo Yamashita] 98c9fd7
* Update the readme [Ryo Yamashita] 52c535a
* Remove `package.authors` [Ryo Yamashita] a5a0daa
* Rename the crate name to `ac_library` [Ryo Yamashita] 4fd6c27
* Merge pull request #121 from TonalidadeHidrica/fix-crt-comment [Ryo Yamashita] 189de2b
* Merge pull request #101 from TonalidadeHidrica/range-bounds [Ryo Yamashita] 44b3805
* Use KaTeX for modint [Ryo Yamashita] f0b95d0
* Fix comment in math https://github.com/atcoder/ac-library/pull/97 [Mizar] bb14859
* Merge pull request #80 from qryxip/doc-for-scc [Ryo Yamashita] c48fcaa
* Merge branch 'master' into range-bounds [TonalidadeHidrica] 686372b
* Merge pull request #120 from mizar/ci_env [Ryo Yamashita] 6b867d6
* ci: ubuntu-20.04, macos-11 [Mizar] 21d042e
* Apply suggestions from code review [TonalidadeHidrica] b622a1b
* Merge pull request #55 from qryxip/remove-unnecessary-vis [TonalidadeHidrica] 7e99fd2
* Tentative solution for CI (Clippy error, deprecated warning) (#117) [Mizar / みざー] d2d8293
* Merge pull request #58 from qryxip/fix-elided-lifetimes-in-paths [Ryo Yamashita] c03e2ca
* Merge pull request #108 from mizar/expand_fix [Ryo Yamashita] 6a2e7f6
* Merge pull request #116 from mizar/katex [Ryo Yamashita] cbde486
* katex 0.16.4 [Mizar] 617c42d
* Workaround for Japanese Windows environment - Avoiding problems with incorrect encoding `UnicodeDecodeError: 'cp932' codec can't decode byte 0x85 in position 2897: illegal multibyte sequence` - Added option to output directly to file PowerShell converts encoding and newline characters when passing stdout [Mizar] 32b7b9c
* Merge pull request #113 from mizar/work_clippy [Ryo Yamashita] 0b92413
* string/maxflow: clippy - https://rust-lang.github.io/rust-clippy/master/index.html#bool_to_int_with_if - https://rust-lang.github.io/rust-clippy/master/index.html#implicit_saturating_sub [Mizar] a62e93b
* Merge pull request #109 from qryxip/rm-hand-made-input-macro [Ryo Yamashita] 307574c
* Remove the hand-made `input!` [Ryo Yamashita] 4a4ca4f
* Update examples due to API changes [TonalidadeHidrica] 293713a
* Accept RangeBounds for LazySegtree::apply_range [TonalidadeHidrica] 3fdff08
* Allow RangeBounds for fenwick and segtrees [TonalidadeHidrica] aa3beb7
* fmt: clippy [nebocco] ac9ab7e
* fmt: cargo clippy [nebocco] 4e6065b
* fix: relax the constraints of floor_sum [nebocco] 38624c9
* Merge pull request #97 from toast-uz/master [Hiroki Kobayashi] d2b35ac
* Merge pull request #78 from qryxip/doc-for-dsu [Ryo Yamashita] 16875d3
* Merge pull request #92 from qryxip/verify-with-oj [Ryo Yamashita] 330306f
* Fix compatibility with cargo-equip [toast-uz] 8e18784
* Fix a misspelling [Ryo Yamashita] 8f62704
* Merge pull request #81 from qryxip/doc-for-twosat [Hiroki Kobayashi] 4ed6203
* Fix document [Ryo Yamashita] 3ec9768
* Fix a grammar error [Ryo Yamashita] e5736c7
* Merge branch 'master' into doc-for-dsu [Ryo Yamashita] 3feb767
* Merge pull request #96 from koba-e964/fix/ci [Ryo Yamashita] 40329df
* Fix a misspelling [Ryo Yamashita] a4212d6
* Prefer assert! to assert_eq! for boolean values [koba-e964] 4481d96
* Merge pull request #86 from qryxip/badges [Hiroki Kobayashi] 08d5aa5
* Verify with problems from Library Checker [Ryo Yamashita] 6b3a9a0
* Add examples/library-checker-unionfind.rs [Ryo Yamashita] 3a1174f
* Add examples/library-checker-sum-of-floor-linear.rs [Ryo Yamashita] faf6ba0
* Add examples/library-checker-static-range-sum.rs [Ryo Yamashita] 98f4faa
* Add examples/library-checker-convolution-mod.rs [Ryo Yamashita] 58c7a9e
* Add `input!` implementation for the examples [Ryo Yamashita] b798acf
* Merge pull request #90 from MiSawa/fix/max-flow-return-condition [Ryo Yamashita] 72fe2a1
* Merge pull request #89 from MiSawa/fix/mcf-initial-dist [Ryo Yamashita] 6593df1
* Fixes #87 [mi_sawa] 463f0e2
* Add challenge case for #87 [mi_sawa] 0f9c8d1
* Use T::max_value() instead of the sum of the cost [mi_sawa] 819a1fd
* Add challenge case for #88 [mi_sawa] 3ce61ef
* Ignore .idea for JetBrains ides [mi_sawa] 220bce7
* Add badges to README.md [Ryo Yamashita] f41baff
* Use "Latin Letter Small Capital" characters as substitutes for `\textsc` [Ryo Yamashita] cdc1075
* Add document for `twosat` [Ryo Yamashita] fc680ae
* Add document for `scc` [Ryo Yamashita] 431d8b0
* Fix a typo [Ryo Yamashita] 09bb404
* Update the documentation for `dsu` [Ryo Yamashita] 8d81951
* Fix `elided-lifetimes-in-paths` warnings [Ryo Yamashita] 8965119
* Remove unnecessary visibilities [Ryo Yamashita] e8ecc80
* Merge pull request #77 from qryxip/doc-for-math [Ryo Yamashita] 19509cd
* Merge pull request #70 from qryxip/comment-to-cargo-config [Ryo Yamashita] 0b88690
* Modify the comment [Ryo Yamashita] d9af17f
* Add document for `math` [Ryo Yamashita] 581d951
* Modify the comment [Ryo Yamashita] 766ea3b
* Modify the comment [Ryo Yamashita] 2b89c59
* Add a comment to `.cargo/config.toml` [Ryo Yamashita] c3b5e58
* Merge pull request #71 from qryxip/atomic-barrett-seqcst [Ryo Yamashita] a8c306d
* Rename `store` to `update` [Ryo Yamashita] b09e6b5
* Merge remote-tracking branch 'upstream/master' into atomic-barrett-seqcst [Ryo Yamashita] 43231bb
* Merge pull request #74 from tamuhey/impl_binop_rhs_int_for_mint [Ryo Yamashita] 006d353
* clippy [Yohei Tamura] 3fdf7d9
* clippy [Yohei Tamura] 92a7243
* lint [Yohei Tamura] e90bb7e
* impl binop<Rhs=int> for Mint [Yohei Tamura] 224d9df
* Merge pull request #72 from tamuhey/refactor_min_cut [Ryo Yamashita] 4e15473
* Merge remote-tracking branch 'upstream/master' into refactor_min_cut [Ryo Yamashita] 29db5ae
* Merge pull request #73 from qryxip/clippy [Ryo Yamashita] 2bcb0d1
* Fix `clippy::stable_sort_primitive` [Ryo Yamashita] b248bf9
* refactor [Yohei Tamura] b64d992
* Undo the document modification [Ryo Yamashita] bac7f67
* Make `modint::Barrett` `(AtomicU32, AtomicU64)` [Ryo Yamashita] b473a61
* Merge pull request #69 from qryxip/katex [Kentaro Matsumoto] 7f950ae
* Merge pull request #67 from qryxip/make-mod-int-base-inherit-from-isize-usize [Kentaro Matsumoto] 61212ca
* Merge pull request #68 from qryxip/doc-for-modint [Kentaro Matsumoto] 533965b
* Merge pull request #54 from qryxip/change-authors [Kentaro Matsumoto] 64213cc
* Merge pull request #53 from qryxip/common-metadata [Kentaro Matsumoto] 867d025
* Use KaTeX in documentation [Ryo Yamashita] 6164c10
* Add document for `modint` [Ryo Yamashita] 8a811fd
* Make `ModIntBase` inherit `From<{isize, usize}>` [Ryo Yamashita] cf303d6
* Change `package.authors` to `["rust-lang-ja Developers"]` [Ryo Yamashita] d0b8173
* Add "commont metadata" to Cargo.toml [Ryo Yamashita] 0e5052d
* Merge pull request #65 from qryxip/fix-mincostflow [Ryo Yamashita] 3056e2a
* Apply fix as atcoder/ac-library#38 [Ryo Yamashita] 861f2f7
* Merge pull request #51 from qryxip/move-allow [Kentaro Matsumoto] ad148b0
* Merge branch 'master' into move-allow [Kentaro Matsumoto] 12c212e
* Merge pull request #50 from qryxip/make-modules-pub-again [Kentaro Matsumoto] fdc049e
* Move the `#[allow]` attribute to string.rs [Ryo Yamashita] 33a0d42
* Make modules `pub` again [Ryo Yamashita] 7cabbe8
* Merge pull request #60 from manta1130/Fix/CRT [Ryo Yamashita] ad3a68d
* Fix #59 [manta1130] ddf8b94
* Merge pull request #49 from manta1130/code-expander [Ryo Yamashita] ee0d3df
* Merge remote-tracking branch 'upstream/master' into code-expander [Ryo Yamashita] 4b7e468
* Merge pull request #44 from TonalidadeHidrica/feature/lazysegtree [Ryo Yamashita] c2bb69c
* Merge pull request #45 from qryxip/convolution [Ryo Yamashita] 319d19b
* Format codes [manta1130] 2bcaf15
* Change module definition(mod -> pub mod) [manta1130] fb47859
* Fix dependency_list [manta1130] 0872289
* Remove unnecessary space [manta1130] fcacbc1
* Sort output_list_all,dependency_list in lexicographical order [manta1130] 23ba807
* Change the test environment construction method [manta1130] 7363691
* Change the process if rustfmt returns error [manta1130] c3cbe59
* Added EOL [manta1130] d6ed636
* Formatted codes [manta1130] 9760ead
* show error message [manta1130] 47bd974
* Added rustfmt install command [manta1130] ee9defa
* Added the process of rustfmt [manta1130] eee7f85
* Changed the test code [manta1130] d8a8147
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] fb9e6c7
* Merge branch 'master' into feature/segtree [TonalidadeHidrica] ed778ef
* Changed if start with no args,it shows usage. [manta1130] 83783b3
* Added -a --all options [manta1130] 2e036ca
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] 331ea67
* Removed some options. [manta1130] e9e290b
* Changed the monoid namme [TonalidadeHidrica] d4bb776
* Changed the signature of f in max_right & min_left [TonalidadeHidrica] 48ceeb6
* Change arguments from value to reference [TonalidadeHidrica] f54acaf
* Fixed help [manta1130] ac05f13
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] 558a652
* cargo fmt [TonalidadeHidrica] 32dbc68
* Modify binary_operation and add some traits [TonalidadeHidrica] 5650b88
* Add tests for expand.py [manta1130] deda790
* Split integral traits into four subtraits [TonalidadeHidrica] d51a67b
* Fixed dependency list [manta1130] 839e3d7
* Added output header [manta1130] ddc4dfb
* Added usage [manta1130] 21d6f5d
* Added comments [manta1130] c1ca28e
* Implemented option(output_comment,output_test) [manta1130] 2f36d26
* Fix `convolution_i64` [Ryo Yamashita] c2e8698
* Translate all of the `convolution` unit tests of the original ones [Ryo Yamashita] af4b04c
* Changed the order of codes. [manta1130] c1ce090
* Implemented expand.py [manta1130] 70d51bc
* Add some tests [Ryo Yamashita] b8043a0
* Merge pull request #43 from manta1130/feature/twosat [Ryo Yamashita] f264623
* Merge pull request #33 from kenkoooo/features/rustfmt_toml [Ryo Yamashita] 5f6bdac
* Revert "Added #[derive(Default)] in some structs" [manta1130] d6a667c
* Added #[derive(Default)] in some structs [manta1130] 209a624
* Changed the test code [manta1130] 4b0747b
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] 4c4516f
* Apply the change that I forgot [TonalidadeHidrica] ce40d2a
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] c6a6d2c
* Merge branch 'master' into feature/segtree [TonalidadeHidrica] 2e3b0be
* Merge pull request #24 from TonalidadeHidrica/feature/maxflow [Ryo Yamashita] 5103743
* Reduce the number of type parameters [Ryo Yamashita] 838b45b
* Add `convolution_raw` [Ryo Yamashita] 4524c31
* Update .rustfmt.toml [kenkoooo] 3404eef
* Changed if expression into explicit type conversion (bool -> usize). [manta1130] cac3df7
* Changed the return value type(&Vec<bool> -> &[bool]) [manta1130] c57c408
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] 14ed754
* Change position of newline [TonalidadeHidrica] 3a43da6
* Added example for Practice2-L [TonalidadeHidrica] 707b56d
* Added example for Practice2-K [TonalidadeHidrica] 4878021
* Implement Debug for LazySegtree [TonalidadeHidrica] 51dd0db
* Implement `convolution` [Ryo Yamashita] 2cee3c9
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] da2c826
* Follow naming convension (mod test -> tests) [TonalidadeHidrica] 93a2906
* Added a test [TonalidadeHidrica] 336bf18
* Merge branch 'feature/segtree' into feature/lazysegtree [TonalidadeHidrica] 61cd506
* Renamme test function [TonalidadeHidrica] e6ac534
* Fix a mistake [TonalidadeHidrica] a8b2cb6
* Merge branch 'master' into feature/maxflow [TonalidadeHidrica] 1c8d2ab
* Make MapMonoid underlying type Clone [TonalidadeHidrica] 3474460
* Added LazySegTree [TonalidadeHidrica] ded9a93
* Changed the struct name in accoding with UpperCamelCase rule. [manta1130] 2a69b8b
* Changed Copy to Clone [TonalidadeHidrica] dc1c6b7
* Fixed Default implementation [TonalidadeHidrica] 6a6259f
* Suppress warnings [manta1130] c0862e8
* Add tests [manta1130] 3bba0ca
* Fixed mistake [manta1130] dbacf02
* Implement 'twosat' [manta1130] 9591c79
* cargo fmt [TonalidadeHidrica] 167cee8
* Added sample for J - Segment Tree [TonalidadeHidrica] b27d10d
* Merge branch 'master' into feature/segtree [TonalidadeHidrica] dc64b31
* Added "Max" monoid and a test [TonalidadeHidrica] 7f70828
* Change identity from constant to function [TonalidadeHidrica] db3e57f
* Merge pull request #31 from ukiyoyo/feature/scc [Kentaro Matsumoto] 6d1e6a7
* Remove Clone trait that doesn't need to be explicitly required [Kentaro Matsumoto] 08a2a48
* Merge pull request #13 from TonalidadeHidrica/feature/internal_queue [Kentaro Matsumoto] dd2a49c
* Merge branch 'master' into feature/segtree [TonalidadeHidrica] 475676f
* Fixed mistakes [TonalidadeHidrica] 831c35a
* Removed newline [TonalidadeHidrica] c35995e
* Added segtree [TonalidadeHidrica] 452f9f1
* Merge branch 'master' into feature/scc [Kentaro Matsumoto] ec2e30b
* Merge pull request #14 from qryxip/modint [Kentaro Matsumoto] 54a54fd
* Initial commit for segtree [TonalidadeHidrica] 0e8242f
* Suppress too many single-character variables warnings [TonalidadeHidrica] 125fc64
* Add tests [TonalidadeHidrica] 7365855
* Merge pull request #32 from qryxip/internal-bit [Hiroki Kobayashi] 6f6f530
* Replace `MfCapacity` with `Integral` from type traits [TonalidadeHidrica] c05ac18
* Added additional assertions and documentation [TonalidadeHidrica] 51a9c98
* Suppress warnings [TonalidadeHidrica] a793068
* Follow the changes of interface in internal_queue [TonalidadeHidrica] abbd181
* Merge branch 'feature/internal_queue' into feature/maxflow [TonalidadeHidrica] b9ed34c
* Merge branch 'master' into feature/maxflow [TonalidadeHidrica] 43dfa84
* Add a test [TonalidadeHidrica] 0859129
* Function `front` now returns Option [TonalidadeHidrica] ec7afe0
* Allow deadcode [TonalidadeHidrica] 036948a
* Merge pull request #28 from manta1130/feature/math [Hiroki Kobayashi] bd58139
* Merge branch 'master' into feature/math [manta1130] c2267fd
* Remove r1,m1 and all dereference operators from ri,mi [manta1130] 4205d0a
* Use assert_eq instead of assert [manta1130] f7688f2
* Update src/internal_bit.rs [Ryo Yamashita] f9b142c
* fmt [ukiyoyo] 5a737c7
* Merge branch 'master' into feature/scc [ukiyo] 0e121a0
* Add .rustfmt.toml [kenkoooo] b543148
* Merge branch 'master' into modint [Kentaro Matsumoto] c828591
* Merge pull request #25 from kenkoooo/features/mincostflow [Kentaro Matsumoto] 0312d8d
* Implement `internal_bit` [Ryo Yamashita] a453716
* Fix typo [ukiyoyo] 561981c
* Fix test name [ukiyoyo] 08649a6
* Fixed according to cargo clippy warnings(tests) [manta1130] d53934f
* Replace Integer with internal_type_traits::Integral [kenkoooo] 9c36e0c
* Re-export [ukiyoyo] 88f7a86
* Add test [ukiyoyo] 9f14656
* Implement scc [ukiyoyo] d78cdaf
* Implement scc [ukiyoyo] 42aadfb
* Add tests [manta1130] 06f3fd4
* Merge branch 'master' into features/mincostflow [kenkoooo] c323b7f
* Fixed mistake [manta1130] bba62af
* Fix the doc [Ryo Yamashita] 80a7ab5
* Fixed according to cargo clippy warnings [manta1130] 88fb6d0
* Merge branch 'master' into feature/math [manta1130] 3ecd3a7
* implement `math` [manta1130] 241ea3f
* Merge remote-tracking branch 'upstream/master' into modint [Ryo Yamashita] 2843c8e
* Merge pull request #12 from hotman78/master [Kentaro Matsumoto] 897c6fd
* Merge pull request #26 from koba-e964/feature/string [Kentaro Matsumoto] 0c9f9b6
* Correct the method name of `RemEuclidU32` [Ryo Yamashita] 59b551f
* Rename `IntoRepresentative` to `RemEuclidU32` [Ryo Yamashita] 7616c3a
* Declare `&LocalKey<Barrett>` in `Id` [Ryo Yamashita] 25019f1
* add use declaration [manta1130] 90bc429
* Use `Barrett::mul` [Ryo Yamashita] 123c360
* Implement `modint` [Ryo Yamashita] 8009562
* Merge pull request #3 from TonalidadeHidrica/feature/internal_math [Ryo Yamashita] 2d43cf6
* Change test function name [TonalidadeHidrica] d510706
* Allow cognitive complexity lint for tests [TonalidadeHidrica] 2a14f63
* Allow unreadable literals in tests in internal_math [TonalidadeHidrica] 47a266a
* Allow dead code in internal_math [TonalidadeHidrica] 280e8af
* Changed visibility [TonalidadeHidrica] 0fc70b0
* Merge pull request #19 from qryxip/internal-type-traits [Ryo Yamashita] 46c1ba5
* Added tests [TonalidadeHidrica] a2d7c81
* Add a comment for Integer trait [kenkoooo] 0f6387c
* Add assertions into mincostflow.rs [kenkoooo] 56a6f71
* Fix minor issues in mincostflow [kenkoooo] 419b128
* clippy: allow many single-char names [TonalidadeHidrica] bd447c0
* Fix suffix_array's signature [koba-e964] 4324451
* Merge remote-tracking branch 'origin/feature/internal_math' into feature/internal_math [TonalidadeHidrica] 65b1f88
* Merge branch 'feature/internal_queue' into feature/maxflow [TonalidadeHidrica] a9bb6e8
* Added sample code for D - Maxflow [TonalidadeHidrica] 55c0d81
* Change visibility of some functions and structs [TonalidadeHidrica] 693cfdb
* Silence clippy warnings [koba-e964] e87e825
* Handle self-loop in add_edge [TonalidadeHidrica] ae780c9
* Add lcp_array, z_algorithm [koba-e964] 72f82b6
* Make `pop` function safer [TonalidadeHidrica] d020fcb
* Fixed some mistakes [TonalidadeHidrica] 67a48d0
* Merge branch 'feature/internal_queue' into feature/maxflow [TonalidadeHidrica] 91ac693
* Return &T in pop method in SimpleQueue [TonalidadeHidrica] f23859d
* Add sa_is [koba-e964] 86ed9cc
* Add subset of string.hpp [koba-e964] 26e4392
* Add `min_value` and `max_value` [Ryo Yamashita] 48ece04
* Add comment [Ryo Yamashita] ee4bdb9
* Update src/internal_type_traits.rs [Ryo Yamashita] e1667d6
* Implement `internal_type_traits` [Ryo Yamashita] 852af52
* Fixed typo & function name [TonalidadeHidrica] a76845c
* Remove unneeded return [TonalidadeHidrica] d4b6324
* Add maxflow [TonalidadeHidrica] 1ff54bd
* Fix clippy issues [kenkoooo] 57d6c5f
* Implement mincostflow [kenkoooo] 6a8e6dc
* Derive Default for SimpleQueue [TonalidadeHidrica] ae02989
* Change visibility [TonalidadeHidrica] 83e4ec0
* Modified according to the pull request comments [TonalidadeHidrica] e8fb094
* Handle overflows [TonalidadeHidrica] f9c3bdc
* Changed important comments to doc comments [hotman] bd8089e
* Add comment about Dsu's size constraints [hotman] 9a13dce
* Merge pull request #17 from matsu7874/master [Kentaro Matsumoto] 6e34caa
* Update README.md [Kentaro Matsumoto] 8f8fad9
* Update README.md [Kentaro Matsumoto] ab2711f
* Update README.md [Kentaro Matsumoto] 343e0a6
* Update README.md [Kentaro Matsumoto] 824994d
* Update README.md [Kentaro Matsumoto] 1492a12
* Add groups function to Dsu [hotman] c4f058d
* Add Dsu to lib.rs [hotman] 2d81cf5
* Describe the current development style in README. [matsu7874] c6cce2a
* Update src/internal_math.rs [TonalidadeHidrica] b284f6d
* Documented additional constraiatns [TonalidadeHidrica] 5fba679
* Fixed mistake (wrapping_add does not mutate of course!) [TonalidadeHidrica] df17bbd
* Add internal_queue [TonalidadeHidrica] fdac8e3
* Fixed according to cargo fmt [hotman] ec563d3
* Fixed according to cargo clippy [hotman] 2ef793f
* Added dsu [hotman] 5bc4228
* Fixed according to review comments [TonalidadeHidrica] 82e8e68
* Maybe I don't need wrap with module (as in namespace) [TonalidadeHidrica] e19b6a3
* Merge branch 'master' into feature/internal_math [TonalidadeHidrica] 771c5b3
* Merge pull request #10 from qryxip/publish-false [Kentaro Matsumoto] 3328ebf
* Add `publish = false` to prevent misoperation [Ryo Yamashita] 671dbfd
* Merge pull request #6 from kenkoooo/fix/visibility [Ryo Yamashita] 297cf39
* Apply cargo-fmt to internal module files [kenkoooo] 942c51f
* Hide modules [kenkoooo] 86f431b
* Merge branch 'master' into fix/visibility [Ryo Yamashita] 6424f03
* Merge pull request #5 from kenkoooo/features/license [Ryo Yamashita] 5809126
* Merge pull request #2 from TonalidadeHidrica/fix/cargo-fmt [Ryo Yamashita] 884277a
* Merge pull request #8 from koba-e964/fix-fenwick [Hiroki Kobayashi] e36f127
* Fix clippy's warnings [koba-e964] 57841ad
* Merge branch 'master' into fix/cargo-fmt [Ryo Yamashita] a568b9a
* Merge pull request #7 from koba-e964/fenwick [Kentaro Matsumoto] 432e783
* Merge pull request #4 from qryxip/github-actions [Kentaro Matsumoto] 235b2df
* Add FenwickTree [koba-e964] 4877090
* Add internal modules into lib.rs and fix visibility [kenkoooo] 5ef2532
* Add LICENSE [kenkoooo] 946b250
* Setup GitHub Actions [Ryo Yamashita] 328f6d4
* Change location of "use", add some newline [TonalidadeHidrica] 20e4747
* Added internal_math [TonalidadeHidrica] 7bf928b
* cargo fmt [TonalidadeHidrica] 459bd7c
* Add rust-toolchain [TonalidadeHidrica] 5ee27b3
* 実装すべきファイルを作成する。 [matsu7874] 04b7581
* add README.md [matsu7874] dcecc05
* cargo init [matsu7874] f830108