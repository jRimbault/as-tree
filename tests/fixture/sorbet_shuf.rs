#[test]
fn test() {
    let paths = "\
test/testdata/infer/attached_class_self_new.rb
gems/sorbet-runtime/lib/types/private/methods
website/docs/non-forcing-constants.md
gems/sorbet/test/snapshot/partial/use-existing-config/expected
test/testdata/desugar/class_def_kind.rb
test/testdata/lsp/workspace_symbols_stdlib.rb
test/testdata/autogen/bare_module.rb
test/testdata/infer/self_type_param.rb
emscripten/BUILD
test/testdata/namer/ancestors.rb.symbol-table-raw.exp
test/testdata/core/fuzz_unparseable.rb
test/cli/forgot-typed
website/docs/union-types.md
test/cli/autocorrect-same-loc/autocorrect-same-loc-2.rb
gems/sorbet-runtime/test/types/method_validation.rb
parser
test/testdata/lsp/completion/snippet_types.A.rbedited
test/testdata/rbi/kernel_class.rb
test/testdata/parser/invalid_syntax_error.rb.parse-tree.exp
test/whitequark/test_character_0.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_27.parse-tree-whitequark.exp
main/lsp/request_dispatch.cc
test/testdata/resolver/object_include_stub.rb
test/whitequark/test_ruby_bug_12402_7.parse-tree-whitequark.exp
test/whitequark/test_var_op_asgn_1.parse-tree-whitequark.exp
test/cli/subprocess-plugin/no_output.rb
third_party/licenses
test/whitequark/test_ruby_bug_11873_8.parse-tree-whitequark.exp
test/cli/suggest-typed-true/suggest-typed-true.rb
test/testdata/infer/generics/lub_lambda_param.rb
test/testdata/lsp/completion/angle_bracket_names.rb
test/testdata/lsp/field_edits.rb.document-symbols.exp
website/docs/adopting.md
test/cli/make_accessible/make_accessible.sh
test/testdata/lsp/definitions_and_usages.rb
test/cli/bad-plugin-spec/bad-plugin-spec.sh
test/cli/metrics-file/with-error-branching.rb
test/testdata/rewriter/t_struct/param_order.rb
test/helpers/BUILD
test/cli/suggest-typed-true/suggest-typed-false.rb
test/testdata/cfg/next.rb
test/cli/suggest-foreign-lambda/suggest-foreign-lambda.out
gems/sorbet/test/hidden-method-finder/shims.rb.source
test/testdata/infer/i1438.rb
definition_validator/variance.h
main/lsp/LocalVarSaver.h
main/lsp/notifications/initialized.h
test/testdata/cfg/class_static_field_children.rb
gems/sorbet/test/snapshot/partial/webmock/src/Gemfile
test/whitequark/test_bug_def_no_paren_eql_begin_0.rb
website/blog
test/testdata/infer/aliases__2.rb
test/whitequark/test_yield_2.parse-tree-whitequark.exp
test/testdata/parser/complement_literal.rb.desugar-tree.exp
main/lsp/NextMethodFinder.h
test/whitequark/test_kwarg_no_paren_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/local_gem/src/Gemfile
gems/sorbet-runtime/lib/types/props/private/deserializer_generator.rb
common/os/mac.cc
gems/sorbet-runtime/test/types/props
test/testdata/infer/non_existent_method.rb
class_flatten/class_flatten.h
test/testdata/infer/void_proc.rb
test/cli/ignore/subfolder2/foo.rb
test/testdata/infer/non_forcing_is_a.rb
main/lsp/LSPConfiguration.h
test/testdata/rewriter/rails
test/cli/store-state/store-state.out
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi
test/whitequark/test_until_1.parse-tree-whitequark.exp
test/whitequark/test_if_nl_then_0.parse-tree-whitequark.exp
test/testdata/parser/long_string.rb.parse-tree.exp
test/testdata/infer/generic_methods/two_bounds.rb
test/testdata/autogen/nested_defs.rb.autogen.exp
test/testdata/resolver/top_level_abstract_typed_true.rb
test/testdata/namer/simple.rb.parse-tree.exp
test/whitequark/test_ruby_bug_11873_a_14.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed
test/whitequark/test_not_1.rb
third_party/parser/README.md
third_party/parser/include/ruby_parser/literal.hh
test/whitequark/test_yield_3.rb
test/whitequark/test_ruby_bug_9669_1.rb
third_party/licenses/libb2.txt
test/testdata/autogen/aliased_ancestors.rb
test/whitequark/test_ruby_bug_14690_0.parse-tree-whitequark.exp
test/whitequark/test_const_op_asgn_2.parse-tree-whitequark.exp
test/cli/autocorrect-extend/autocorrect-extend.sh
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0
website/docs/stdlib-generics.md
test/testdata/cfg/retry_multiple.rb
test/whitequark/test_or_asgn_1.rb
website/docs/overview.md
test/lsp
test/whitequark/test_string_dvar_0.rb
test/testdata/infer/generics/arity_mismatch.rb
rbi/core/io.rbi
test/whitequark/test_non_lvar_injecting_match_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_14.parse-tree-whitequark.exp
test/whitequark/test_block_kwarg_0.rb
test/whitequark/test_procarg0_0.rb
test/whitequark/test_yield_block_0.rb
test/whitequark/test_send_plain_cmd_1.parse-tree-whitequark.exp
website/static/img/factorial-logo.png
test/cli/autocorrect-private/autocorrect-private.out
core/Names.cc
test/testdata/rewriter/struct.rb.symbol-table-raw.exp
main/lsp/LSPTypechecker.cc
test/testdata/cfg/break_in_junk.rb.cfg.exp
test/testdata/autogen/nested_defs.rb
test/testdata/namer/multiple_stubs.rb.flatten-tree.exp
test/cli/autocorrect-bare-stdlib-generics/autocorrect-bare-stdlib-generics.out
core/Names.h
test/testdata/infer/attached_class_printing.rb
test/whitequark/test_ruby_bug_11873_a_18.rb
main/lsp/UndoState.cc
test/testdata/infer/lub_tuples.rb.symbol-table-raw.exp
test/testdata/infer/toplevel.rb
website/docs/shapes.md
test/cli/incremental-resolver
test/cli/suggest-t-let/suggest-t-let.rb
test/whitequark/test_optarg_1.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_12402_13.rb
test/testdata/deviations/non_ruby_names.rb.parse-tree.exp
test/testdata/cfg/wtf_ensure.rb
test/cli/autocorrect-remove-body/post.rbi
test/cli/remove-path-prefix-no-match/remove-path-prefix-no-match.sh
docs/internals.md
rbi/core/enum.rbi
test/cli/symbol-table-json
test/whitequark/test_var_and_asgn_0.parse-tree-whitequark.exp
test/testdata/resolver/sig_on_failure.rb
test/whitequark/test_int_LINE_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_7.rb
test/cli/suggest_static/suggest_static.rb
website/docs/talks/strange-loop-2018.md
test/helpers/lsp.h
test/whitequark/test_ruby_bug_11873_a_5.parse-tree-whitequark.exp
test/whitequark/test_def_3.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/empty/expected/sorbet/config
test/cli/remove-path-prefix/remove-path-prefix.rb
gems/sorbet/lib
test/testdata/cfg/rescue_with_return.rb
test/testdata/desugar/class_def_kind.rb.desugar-tree-raw.exp
docs/compressors.md
test/pipeline_test.bzl
test/testdata/rewriter/fuzz_encrypted_prop.rb
test/testdata/lsp/completion/sig_redefinition__2.A.rbedited
rewriter/Rails.h
test/testdata/infer/zsuper.rb
test/testdata/lsp/document_symbols_crash.rb
test/testdata/lsp/document_symbols.rb.document-symbols.exp
ast/desugar/BUILD
test/testdata/namer/blocks_in_reopened_class.rb
test/testdata/rewriter/empty_until.rb
test/whitequark/test_send_block_chain_cmd_4.parse-tree-whitequark.exp
test/testdata/rewriter/flatten_nested.rb
test/cli/autocorrect-abstract
test/cli/autocorrect-remove-body
test/testdata/resolver/enumerable_strict.rb
third_party/crcpp.BUILD
test/cli/help/help.sh
test/whitequark/test_hash_hashrocket_1.rb
test/testdata/desugar/integers.rb
test/testdata/cfg/next.rb.cfg.exp
rbi/stdlib/bigdecimal.rbi
docs/img/cfg-example.svg
test/cli/help
test/cli/autocorrect-same-loc/autocorrect-same-loc.sh
test/whitequark/test_args_args_assocs_1.parse-tree-whitequark.exp
test/whitequark/test_optarg_0.parse-tree-whitequark.exp
test/cli/empty-file/empty-file.out
test/testdata/infer/generics/self_params.rb
test/whitequark/test_arg_combinations_4.parse-tree-whitequark.exp
test/whitequark/test_send_index_cmd_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_12402_7.rb
test/testdata/rbi/bigdecimal.rb
test/whitequark/test_ruby_bug_12402_0.rb
test/testdata/parser/offset0.rb.parse-tree.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib
test/fuzz
test/whitequark/test_return_1.parse-tree-whitequark.exp
cfg/Instructions.cc
test/testdata/lsp/completion/sig_all_untyped.rb
test/whitequark/test_send_lambda_args_noparen_0.rb
test/whitequark/test_ruby_bug_12402_2.rb
gems/sorbet-runtime/test/types/interface_validation.rb
test/whitequark/test_send_binary_op_10.rb
emscripten
test/testdata/resolver/bad_sealed_class__2.rb
test/testdata/infer/control_flow/present_p.rb
test/whitequark/test_send_call_0.parse-tree-whitequark.exp
test/whitequark/test_space_args_block_0.rb
test/testdata/parser/trailing_comas.rb
test/testdata/todo/const_in_block.rb.symbol-table-raw.exp
test/testdata/desugar/sclass_inheritance.rb.flatten-tree.exp
test/cli/folder-input/folder-input.sh
gems/sorbet/test/snapshot/partial/rails-double-require/src/app/models
test/whitequark/test_class_definition_in_while_cond_0.parse-tree-whitequark.exp
test/whitequark/test_arg_duplicate_2.rb
test/whitequark/test_until_post_0.parse-tree-whitequark.exp
test/testdata/resolver/optional_cyclic.rb
test/testdata/deviations/keyword_method_names.rb
third_party/licenses/pdqsort.txt
third_party/progressbar/BUILD
gems/sorbet/test/snapshot/partial/rails-double-require/src/app/models/article.rb
test/testdata/infer/attached_class.rb
test/LSPTest.cc
gems/sorbet-runtime/test/types/props/struct.rb
common/statsd
core/ErrorFlusher.cc
test/whitequark/test_ruby_bug_11873_2.parse-tree-whitequark.exp
website/pages/en/community.js
third_party/clang.BUILD
test/whitequark/test_break_3.rb
test/testdata/resolver/abstract.rb
test/cli/suggest-sig-override/suggest-sig-override.sh
main/lsp/LSPIndexer.h
test/whitequark/test_rescue_mod_op_assign_0.parse-tree-whitequark.exp
test/whitequark/test_and_1.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_a_4.parse-tree-whitequark.exp
test/cli/arity-messages/arity-messages.out
proto/pay-server/SourceMetrics.proto
test/testdata/cfg/retry.rb.cfg.exp
core/BUILD
test/cli/module-redefinition/module-redefinition.out
test/testdata/infer/crash_after_parse_errors.rb
test/whitequark/test_return_in_class_0.rb
test/testdata/rewriter/t_struct/override.rb
test/testdata/lsp/hover.rb
test/whitequark/test_arg_combinations_13.rb
test/testdata/lsp/untyped_field_reference__1.rb
test/whitequark/test_super_2.rb
test/testdata/resolver/invalid_alias.rb
rewriter/Util.h
test/testdata/resolver/alias_without_alias.rb
test/whitequark/test_hash_hashrocket_0.rb
test/whitequark/test_send_binary_op_15.parse-tree-whitequark.exp
test/whitequark/test_cpath_1.rb
third_party/ruby/BUILD
test/cli/autogen-subclasses-ignore/ignored
test/whitequark/test_def_4.parse-tree-whitequark.exp
test/whitequark/test_cond_iflipflop_1.rb
test/cli/suggest-sig-override/suggest-sig-override.out
test/testdata/local_vars
test/whitequark/test_block_arg_combinations_0.rb
test/cli/dedup-input-files
test/cli/escaping/escaping.out
third_party/parser/include/ruby_parser/driver.hh
test/testdata/call_with_splat_and_block.rb
test/testdata/namer/module_function.rb
test/testdata/cfg/rescue_complex.rb.desugar-tree.exp
test/cli/remove-path-prefix/remove-path-prefix.sh
test/testdata/resolver/fuzz_infinite_type.rb
test/whitequark/test_hash_label_end_2.parse-tree-whitequark.exp
test/testdata/infer/generics/generics_class_of.rb
test/cli/ignore/subfolder2
test/whitequark/test_masgn_nested_1.parse-tree-whitequark.exp
test/testdata/resolver/linearization/linearization6.rb
test/cli/web-trace-file-non-ascii/web-trace-file-non-ascii.out
test/whitequark/test_bug_do_block_in_hash_brace_4.rb
test/testdata/rbi/enumerable_flat_map.rb
test/testdata/lsp/completion/private.rb
rbi/stdlib/socket.rbi
test/whitequark/test_hash_label_end_0.parse-tree-whitequark.exp
test/cli/suggest_autogen/suggest_autogen.sh
test/testdata/desugar/complex.rb
tools/buildstamp/get_workspace_status
test/cli/parser-error/parser-error-3.rb
test/testdata/resolver/fuzz_suggest.rb
main/lsp/LSPInput.h
test/testdata/resolver/class_ivar.rb
test/testdata/lsp/document_symbols_crash.1.rbupdate.document-symbols.exp
test/whitequark/test_sclass_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/private/methods/call_validation.rb
gems/sorbet/test/hidden-method-finder/hidden-method-tests.rb
test/whitequark/test_bug_cmdarg_2.rb
website/docs/rbi.md
test/cli/configatron/configatron.yaml
test/whitequark/test_bug_473_0.rb
gems/sorbet-runtime/test/types/attached_class.rb
test/whitequark/test_arg_invalid_2.rb
test/testdata/resolver/sig_returns_nil.rb
test/testdata/lsp/hover_untyped.rb
test/cli/subprocess-plugin/multi2.rb
test/testdata/autogen/bad_prop.rb
test/cli/autogen-subclasses-ignore/ignored/ignored.rb
gems/sorbet/sorbet.gemspec
test/whitequark/test_yield_0.parse-tree-whitequark.exp
test/whitequark/test_character_0.rb
test/testdata/infer/generics/TypeSyntax.rb
test/cli/autocorrect-extend/autocorrect-extend.rb
gems/sorbet-runtime/lib/types/private/methods/_methods.rb
gems/sorbet-runtime/bench/constructor.rb
test/whitequark/test_not_2.parse-tree-whitequark.exp
definition_validator/validator.cc
test/testdata/lsp/code_actions/sig_missing__parent.rb
test/whitequark/test_unless_1.rb
test/testdata/namer/class_module.rb
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/resolv.rbi
test/cli/print-procs/print-procs.rb
test/testdata/infer/compact.rb
test/testdata/infer/control_flow/complex_implications_2.rb
test/testdata/lsp/bad_field_edits.rb.document-symbols.exp
gems/sorbet-runtime/test/types/fixtures/sealed_module/sealed_abstract__1.rb
website/docs/runtime.md
test/whitequark/test_xstring_interp_0.parse-tree-whitequark.exp
test/whitequark/test_bug_435_0.parse-tree-whitequark.exp
test/testdata/desugar/ensure_without_rescue.rb.desugar-tree.exp
test/testdata/union_method_arity_error.rb
ast/treemap
test/testdata/cfg/rescue_expression.rb.cfg.exp
test/testdata/lsp/code_actions/loop_type_change.A.rbedited
test/whitequark/test_ruby_bug_13547_0.rb
test/testdata/lsp/bad_field_edits.rb
test/whitequark/test_complex_3.parse-tree-whitequark.exp
main/lsp/requests/definition.cc
test/whitequark/test_emit_arg_inside_procarg0_legacy_0.rb
test/whitequark/test_alias_gvar_1.parse-tree-whitequark.exp
test/whitequark/test_cpath_invalid_0.rb
test/whitequark/test_ruby_bug_11873_a_4.rb
gems/sorbet/test/snapshot/partial/bad-t/src/src.rb
test/testdata/infer/ref_eq.rb
gems/sorbet/test/snapshot/partial/webmock/src/webmock.rb
test/testdata/resolver/proc.rb
test/testdata
test/testdata/resolver/final_method.rb
test/testdata/resolver/ancestor_scope.rb.flatten-tree.exp
test/whitequark/test_block_kwarg_combinations_2.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder
test/whitequark/test_unless_mod_0.parse-tree-whitequark.exp
rbi
test/whitequark/test_while_post_0.rb
namer
core/errors/resolver.h
infer/infer.h
test/testdata/lsp/completion/constants.A.rbedited
test/testdata/namer/alias_cross_file.symbol-table-raw.exp
test/whitequark/test_until_mod_0.rb
website/static/blog/2019
test/whitequark/test_bug_do_block_in_hash_brace_3.parse-tree-whitequark.exp
test/whitequark/test_const_scoped_0.parse-tree-whitequark.exp
test/testdata/lsp/good_field_edits.1.rbupdate
third_party/licenses/protobuf.txt
core/proto/proto.h
test/testdata/namer/self_disallowed.rb
test/testdata/rewriter/t_struct/nilable.rb
third_party/gems/gemfile.bzl
website/docs/talks/ruby-kaigi-2018.md
test/whitequark/test_send_self_block_0.rb
test/testdata/resolver/type_alias_order.rb
test/whitequark/test_ambiuous_quoted_label_in_ternary_operator_0.rb
test/whitequark/test_block_arg_combinations_9.rb
test/cli/metrics-file/sorbet/plugin.rb
test/whitequark/test_arg_combinations_14.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_cmd_1.rb
test/testdata/lsp/completion/sig_many_defs.rb
third_party/libb2.BUILD
test/testdata/infer/generic_methods/genericMethods2.rb
test/whitequark/test_ruby_bug_11873_a_16.parse-tree-whitequark.exp
test/testdata/infer/yield_multiple.rb
test/whitequark/test_hash_label_end_1.rb
rewriter/Flatten.cc
test/whitequark/test_multiple_args_with_trailing_comma_0.parse-tree-whitequark.exp
test/testdata/resolver/linearization/linearization4.rb.symbol-table-raw.exp
test/testdata/resolver/fuzz_class_of_static_field.rb
test/whitequark/test_defs_invalid_1.rb
test/testdata/disabled/whitequark/parse_dedenting_heredoc_13.rb
test/cli/autocorrect-strict/post.rb
test/testdata/autogen/cbase_const.rb
test/testdata/lsp/fast_path/class_change_superclass.1.rbupdate
test/cli/empty-file/empty.rb
common/kvstore
website/docs/talks/jvm-ls-2019.md
test/whitequark/test_args_args_assocs_comma_0.rb
test/whitequark/test_marg_combinations_8.rb
ast/desugar/Desugar.h
test/whitequark/test_masgn_1.parse-tree-whitequark.exp
test/cli/suggest-typed-true/suggest-typed-strict.rb
core/lsp/TypecheckEpochManager.h
test/whitequark/test_for_mlhs_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset/gems/my_gem-0.0.0/my_gem.gemspec
test/testdata/infer/isa_generic.rb
test/testdata/resolver/linearization/linearization2.rb
test/whitequark/test_next_1.rb
test/whitequark/test_return_0.parse-tree-whitequark.exp
tools/scripts/try_fast_path_tests.sh
third_party/licenses/jemalloc.txt
test/testdata/desugar/opasgn.rb.desugar-tree.exp
test/whitequark/test_lvar_0.rb
test/testdata/infer/self_type_param_bounded.rb
test/whitequark/test_method_definition_in_while_cond_0.rb
test/whitequark/test_masgn_splat_0.rb
gems/sorbet/test/snapshot/partial/extconf/expected/out.log
test/testdata/resolver/flatten.rb.symbol-table-raw.exp
test/testdata/rewriter/t_enum_all.rb
test/testdata/lsp/workspace_symbols_fullname.rb
test/testdata/cfg/break_in_while.rb.cfg.exp
test/cli/suggest-typos/suggest-typos.out
website/static/img/stripe-logo.svg
test/whitequark/test_or_asgn_1.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_3.rb
test/whitequark/test_ruby_bug_11873_a_1.rb
gems/sorbet-runtime/lib/types/props/private/setter_factory.rb
website/docs/tconfiguration.md
test/testdata/lsp/fast_path/class_change_include_multifile__included.rb
test/whitequark/test_ruby_bug_11873_a_7.rb
test/testdata/intrinsics
test/cli/stop-after
test/testdata/lsp/fast_path/method_rename_argument.rb
test/testdata/lsp/completion/snippet_repeated.rb
test/testdata/namer/arguments.rb.desugar-tree.exp
rbi/sorbet/tprops.rbi
test/cli/bad-plugin-spec/values-not-scalar.yaml
test/cli/rbi-overrides/t4.rb
main/lsp/LSPTask.cc
test/testdata/resolver/mixes_in_class_methods.rb
test/cli/subprocess-plugin/bad_plugin.yaml
test/cli/suggest-typed-true/suggest-typed-true.out
test/whitequark/test_block_kwarg_combinations_0.parse-tree-whitequark.exp
test/cli/index-cache-invalidation
test/fuzz/TextDocumentPositionParamsWithoutTextDocumentIdentifier.proto
third_party/parser/include/ruby_parser/diagnostic.hh
test/helpers/position_assertions.cc
gems/sorbet/test/snapshot/partial/bad_gem/sorbet/config
test/whitequark/test_rational_0.parse-tree-whitequark.exp
test/testdata/resolver/missing_type_combinator_args.rb.flatten-tree.exp
test/cli/suggest-typed-true/suggest-typed-ignore.rb
test/whitequark/test_args_assocs_comma_0.rb
test/testdata/lsp/completion/class_and_module.rb
test/cli/incremental-resolver/type-member.rb
website/static/img/editor_error_squiggles.gif
test/whitequark/test_kwarg_no_paren_1.rb
test/testdata/infer/generics/glb.rb
test/cli/lsp-invalid-json-and-exit
test/cli/suggest-singleton/suggest-singleton.rb
test/testdata/resolver/fuzz_type_member_forget.rb
test/whitequark/test_ruby_bug_11873_a_13.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/expected/sorbet/rbi
rbi/core/signal.rbi
test/cli/backtrace/backtrace.out
test/cli/folder-input-dir-and-file/input
test/testdata/deviations/non_ruby_names.rb.symbol-table-raw.exp
third_party/emscripten-toolchain.BUILD
website/static/img
test/whitequark/test_parser_bug_490_2.rb
test/testdata/lsp/fast_path/parse_errors.1.rbupdate
test/whitequark/test_when_then_0.parse-tree-whitequark.exp
test/testdata/lsp/sig_deletion.rb
test/testdata/rbi/process.rb
test/whitequark/test_complex_3.rb
test/testdata/desugar/star_in_block_arg.rb
parser/Dedenter.h
test/whitequark/test_for_1.parse-tree-whitequark.exp
third_party/gems/build_defs.BUILD
test/cli/subprocess-plugin/subprocess-plugin.sh
test/cli/suggest-type-alias/suggest-type-alias.sh
main/pipeline/ProgressIndicator.h
test/cli/conflicting-definition
third_party/emscripten-clang.BUILD
test/cli/correct-bare-stdlib-generics
gems/sorbet/test/snapshot/partial/bad_gem/src/lib/bad-gem.rb
common/os
rewriter/rewriter.h
test/testdata/rewriter/flatfile_dsl.rb.rewrite-tree.exp
test/whitequark/test_rescue_else_0.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/class_change_superclass_multifile__super.rb
test/whitequark/test_op_asgn_cmd_0.parse-tree-whitequark.exp
third_party/msgpack.BUILD
test/testdata/perf/enum_canBeTruthy_regression.rb
test/cli/counters/counters.sh
test/testdata/lsp/fast_path/method_change_argument_kind.1.rbupdate
rewriter/Regexp.cc
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_class__1.rb
resolver/resolver.h
test/testdata/autogen/no_dsls.rb.autogen.exp
test/whitequark/test_symbol_interp_0.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder/thorough/ruby_2_6_hidden.rbi.exp
test/testdata/infer/bad_suggest.rb
test/cli/arity-messages
test/print_document_symbols.cc
test/testdata/rewriter/prop_get_logic.rb
test/testdata/namer/arguments.rb.desugar-tree-raw.exp
test/testdata/infer/generic_methods/genericMethodsErrors.rb
test/whitequark/test_send_plain_1.parse-tree-whitequark.exp
test/testdata/todo/const_in_block.rb
rbi/core/argf.rbi
gems/sorbet/test/snapshot/partial/typed-ignore/src
test/testdata/cfg/retry_invalid.rb
test/testdata/desugar/destructure.rb
test/testdata/desugar/for.rb.desugar-tree.exp
test/testdata/cfg/blocks.rb.symbol-table-raw.exp
test/testdata/infer/singleton_enum_case.rb
test/whitequark/test_args_args_assocs_1.rb
main/main.cc
test/whitequark/test_send_plain_cmd_0.parse-tree-whitequark.exp
test/whitequark/test_marg_combinations_0.rb
test/whitequark/test_rescue_in_lambda_block_0.rb
test/testdata/rewriter/t_struct/some_default.rb
test/cli/autocorrect-attached-class/autocorrect-attached-class.rb
test/testdata/resolver/abstract_types.rb
test/whitequark/test_and_or_masgn_0.rb
main/lsp/requests/definition.h
test/cli/stop-after-namer/stop-after-namer.rb
test/whitequark/test_symbol_plain_1.parse-tree-whitequark.exp
test/testdata/infer/fuzz_nonexistant_method.rb
gems/sorbet-runtime/test/types/method_patches.rb
test/cli/parser-error/parser-error-2.rb
third_party/rang.BUILD
test/whitequark/test_ruby_bug_12402_9.parse-tree-whitequark.exp
test/cli/conflicting-definition/b.rb
test/whitequark/test_send_block_chain_cmd_0.rb
third_party/lizard.BUILD
gems/sorbet/test/snapshot/partial/rails-double-require/src
test/testdata/infer/block_arg.rb
test/whitequark/test_and_1.rb
test/testdata/desugar/destructure.rb.symbol-table-raw.exp
test/whitequark/test_def_2.parse-tree-whitequark.exp
test/testdata/lsp/completion/keywords.rb
gems/sorbet/test/hidden-method-finder/simple/src/sorbet
test/testdata/infer/infer1.rb.flatten-tree.exp
third_party/jemalloc.BUILD
test/whitequark/test_next_0.rb
test/whitequark/test_block_arg_combinations_20.rb
test/whitequark/test_if_mod_0.rb
test/testdata/lsp/completion/snippet_block_arity.rb
gems/sorbet/test/snapshot/partial/webmock
test/testdata/desugar/assign_empty_parens.rb
test/whitequark/test_ruby_bug_13547_12.parse-tree-whitequark.exp
website/static/img/editor_go_to_definition.gif
main/lsp/ErrorReporter.h
test/whitequark/test_defs_1.parse-tree-whitequark.exp
core/errors/desugar.h
test/whitequark/test_hash_kwsplat_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/private/mixins
core/DebugOnlyCheck.h
test/testdata/namer/alias_cross_file__02.rb
test/cli/suggest-sig-override-edge/suggest-sig-override-edge.sh
test/whitequark/test_masgn_splat_3.parse-tree-whitequark.exp
test/testdata/lsp/completion/constants_via_mixins.rb
third_party/gems/rules.bzl
main/cache/cache-orig.cc
gems/sorbet/test/snapshot/total/empty/expected/out.log
core/errors/infer.h
test/whitequark/test_var_op_asgn_cmd_0.rb
gems/sorbet/test/snapshot/partial/rspec-lots/src
test/cli/subprocess-plugin/baz_gen.rb
website/core
proto/Symbol.proto
test/testdata/rewriter/prop_computed_by.rb
test/testdata/infer/self_type.rb.cfg.exp
test/whitequark/test_marg_combinations_6.rb
test/cli/autogen-subclasses/a.rb
test/whitequark/test_block_arg_combinations_6.rb
test/cli/print_to_file/a.rb
test/testdata/lsp/fast_path/method_signature_update_static__usage.rb
test/testdata/namer/type_member_redefs__1.rb
test/whitequark/test_block_arg_combinations_2.rb
test/whitequark/test_undef_0.parse-tree-whitequark.exp
rbi/stdlib/openssl.rbi
test/testdata/resolver/rbi_final_re_sig__1.rb
test/whitequark/test_send_block_chain_cmd_5.rb
core/types/types.cc
test/whitequark/test_send_binary_op_3.parse-tree-whitequark.exp
test/whitequark/test_string_interp_0.rb
test/testdata/autogen/bare_class.rb.autogen.exp
test/cli/model_mutator_behavior/model_mutator_behavior__1.rb
core/TypeConstraint.cc
test/whitequark/test_ruby_bug_10653_2.rb
test/cli/autogen-autoloader/inplace.rb
main/cache/cache.h
test/testdata/resolver/let_var.rb.symbol-table-raw.exp
gems/sorbet-runtime/lib/types/utils.rb
test/whitequark/test_ruby_bug_11873_8.rb
test/testdata/lsp/workspace_symbols_sparse.rb
tools/scripts/update_testdata_exp.sh
test/testdata/infer/arity.rb
gems/sorbet/test/snapshot/partial/local_gem/expected/sorbet
test/cli/license/license.sh
test/whitequark/test_send_lambda_args_0.parse-tree-whitequark.exp
main/lsp/requests/initialize.h
docs/img/chrome-tracing-pipeline.png
core/Context.cc
test/lsp/no-trailing-newline/no-trailing-newline.rec
core/ErrorQueue.h
gems/sorbet/test
test/testdata/resolver/simple.rb.symbol-table-raw.exp
main/lsp/LSPFileUpdates.cc
test/testdata/testrunner/pos.rb
test/whitequark/test_ruby_bug_11873_a_10.parse-tree-whitequark.exp
test/whitequark/test_break_0.parse-tree-whitequark.exp
common/concurrency/BUILD
ast/TreeSanityChecks.cc
website/static/img/editor_autocomplete.gif
gems/sorbet-runtime/lib/types
test/cli/lsp-common-case-exit/lsp-common-case-exit.out
test/testdata/infer/singleton_of_singleton.rb.cfg.exp
rbi/stdlib/time.rbi
test/whitequark/test_ruby_bug_11873_a_1.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/method_change_kw_arg_name.rb
rewriter/AttrReader.h
core/AutocorrectSuggestion.h
test/fuzz/ruby.dict
test/whitequark/test_int_0.parse-tree-whitequark.exp
test/testdata/lsp/code_actions/typo.D.rbedited
gems/sorbet/test/snapshot/partial/webmock/src/Gemfile.lock
test/whitequark/test_space_args_arg_0.rb
test/whitequark/test_yield_0.rb
test/testdata/desugar/lambda.rb
main/lsp/watchman
test/whitequark/test_rescue_0.parse-tree-whitequark.exp
test/cli/parallel-ordering/parallel-ordering.out
test/whitequark/test_bug_def_no_paren_eql_begin_0.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_cmd_2.rb
test/whitequark/test_masgn_const_1.parse-tree-whitequark.exp
gems/sorbet/lib/find-gem-rbis.rb
main/lsp/tools/generate_lsp_messages.cc
gems/sorbet-runtime/lib/types/props/weak_constructor.rb
test/whitequark/test_def_0.rb
test/testdata/namer/arguments.rb.symbol-table-raw.exp
test/cli/autocorrect-abstract/autocorrect-abstract.sh
test/whitequark/test_arg_combinations_12.rb
main/lsp/json_types.h
test/testdata/infer/rebind.rb.cfg.exp
test/autocorrect-test.cc
test/whitequark/test_regex_plain_0.rb
test/cli/autogen-autoloader
test/whitequark/test_bug_do_block_in_call_args_0.rb
third_party/parser/cc/lexer.rl
test/testdata/namer/simple.rb.desugar-tree.exp
test/whitequark/test_block_arg_combinations_11.rb
third_party/gems/known_gems.bzl
common/Counters_impl.h
gems/sorbet/test/snapshot/BUILD
test/whitequark/test_ruby_bug_11873_b_0.rb
test/cli/remove-path-prefix-https/remove-path-prefix-https.rb
rewriter/Private.h
ast/desugar
test/testdata/parser/invalid_fatal.rb
test/whitequark/test_masgn_splat_4.parse-tree-whitequark.exp
test/whitequark/test_class_definition_in_while_cond_2.parse-tree-whitequark.exp
test/whitequark/test_arg_combinations_9.rb
test/testdata/desugar/for.rb.cfg.exp
test/testdata/deviations/non_ruby_names.rb.desugar-tree.exp
test/testdata/lsp/fast_path/method_add_argument.rb
rbi/stdlib/benchmark.rbi
test/whitequark/test_beginless_range_before_27_1.rb
test/cli/parser-error/parser-error-1.rb
website/static/img/vonage-logo.png
test/testdata/resolver/fuzz_include_type_alias.rb
gems/sorbet-runtime/BUILD
test/whitequark/test_kwarg_0.rb
test/whitequark/test_case_expr_else_0.rb
test/cli/autogen-subclasses/autogen-subclasses.sh
test/whitequark/test_marg_combinations_5.parse-tree-whitequark.exp
test/whitequark/test_preexe_0.parse-tree-whitequark.exp
test/whitequark/test_parser_bug_272_0.rb
test/testdata/lsp/completion
test/testdata/resolver/linearization/linearization1.rb
test/whitequark/test_bug_do_block_in_hash_brace_1.rb
test/cli/dedup-input-files/dedup-input-files.sh
third_party/parser/cc/token.cc
test/whitequark/test_args_args_assocs_0.rb
rbi/sorbet/sorbet.rbi
test/cli/folder-input-not-found/folder-input-not-found.out
test/testdata/autogen/bare_casgn.rb
test/whitequark/test_block_arg_combinations_25.rb
payload/binary/binary.h
test/testdata/parser/error_recovery_multiple_stmts.rb
gems/sorbet/test/hidden-method-finder/simple/src/sorbet/config
main/lsp/BUILD
main/pipeline/semantic_extension/DefaultImplementation.cc
test/cli/folder-input-not-dir/folder-input-not-dir.out
rbi/stdlib/base64.rbi
test/whitequark/test_arg_combinations_5.parse-tree-whitequark.exp
test/whitequark/test_parser_bug_518_0.parse-tree-whitequark.exp
common/statsd/statsd-emscripten.cc
test/testdata/parser/and_and_bug.rb.parse-tree.exp
website/blog/2019-05-16-state-of-sorbet-spring-2019.md
test/testdata/lsp/bad_field_edits.1.rbupdate
test/cli/sigil-rbi/multiple_definition.rbi
test/testdata/parser/fuzz_def_begin.rb
test/cli/override-typed-bad/override-typed-bad.yaml
test/testdata/lsp/fast_path/method_change_return_type__def.1.rbupdate
test/whitequark/test_for_0.rb
gems/sorbet-runtime/lib/types/private/types/not_typed.rb
common/kvstore/test/kvstore_test.cc
gems/sorbet/test/snapshot/partial/type_member
website/docs/troubleshooting.md
test/testdata/infer/generics/Boxes.rb
gems/sorbet-runtime/test/types/props/decorator.rb
gems/sorbet/test/snapshot/partial/non-utf-8-file/src
test/testdata/resolver/bad_param_ordering.rb
gems/sorbet-static/sorbet-static.gemspec
test/cli/folder-input-not-dir
gems/sorbet-runtime/test/types/props/serializable.rb
test/testdata/resolver/infinite.rb
test/testdata/resolver/field_across_file__01.rb
test/whitequark/test_send_binary_op_2.parse-tree-whitequark.exp
test/testdata/rbi/array.rb
rbi/core/object.rbi
test/cli/progressbar/progressbar.out
test/testdata/infer/generics/bad_bound_typed_false.rb
test/testdata/desugar/defined.rb
test/testdata/infer/kwargs.rb
test/cli/model_mutator_behavior
test/whitequark/test_var_and_asgn_0.rb
gems/sorbet/lib/create-config.rb
test/cli/kwarg-loc/kwarg-loc.out
test/cli/error-url/error-url.out
rbi/stdlib/webrick.rbi
main/lsp/ErrorReporter.cc
plugin/BUILD
main/lsp/wrapper.cc
test/whitequark/test_bug_cmdarg_1.rb
proto/Loc.proto
test/cli/ignore-slash/ignore-slash.sh
rbi/tools
test/testdata/infer/generics/bounds.rb
test/cli/autocorrect-lazy-type-alias
test/cli/cache-keeps-print-options/cache-keeps-print-options.rb
main/lsp/lsp.h
test/whitequark/test_args_args_comma_0.parse-tree-whitequark.exp
test/testdata/parser/error_recovery_const_case.rb.parse-tree.exp
test/whitequark/test_asgn_keyword_invalid_0.rb
test/testdata/lsp/hover_method_for_building_arrays_and_hashes.rb
test/whitequark/test_bug_480_0.parse-tree-whitequark.exp
test/testdata/infer/case_subclass.rb
test/testdata/namer/duplicate_scope.rb
test/testdata/rewriter/prop_in_module.rb.rewrite-tree.exp
test/whitequark/test_next_2.parse-tree-whitequark.exp
test/testdata/desugar/ops.rb.desugar-tree.exp
rbi/core/kernel.rbi
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/open3.rbi
gems/sorbet-runtime/lib/types/types/t_enum.rb
test/cli/index-cache-invalidation/index-cache-invalidation.sh
test/whitequark/test_casgn_invalid_3.rb
test/whitequark/test_bug_435_0.rb
main/lsp/watchman/WatchmanProcess.cc
main/options
test/whitequark/test_arg_scope_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/empty/src
third_party/parser/include
test/testdata/infer/case.rb
test/testdata/cfg/fuzz_recursive_alias.rb
test/testdata/lsp/completion/constants_via_inherit.rb
main/lsp/notifications/sorbet_pause.cc
test/whitequark/test_asgn_mrhs_2.rb
core/lsp/TypecheckEpochManager.cc
rewriter/Struct.h
core/test/core_test.cc
test/testdata/rbi/ruby_typer.rb
proto/pay-server/BUILD
website/static/docs/ruby-3.html
main/pipeline/pipeline.h
test/cli/error-whitelist/error-whitelist.out
test/whitequark/test_send_block_chain_cmd_6.rb
test/whitequark/test_class_super_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/typed-ignore/src/Gemfile.lock
test/whitequark/test_args_args_star_0.rb
gems/sorbet/test/snapshot/partial/bad-hash/src/Gemfile.lock
test/testdata/parser/offset0.rb
gems/sorbet/test/hidden-method-finder/thorough/src/sorbet/rbi
website/static/blog/2019/05/16
rbi/tools/generate_procs.cc
test/whitequark/test_array_words_empty_1.parse-tree-whitequark.exp
test/testdata/rewriter/class_new.rb.rewrite-tree.exp
test/testdata/resolver/sig_in_block.rb
test/whitequark/test_block_arg_combinations_8.rb
ast/ast.h
test/testdata/infer/must.rb
test/whitequark/test_const_toplevel_0.parse-tree-whitequark.exp
test/whitequark/test_break_block_0.rb
test/testdata/namer/constant_in_block.rb.symbol-table-raw.exp
gems/sorbet/test/snapshot/partial/bad-t/src
gems/sorbet/test/hidden-method-finder/thorough/src/sorbet/rbi/hidden-definitions/errors.txt
website/README.md
test/whitequark/test_bug_heredoc_xstring_0.rb
test/testdata/lsp/hover_untyped_proc_arg.rb
test/whitequark/test_symbol_plain_0.rb
test/testdata/lsp/completion/snippet_keywords.A.rbedited
test/fuzz/empty.cc
test/testdata/desugar/ensure_without_rescue.rb
test/testdata/infer/singleton_if_exhaustiveness.rb
test/whitequark/test_cond_eflipflop_0.parse-tree-whitequark.exp
test/testdata/infer/meta_types.rb
test/whitequark/test_masgn_attr_1.rb
test/whitequark/test_send_binary_op_3.rb
test/cli/stop-after-namer/stop-after-namer.sh
test/whitequark/test_int_0.rb
cfg/Instructions.h
test/cli/file-table-json/file-table-json.sh
test/cli/autogen-errors/autogen-errors.sh
test/testdata/rbi/falseclass.rb
test/cli/folder-input-not-found/folder-input-not-found.sh
test/testdata/infer/block_given.rb
website/static/blog/2019/05/16/State-of-Sorbet-May-2019/index.html
test/whitequark/test_ruby_bug_11873_3.rb
gems/sorbet-runtime/lib/types/types/type_member.rb
rewriter/TypeMembers.h
test/whitequark/test_asgn_keyword_invalid_4.rb
test/cli/error-whitelist/error-whitelist.sh
test/whitequark/test_arg_duplicate_ignored_1.parse-tree-whitequark.exp
test/whitequark/test_cpath_0.parse-tree-whitequark.exp
test/testdata/resolver/type_member_constant_assignment.rb
test/testdata/lsp/fast_path/method_signature_update_generics__def.rb
proto/pay-server/README
test/testdata/namer/fuzz_class_in_field.rb
gems/sorbet-runtime/Gemfile
test/whitequark/test_next_2.rb
test/whitequark/test_args_block_pass_0.parse-tree-whitequark.exp
test/testdata/resolver/unsigged_default.rb
test/cli/model_mutator_behavior/model_mutator_behavior.out
test/testdata/infer/next_in_while.rb
test/cli/forbid-autocorrect-with-e/forbid-autocorrect-with-e.sh
test/cli/autocorrect-private/autocorrect-private.sh
test/testdata/namer/type_alias.rb
gems/sorbet-runtime/test
test/testdata/resolver/stub_assign.rb
test/cli/module-redefinition/module-redefinition.sh
core/GlobalSubstitution.h
test/whitequark/test_kwoptarg_0.rb
test/testdata/namer/conflicting_names.rb
test/testdata/rewriter/rails/migration.rb
test/testdata/namer/constant_redefinition/constant_then_class.rb
gems/sorbet/test/snapshot/partial/rspec-lots/src/Gemfile
gems/sorbet-runtime/test/types/sig.rb
rewriter/Mattr.h
test/cli/print-procs/print-procs.out
core/ErrorQueueMessage.h
test/whitequark/test_break_1.rb
docs/logo/sorbet-logo-45-deg@2x.png
third_party/licenses/rapidjson.txt
test/cli/web-trace-file
test/cli/autogen-errors
gems/sorbet-runtime/lib/types/props/private
rbi/stdlib/e2mmap.rbi
test/whitequark/test_bang_cmd_0.rb
core/lsp/QueryResponse.h
test/cli/configatron
test/whitequark/test_bug_rescue_empty_else_0.rb
test/whitequark/test_int_2.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/class_change_include_multifile__using.rb
test/whitequark/test_nil_expression_0.rb
test/cli/counters
test/testdata/infer/segfault_generic.rb
core/proto/BUILD
test/LSPTest.h
test/testdata/autogen/generator.rb.autogen.exp
test/whitequark/test_masgn_nested_0.rb
test/whitequark/test_send_lambda_args_noparen_1.rb
test/whitequark/test_ruby_bug_13547_1.rb
third_party/blake2.BUILD
tools/toolchain/webasm-linux/cc_toolchain_config.bzl
test/whitequark/test_send_plain_0.parse-tree-whitequark.exp
test/cli/override-typed-bad/override-typed-bad.out
tools/scripts/cfg-view.sh
test/cli/rbi-overrides/t3.rbi
test/testdata/cfg/blocks.rb.cfg.exp
rewriter/ProtobufDescriptorPool.cc
test/testdata/resolver/sig_compat.rb
test/testdata/resolver/let_errors.rb.symbol-table-raw.exp
test/whitequark/test_complex_0.rb
gems/sorbet-runtime/test/types/types.rb
test/testdata/namer/superclass_redefinition.rb.symbol-table-raw.exp
test/whitequark/test_block_arg_combinations_4.parse-tree-whitequark.exp
test/whitequark/test_while_1.rb
test/testdata/resolver/resolution_scoping.rb.symbol-table-raw.exp
test/whitequark/test_arg_combinations_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_9.parse-tree-whitequark.exp
test/whitequark/test_masgn_splat_8.rb
test/testdata/namer/arguments.rb
test/testdata/rewriter/t_struct/inexact.rb
test/whitequark/test_float_0.parse-tree-whitequark.exp
test/whitequark/test_bug_interp_single_0.rb
test/testdata/cfg/dealias_with_return.rb.cfg.exp
test/whitequark/test_arg_combinations_4.rb
core/tools
test/whitequark/test_send_binary_op_17.rb
proto
test/testdata/infer/shape_merge.rb
tools/scripts/lint_cxx.sh
test/cli/suggest-sig-override
test/whitequark/test_nil_0.parse-tree-whitequark.exp
docs/pipeline.md
gems/sorbet/test/snapshot/partial/db_schema/src/sorbet
test/cli/autocorrect-remove-body/autocorrect-remove-body.out
gems/sorbet/test/hidden-method-finder/thorough/expectations.json
main/lsp/lsp_helpers.cc
test/whitequark/test_ruby_bug_12402_0.parse-tree-whitequark.exp
test/testdata/infer/fuzz_tripleeq_sig_suggestion.rb
test/testdata/lsp/completion/snippet_arity.C.rbedited
website/style-guide.md
test/whitequark/test_casgn_invalid_0.rb
test/testdata/infer/more_after_return.rb
test/whitequark/test_defined_0.parse-tree-whitequark.exp
test/cli/suggest-constant-type/suggest-constant-type.out
test/testdata/resolver/sealed_aliases.rb
test/whitequark/test_rescue_else_ensure_0.parse-tree-whitequark.exp
test/testdata/rbi/string.rb
common/crypto_hashing/crypto_hashing.h
gems/sorbet/test/snapshot/partial/stack_master/src/Gemfile.lock
gems/sorbet-runtime/bench/setters.rb
test/whitequark/test_send_plain_cmd_2.rb
test/testdata/infer/generics/bounds_super.rb
test/whitequark/test_optarg_1.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/open3.rbi
test/cli/autocorrect-abstract/pre.rb
test/testdata/resolver/bind.rb
test/testdata/desugar/sclass_inheritance.rb
third_party/parser/include/ruby_parser/node.hh
test/testdata/infer/singleton_methods.rb
gems/sorbet-runtime/lib/types/props/utils.rb
test/cli/suggest-constant-type/suggest-constant-type.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/expected
test/helpers
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_class__3.rb
test/cli/phases
website/sidebars.json
test/testdata/resolver/nested_sealed.rb
test/cli/rbi-overrides
test/testdata/resolver/fuzz_include_infinite_typealias.rb
test/whitequark/test_log_asgn_invalid_0.rb
gems/sorbet/test/hidden-method-finder/simple/ruby_2_6_hidden.rbi.exp
test/cli/suggest-constant-type/suggest-constant-type.sh
test/whitequark/test_ruby_bug_9669_0.parse-tree-whitequark.exp
resolver/resolver.cc
test/cli/subprocess-plugin/trigger_bad_plugin.rb
rewriter/Prop.h
test/testdata/infer/transitive.rb
gems/sorbet/test/snapshot/partial/rails6/src
gems/sorbet/test/snapshot/partial/bad_gem/sorbet
test/cli/folder-input/input/subfolder/baz.rb
test/testdata/cfg/rescue_two_return.rb.cfg.exp
test/testdata/infer/case_all.rb
website/docs/legal
core/Loc.cc
test/whitequark/test_block_arg_combinations_26.rb
test/testdata/infer/hashes.rb
test/cli/conflicting-definition/conflicting-definition.sh
test/testdata/desugar/splat.rb.desugar-tree.exp
test/testdata/infer/blocks2.rb.cfg.exp
test/cli/parallel-ordering
test/testdata/desugar/top_level_const.rb.symbol-table-raw.exp
test/whitequark/test_ambiuous_quoted_label_in_ternary_operator_2.rb
test/testdata/lsp/hover_ancestors.rb
gems/sorbet/test/snapshot/partial/fake-rails/src/config
test/cli/suggest_static/suggest_static.sh
gems/sorbet-runtime/lib/types/types
test/testdata/infer/nilable_and.rb
namer/namer.cc
test/whitequark/test_class_definition_in_while_cond_3.rb
test/whitequark/test_ruby_bug_12402_8.rb
test/testdata/autogen/multiple_alias.rb.autogen.exp
test/testdata/infer/control_flow/normalize_params.rb.cfg.exp
test/cli/sigil-rbi/sigil-rbi.sh
test/whitequark/test_arg_label_0.rb
test/testdata/infer/fuzz_dangling_type_parameter.rb
gems/sorbet/test/snapshot/partial/bad-t
third_party/libprotobuf-mutator.BUILD
main/lsp/LSPTask.h
test/whitequark/test_bang_0.parse-tree-whitequark.exp
test/cli/error-blacklist/error-blacklist.out
test/whitequark/test_and_or_masgn_1.parse-tree-whitequark.exp
test/whitequark/test_hash_label_0.rb
gems/sorbet/test/snapshot/partial/ignore_file_table/expected
gems/sorbet-runtime/test/types/returns.rb
test/testdata/rewriter/t_struct/param_order.rb.rewrite-tree.exp
third_party/statsd.BUILD
rbi/core/dir.rbi
test/cli/no-did-you-mean/no-did-you-mean.sh
test/whitequark/test_send_op_asgn_conditional_0.parse-tree-whitequark.exp
test/testdata/desugar/file.rb.parse-tree.exp
test/testdata/resolver/resolution_order.rb.symbol-table-raw.exp
gems/sorbet/test/snapshot/partial/bad-hash/expected/sorbet
test/cli/errors
test/whitequark/test_complex_1.parse-tree-whitequark.exp
test/testdata/desugar/nthref.rb
main/lsp/wrapper.h
test/whitequark/test_send_binary_op_0.rb
test/cli/bad-plugin-spec/bad-plugin-spec.out
test/whitequark/test_rescue_in_lambda_block_0.parse-tree-whitequark.exp
rbi/BUILD
test/cli/autogen-autoloader/foo.rb
test/testdata/cfg/uaf1.rb
gems/sorbet-runtime/lib/types/types/class_of.rb
cfg
test/whitequark/test_self_0.rb
test/whitequark/test_casgn_scoped_0.parse-tree-whitequark.exp
test/testdata/namer/multiple_stubs.rb.symbol-table-raw.exp
test/testdata/namer/arguments.rb.flatten-tree-raw.exp
test/testdata/resolver/let_var.rb
test/whitequark/test_complex_2.parse-tree-whitequark.exp
test/testdata/infer/kwargs_generics.rb
test/whitequark/test_parser_bug_490_1.rb
website/static/img/sorbet-logo-white-sparkles.svg
test/whitequark/test_def_2.rb
rbi/stdlib/monitor.rbi
gems/sorbet/test/snapshot/partial/bad-hash
test/whitequark/test_bug_466_0.rb
test/whitequark/test_kwrestarg_unnamed_0.rb
test/testdata/autogen/foo.fixturerb.autogen.exp
gems/sorbet/test/snapshot/partial/fake-rails/src/config/application.rb
third_party/openssl
test/cli/parallel-ordering/parallel-ordering.sh
test/testdata/namer/constant_types.rb.symbol-table-raw.exp
common/Levenstein.h
test/whitequark/test_super_1.rb
test/whitequark/test_ruby_bug_14690_0.rb
test/testdata/resolver/linearization/linearization3.rb
test/testdata/desugar/ops.rb
test/testdata/infer/flatten_private.rb
test/cli/rbi-overrides/rbi-overrides.sh
test/cli/cache-add-typed
definition_validator/BUILD
test/whitequark/test_send_lambda_2.rb
test/testdata/infer/pinned_control_flow.rb
test/testdata/lsp/cvar__definition.rb
gems/sorbet/test/snapshot/partial/use-existing-config/src/Gemfile
test/whitequark/test_class_1.rb
cfg/builder
test/whitequark/test_defined_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/ruby/all
test/testdata/namer/constants.rb.symbol-table-raw.exp
website/docs/sealed.md
test/cli/print_generics
test/testdata/rewriter/dsl_builder.rb
test/testdata/desugar/csend.rb
main/lsp/tools
test/testdata/infer/loop_in_self_reassignments.rb
test/whitequark/test_if_mod_0.parse-tree-whitequark.exp
rbi/stdlib/csv.rbi
test/whitequark/test_bug_do_block_in_hash_brace_3.rb
test/whitequark/test_masgn_const_1.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/gem.rbi
test/cli/override-typed/override-typed.sh
resolver/CorrectTypeAlias.h
core/test
test/cli/version-returns-0
website/docs/attached-class.md
test/testdata/error_recovery_send_after_end.rb.parse-tree.exp
test/cli/sigil-rbi/sigil-rbi.out
gems/sorbet-runtime/lib/types/private/abstract
website/static
test/whitequark/test_ternary_ambiguous_symbol_0.rb
test/testdata/parser/invalid_trailing_in_number.rb
sorbet_version/sorbet_version.c
test/testdata/namer/redefines_object.rb
test/whitequark/test_until_0.parse-tree-whitequark.exp
test/testdata/lsp
test/testdata/resolver/field.rb.flatten-tree-raw.exp
test/testdata/infer/subtype_symbol.rb
main/pipeline/semantic_extension/SemanticExtension.h
test/testdata/infer/overloads_test.rb
test/cli/suggest-not-stub
test/whitequark/test_args_args_star_0.parse-tree-whitequark.exp
test/whitequark/test_bug_481_0.parse-tree-whitequark.exp
test/whitequark/test_masgn_1.rb
test/testdata/resolver/fuzz_alias_to_type_alias.rb
Rakefile
test/testdata/cfg/retry_nested.rb.cfg.exp
core/Files.cc
test/whitequark/test_asgn_cmd_0.rb
gems/sorbet-runtime/lib/types/types/union.rb
test/testdata/desugar/defined.rb.desugar-tree.exp
test/cli/constant-fuzzy
gems/sorbet-runtime/lib/types/props/private/parser.rb
test/whitequark/test_unknown_percent_str_0.rb
test/testdata/lsp/field_edits.2.rbupdate
common/web_tracer_framework/tracing.cc
test/cli/forbid-autocorrect-with-quiet
test/whitequark/test_block_arg_combinations_23.rb
test/whitequark/test_send_index_asgn_0.parse-tree-whitequark.exp
test/whitequark/test_string_FILE_0.rb
gems/sorbet/test/snapshot/partial/ignore_file_table/src/sorbet
test/cli/suggest-typed-true
gems/sorbet-runtime/lib/types/runtime_profiled.rb
test/testdata/cfg/singleton_lazy.rb
test/testdata/rewriter/prop_in_module.rb
test/cli/errors/errors.sh
test/testdata/resolver/resolve_tree_printing.rb.flatten-tree-raw.exp
core/types/subtyping.cc
core/lsp/Task.h
rbi/core/process.rbi
common/FileOps.h
test/testdata/desugar/op_eq.rb.desugar-tree.exp
test/testdata/resolver/type_member_in_method.rb
test/cli/ignore-slash/ignore-slash.out
test/whitequark/test_array_symbols_empty_0.parse-tree-whitequark.exp
test/testdata/autogen/hidden_include.rb
test/whitequark/test_bug_cmd_string_lookahead_0.parse-tree-whitequark.exp
test/cli/module-redefinition
test/testdata/infer/restargsbox.rb
common/Timer.h
gems/sorbet-runtime/lib/types/private/abstract/validate.rb
test/cli/web-trace-file/web-trace-file.out
test/whitequark/test_when_multi_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/non-utf-8-file/src/Gemfile
test/testdata/namer/class_and_alias.rb.symbol-table-raw.exp
test/testdata/resolver/type_member_fixed_order.rb
test/testdata/resolver/type_member_singleton_members.rb
test/testdata/rewriter/rails/class_attribute.rb
test/whitequark/test_rescue_ensure_0.parse-tree-whitequark.exp
rbi/core/false_class.rbi
test/cli/autocorrect-lazy-type-alias/autocorrect-lazy-type-alias.sh
test/testdata/rewriter/t_enum_snapshot.rb.rewrite-tree.exp
test/whitequark/test_cond_eflipflop_1.parse-tree-whitequark.exp
gems/sorbet/lib/require_everything.rb
gems/sorbet/test/hidden-method-finder/hidden_methods.bzl
gems/sorbet-runtime/lib/types/props/decorator.rb
gems/sorbet/test/snapshot/partial/create-config/src/Gemfile.lock
third_party/parser/include/ruby_parser/token.hh
gems/sorbet/test/snapshot/total/sorbet-runtime/expected
test/cli/suggest-foreign-lambda/suggest-foreign-lambda.sh
test/whitequark/test_masgn_splat_6.rb
test/lsp/ProtocolTest.h
infer/SigSuggestion.cc
test/testdata/cfg/side_effects2.rb
test/cli/suggest_autogen/suggest_autogen.out
test/whitequark/test_kwarg_invalid_0.rb
test/testdata/resolver/self.rb
test/cli/remove-path-prefix-no-match
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/err.log
rewriter/Command.h
test/cli/parser-error/parser-error.sh
test/whitequark/test_ruby_bug_11873_a_8.parse-tree-whitequark.exp
rbi/stdlib/date.rbi
test/whitequark/test_resbody_var_0.rb
test/whitequark/test_masgn_0.parse-tree-whitequark.exp
test/whitequark/test_asgn_keyword_invalid_3.rb
test/testdata/resolver/type_member_cycle.rb
test/cli/config-file/sorbet/bad_no_config
gems/sorbet/test/snapshot/validate_utils.sh
test/whitequark/test_symbol_plain_1.rb
common/statsd/BUILD
test/cli/rbi-with-code/rbi-with-code.sh
test/whitequark/test_parser_bug_490_0.rb
test/testdata/lsp/fast_path/class_remove_member.rb
test/whitequark/test_arg_combinations_10.rb
test/whitequark/test_marg_combinations_1.rb
test/whitequark/test_range_endless_1.rb
test/testdata/cfg/hash.rb.cfg.exp
test/whitequark/test_marg_combinations_5.rb
test/testdata/lsp/field_edits.2.rbupdate.document-symbols.exp
test/cli/subprocess-plugin/bad_plugin.rb
test/testdata/desugar/file.rb
gems/sorbet-runtime/lib/types/private/final.rb
test/whitequark/test_lbrace_arg_after_command_args_0.rb
test/testdata/cfg/retry.rb.desugar-tree.exp
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/bundler
test/testdata/lsp/completion/sig_snippet.B.rbedited
test/whitequark/test_arg_combinations_12.parse-tree-whitequark.exp
test/testdata/infer/transitive.rb.cfg.exp
test/testdata/cfg/rescue_expression.rb
test/testdata/infer/proc_args.rb
ast/treemap/BUILD
docs/README.md
rbi/core/marshal.rbi
test/testdata/resolver/alias_define_method__01.rb
test/testdata/rewriter/prop.rb.rewrite-tree-raw.exp
test/whitequark/test_while_0.rb
website/docs/talks/ruby-conf-2019.md
test/testdata/desugar/multi_assign.rb
gems/sorbet/test/hidden-method-finder/simple/expectations.json
test/testdata/infer/control_flow/truthiness_bug.rb
test/testdata/infer/yield_inside_block.rb
main/lsp/DefLocSaver.cc
test/whitequark/test_arg_combinations_11.rb
tools/toolchain/webasm-darwin/ar.sh
README.md
test/testdata/cfg/reassign_dead_block_bug.rb
test/testdata/lsp/genericMethods.rb
test/whitequark/test_send_block_chain_cmd_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib
test/whitequark/test_ensure_0.rb
ast/substitute/BUILD
test/testdata/parser/index_assign.rb
test/testdata/infer/class.rb
test/testdata/infer/dead_after_return.rb
plugin
rbi/stdlib/digest.rbi
main/lsp/tools/make_lsp_types.h
test/testdata/resolver/bad_sealed_class_absurd__1.rb
test/testdata/lsp/document_symbols.rb
test/cli/config-file-recursive/config-file-recursive.sh
test/testdata/resolver/unnamed_proc_arguments.rb
test/whitequark/test_hash_empty_0.parse-tree-whitequark.exp
test/whitequark/test_cond_match_current_line_1.parse-tree-whitequark.exp
test/whitequark/test_bang_0.rb
test/testdata/infer/case_when_any.rb
test/testdata/call_with_block_strict.rb
test/cli/escaping/escaping.rb
main/lsp/requests/references.h
test/whitequark/test_yield_3.parse-tree-whitequark.exp
gems/sorbet-runtime/test/types
test/whitequark/test_arg_combinations_11.parse-tree-whitequark.exp
test/whitequark/test_block_kwarg_0.parse-tree-whitequark.exp
test/whitequark/test_arg_0.rb
test/cli/subprocess-plugin/foo_gen.rb
test/testdata/resolver/constant_in_typealias.rb
test/whitequark/test_case_expr_else_0.parse-tree-whitequark.exp
test/whitequark/test_complex_1.rb
test/cli/autocorrect-abstract/autocorrect-abstract.out
gems/sorbet/test/snapshot/partial/ignore_file_table/src/sorbet/config
resolver/type_syntax.cc
test/testdata/namer/gvar.rb
test/testdata/class_not_class_of.rb
gems/sorbet/test/snapshot/partial/db_schema/src/db
test/whitequark/test_case_expr_0.rb
parser/Builder.cc
test/whitequark/test_retry_0.parse-tree-whitequark.exp
test/testdata/resolver/overloads_test.rb
test/testdata/deviations/non_ruby_names.rb.flatten-tree.exp
test/whitequark/test_block_arg_combinations_21.parse-tree-whitequark.exp
test/testdata/lsp/completion/intersection.rb
test/testdata/resolver/type_members.rb
gems/sorbet-runtime/lib/types/private/sealed.rb
rbi/core/nil_class.rbi
test/cli/cache-doesnt-parse
gems/sorbet-runtime/lib/types/props/serializable.rb
test/testdata/infer/control_flow
test/whitequark/test_true_0.rb
test/testdata/desugar/file.rb.desugar-tree.exp
test/cli/sigil-rbi/typed.rbi
test/whitequark/test_and_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_19.parse-tree-whitequark.exp
test/testdata/infer/flatten_methods.rb
test/cli/constant-fuzzy/constant-fuzzy.out
test/cli/suggest-not-stub/suggest-not-stub.out
test/testdata/lsp/completion/overloads_test.A.rbedited
test/cli/suggest-t-let/suggest-t-let.out
test/whitequark/test_var_op_asgn_0.parse-tree-whitequark.exp
test/whitequark/test_masgn_keyword_invalid_0.rb
test/cli/allowed-extension/lib/d.rake
main/lsp/LocalVarSaver.cc
test/whitequark/test_def_4.rb
local_vars/BUILD
test/testdata/infer/sealed_singleton_class.rb
test/whitequark/test_blockarg_0.rb
test/whitequark/test_ruby_bug_12402_8.parse-tree-whitequark.exp
test/cli/typed-src
gems/sorbet-runtime/lib/types/private/types/string_holder.rb
test/testdata/lsp/missing_typed_sigil.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/src
website/i18n
main/lsp
test/testdata/namer/self_constant.rb
test/cli/autocorrect-bare-stdlib-generics/autocorrect-bare-stdlib-generics.rb
gems/sorbet/test/snapshot/partial/bad-hash/src/Gemfile
test/testdata/lsp/completion/sig_no_defs.A.rbedited
test/whitequark/test_while_1.parse-tree-whitequark.exp
test/whitequark/test_asgn_keyword_invalid_2.rb
test/testdata/resolver/proc.rb.symbol-table-raw.exp
gems/sorbet-runtime/lib/types/private/methods/signature.rb
test/testdata/rbi/object.rb
gems/sorbet/test/snapshot/partial/bad_gem/src/bad-gem.gemspec
test/testdata/infer/lub_and_glb_corner_cases.rb
test/testdata/rewriter/interface_wrapper.rb.rewrite-tree.exp
test/testdata/infer/sigil.rb
resolver
test/cli/no-error-count/no-error-count.sh
test/testdata/rewriter/flatten_module_private_class_method.rb
test/testdata/resolver/bind_class_of.rb.symbol-table-raw.exp
test/testdata/cfg/next_in_while.rb
test/whitequark/test_send_block_chain_cmd_1.rb
test/testdata/todo/block_in_class.rb
test/whitequark/test_var_or_asgn_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_a_17.parse-tree-whitequark.exp
test/testdata/infer/loop_with_rescue_next.rb
tools/config
test/whitequark/test_ruby_bug_12402_3.rb
gems/sorbet/test/snapshot/partial/db_schema/expected
test/testdata/lsp/completion/constants_magic.rb
gems/sorbet-runtime/test/types/must.rb
test/testdata/desugar/fuzz_bad_loc_dstring.rb
common/sort.h
test/testdata/resolver/self_ancestor.rb
test/lsp/incremental-lsp-changes/incremental-lsp-changes.rec
test/testdata/infer/false_dead_code.rb
infer/test
gems/sorbet-runtime/test/types/fixtures/always_raise.rb
test/whitequark/test_ruby_bug_12402_13.parse-tree-whitequark.exp
test/backtrace-test-error.cc
third_party/licenses/BUILD
test/testdata/resolver/let_errors_nilable.rb
core/lsp/Query.cc
test/testdata/resolver/fuzz_sig_weird.rb
test/testdata/rbi/module.rb
website/static/blog/2019/05/16/State-of-Sorbet-May-2019
tools/toolchain/webasm-linux
test/testdata/rewriter/default_args_edge.rb
test/whitequark/test_arg_duplicate_3.rb
test/testdata/lsp/hover_override.rb
test/whitequark/test_ruby_bug_12402_11.parse-tree-whitequark.exp
test/whitequark/test_nth_ref_0.parse-tree-whitequark.exp
test/whitequark/test_nil_0.rb
test/testdata/lsp/completion/locals.rb
test/testdata/lsp/completion/snippet_arity.D.rbedited
test/whitequark/test_ruby_bug_11380_0.rb
test/cli/print_generics/print_generics.rb
test/testdata/cfg/rescue_else_block.rb.cfg.exp
gems/sorbet/test/snapshot/partial/codecov/src/Gemfile
gems/sorbet/test/snapshot/partial/typed-ignore/src/Gemfile
test/whitequark/test_arg_label_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/rails-double-require/src/Gemfile
test/cli/cache-reserve-mem/cache-reserve-mem.out
test/whitequark/test_bug_447_1.parse-tree-whitequark.exp
core/proto
test/whitequark/test_case_cond_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/non-utf-8-file/expected/sorbet
test/cli/phases/phases.sh
third_party/parser/include/ruby_parser/builder.hh
tools/scripts/lint_sh.sh
gems/sorbet-runtime/test/types/sealed_module.rb
test/testdata/desugar/sclass.rb.symbol-table-raw.exp
test/whitequark/test_rescue_0.rb
test/testdata/lsp/fast_path/string_literal_change.rb
test/testdata/rbi/date.rb
test/whitequark/test_marg_combinations_6.parse-tree-whitequark.exp
test/testdata/rewriter/not_prop.rb
test/testdata/namer/array_sum.rb
website/docs/self-type.md
test/whitequark/test_arg_duplicate_7.rb
main/lsp/LSPTypecheckerCoordinator.cc
test/whitequark/test_op_asgn_invalid_0.rb
third_party/licenses/crcpp.txt
test/whitequark/test_if_else_1.rb
third_party/parser/include/ruby_parser/capi.hh
website/docs/type-annotations.md
test/cli/subprocess-plugin/multi3.rb
test/cli/autocorrect-remove-body/pre.rbi
test/cli/folder-input/folder-input.out
test/cli/suggest-typos/suggest-typos.sh
test/whitequark/test_send_lambda_1.rb
test/testdata/rewriter/t_struct/default_bad.rb
test/testdata/autogen/resolving_refs.rb
test/testdata/resolver/bad_alias.rb
test/cli/parse-tree-whitequark/parse-tree-whitequark.rb
gems/sorbet-runtime/bench/prop_definition.rb
test/testdata/parser/error_recovery_assign.rb.parse-tree.exp
test/testdata/lsp/untyped_field_reference__2.rb
test/whitequark/test_array_splat_2.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_1.rb
test/testdata/rewriter/protobuf_descriptor_pool.rb
test/testdata/disabled
test/testdata/namer/constant_types.rb
test/whitequark/test_ivar_0.rb
test/testdata/infer/control_flow/singletons.rb
test/testdata/lsp/completion/constants_root.rb
ast/Helpers.h
test/whitequark/test_send_attr_asgn_3.parse-tree-whitequark.exp
test/testdata/lsp/completion/snippet_block_arity.A.rbedited
test/cli/module-redefinition/module-redefinition-2.rb
test/whitequark/test_ruby_bug_11873_a_14.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/class_add_member.1.rbupdate
core/tools/generate_names.cc
website/docs/sigs.md
test/whitequark/test_not_0.rb
test/testdata/infer/isa_bug.rb
test/testdata/resolver/sig_bad.rb.symbol-table-raw.exp
test/cli/remove-path-prefix-https/remove-path-prefix-https.sh
test/whitequark/test_cond_begin_0.parse-tree-whitequark.exp
test/testdata/desugar/sclass_inheritance.rb.symbol-table-raw.exp
ast/desugar/test
test/whitequark/test_redo_0.rb
test/testdata/infer/magic_dead.rb
test/testdata/infer/glb_generic_with_class.rb
test/cli/override-typed-bad/bad-strictness.yaml
main/lsp/requests/references.cc
website/static/js/error-reference.js
test/testdata/lsp/fast_path/method_signature_update_static__def.rb
test/testdata/rewriter/attr_multi.rb.symbol-table-raw.exp
test/whitequark/test_while_mod_0.parse-tree-whitequark.exp
test/testdata/resolver/bad_sealed_module__1.rb
test/whitequark/test_resbody_list_var_0.rb
test/testdata/resolver/sig_misc.rb
rewriter/SelfNew.cc
test/whitequark/test_masgn_splat_2.rb
test/whitequark/test_args_args_star_1.rb
test/testdata/lsp/fast_path/method_add_sig.1.rbupdate
gems/sorbet/lib/fetch-rbis.rb
core/Types.h
test/whitequark/test_unless_0.parse-tree-whitequark.exp
test/whitequark/test_send_attr_asgn_2.parse-tree-whitequark.exp
test/testdata/infer/blocks2.rb
test/testdata/lsp/fast_path/undefined_constant.rb
website/docs/flow-sensitive.md
test/cli/metrics-file/metrics-file.sh
test/testdata/infer/setters.rb
gems/sorbet/test/snapshot/partial/real_singleton_class/src/Gemfile
test/whitequark/test_array_symbols_interp_0.parse-tree-whitequark.exp
test/cli/at
test/testdata/infer/control_flow/blank_p.rb
gems/sorbet/test/snapshot/total/empty/expected/sorbet
gems/sorbet/test/hidden-method-finder/thorough
test/whitequark/test_ruby_bug_11873_a_0.rb
main/lsp/requests/completion.cc
test/testdata/desugar/dsymbol.rb
gems/sorbet/lib/step_interface.rb
test/testdata/rbi/etc.rb
test/cli/suggest-sig-garbage/suggest-sig-garbage.out
rbi/stdlib/date_time.rbi
test/whitequark/test_optarg_0.rb
test/testdata/lsp/completion/sig_no_method.A.rbedited
test/whitequark/test_ruby_bug_12402_4.parse-tree-whitequark.exp
rbi/stdlib/tempfile.rbi
test/whitequark/test_var_op_asgn_3.parse-tree-whitequark.exp
test/whitequark/test_masgn_backref_invalid_0.rb
gems/sorbet-runtime/lib/types/private/abstract/data.rb
test/whitequark/test_lparenarg_after_lvar_since_25_0.parse-tree-whitequark.exp
test/testdata/namer/type_member_redefs__2.rb
test/whitequark/test_const_op_asgn_0.parse-tree-whitequark.exp
test/testdata/resolver/bad_sealed_class_absurd__2.rb
test/whitequark/test_space_args_arg_block_0.rb
main/options/options.h
test/testdata/resolver/fuzz_sig_parsing.rb
test/cli/override-typed/override-typed.rb
test/cli/autocorrect/autocorrect.rb
gems/sorbet/test/snapshot/partial/real_singleton_class/src/Gemfile.lock
test/whitequark/test_casgn_invalid_2.rb
test/whitequark/test_hash_empty_0.rb
website/static/img/testimonial_once_every_never.png
test/testdata/perf
test/whitequark/test_break_2.rb
test/testdata/resolver/ancestor_scope.rb
main/lsp/notifications/sorbet_workspace_edit.cc
common/crypto_hashing
test/cli/configatron/configatron.out
test/whitequark/test_defined_0.rb
main/lsp/protocol.cc
test/testdata/lsp/completion/locals_and_methods.rb
test/cli/suggest-singleton/suggest-singleton.out
test/cli/incremental-resolver/expect-failures/multiple_sigs.rb
third_party/gems/gems.BUILD
test/testdata/lsp/code_actions/typo.A.rbedited
test/cli/autogen-errors/autogen-errors.out
rewriter/Delegate.cc
test/whitequark/test_const_unscoped_0.rb
test/whitequark/test_marg_combinations_4.parse-tree-whitequark.exp
test/whitequark/test_send_attr_asgn_0.rb
tools/config/BUILD
test/testdata/rewriter/prop.rb.symbol-table-raw.exp
ast/verifier/verifier.h
test/whitequark/test_case_expr_0.parse-tree-whitequark.exp
class_flatten/BUILD
test/testdata/desugar/range.rb
test/testdata/desugar/csend.rb.desugar-tree.exp
test/whitequark/test_args_star_1.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_1.parse-tree-whitequark.exp
test/testdata/desugar/undef.rb
common/test/common_test.cc
main/lsp/LSPOutput.h
third_party/licenses/cxxopts.txt
test/whitequark/test_send_attr_asgn_0.parse-tree-whitequark.exp
main/pipeline/pipeline.cc
test/testdata/infer/fuzz_oneline_conditional.rb
main/lsp/notifications/sorbet_pause.h
test/whitequark/test_block_arg_combinations_24.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/extconf/src/lib/extconf.rb
resolver/BUILD
gems/sorbet/lib/status.rb
main/autogen/autoloader.cc
test/testdata/namer/dynamic_method_with_class.rb
rbi/core/fixnum.rbi
test/testdata/lsp/hover_method_includes_defs.rb
test/whitequark/test_bug_do_block_in_hash_brace_4.parse-tree-whitequark.exp
test/whitequark/test_not_2.rb
test/testdata/lsp/fast_path/method_change_argument_type__usage.1.rbupdate
test/whitequark/test_kwarg_combinations_2.rb
website/core/PageSection.js
test/whitequark/test_send_self_block_3.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_index_cmd_0.parse-tree-whitequark.exp
docs/logo/sorbet-logo-monochrome.svg
test/whitequark/test_const_scoped_0.rb
test/testdata/autogen/ancestors.rb
test/whitequark/test_module_invalid_0.rb
main/lsp/notifications/cancel_request.cc
test/whitequark/test_send_unary_op_2.rb
gems/sorbet-runtime/lib/types/private/types/type_alias.rb
test/testdata/parser/error_recovery_constant_only_scope.rb.parse-tree.exp
gems/sorbet-runtime/lib/types/private/runtime_levels.rb
test/whitequark/test_marg_combinations_1.parse-tree-whitequark.exp
rbi/gems
third_party/licenses/spdlog.txt
gems/sorbet/bin/srb-rbi
test/testdata/resolver/sig_misc.rb.symbol-table-raw.exp
test/testdata/namer/locals.rb
test/whitequark/test_args_args_star_1.parse-tree-whitequark.exp
test/whitequark/test_const_op_asgn_3.parse-tree-whitequark.exp
test/testdata/parser/invalid_syntax_error.rb
main/lsp/UndoState.h
test/cli/statsd
test/whitequark/test_send_unary_op_1.rb
test/whitequark/test_asgn_cmd_1.parse-tree-whitequark.exp
ast/BUILD
test/cli/forbid-autocorrect-with-quiet/forbid-autocorrect-with-quiet.sh
test/testdata/resolver/bind_class_of.rb
test/cli/parallel-ordering/3.rb
rbi/core/array.rbi
third_party/progressbar
test/cli/print_to_file/print_to_file.sh
tools/toolchain/webasm-darwin/BUILD
test/whitequark/test_next_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/src
cfg/CFG.h
gems/sorbet/test/hidden-method-finder/thorough/src/Gemfile
test/cli/cache-reserve-mem/input.rb
test/testdata/resolver/bare_generics.rb
third_party/licenses/lizard.txt
test/whitequark/test_symbol_plain_0.parse-tree-whitequark.exp
test/testdata/parser/error_recovery_missing_fun.rb.parse-tree.exp
test/whitequark/test_ruby_bug_11873_a_0.parse-tree-whitequark.exp
rbi/core/hash.rbi
test/whitequark/test_send_lambda_args_1.rb
test/cli/suggest_autogen
test/testdata/parser/long_string.rb
test/testdata/lsp/hover_abstract.rb
gems/sorbet-runtime/lib/types/private/methods/signature_validation.rb
test/testdata/lsp/fast_path/method_change_argument_type__def.rb
test/whitequark/test_send_self_1.parse-tree-whitequark.exp
test/testdata/lsp/type_aliases.rb
test/whitequark/test_ruby_bug_9669_0.rb
test/testdata/lsp/completion/no_parens.A.rbedited
test/testdata/infer/casts.rb.cfg.exp
third_party/licenses/tools
test/testdata/infer/metatype_in_lub.rb
test/whitequark/test_next_0.parse-tree-whitequark.exp
proto/BUILD
gems/sorbet-runtime/test/types/method_modes.rb
test/cli/suggest-sig-override-edge/suggest-sig-override-edge.out
test/whitequark/test_parser_bug_507_0.parse-tree-whitequark.exp
rewriter/InterfaceWrapper.cc
gems/sorbet-runtime/test/types/struct.rb
test/whitequark/test_rescue_mod_op_assign_0.rb
third_party/parser/cc
test/testdata/rbi/with_without__3.rb
test/cli/suggest-object/suggest-object.rb
test/cli/autocorrect-strict/pre.rb
test/testdata/infer/return_in_if.rb
test/testdata/rewriter/rails/delegate.rb
test/whitequark/test_ruby_bug_11873_2.rb
test/whitequark/test_ruby_bug_12073_0.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_cmd_3.rb
test/cli/forgot-typed/forgot-typed.sh
test/whitequark/test_send_binary_op_9.parse-tree-whitequark.exp
test/whitequark/test_ivar_0.parse-tree-whitequark.exp
website/docs/untyped.md
core/Symbols.h
test/whitequark/test_block_kwarg_combinations_1.parse-tree-whitequark.exp
test/whitequark/test_const_unscoped_0.parse-tree-whitequark.exp
test/cli/override-typed-bad/override-typed-bad.rb
test/testdata/resolver/stubs_typed_untyped__1.rb
test/cli/suppress-non-critical/suppress-non-critical.out
main/options/test/options_test.cc
test/whitequark/test_op_asgn_index_0.parse-tree-whitequark.exp
test/whitequark/test_send_index_asgn_legacy_0.parse-tree-whitequark.exp
test/whitequark/test_restarg_named_0.parse-tree-whitequark.exp
test/testdata/lsp/cvar__usage.rb
test/testdata/namer/alias_method.rb
test/testdata/cfg/block_in_deadcode.rb.cfg.exp
test/testdata/infer/product.rb
test/whitequark/test_bug_473_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/stupidedi/src/src.rb
test/testdata/resolver/alias.rb.symbol-table-raw.exp
test/whitequark/test_array_assocs_0.parse-tree-whitequark.exp
third_party/openssl/BUILD
test/testdata/cfg/undeclared_variable.rb
test/testdata/deviations/keyword_method_names.rb.parse-tree.exp
main/lsp/LSPConfiguration.cc
test/testdata/namer/extend.rb.symbol-table-raw.exp
test/testdata/rbi/proc_sig_strong.rb
test/testdata/resolver/overrides.rb
rbi/stdlib/cgi.rbi
test/whitequark/test_preexe_invalid_0.rb
test/testdata/resolver/final_module.rb
test/cli/dedup_loc/dedup_loc.sh
test/whitequark/test_ruby_bug_11873_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/typed-ignore
test/whitequark/test_alias_0.rb
common/FileSystem.cc
test/testdata/infer/infer1.rb.symbol-table-raw.exp
test/testdata/rewriter/rails/mattr_accessor.rb
gems/sorbet-runtime/lib/types/private/abstract/declare.rb
test/testdata/infer/reveal_type.rb
test/lsp/workspaceSymbol
test/testdata/cfg/triple_eq.rb
common/concurrency
test/whitequark/test_send_binary_op_7.parse-tree-whitequark.exp
test/testdata/resolver/resolution_scoping.rb
test/whitequark/test_def_3.rb
test/cli/at/at.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/expected/sorbet
test/whitequark/test_ruby_bug_12073_0.rb
test/whitequark/test_asgn_cmd_1.rb
test/whitequark/test_rescue_in_lambda_block_1.rb
tools/scripts/import_whitequark.sh
test/cli/backtrace
test/cli/autogen-autoloader/autogen-autoloader.out
test/testdata/lsp/fast_path/class_remove_member.1.rbupdate
test/testdata/parser/unary_num.rb.parse-tree.exp
website/static/img/atrium-logo.jpg
test/testdata/rewriter/rails/class_attribute.rb.rewrite-tree.exp
gems/sorbet-runtime/lib/types/private/decl_state.rb
gems/sorbet-runtime/test/types/props/_props.rb
test/cli/autocorrect-lazy-type-alias/post.rb
main/lsp/tools/generate_lsp_messages.h
test/testdata/namer/module_function.rb.symbol-table-raw.exp
test/whitequark/test_block_arg_combinations_8.parse-tree-whitequark.exp
test/testdata/autogen/ancestors_ref.rb
test/testdata/namer/redefinition_method.rb
test/whitequark/test_send_plain_cmd_2.parse-tree-whitequark.exp
test/cli/suggest-sig/suggest-sig.rb
test/cli/arity-messages/arity-messages.sh
test/testdata/rbi/regexp.rb
rbi/core
test/testdata/desugar/accidentally_quadratic.rb
gems/sorbet/test/snapshot/total/empty/expected
core/TypeConstraint.h
rewriter/Private.cc
test/whitequark/test_block_arg_combinations_20.parse-tree-whitequark.exp
test/whitequark/test_send_unary_op_1.parse-tree-whitequark.exp
test/testdata/lsp/good_field_edits.rb.document-symbols.exp
ast/Trees.cc
test/cli/config-file/sorbet
test/whitequark/test_ruby_bug_13547_7.rb
tools/buildstamp
parser/Builder.h
test/whitequark/test_send_lambda_2.parse-tree-whitequark.exp
test/whitequark/test_arg_scope_0.rb
gems/sorbet/test/snapshot/check_one.sh
test/cli/incremental-resolver/expect-failures/abstract_impl.rb
test/whitequark/test_ruby_bug_12402_10.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/explosive-object/src
test/testdata/desugar/regexp.rb
test/whitequark/test_break_1.parse-tree-whitequark.exp
test/testdata/resolver/rbi_final_re_sig__2.rb
test/cli/subprocess-plugin/echo_argv.yaml
test/testdata/lsp/field_edits.1.rbupdate
test/whitequark/test_class_0.parse-tree-whitequark.exp
rewriter/ModuleFunction.cc
test/cli/suggest-new
test/cli/suggest-type-alias/suggest-type-alias.out
gems/sorbet-runtime/lib/types/types/typed_hash.rb
test/whitequark/test_bug_rescue_empty_else_0.parse-tree-whitequark.exp
test/cli/bad-plugin-spec/not-map.yaml
test/cli/error-url
test/whitequark/test_method_definition_in_while_cond_3.rb
test/testdata/infer/lub_between_self_params.rb
test/whitequark/test_ambiuous_quoted_label_in_ternary_operator_3.rb
test/testdata/namer/type_alias.rb.symbol-table-raw.exp
rbi/core/constants.rbi
core/errors/namer.h
test/testdata/cfg/extra_bb_args.rb.cfg.exp
gems/sorbet-runtime/test/types/fixtures/sealed_module/whitelist_violation__1.rb
gems/sorbet-runtime/test/types/absurd.rb
gems/sorbet-runtime/test/types/validate_override_types.rb
test/testdata/infer/control_flow/dynamic.rb
main/lsp/lsp_messages_gen_helpers.cc
test/whitequark/test_return_3.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_cmd_0.rb
gems
test/whitequark/test_parser_bug_604_0.parse-tree-whitequark.exp
tools
test/whitequark/test_arg_combinations_0.rb
test/cli/conflicting-definition/a.rb
third_party/parser/cc/grammars
gems/sorbet/test/hidden-method-finder/thorough/src/sorbet/rbi/hidden-definitions
test/cli/rbi-overrides/rbi-overrides.out
test/testdata/rewriter/prop_prohibit_shapes_and_tuples.rb
test/cli/config-file/sorbet/config
test/whitequark/test_parser_bug_490_0.parse-tree-whitequark.exp
test/whitequark/test_arg_invalid_3.rb
test/whitequark/test_masgn_const_0.rb
test/whitequark/test_var_op_asgn_cmd_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/bad-hash/expected/sorbet/config
gems/sorbet/test/snapshot/logging.sh
test/cli/autocorrect
test/testdata/cfg/rescue_two_return.rb
parser/test
website/blog/2019-06-20-open-sourcing-sorbet.md
proto/Type.proto
test/testdata/resolver/linearization/linearization4.rb
test/whitequark/test_rescue_else_useless_1.rb
gems/sorbet-runtime/lib/types/types/type_parameter.rb
test/testdata/infer/generics/others_type_members.rb
test/whitequark/test_if_0.parse-tree-whitequark.exp
test/cli/at/at.out
gems/sorbet/test/snapshot/partial/use-existing-config
test/testdata/rewriter/rails/delegate.rb.rewrite-tree.exp
gems/sorbet-runtime/bench/tasks.rb
gems/sorbet/test/snapshot/partial/bad_gem/expected/sorbet
test/cli/remove-path-prefix
test/whitequark/test_restarg_unnamed_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_2.rb
test/testdata/cfg/floats.rb.desugar-tree.exp
test/whitequark/test_block_arg_combinations_9.parse-tree-whitequark.exp
test/whitequark/test_hash_kwsplat_0.rb
test/testdata/desugar/fuzz_block_pass.rb
docs/logo/sorbet-logo.sketch
gems/sorbet/test/snapshot/partial/fake-object
test/testdata/lsp/completion/constants_via_nesting.rb
test/whitequark/test_send_self_2.rb
test/lint/buildifier/BUILD
test/testdata/lsp/fast_path/string_literal_change.1.rbupdate
rbi/core/enumerator.rbi
gems/sorbet/test/snapshot/partial/missing-instance-methods/src/Gemfile.lock
test/testdata/parser/misc.rb
common/common.h
test/testdata/intrinsics/to_h.rb
test/testdata/lsp/completion/snippet_block.rb
gems/sorbet/test/snapshot/partial/local_gem/expected/sorbet/rbi
main/lsp/requests/shutdown.h
test/whitequark/test_arg_label_2.rb
ast
test/testdata/parser
test/whitequark/test_arg_duplicate_9.rb
test/testdata/infer/control_flow/csend_and.rb
test/whitequark/test_defs_invalid_5.rb
test/testdata/resolver/class_instance_vars.rb.symbol-table-raw.exp
common/crypto_hashing/BUILD
test/whitequark/test_lparenarg_after_lvar_since_25_1.parse-tree-whitequark.exp
test/whitequark/test_send_self_block_0.parse-tree-whitequark.exp
test/whitequark/test_send_self_0.rb
test/whitequark/test_and_asgn_0.rb
gems/sorbet/test/snapshot/partial/fake-rails
test/whitequark/test_args_star_0.rb
test/whitequark/test_cond_match_current_line_1.rb
gems/sorbet-runtime/test/types/props/constructor.rb
test/whitequark/test_ruby_bug_12669_0.rb
test/testdata/resolver/sig_void.rb
test/whitequark/test_send_block_chain_cmd_5.parse-tree-whitequark.exp
test/testdata/lsp/completion/snippet_block_arity.C.rbedited
test/testdata/resolver/simple.rb.flatten-tree.exp
test/testdata/lsp/code_actions/typo.rb
test/whitequark/test_masgn_splat_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/local_gem/expected/sorbet/rbi/gems
test/testdata/resolver/lazy_field.rb
rewriter/SigRewriter.cc
test/testdata/lsp/hover_ampersand_operations.rb
payload/text/tools
test/testdata/resolver/top_level_sig.rb
test/testdata/cfg/retry_nested.rb
test/testdata/rewriter/not_prop.rb.symbol-table.exp
test/whitequark/test_bug_while_not_parens_do_0.parse-tree-whitequark.exp
common/kvstore/KeyValueStore.cc
test/whitequark/test_alias_gvar_1.rb
main/cache/cache.cc
test/whitequark/test_marg_combinations_3.rb
test/testdata/resolver/fuzz_ancester.rb
test/whitequark/test_if_else_0.parse-tree-whitequark.exp
test/cli/conflicting-definition/conflicting-definition.out
test/whitequark/test_send_binary_op_12.rb
rbi/stdlib/open3.rbi
test/whitequark/test_while_post_0.parse-tree-whitequark.exp
website/static/css/custom.css
gems/sorbet/test/snapshot/partial/missing-instance-methods/src
test/whitequark/test_ruby_bug_11873_0.rb
main/lsp/requests/document_symbol.h
gems/sorbet/test/snapshot/partial/stack_master/src/src.rb
test/testdata/cfg/rescue_else_block.rb
rbi/gems/stringscanner.rbi
rbi/gems/chalk.rbi
test/testdata/infer/glb_corner_case.rb
test/whitequark/test_array_splat_0.rb
core/NameRef.h
gems/sorbet-runtime/lib/types/private/methods/modes.rb
third_party/parser/include/ruby_parser/context.hh
test/testdata/namer/next_break.rb.flatten-tree.exp
test/cli/autocorrect-extend/autocorrect-extend.out
test/whitequark/test_cond_iflipflop_1.parse-tree-whitequark.exp
main/options/ConfigParser.cc
test/testdata/cfg/side_effects.rb
test/whitequark/test_parser_bug_490_1.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/class_change_superclass_multifile__child.1.rbupdate
test/testdata/rewriter/rails/cattr_accessor.rb
rewriter
gems/sorbet/test/snapshot/partial/bad_gem/src/src.rb
test/whitequark/test_ruby_bug_10653_1.rb
gems/sorbet/test/snapshot/partial
gems/sorbet-runtime/lib/types/abstract_utils.rb
test/whitequark/test_send_lambda_legacy_0.parse-tree-whitequark.exp
test/cli/ignore/foo.rb
test/whitequark/test_postexe_0.rb
test/whitequark/test_codepoint_too_large_0.rb
test/testdata/rewriter/nesting.rb
common/kvstore/BUILD
test/testdata/infer/toplevel.rb.flatten-tree.exp
third_party/gems
test/cli/configatron-yaml-error/configatron-yaml-error.rb
test/whitequark/test_masgn_splat_6.parse-tree-whitequark.exp
test/whitequark/test_parser_bug_272_0.parse-tree-whitequark.exp
main/lsp/notifications/sorbet_resume.h
test/cli/dash-e/dash-e.sh
test/testdata/rewriter/rails/mattr_reader.rb.rewrite-tree.exp
test/whitequark/test_arg_combinations_8.rb
test/testdata/namer/redefinition_method.rb.flatten-tree.exp
test/testdata/cfg/do_while.rb
test/testdata/rbi/time.rb
test/whitequark/test_nil_expression_1.parse-tree-whitequark.exp
rbi/core/trace_point.rbi
test/testdata/rbi/object_constant.rb
test/testdata/desugar/splat.rb
test/testdata/infer/lub_generics.rb
test/whitequark/test_send_lambda_args_1.parse-tree-whitequark.exp
test/whitequark/test_send_index_legacy_0.parse-tree-whitequark.exp
test/cli/folder-input-not-dir/foo.rb
gems/sorbet-runtime/test/types/casts.rb
common/Subprocess.cc
test/whitequark/test_array_symbols_empty_0.rb
test/testdata/rbi/basicobject_instance_eval.rb
test/testdata/cfg/block_return_bug.rb
cfg/builder/builder.h
test/whitequark/test_string_dvar_0.parse-tree-whitequark.exp
test/whitequark/test_ambiuous_quoted_label_in_ternary_operator_0.parse-tree-whitequark.exp
rewriter/Initializer.cc
gems/sorbet/test/snapshot/partial/bad_gem/src/Gemfile
test/testdata/lsp/completion/duplicate_locals.rb
test/whitequark/test_send_binary_op_14.rb
test/testdata/lsp/code_actions/sig_missing__parent.A.rbedited
test/testdata/resolver/linearization/linearization5.rb.symbol-table-raw.exp
gems/sorbet-runtime/lib/types/types/enum.rb
ast/treemap/treemap.h
gems/sorbet/test/snapshot/total/empty/src/Gemfile
test/testdata/infer/generic_type_template_arg.rb
main/options/BUILD
test/whitequark/test_send_block_chain_cmd_3.rb
third_party/BUILD
test/whitequark/test_send_binary_op_4.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder/thorough/ruby_2_4_hidden.rbi.exp
gems/sorbet-runtime/lib/types/props
test/whitequark/test_method_definition_in_while_cond_0.parse-tree-whitequark.exp
test/whitequark/test_send_conditional_0.parse-tree-whitequark.exp
main/lsp/test
website/siteConfig.js
gems/sorbet-runtime/lib/types/boolean.rb
test/cli/subprocess-plugin/subprocess-plugin.out
test/whitequark/test_marg_combinations_9.rb
common/os/os.cc
test/testdata/infer/generic_methods/genericMethods1.rb
test/whitequark/test_send_unary_op_0.rb
test/whitequark/test_true_0.parse-tree-whitequark.exp
test/testdata/rbi/kernel_array.rb
test/whitequark/test_parser_bug_490_2.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/sig.rb
test/cli/autogen-classlist
infer/SigSuggestion.h
test/whitequark/test_args_assocs_0.parse-tree-whitequark.exp
payload/otherwise_compdb_bugs_out.cc
test/cli/backtrace/backtrace.sh
rewriter/MixinEncryptedProp.h
test/cli/print-procs/print-procs.sh
test/whitequark/test_class_definition_in_while_cond_2.rb
test/whitequark/test_space_args_arg_call_0.rb
test/testdata/rewriter/fuzz_attr_no_args.rb
parser/Node.h
ast/ArgParsing.h
gems/sorbet/test/snapshot/partial/extconf/src/Gemfile
test/whitequark/test_until_post_0.rb
test/whitequark/test_rescue_without_begin_end_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/src/Gemfile.lock
test/testdata/lsp/workspace_symbols_multiply_defined.rb
website/static/img/talk-thumb.png
common/kvstore/KeyValueStore-emscripten.cc
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/bundler
test/testdata/lsp/completion/snippet_keywords.E.rbedited
test/whitequark/test_rational_1.parse-tree-whitequark.exp
test/testdata/cfg/break_in_junk.rb
test/whitequark/test_block_arg_combinations_13.rb
test/whitequark/test_ruby_bug_12402_11.rb
test/testdata/lsp/fast_path/method_add_sig.rb
test/whitequark/test_casgn_scoped_0.rb
test/testdata/parser/compare_overload_parse_error.rb
test/cli/cache-dsl/attr_accessor.rb
test/testdata/desugar/opasgn.rb
test/testdata/lsp/completion/sig_many_defs.A.rbedited
website/core/Footer.js
test/testdata/namer/ancestors.rb
test/whitequark/test_send_binary_op_10.parse-tree-whitequark.exp
test/testdata/namer/simple.rb
rewriter/ProtobufDescriptorPool.h
test/whitequark/test_xstring_plain_0.parse-tree-whitequark.exp
test/cli/bad-perm
test/cli/override-typed/override-typed.yaml
main/options/options.cc
common/os/linux.cc
gems/sorbet/test/snapshot/total/sorbet-runtime/src/Gemfile
test/testdata/disabled/whitequark/parse_encoding_.rb
website/docs/talks/curry-on-2019.md
test/testdata/desugar/sclass.rb.flatten-tree.exp
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/ruby
test/whitequark/test_ruby_bug_11873_7.parse-tree-whitequark.exp
test/testdata/namer
test/testdata/desugar/multi_assign.rb.symbol-table-raw.exp
test/cli/bad-perm/bad-perm.out
common/concurrency/WorkerPoolImpl.cc
test/testdata/namer/bug_1425.rb
test/testdata/cfg/examples.rb.desugar-tree.exp
test/whitequark/test_string_interp_0.parse-tree-whitequark.exp
test/testdata/lsp/completion/redefinition.rb
test/whitequark/test_block_arg_combinations_16.rb
gems/sorbet/test/snapshot/partial/explosive-object/src/Gemfile
test/whitequark/test_xstring_interp_0.rb
test/cli/make_accessible/suit.rb
rbi/core/binding.rbi
test/whitequark/test_ruby_bug_13547_11.rb
main/autogen
test/testdata/infer/generics
test/testdata/cfg/retry_multiple.rb.cfg.exp
website/static/img/sorbet-logo-card@2x.png
common/web_tracer_framework/tracing.h
proto/Name.proto
test/whitequark/test_yield_2.rb
test/whitequark/test_array_splat_2.rb
test/whitequark/test_defs_2.rb
test/testdata/resolver/bad_sealed_module__2.rb
bazel
test/testdata/infer/yield_multiple.rb.desugar-tree-raw.exp
test/testdata/rewriter/t_struct/none.rb
test/whitequark/test_or_0.rb
test/whitequark/test_block_arg_combinations_7.rb
test/testdata/resolver/bool_alias.rb
test/testdata/lsp/fast_path
test/whitequark/test_restarg_unnamed_0.rb
test/testdata/resolver/mixes_in_class_methods.rb.symbol-table-raw.exp
test/testdata/lsp/definitions_and_usages_2.rb
test/testdata/resolver/type_member_missing.rb.symbol-table-raw.exp
test/whitequark/test_ENCODING_0.rb
test/testdata/resolver/field_across_file__02.rb
gems/sorbet/test/snapshot/partial/codecov/src
core/Loc.h
test/cli/autocorrect-lazy-type-alias/autocorrect-lazy-type-alias.out
test/testdata/resolver/circle_of_itself.rb
gems/sorbet-runtime/test/types/fixtures/sealed_module
test/whitequark/test_next_3.parse-tree-whitequark.exp
test/cli/forgot-typed/permit-dsl-sig.rb
test/whitequark/test_bug_452_0.rb
gems/sorbet/test/snapshot/partial/missing-instance-methods/src/Gemfile
test/whitequark/test_array_splat_0.parse-tree-whitequark.exp
test/testdata/rewriter/attr.rb.symbol-table-raw.exp
test/cli/kwarg-loc
website/docs/override-checking.md
test/whitequark/test_masgn_splat_1.parse-tree-whitequark.exp
test/whitequark/test_args_star_1.rb
test/whitequark/test_ruby_bug_12402_12.parse-tree-whitequark.exp
test/testdata/infer/control_flow/isa_module.rb
test/cli/autocorrect-attached-class/autocorrect-attached-class.out
rbi/gems/bundler.rbi
test/testdata/resolver/missing_type_combinator_args.rb
gems/sorbet-runtime/test/types/fixtures/sealed_module/sealed_abstract__2.rb
test/whitequark/test_int_1.rb
payload/binary/empty.cc
plugin/Plugins.cc
test/cli/configatron-yaml-error/configatron-yaml-error.out
test/testdata/lsp/completion/constants.C.rbedited
test/whitequark/test_arg_duplicate_6.rb
test/testdata/rbi/basicobject_instance_exec.rb
rbi/stdlib/timeout.rbi
test/cli/subprocess-plugin/multi6.rb
test/testdata/resolver/linearization
test/cli/symbol-table-json-alias/symbol-table-json-alias.sh
third_party/parser/include/ruby_parser/pool.hh
test/whitequark/test_and_asgn_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/empty
test/whitequark/test_block_arg_combinations_17.rb
test/cli/statsd/statsd.sh
test/whitequark/test_rescue_mod_0.rb
ast/substitute/substitute.h
test/cli/silence-dev-message
test/testdata/namer/super.rb
test/whitequark/test_unary_num_pow_precedence_1.rb
test/whitequark/test_unless_1.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/helpers.rb
gems/sorbet/test/snapshot/partial/explosive-object
test/testdata/lsp/completion/snippet_types.B.rbedited
test/whitequark/test_send_binary_op_18.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_all_untyped.A.rbedited
test/cli/suggest-sig-literal/suggest-sig-literal.sh
test/testdata/resolver/override_shapes.rb
test/testdata/infer/flatten.rb
test/testdata/cfg/break.rb.cfg.exp
test/whitequark/test_rescue_mod_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/bad-hash/src
test/testdata/infer/multi_assign.rb
rbi/core/random.rbi
test/testdata/namer/conflicting_names.rb.flatten-tree.exp
gems/sorbet-runtime/test/types/non_forcing_constants.rb
test/testdata/infer/generics/t_magic.rb
payload/text/tools/generate_payload.cc
test/testdata/infer
test/cli/configatron-yaml-error/configatron-yaml-error.yaml
test/testdata/rewriter/t_enum.rb
test/whitequark/test_masgn_attr_0.parse-tree-whitequark.exp
test/whitequark/test_array_symbols_0.parse-tree-whitequark.exp
tools/toolchain/webasm-darwin/em_cache_existing.tar.gz
test/cli/typed-src/typed-src.out
test/testdata/infer/sigil.rb.cfg.exp
test/testdata/rbi/argf.rb
test/whitequark/test_masgn_attr_0.rb
test/cli/forbid-autocorrect-with-e
test/testdata/namer/net_imap.rb
test/testdata/resolver/flatten.rb
test/cli/hup-hack/hup-hack.sh
test/testdata/rewriter/interface_wrapper.rb
test/cli/bad-plugin-spec/duplicate-triggers.yaml
test/testdata/rewriter
test/testdata/namer/block_in_class.rb
test/testdata/parser/and_and_bug.rb
test/whitequark/test_bug_do_block_in_cmdarg_0.parse-tree-whitequark.exp
third_party/licenses/README
main
common/common.cc
test/cli/hup-hack
test/testdata/infer/attached_class_params.rb
test/whitequark/test_block_arg_combinations_19.rb
gems/sorbet/test/snapshot/partial/bad_gem/expected/out.log
test/testdata/infer/control_flow/simple.rb
rewriter/SelfNew.h
test/cli/suggest-sig/suggest-sig.out
test/testdata/desugar/star_in_block_arg.rb.desugar-tree.exp
tools/scripts/update-sorbet.run.sh
gems/sorbet/test/snapshot/partial/bad-hash/src/src.rb
gems/sorbet-runtime/lib/types/props/constructor.rb
test/cli/subprocess-plugin/multi_empty.yaml
test/testdata/lsp/go_to_type_definition.rb
main/lsp/requests/sorbet_error.cc
test/whitequark/test_send_attr_asgn_2.rb
website/docs/noreturn.md
test/whitequark/test_unary_num_pow_precedence_1.parse-tree-whitequark.exp
test/testdata/infer/generics/specified.rb
test/cli/empty-file/empty-file.sh
test/testdata/resolver/cbase.rb
test/testdata/infer/control_flow/class_less_than.rb
gems/sorbet/test/snapshot/partial/create-config/src/Gemfile
test/testdata/cfg
test/whitequark/test_send_lambda_args_noparen_1.parse-tree-whitequark.exp
test/whitequark/test_send_self_block_2.parse-tree-whitequark.exp
test/testdata/rewriter/coerce_in_prop.rb
test/testdata/infer/class_new.rb
test/whitequark/test_float_1.rb
test/whitequark/test_beginless_range_before_27_0.rb
test/testdata/rewriter/attr.rb
test/testdata/lsp/completion/snippet_repeated.C.rbedited
website/static/blog
test/testdata/infer/zsuper.rb.cfg.exp
test/whitequark/test_bug_do_block_in_call_args_0.parse-tree-whitequark.exp
payload/text/text.h
gems/sorbet/lib/gem-generator-tracepoint
test/cli/autogen-autoloader/scripts/baz.rb
test/testdata/rewriter/t_struct/normal.rb.rewrite-tree.exp
test/testdata/autogen/foo.fixturerb
test/cli/folder-input/input/file_with_no_dot
test/whitequark/test_arg_combinations_7.rb
core/Error.cc
rbi/core/module.rbi
main/lsp/notifications
test/testdata/lsp/fast_path/method_signature_update_generics__def.1.rbupdate
test/cli/suggest-sig-literal/suggest-sig-literal.rb
test/cli/version-returns-0/version-returns-0.sh
namer/test/namer_test.cc
test/whitequark/test_gvar_0.parse-tree-whitequark.exp
test/whitequark/test_when_splat_0.rb
test/lint/buildifier
core/core.h
test/whitequark/test_rescue_else_0.rb
test/cli/remove-path-prefix-https
docs/tracing.md
test/whitequark/test_ruby_bug_11873_a_6.rb
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/bundler/all/bundler.rbi
test/testdata/rewriter/t_struct/default.rb.rewrite-tree.exp
gems/sorbet/test/snapshot/partial/ignore_file_table/expected/sorbet/config
sorbet_version/BUILD
gems/sorbet/lib/suggest-typed.rb
tools/bazel
test/testdata/todo/block_in_class.rb.flatten-tree.exp
test/testdata/cfg/dealias_with_return.rb
test/testdata/cfg/rescue_complex.rb
test/cli/lsp-common-case-exit
test/testdata/infer/generics/adapt_child_to_parent.rb
test/whitequark/test_float_0.rb
rbi/core/struct.rbi
test/whitequark/test_procarg0_1.rb
test/whitequark/test_arg_label_0.parse-tree-whitequark.exp
test/cli/index-cache-invalidation/index-cache-invalidation.out
test/cli/progressbar
test/testdata/lsp/completion/constants.B.rbedited
test/whitequark/test_while_0.parse-tree-whitequark.exp
test/testdata/namer/constant_redefinition/module_then_constant_then_reopen.rb
test/cli/folder-input-dir-and-file/input/subfolder
gems/sorbet/lib/hidden-definition-finder.rb
test/whitequark/test_args_assocs_1.parse-tree-whitequark.exp
test/testdata/namer/yield.rb.flatten-tree.exp
main/autogen/BUILD
test/cli/suggest-singleton
test/whitequark/test_arg_combinations_14.rb
test/testdata/parser/complement_literal.rb
website/docs/procs.md
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/bundler/all
test/testdata/namer/constant_redefinition/class_then_constant.rb
third_party/progressbar/progressbar/statusbar.h
test/whitequark/test_ruby_bug_11873_a_11.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/create-config/expected/sorbet
test/whitequark/test_send_call_0.rb
test/cli/override-typed
test/testdata/lsp/fast_path/method_signature_update.rb
test/whitequark/test_return_2.parse-tree-whitequark.exp
tools/toolchain/webasm-linux/BUILD
third_party/rapidjson.BUILD
test/cli/non-existing-option/non-existing-option.out
test/cli/suggest-sig-garbage/suggest-sig-garbage.sh
test/cli/ignore-slash
rbi/core/rb_config.rbi
test/testdata/resolver/optional_nil.rb
website/static/talks
third_party/parser/cc/literal.cc
test/testdata/rewriter/t_struct/custom_initialize.rb
common/typecase.h
test/testdata/lsp/completion/sig_redefinition__2.rb
test/cli/lsp-requires-input-dir/lsp-requires-input-dir.sh
test/cli/module-redefinition/module-redefinition-1.rb
test/whitequark/test_class_0.rb
test/whitequark/test_block_arg_combinations_26.parse-tree-whitequark.exp
test/testdata/desugar/empty_string_concatenation.rb
gems/sorbet-runtime/lib/types/private/mixins/mixins.rb
test/testdata/infer/control_flow/complex_implication_1.rb
gems/sorbet/test/snapshot/partial/fake-object/src/Gemfile
gems/sorbet-runtime/test/types/fixtures/sealed_module/sealed_abstract__3.rb
test/testdata/cfg/rescue.rb
test/testdata/rewriter/initializer.rb
test/whitequark/test_redo_0.parse-tree-whitequark.exp
test/testdata/desugar/op_eq.rb
test/testdata/cfg/block_in_deadcode.rb
test/testdata/lsp/fast_path/undefined_constant.1.rbupdate
test/cli/print_generics/print_generics.out
test/cli/autogen-classlist/a.rb
rbi/core/proc.rbi
test/lsp/ProtocolTest.cc
third_party/licenses/tools/generate_licenses.cc
gems/sorbet/lib/t.rb
test/whitequark/test_array_plain_0.rb
docs/logo/sorbet-logo@2x.png
test/cli/cache-keeps-print-options/cache-keeps-print-options.out
test/cli/sigil-rbi/overrides.rbi
test/cli/autocorrect-attached-class
test/whitequark/test_bug_do_block_in_hash_brace_0.rb
test/testdata/resolver/crash.rb
third_party/parser/cc/state_stack.cc
test/testdata/parser/empty.rb.parse-tree.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/ruby
test/whitequark/test_gvar_0.rb
test/cli/suggest-t-let/suggest-t-let.sh
test/testdata/resolver/sig_bad.rb
test/whitequark/test_send_binary_op_1.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_4.rb
test/cli/bad-plugin-spec/triggers-not-map.yaml
test/cli/autocorrect-bare-stdlib-generics
gems/sorbet/test/hidden-method-finder/simple/src/Gemfile
test/testdata/autogen/dynamic_superclass.rb
test/testdata/infer/infer1.rb.desugar-tree.exp
test/cli/suggest-constant-type
test/testdata/lsp/struct_fuzz.rb
test/cli/cache-add-typed/cache-add-typed.out
test/testdata/rewriter/dsl_builder.rb.rewrite-tree.exp
rbi/core/thread.rbi
gems/sorbet/test/snapshot/partial/db_schema/src/sorbet/foo.txt
website/pages/en
test/testdata/lsp/hover_untyped_keyword_arg.rb
test/testdata/rewriter/flatten_nested_sclass.rb
gems/sorbet-runtime/test/types/builder_syntax.rb
test/testdata/lsp/missing_typed_sigil.A.rbedited
test/testdata/resolver/type_member_missing.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/bundler/all/bundler.rbi
test/whitequark/test_nth_ref_0.rb
test/cli/ignore/bar.rb
test/whitequark/test_args_assocs_comma_0.parse-tree-whitequark.exp
test/testdata/deviations/superclass_implicit.rb
gems/sorbet/test/snapshot/hermetic_tar.sh
test/whitequark/test_asgn_mrhs_0.rb
test/cli/config-file-recursive
gems/sorbet/test/snapshot/partial/bad_gem/expected/sorbet/config
test/testdata/rbi/lazy_enumerator.rb
test/cli/metrics-file
gems/sorbet-runtime/lib/types/props/optional.rb
core/Symbols.cc
test/whitequark/test_rescue_ensure_0.rb
test/whitequark/test_ruby_bug_11107_0.rb
third_party/externals.bzl
test/whitequark/test_def_1.rb
test/testdata/resolver/non_builder_sig.rb
gems/sorbet/test/snapshot/partial/rails6/src/Gemfile.lock
test/testdata/rewriter/flatten_module_private_class_method.rb.symbol-table-raw.exp
test/cli/line-splitting
gems/sorbet/test/snapshot/partial/non-utf-8-file/expected/sorbet/config
test/cli/lsp-invalid-json-and-exit/lsp-invalid-json-and-exit.sh
test/testdata/desugar/blockpass.rb.desugar-tree.exp
core/lsp
test/cli/allowed-extension/allowed-extension.sh
test/whitequark/test_rational_0.rb
gems/sorbet/test/snapshot/partial/stupidedi/src/Gemfile
test/testdata/resolver/alias_define_method__02.rb
test/whitequark/test_bug_do_block_in_hash_brace_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/private/types/void.rb
test/whitequark/test_array_words_interp_1.parse-tree-whitequark.exp
test/testdata/infer/show.rb
test/testdata/resolver/redefinition_of_subclass_type_member.rb
test/whitequark/test_block_kwarg_combinations_2.rb
BUILD
website/docs/metaprogramming-plugins.md
test/testdata/resolver/optional_nested.rb
rbi/stdlib/big_math.rbi
test/testdata/namer/constant_redefinition
test/testdata/rewriter/t_struct/root.rb.rewrite-tree.exp
parser/tools
rbi/stdlib/logger.rbi
test/whitequark/test_hash_label_end_1.parse-tree-whitequark.exp
test/testdata/cfg/override_bang.rb
rewriter/Command.cc
ast/Trees.h
test/testdata/parser/misc.rb.desugar-tree.exp
docs/img/sorbet-pipeline.monopic
test/cli/missing-constants/missing-constants.sh
test/cli/subprocess-plugin/multi1.rb
test/testdata/rewriter/rails/mattr_writer.rb
test/testdata/infer/non_forcing_is_a_dead.rb
rbi/tools/sync-rdoc.rb
test/testdata/rewriter/rails/mattr_reader.rb
gems/sorbet/test/hidden-method-finder/simple/src
gems/sorbet-runtime/test/types/props/private
test/cli/web-trace-file-non-ascii
test/testdata/lsp/fast_path/method_change_return_type__usage.rb
test/helpers/position_assertions.h
gems/sorbet
test/testdata/infer/attached_class_reopen.rb
test/cli/bad-plugin-spec
test/cli/cache-reserve-mem/cache-reserve-mem.sh
test/whitequark/test_unless_else_0.rb
gems/sorbet/lib/todo-rbi.rb
gems/sorbet-runtime/test/types/enum.rb
test/cli/subprocess-plugin/verify_ruby_options.rb
test/testdata/lsp/completion/depth.rb
test/whitequark/test_or_1.parse-tree-whitequark.exp
test/testdata/autogen/simple_refs.rb.autogen.exp
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib
test/whitequark/test_or_asgn_0.rb
test/whitequark/test_not_cmd_0.parse-tree-whitequark.exp
test/testdata/desugar/range.rb.desugar-tree-raw.exp
test/cli/lsp-large-message/lsp-large-message.sh
test/cli/folder-input-dir-and-file/input/subfolder/baz.rb
test/cli/cache-dsl
test/testdata/resolver
gems/sorbet-runtime/lib/types/types/noreturn.rb
test/whitequark/test_until_1.rb
tools/scripts/fuzz_minimize_all.sh
test/whitequark/test_defined_2.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_redefinition__1.rb
test/whitequark/test_and_asgn_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/stack_master/src/Gemfile
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/out.log
third_party/licenses/licenses.h
test/testdata/cfg/ivar_assign.rb.cfg.exp
test/testdata/rewriter/struct.rb
test/whitequark/test_bug_interp_single_0.parse-tree-whitequark.exp
test/BUILD
test/cli/metrics-file/sorbet/triggers.yml
test/whitequark/test_ruby_bug_13547_4.rb
test/whitequark/test_regex_interp_0.parse-tree-whitequark.exp
test/whitequark/test_op_asgn_1.parse-tree-whitequark.exp
test/whitequark/test_arg_1.parse-tree-whitequark.exp
test/testdata/resolver/mixes_in_class_methods_twice.rb
website/docs/exhaustiveness.md
rbi/stdlib/mutex_m.rbi
test/testdata/lsp/completion/sig_snippet.C.rbedited
core/lsp/Query.h
test/cli/lsp-large-message/lsp-large-message.out
test/testdata/cfg/rescue_bad_class.rb
test/testdata/resolver/linearization/constant_resolution_before_linearization.rb
test/testdata/rewriter/fuzz_dup_type_not_constant.rb
test/cli/lsp-invalid-json-and-exit/lsp-invalid-json-and-exit.out
test/whitequark/test_casgn_unscoped_0.rb
test/whitequark/test_unary_num_pow_precedence_0.rb
test/whitequark/test_space_args_arg_block_2.parse-tree-whitequark.exp
test/testdata/infer/stubs_are_dynamic.rb
test/testdata/rbi/t.rb
test/whitequark/test_kwarg_no_paren_0.rb
test/cli/store-state
test/testdata/resolver/bad_final_sig.rb
test/testdata/resolver/linearization/linearization6.rb.symbol-table-raw.exp
test/testdata/desugar/blockpass.rb
test/testdata/lsp/completion/snippet_block.A.rbedited
test/testdata/lsp/code_actions/loop_type_change.rb
gems/sorbet/test/snapshot/partial/bad_gem
test/whitequark/test_rescue_without_begin_end_0.rb
test/cli/config-file-recursive/sorbet/config
gems/sorbet-runtime/lib/types/types/fixed_array.rb
gems/sorbet/test/snapshot/partial/rails6/src/Gemfile
test/whitequark/test_ternary_0.rb
website/docs/gradual.md
test/whitequark/test_retry_0.rb
test/testdata/substitutions/double_subsitutions.rb
website/static/docs/attr_reader.html
test/testdata/parser/complement_literal.rb.parse-tree.exp
gems/sorbet/test/snapshot/partial/real_singleton_class/src
test/testdata/call_with_block.rb
test/testdata/autogen/defines_behavior.rb.autogen.exp
test/cli/web-trace-file/web-trace-file.sh
test/testdata/resolver/default_arg_in_block.rb
test/whitequark/test_send_lambda_1.parse-tree-whitequark.exp
test/whitequark/test_or_asgn_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_a_18.parse-tree-whitequark.exp
test/testdata/namer/constants.rb.flatten-tree.exp
test/whitequark/test_kwarg_combinations_2.parse-tree-whitequark.exp
test/whitequark/test_yield_1.parse-tree-whitequark.exp
test/testdata/desugar/strings.rb.desugar-tree.exp
test/whitequark/test_kwarg_combinations_1.parse-tree-whitequark.exp
test/testdata/namer/alias_cross_file__01.rb
gems/sorbet/test/hidden-method-finder/update_hidden_methods_exp.sh
test/whitequark/test_ruby_bug_11873_11.rb
test/cli/suggest-singleton/suggest-singleton.sh
test/whitequark/test_int_LINE_0.rb
test/cli/dedup_loc/dedup_loc.out
gems/sorbet-runtime/lib/types/types/base.rb
test/testdata/resolver/t_struct_subclass.rb
test/testdata/parser/chained_sends_lots.rb.desugar-tree.exp
test/whitequark/test_and_0.rb
cfg/builder/builder_entry.cc
gems/sorbet/test/snapshot/partial/rails-double-require/src/config/application.rb
website/package.json
test/testdata/lsp/fast_path/method_rename_argument.1.rbupdate
test/cli/web-trace-file-non-ascii/web-trace-file-non-ascii.sh
test/testdata/infer/loops.rb.cfg.exp
test/testdata/infer/singleton_non_final_enum.rb
test/testdata/infer/toplevel_var.rb
third_party/pdqsort.BUILD
test/testdata/infer/generics/all_bottom.rb
test/whitequark/test_arg_combinations_3.rb
test/testdata/lsp/fast_path/class_add_member.rb
test/testdata/infer/generics/variance_methods.rb
gems/sorbet-runtime/lib/types/props/pretty_printable.rb
test/whitequark/test_undef_0.rb
website/docs/abstract.md
test/cli/error-blacklist/error-blacklist.rb
test/whitequark/test_begin_cmdarg_0.parse-tree-whitequark.exp
test/testdata/lsp/workspace_symbols_shortname.rb
test/cli/autogen-classlist/b.rb
gems/sorbet/Rakefile
test/whitequark/test_marg_combinations_4.rb
test/testdata/resolver/linearization/linearization1.rb.symbol-table-raw.exp
test/cli/escaping
test/testdata/cfg/blocks.rb.desugar-tree.exp
test/cli/print-procs
test/whitequark/test_block_arg_combinations_27.rb
test/testdata/parser/unary_num.rb
rbi/stdlib/optparse.rbi
test/whitequark/test_masgn_splat_7.rb
test/cli/ignore/subfolder2/subfolder/bar.rb
test/testdata/infer/singleton_enum_eqeq.rb
main/pipeline/BUILD
test/testdata/rbi/range.rb
test/testdata/infer/generics/lub.rb
test/testdata/resolver/empty_sealed.rb
test/whitequark/test_int_1.parse-tree-whitequark.exp
test/testdata/resolver/choose_sig.rb
test/whitequark/test_bug_interp_single_1.rb
test/testdata/lsp/completion/snippet_default.A.rbedited
test/cli/sigil-rbi/strict.rbi
ast/verifier
test/testdata/lsp/completion/sig_snippet.A.rbedited
test/whitequark/test_super_block_1.rb
third_party/spdlog.BUILD
test/testdata/desugar/dsymbol.rb.desugar-tree.exp
test/whitequark/test_const_op_asgn_4.rb
proto/File.proto
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset
test/whitequark/test_ruby_bug_13547_5.rb
test/whitequark/test_block_arg_combinations_24.rb
payload/payload.cc
main/lsp/test/lsp_test.cc
core/AutocorrectSuggestion.cc
class_flatten
test/testdata/lsp/symbol_query_filter__main.rb
test/testdata/error_recovery_send_after_end.rb
test/cli/BUILD
test/whitequark/test_range_inclusive_0.parse-tree-whitequark.exp
test/cli/autocorrect-same-loc/autocorrect-same-loc.out
test/testdata/namer/alias_method_redefinition.rb
test/testdata/namer/yield.rb.symbol-table-raw.exp
test/testdata/disabled/whitequark
ast/TreeCopying.cc
test/whitequark/test_ruby_bug_11873_a_2.parse-tree-whitequark.exp
website/pages/en/index.js
test/testdata/autogen/strong_sigil.rb
test/testdata/parser/error_recovery_multiple_stmts.rb.parse-tree.exp
test/whitequark/test_const_toplevel_0.rb
test/testdata/resolver/inherit_alias.rb
test/whitequark/test_and_or_masgn_1.rb
gems/sorbet/README.md
gems/sorbet/test/snapshot/partial/rails-double-require/expected
test/testdata/lsp/hover_operator_overload.rb
test/testdata/cfg/examples.rb.parse-tree.exp
docs/img/chrome-tracing-loaded.png
gems/sorbet-runtime/lib/types/types/typed_set.rb
test/testdata/lsp/code_actions/sig_missing__child.rb
test/testdata/resolver/resolve_via_ancestors/superclass_three_pass.rb
test/testdata/rewriter/chalk_odm_document.rb
test/whitequark/test_cond_match_current_line_0.parse-tree-whitequark.exp
test/whitequark/test_arg_duplicate_5.rb
test/whitequark/test_arg_combinations_2.rb
test/cli/allowed-extension/lib/c.ru
test/whitequark/test_ruby_bug_11873_a_15.parse-tree-whitequark.exp
test/testdata/lsp/completion/snippet_keywords.B.rbedited
namer/namer.h
gems/sorbet/lib/gem-generator-tracepoint/tracer.rb
test/whitequark/test_parser_bug_518_0.rb
gems/sorbet-runtime/lib/types/types/intersection.rb
test/whitequark/test_asgn_backref_invalid_0.rb
gems/sorbet-runtime/lib/types/_types.rb
gems/sorbet/test/snapshot/partial/bad_gem/src
main/lsp/notifications/notifications.h
test/testdata/desugar/undef_strict.rb
test/whitequark/test_send_plain_cmd_1.rb
test/whitequark/test_block_arg_combinations_13.parse-tree-whitequark.exp
test/testdata/namer/defs_in_blocks.rb.symbol-table-raw.exp
test/testdata/autogen/dynamic_superclass.rb.autogen.exp
test/testdata/lsp/fast_path/method_signature_update_static__def.1.rbupdate
gems/sorbet/test/snapshot/partial/non-utf-8-file/expected
namer/configatron/configatron.h
third_party/licenses/msgpack-c.txt
definition_validator
test/testdata/rewriter/struct_initialize.rb
test/testdata/cfg/rescue_complex.rb.cfg.exp
test/whitequark/test_procarg0_legacy_0.rb
test/whitequark/test_class_definition_in_while_cond_3.parse-tree-whitequark.exp
rewriter/ClassNew.h
test/testdata/rewriter/command.rb.rewrite-tree.exp
test/testdata/desugar/sclass_inheritance.rb.desugar-tree.exp
test/testdata/lsp/completion/snippet_repeated.B.rbedited
test/testdata/resolver/type_member_singleton_members.rb.symbol-table-raw.exp
rewriter/MixinEncryptedProp.cc
test/testdata/infer/massign_return_rhs.rb
test/testdata/infer/let.rb
test/cli/sigil-rbi/no_type.rbi
test/whitequark/test_ivasgn_0.rb
third_party/progressbar/README
test/testdata/parser/chained_sends_lots.rb
rbi/core/errors.rbi
test/testdata/infer/overload_block.rb
website/docs/metrics.md
gems/sorbet-runtime/lib/types/types/typed_enumerable.rb
test/testdata/lsp/completion/snippet_block.C.rbedited
test/testdata/rewriter/rails/mattr_accessor.rb.rewrite-tree.exp
test/testdata/lsp/fast_path/parse_errors.rb
core/Files.h
main/lsp/LSPTypechecker.h
test/whitequark/test_ruby_bug_12669_3.parse-tree-whitequark.exp
test/whitequark/test_or_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/extconf
test/testdata/infer/proc.rb
gems/sorbet/test/snapshot/partial/codecov/src/Gemfile.lock
main/autogen/autogen.cc
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0/gems/my_gem-0.0.0/lib/my_gem.rb
test/whitequark/test_range_inclusive_0.rb
test/testdata/infer/self_type.rb
test/whitequark/test_block_arg_combinations_12.rb
test/testdata/infer/flatten_private.rb.flatten-tree.exp
test/whitequark/test_kwrestarg_named_0.rb
test/testdata/core/fuzz_bad_subtyping.rb
gems/sorbet/test/snapshot/partial/extconf/expected
test/testdata/resolver/stubs_typed_untyped.flatten-tree.exp
test/whitequark/test_ruby_bug_11873_a_3.rb
gems/sorbet/test/snapshot/partial/rails-double-require
test/cli/suggest-object/suggest-object.out
test/testdata/rbi/kernel.rb
test/testdata/infer/or_and_and_or.rb
rbi/core/float.rbi
test/testdata/local_vars/z_super_args.rb
test/testdata/resolver/linearization/linearization3.rb.symbol-table-raw.exp
test/whitequark/test_string_concat_0.rb
test/whitequark/test_bug_480_0.rb
test/whitequark/test_send_self_block_1.parse-tree-whitequark.exp
test/testdata/rewriter/t_struct/normal.rb
test/testdata/parser/trailing_comas.rb.parse-tree-json.exp
website/static/docs/why-nil.html
main/lsp/notifications/sorbet_resume.cc
test/testdata/namer/redefines_object.rb.cfg.exp
test/cli/rbi-overrides/t2.rbi
third_party/parser/codegen/generate_diagnostics.cc
main/lsp/NextMethodFinder.cc
main/lsp/requests/sorbet_error.h
third_party/licenses/rang.txt
test/whitequark/test_if_masgn_24_0.parse-tree-whitequark.exp
test/testdata/resolver/let_errors.rb
test/whitequark/test_defs_1.rb
test/whitequark/test_alias_nth_ref_0.rb
test/testdata/lsp/fast_path/method_add_argument.1.rbupdate
third_party/concurrentqueue.BUILD
test/testdata/resolver/stub_missing_class_alias.rb
test/testdata/infer/control_flow_in_ensure.rb
test/whitequark/test_while_mod_0.rb
rewriter/Util.cc
test/whitequark/test_ruby_bug_11873_a_6.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_5.rb
test/fuzz/BUILD
docs/logo/sorbet-logo-white-sparkles.svg
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet
test/whitequark/test_unless_mod_0.rb
main/cache
gems/sorbet/test/snapshot/partial/ignore_file_table/expected/sorbet/important_file.txt
core/errors/cfg.h
test/testdata/lsp/highlight_simple.rb
third_party/licenses/abseil.txt
test/whitequark/test_arg_duplicate_0.rb
test/cli/error-blacklist
rbi/sorbet/tprivate.rbi
gems/sorbet/test/snapshot/partial/bad-t/src/Gemfile.lock
rbi/core/ruby_vm.rbi
test/testdata/desugar/sclass.rb.desugar-tree.exp
test/cli/override-typed-bad/bad-filename.yaml
test/testdata/rbi/hash.rb
core/errors/plugin.h
test/cli/configatron/configatron.sh
test/whitequark/test_const_op_asgn_1.rb
test/testdata/lsp/fast_path/method_add_keyword_arg.rb
test/testdata/cfg/singleton_lazy.rb.symbol-table-raw.exp
test/cli/store-state/store-state.sh
test/testdata/rbi
main/lsp/tools/make_lsp_types.cc
third_party/progressbar/src
test/testdata/infer/infer1.rb
test/whitequark/test_kwoptarg_0.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_singleton.B.rbedited
test/testdata/namer/dynamic_constant.rb
gems/sorbet/Gemfile
core/Context.h
test/cli/autogen-errors/autogen-errors.rb
test/cli/hup-hack/hup-hack.out
gems/sorbet-runtime/test/test_helper.rb
main/lsp/notifications/sorbet_fence.h
common/statsd/statsd.h
gems/sorbet-runtime/test/types/mixins.rb
test/testdata/rewriter/flatten_nested_sclass.rb.symbol-table-raw.exp
test/cli/remove-path-prefix-no-match/remove-path-prefix-no-match.rb
test/testdata/lsp/completion/overloads_test.rb
test/cli/error-whitelist/error-whitelist.rb
gems/sorbet/test/snapshot/partial/type_member/src
test/whitequark/test_send_binary_op_19.rb
test/testdata/lsp/completion/snippet_default.rb
test/whitequark/test_resbody_var_1.parse-tree-whitequark.exp
test/testdata/namer/nested_class.rb
test/whitequark/test_bug_while_not_parens_do_0.rb
test/whitequark/test_arg_invalid_0.rb
rbi/stdlib/objspace.rbi
test/whitequark/test_defs_2.parse-tree-whitequark.exp
test/cli/sigil-rbi/abstract.rbi
test/whitequark/test_asgn_mrhs_0.parse-tree-whitequark.exp
test/testdata/infer/generics/variance_user.rb
main/lsp/LSPOutput.cc
test/cli/suggest-type-alias
gems/sorbet/test/snapshot/partial/use-existing-config/src/Gemfile.lock
test/whitequark/test_class_super_label_0.parse-tree-whitequark.exp
test/cli/missing-constants
test/cli/subprocess-plugin/bar_gen.rb
test/whitequark/test_send_binary_op_16.parse-tree-whitequark.exp
test/cli/no-did-you-mean/no-did-you-mean.rb
test/whitequark/test_space_args_arg_0.parse-tree-whitequark.exp
test/cli/web-trace-file-non-ascii/web-trace-file-non-ascii.rb
test/testdata/rbi/to_sym.rb
test/whitequark/test_masgn_splat_3.rb
third_party/openssl/linux.BUILD
test/testdata/infer/loops.rb
test/cli/ignore-slash/bar.rb
test/whitequark/test_range_exclusive_0.parse-tree-whitequark.exp
test/cli/parser-error/parser-error-4.rb
main/options/test
test/testdata/infer/pinned_control_flow1.rb
test/whitequark/test_send_lambda_args_0.rb
test/whitequark/test_masgn_nested_1.rb
test/cli/autocorrect-strict
parser/parser.h
payload/text/populate.cc
tools/scripts/update_exp_files.sh
gems/sorbet-runtime/lib/types/types/untyped.rb
test/whitequark/test_kwarg_combinations_3.rb
test/testdata/autogen/strong_sigil.rb.autogen.exp
third_party/yaml_cpp.BUILD
test/whitequark/test_method_definition_in_while_cond_1.parse-tree-whitequark.exp
test/cli/metrics-file/metrics-file.out
test/whitequark/test_bug_regex_verification_0.parse-tree-whitequark.exp
test/cli/stop-after/stop-after.out
test/whitequark/test_array_symbols_empty_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/empty/src/Gemfile.lock
test/testdata/lsp/fast_path/method_signature_update_generics__usage.rb
test/cli/autocorrect-same-loc
test/whitequark/test_block_arg_combinations_10.rb
test/cli/autocorrect/autocorrect.out
test/whitequark/test_masgn_splat_2.parse-tree-whitequark.exp
test/whitequark/test_lvasgn_0.rb
ast/substitute/Substitute.cc
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0/gems/my_gem-0.0.0/lib
test/lsp/update_one.sh
test/whitequark/test_cond_eflipflop_1.rb
gems/sorbet/test/snapshot/partial/create-config/src
test/whitequark/test_defs_4.rb
rbi/stdlib/coverage.rbi
rewriter/DSLBuilder.cc
test/whitequark/test_bug_heredoc_do_0.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder/BUILD
test/whitequark/test_send_self_1.rb
test/cli/help/help.out
test/whitequark/test_ruby_bug_11873_a_9.rb
test/cli/cache-doesnt-parse/cache-doesnt-parse.out
gems/sorbet/test/snapshot/update_one.sh
third_party/lmdb.BUILD
test/cli/stop-after-namer/stop-after-namer.out
rbi/core/symbol.rbi
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/expected/sorbet/rbi/gems/my_gem.rbi
test/whitequark/test_send_self_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/rspec-lots/src/Gemfile.lock
test/testdata/lsp/definition_untyped.rb
test/testdata/infer/singleton_classes.rb
test/testdata/resolver/rbi_final_no_sig__2.rb
test/cli/cache-reserve-mem
test/cli/config-file-recursive/config-file-recursive.rb
infer
test/cli/no-did-you-mean
docs/img/chrome-tracing-scrolled.png
rbi/stdlib/net.rbi
test/testdata/resolver/sig_good.rb.symbol-table-raw.exp
test/whitequark/test_ruby_bug_13547_6.rb
test/testdata/infer/metatype_instantiation.rb
test/cli/cache-keeps-print-options/cache-keeps-print-options.sh
test/cli/lsp-common-case-exit/lsp-common-case-exit.sh
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset/gems/my_gem-0.0.0/lib/my_gem.rb
test/cli/suggest-typed-true/suggest-typed-strong.rb
test/whitequark/test_send_plain_2.parse-tree-whitequark.exp
proto/pay-server
gems/sorbet/test/snapshot/partial/fake-rails/src/Gemfile.lock
core/serialize/test
gems/sorbet-runtime/test/types/interface_wrapper.rb
test/cli/cache-dsl/cache-dsl.out
gems/sorbet/test/snapshot/partial/ignore_file_table/src/sorbet/important_file.txt
test/whitequark/test_arg_combinations_3.parse-tree-whitequark.exp
main/lsp/requests/shutdown.cc
test/whitequark/test_when_multi_0.rb
test/whitequark/test_kwarg_combinations_0.parse-tree-whitequark.exp
core/errors/internal.h
gems/sorbet-runtime/lib/types/private/types
test/whitequark/test_hash_label_end_0.rb
test/whitequark/test_ruby_bug_12669_1.rb
test/whitequark/test_def_1.parse-tree-whitequark.exp
third_party/licenses/yamlcpp.txt
test/whitequark/test_ruby_bug_11873_9.rb
test/whitequark/test_until_0.rb
test/whitequark/test_arg_combinations_10.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/bad_gem/src/lib
gems/sorbet/test/snapshot/partial/stupidedi/src
main/options/ConfigParser.h
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_extend__1.rb
tools/toolchain/webasm-linux/ar.sh
test/whitequark/test_arg_combinations_13.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/use-existing-config/src
test/whitequark/test_space_args_arg_call_0.parse-tree-whitequark.exp
test/whitequark/test_string_plain_1.parse-tree-whitequark.exp
test/cli/non-existing-option/non-existing-option.sh
core/ErrorFlusher.h
test/testdata/infer/singleton_of_singleton.rb
test/whitequark/test_send_binary_op_15.rb
main/lsp/watchman/WatchmanProcess.h
test/whitequark/test_ruby_bug_11873_11.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_root.rb
test/cli/configatron-yaml-error/configatron-yaml-error.sh
gems/sorbet-runtime/lib/types/props/_props.rb
core/errors
test/whitequark/test_op_asgn_2.parse-tree-whitequark.exp
test/whitequark/test_break_0.rb
test/cli/dash-e/dash-e.out
test/testdata/infer/literal_is_array.rb
test/cli/forbid-autocorrect-with-e/forbid-autocorrect-with-e.out
core/serialize
test/testdata/rbi/sorbet.rb
test/cli/symbol-table-json/symbol-table-json.out
test/cli/error-url/error-url.rb
gems/sorbet/test/snapshot/partial/rspec-lots
test/fuzz/fuzz_doc_symbols.cc
test/cli/suggest-foreign-lambda
rbi/stdlib/stringio.rbi
test/cli/print_to_file
gems/sorbet/test/snapshot/partial/fake-object/src/src.rb
rbi/core/match_data.rbi
gems/sorbet/test/hidden-method-finder/simple
test/testdata/namer/yield.rb.cfg.exp
test/testdata/resolver/bare_generics_strict.rb
test/whitequark/test_arg_combinations_6.rb
test/cli/suggest-sig
test/testdata/lsp/workspace_symbols_minitest.rb
gems/sorbet/test/snapshot/partial/stack_master
ast/substitute
test/testdata/namer/constant_in_block.rb
test/whitequark/test_args_args_comma_0.rb
test/whitequark/test_ensure_empty_0.rb
test/testdata/autogen/ancestors_ref.rb.autogen.exp
test/testdata/lsp/completion/no_parens.rb
gems/sorbet/test/snapshot/partial/bad-hash/expected
gems/sorbet/test/snapshot/partial/extconf/src/Gemfile.lock
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0/gems/my_gem-0.0.0/my_gem.gemspec
test/testdata/resolver/abstract_out_of_order.rb
gems/sorbet/test/snapshot/partial/type_member/src/Gemfile
test/whitequark/test_xstring_plain_0.rb
test/cli/incremental-resolver/expect-failures/constant_override.rb
test/whitequark/test_args_cmd_0.rb
test/testdata/lsp/completion/sig.A.rbedited
test/testdata/cfg/floats.rb
test/testdata/lsp/completion/sig_root.A.rbedited
test/cli/cache-add-typed/cache-add-typed.sh
test/whitequark/test_ruby_bug_11873_b_0.parse-tree-whitequark.exp
test/testdata/rewriter/class_new.rb
test/testdata/infer/star_star_args.rb
test/testdata/parser/invalid_trailing_in_number.rb.parse-tree.exp
third_party/gtest.BUILD
test/testdata/infer/block_parameter.rb
gems/sorbet/test/snapshot/partial/real_singleton_class
test/whitequark/test_ruby_bug_11873_a_10.rb
test/whitequark/test_send_lambda_legacy_0.rb
test/testdata/namer/multiple_stubs.rb
test/testdata/infer/bad_child_class.rb
test/testdata/infer/absurd_false.rb
test/testdata/resolver/type_arguments.rb
test/testdata/lsp/include_and_extend.rb
tools/scripts/format_cxx.sh
test/whitequark/test_kwrestarg_unnamed_0.parse-tree-whitequark.exp
test/testdata/rewriter/t_struct/override_bad.rb
test/whitequark/test_hash_hashrocket_0.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder/check_one_bazel.sh
test/whitequark/test_send_block_chain_cmd_6.parse-tree-whitequark.exp
gems/sorbet/test/hidden-method-finder/logging.sh
gems/sorbet/test/snapshot/partial/real_singleton_class/src/src.rb
test/whitequark/test_ruby_bug_9669_1.parse-tree-whitequark.exp
rewriter/ClassNew.cc
test/testdata/namer/nested_static_field.rb
test/whitequark/test_bug_447_0.parse-tree-whitequark.exp
main/pipeline/semantic_extension
test/whitequark/test_restarg_named_0.rb
test/testdata/resolver/optional_nil.rb.name-tree.exp
rbi/core/class.rbi
test/whitequark/test_range_endless_1.parse-tree-whitequark.exp
third_party
test/whitequark/test_asgn_mrhs_1.rb
gems/sorbet/test/snapshot/partial/use-existing-config/src/sorbet
test/whitequark/test_ruby_bug_11873_5.rb
test/testdata/lsp/completion/sig_snippet.D.rbedited
gems/sorbet-runtime/lib/types/profile.rb
gems/sorbet/test/snapshot/partial/local_gem/src
test/lsp/lsp_test_runner.sh
rbi/stdlib/erb.rbi
test/testdata/cfg/next_in_while.rb.cfg.exp
test/whitequark/test_return_block_0.parse-tree-whitequark.exp
test/testdata/rbi/random.rb
test/testdata/infer/generics/generic_self_method.rb
cfg/CFG.cc
gems/sorbet/test/snapshot/partial/bad-t/src/Gemfile
test/cli/at/at.sh
test/testdata/resolver/fuzz_mixes_in_class_methods.rb
test/testdata/parser/error_recovery_if_no_end.rb
test/whitequark/test_send_binary_op_6.rb
gems/sorbet/test/snapshot/partial/rails6
website/blog/2019-12-20-announcing-sorbet-0.5.md
test/testdata/resolver/strict.rb
test/cli/suppress-non-critical
test/cli/folder-input-dir-and-file/folder-input-dir-and-file.out
test/cli/remove-path-prefix-no-match/remove-path-prefix-no-match.out
test/testdata/desugar/constant_reassignment.rb
gems/sorbet/test/snapshot/partial/ignore_file_table/src/Gemfile
test/testdata/resolver/recover_from_bad_superclass.rb.symbol-table-raw.exp
test/cli/autocorrect-strict/autocorrect-strict.out
test/testdata/infer/generic_methods/countraints_crosstalk.rb
test/testdata/resolver/sig_compat.rb.symbol-table-raw.exp
test/whitequark/test_ruby_bug_12686_1.rb
test/whitequark/test_send_binary_op_11.parse-tree-whitequark.exp
test/testdata/rewriter/prop_mutator.rb
test/whitequark/test_ruby_bug_11873_10.parse-tree-whitequark.exp
rbi/stdlib/json.rbi
test/testdata/cfg/class_static_field.rb
test/testdata/todo
test/testdata/resolver/stubs_typed_untyped__2.rb
test/helpers/MockFileSystem.h
rbi/core/time.rbi
test/testdata/lsp/highlight.rb
test/whitequark/test_send_index_0.parse-tree-whitequark.exp
website/docs/class-types.md
gems/sorbet/test/snapshot/partial/stupidedi
test/whitequark/test_defs_invalid_6.rb
test/testdata/rbi/with_without__2.rbi
test/whitequark/test_kwbegin_compstmt_0.rb
test/testdata/rbi/enumerable.rb
test/testdata/rbi/json.rb
test/whitequark/test_resbody_list_var_0.parse-tree-whitequark.exp
website/static/img/shopify-logo.svg
test/whitequark/test_const_op_asgn_0.rb
test/cli/line-splitting/line-splitting.out
test/cli/suggest-sig-literal/suggest-sig-literal.out
main/lsp/requests/code_action.h
test/testdata/testrunner
test/lsp/lsp_test.bzl
test/cli/subprocess-plugin/permute.rb
test/testdata/resolver/resolve_via_ancestors/simple.rb
main/lsp/lsp_messages_gen_helpers.h
gems/sorbet/test/hidden-method-finder/simple/src/simple.rb
test/whitequark/test_block_arg_combinations_22.rb
test/cli/forgot-typed/forgot-typed.rb
test/testdata/resolver/simple.rb
test/cli/suggest_static
test/cli/rbi-with-code
test/testdata/resolver/resolution_order.rb
test/testdata/rewriter/t_struct/inexact.rb.rewrite-tree.exp
test/cli/folder-input-dir-and-file
test/testdata/rbi/trueclass.rb
test/whitequark/test_arg_label_1.rb
core/LocalVariable.cc
test/whitequark/test_masgn_const_invalid_0.rb
core/GlobalState.h
test/whitequark/test_arg_duplicate_1.rb
test/whitequark/test_masgn_attr_1.parse-tree-whitequark.exp
test/cli/logging/logging.out
test/testdata/resolver/resolve_errors.rb
test/whitequark/test_op_asgn_index_0.rb
main/lsp/lsp.cc
test/cli/suggest-kernel/suggest-kernel.rb
core/lsp/PreemptionTaskManager.h
test/testdata/infer/fields.rb
test/testdata/todo/generics_should_fail.rb
test/whitequark/test_ruby_bug_12402_5.rb
test/whitequark/test_asgn_keyword_invalid_1.rb
test/cli/model_mutator_behavior/model_mutator_behavior.sh
test/testdata/parser/invalid_fatal.rb.parse-tree.exp
infer/environment.h
rewriter/DefaultArgs.h
website/static/css
test/whitequark/test_rescue_mod_asgn_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/run_one.sh
test/testdata/parser/encoding.rb
plugin/SubprocessTextPlugin.h
rbi/core/method.rbi
test/testdata/cfg/rescue.rb.desugar-tree.exp
test/testdata/resolver/class_instance_vars.rb
test/whitequark/test_if_1.rb
gems/sorbet-runtime/test/types/fixtures/sealed_module/whitelist_violation__2.rb
third_party/parser/include/ruby_parser
gems/sorbet/test/snapshot/partial/type_member/src/Gemfile.lock
website/docs/type-assertions.md
test/whitequark/test_not_0.parse-tree-whitequark.exp
payload/text/BUILD
test/cli/autocorrect-extend
test/whitequark/test_back_ref_0.parse-tree-whitequark.exp
test/whitequark/test_string_plain_0.rb
test/testdata/resolver/untyped_generics.rb
test/whitequark/test_ruby_bug_11873_7.rb
common/JSON.h
common/Counters.h
test/whitequark/test_bug_cmd_string_lookahead_0.rb
gems/sorbet-runtime/test/typecheck-runtime.sh
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_include__1.rb
test/testdata/cfg/rescue_bad_class.rb.flatten-tree.exp
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_extend__3.rb
test/cli/incremental-resolver/expect-failures
test/cli/rbi-with-code/rbi-with-code.out
main/lsp/requests/type_definition.cc
test/cli/no-stdlib
test/testdata/infer/type_substraction.rb
test/testdata/namer/gvar.rb.flatten-tree.exp
test/whitequark/test_and_or_masgn_0.parse-tree-whitequark.exp
test/testdata/infer/fields.rb.cfg.exp
common/formatting.h
test/cli/autocorrect-abstract/post.rb
test/whitequark/test_resbody_var_0.parse-tree-whitequark.exp
website/static/docs/include-kernel.html
rewriter/TEnum.cc
test/testdata/desugar/top_level_const.rb.desugar-tree.exp
third_party/ruby
common/Exception.h
test/cli/rbi-overrides/t1.rb
test/cli/autogen-subclasses-ignore/autogen-subclasses-ignore.sh
test/cli/autogen-subclasses-ignore/not-ignored/not-ignored.rb
test/testdata/namer/include_noarg.rb
test/cli/autocorrect-lazy-type-alias/pre.rb
third_party/parser/codegen/builder.rb
test/cli/folder-input/input/subfolder
main/lsp/requests/document_highlight.h
gems/sorbet-runtime/test/types/props/private/setter_factory.rb
test/testdata/lsp/workspace_symbols_namespaces.rb
test/testdata/resolver/bad_hash.rb
cfg/BUILD
test/testdata/proc_variance.rb
gems/sorbet/test/hidden-method-finder/simple/ruby_2_4_hidden.rbi.exp
test/whitequark/test_until_mod_0.parse-tree-whitequark.exp
test/testdata/lsp/hover_conditional_type_narrowing.rb
test/whitequark/test_send_block_chain_cmd_3.parse-tree-whitequark.exp
test/whitequark/test_args_block_pass_0.rb
test/cli/sigil-rbi
test/testdata/resolver/void.rb
test/testdata/rbi/proc.rb
test/testdata/lsp/fast_path/class_change_superclass.rb
gems/sorbet-runtime/lib/types/types/type_template.rb
gems/sorbet-runtime/lib/types/private/class_utils.rb
test/whitequark/test_when_splat_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/props/private/apply_default.rb
ast/verifier/Verifier.cc
gems/sorbet-runtime/lib/types/compatibility_patches.rb
test/whitequark/test_send_conditional_0.rb
test/lsp/protocol_test_corpus.cc
core/errors/parser.h
test/testdata/namer/redefine.rb
core/proto/proto.cc
test/testdata/lsp/completion/snippet_arity.B.rbedited
test/cli/test_one.sh
gems/sorbet/test/snapshot/partial/extconf/src/lib
rbi/stdlib/gem.rbi
CONTRIBUTING.md
test/testdata/rewriter/attr_multi.rb.rewrite-tree.exp
sorbet_version
main/pipeline
test/cli/ignore/subfolder
test/testdata/desugar/multi_assign.rb.desugar-tree.exp
main/lsp/test/error_reporter_test.cc
test/whitequark/test_procarg0_1.parse-tree-whitequark.exp
test/testdata/rewriter/t_enum_type_syntax.rb
test/testdata/resolver/type_arguments.rb.symbol-table-raw.exp
test/testdata/parser/error_recovery_const_case.rb
test/whitequark/test_array_words_interp_0.rb
rbi/core/enumerable.rbi
test/whitequark/test_var_op_asgn_2.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/method_change_return_type__usage.1.rbupdate
test/whitequark/test_masgn_0.rb
test/whitequark/test_defs_invalid_3.rb
gems/sorbet-runtime/lib/types/non_forcing_constants.rb
test/testdata/rbi/with_without__1.rbi
rbi/core/exception.rbi
test/testdata/lsp/hover_constants.rb
infer/inference.h
test/error-check-test.cc
test/testdata/parser/error_recovery_if_no_end.rb.parse-tree.exp
test/testdata/lsp/completion/snippet_arity.rb
rbi/stdlib/dir.rbi
test/testdata/namer/class_and_alias.rb.flatten-tree.exp
test/cli/configatron/configatron.rb
test/testdata/resolver/missing_helpers_interface.rb
website/docs/quickref.md
test/testdata/infer/generic_methods/untyped_in_container.rb
test/testdata/infer/fuzz_uninitialized_vars.rb
test/testdata/rewriter/fuzz_optinal_crash.rb
test/whitequark/test_space_args_arg_newline_0.rb
test/testdata/resolver/inherit_alias.rb.symbol-table-raw.exp
test/testdata/autogen/aliased_ancestors.rb.autogen.exp
test/testdata/namer/dynamic_method_with_class.rb.symbol-table-raw.exp
test/whitequark/test_if_else_1.parse-tree-whitequark.exp
test/testdata/resolver/bad_synthesize.rb
test/testdata/lsp/good_field_edits.rb
test/whitequark/test_args_cmd_0.parse-tree-whitequark.exp
test/whitequark/test_bug_481_0.rb
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0/gems/my_gem-0.0.0
gems/sorbet/test/snapshot/partial/explosive-object/src/src.rb
gems/sorbet/test/snapshot/partial/local_gem/expected
test/cli/autogen-classlist/autogen-classlist.out
test/whitequark/test_ruby_bug_10653_2.parse-tree-whitequark.exp
test/whitequark/test_ternary_ambiguous_symbol_0.parse-tree-whitequark.exp
main/lsp/notifications/sorbet_fence.cc
test/whitequark/test_ruby_bug_12669_3.rb
test/testdata/rbi/uri.rb
test/whitequark/test_return_2.rb
gems/sorbet-runtime/lib/types/types/fixed_hash.rb
test/cli/autocorrect-attached-class/autocorrect-attached-class.sh
test/whitequark/test_array_words_0.parse-tree-whitequark.exp
test/cli/update_one.sh
rewriter/ModuleFunction.h
rewriter/Flatfiles.cc
test/testdata/rewriter/minitest_tables.rb.rewrite-tree.exp
test/whitequark/test_send_binary_op_20.parse-tree-whitequark.exp
gems/sorbet-static
website/static/img/favicon.ico
main/lsp/LSPTypecheckerCoordinator.h
test/testdata/desugar/nthref.rb.desugar-tree-raw.exp
rbi/stdlib
test/cli/allowed-extension/lib/b.rbi
infer/environment.cc
common/Levenstein.cc
test/testdata/namer/constant_redefinition/module_then_constant.rb
test/whitequark/test_cond_iflipflop_0.rb
tools/scripts/build_compilation_db.sh
test/testdata/cfg/retry.rb.desugar-tree-raw.exp
test/whitequark/test_module_0.rb
test/testdata/lsp/completion/sig_no_method.rb
test/whitequark/test_casgn_invalid_4.rb
test/whitequark/test_ruby_bug_11873_a_16.rb
gems/sorbet/test/snapshot/partial/fake-rails/src
test/whitequark/test_next_block_0.rb
test/testdata/lsp/completion/overloads_test.B.rbedited
test/testdata/namer/constant_redefinition/constant_then_module.rb
test/testdata/desugar/lambda.rb.desugar-tree.exp
test/testdata/infer/singleton_case_exhaustiveness.rb
gems/sorbet/test/snapshot/partial/webmock/src
test/testdata/infer/attached_class_factory_example.rb
test/cli/no-did-you-mean/no-did-you-mean.out
test/testdata/cfg/rescue.rb.cfg.exp
gems/sorbet/test/snapshot/partial/rails-double-require/src/config/database.yml
test/testdata/desugar/defs_not_self.rb
gems/sorbet/test/snapshot/partial/non-utf-8-file/src/Gemfile.lock
gems/sorbet/test/snapshot/partial/use-existing-config/src/foo.rb
payload/binary/BUILD
core/lsp/PreemptionTaskManager.cc
gems/sorbet/test/snapshot/partial/ignore_file_table
test/whitequark/test_send_call_1.rb
gems/sorbet/test/snapshot/partial/bad_gem/expected
main/lsp/requests/hover.cc
test/cli/suggest_static/suggest_static.out
gems/sorbet/lib/gem-generator-tracepoint/tracepoint_serializer.rb
common
test/cli/folder-input-not-found
main/lsp/LSPMessage.cc
test/whitequark/test_space_args_cmd_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_12669_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_8.parse-tree-whitequark.exp
rbi/core/rational.rbi
test/testdata/resolver/fuzz_type_member_scope.rb.symbol-table-raw.exp
test/helpers/lsp.cc
test/whitequark/test_asgn_mrhs_1.parse-tree-whitequark.exp
test/testdata/rewriter/prop_skipped.rb
test/whitequark/test_defined_2.rb
test/testdata/namer/extend.rb
test/testdata/parser/misc.rb.parse-tree.exp
gems/sorbet/test/snapshot/partial/local_gem/gems
test/whitequark/test_false_0.rb
gems/sorbet-runtime/lib/types/private/abstract/hooks.rb
rewriter/Delegate.h
rewriter/Rails.cc
test/cli/bad-plugin-spec/non-string-in-ruby-extra-args.yaml
test/whitequark/test_send_block_chain_cmd_2.parse-tree-whitequark.exp
test/whitequark/test_zsuper_0.parse-tree-whitequark.exp
website/docs/intersection-types.md
docs/img/chrome-tracing-typecheck-one.png
test/testdata/resolver/invalid_alias.rb.symbol-table-raw.exp
test/whitequark/test_masgn_splat_4.rb
test/testdata/rewriter/default_args.rb.rewrite-tree.exp
test/cli/errors/errors.rb
test/whitequark/test_def_0.parse-tree-whitequark.exp
test/testdata/intrinsics/shapes.rb
test/whitequark/test_send_attr_asgn_3.rb
test/testdata/namer/gvar.rb.symbol-table-raw.exp
third_party/licenses/progressbar.txt
test/cli/autogen-subclasses-ignore/not-ignored
test/cli/suggest_t_must/suggest_t_must.out
gems/sorbet-runtime/lib/types/props/plugin.rb
test/lsp/watchman_test_corpus.cc
test/whitequark/test_gvasgn_0.rb
ast/verifier/BUILD
tools/toolchain/webasm-linux/emcc.sh
test/cli/autogen-subclasses
rewriter/Minitest.cc
test/whitequark/test_masgn_splat_5.parse-tree-whitequark.exp
tools/toolchain/webasm-darwin
test/testdata/resolver/sealed_with_rbi__3.rbi
docs/logo/README.md
test/testdata/resolver/fuzz_sig_generic_field.rb
test/testdata/rewriter/rails/mattr_writer.rb.rewrite-tree.exp
test/cli/suggest-typed-true/suggest-typed-behaviour-over-multiple-1.rb
payload/binary
test/whitequark/test_asgn_keyword_invalid_5.rb
test/whitequark/test_regex_plain_0.parse-tree-whitequark.exp
test/cli/stop-after/stop-after.sh
test/testdata/rewriter/prop_enum.rb
test/whitequark/test_arg_label_2.parse-tree-whitequark.exp
docs/img
test/testdata/rbi/each_with_object.rb
test/testdata/infer/generic_methods
test/cli/suggest_t_must/suggest_t_must.sh
test/testdata/infer/huge_unions.rb
test/whitequark/test_var_op_asgn_keyword_invalid_0.rb
test/whitequark/test_send_plain_1.rb
core/types/calls.cc
test/whitequark/test_yield_block_1.rb
test/testdata/parser/error_recovery_assign.rb
test/whitequark/test_send_binary_op_20.rb
test/cli/autocorrect-same-loc/autocorrect-same-loc-1.rb
test/testdata/deviations/non_ruby_names.rb
rbi/core/comparable.rbi
test/testdata/infer/forward_proc.rb
test/testdata/autogen
test/testdata/namer/module_function.rb.cfg.exp
test/cli/suggest-object/suggest-object.sh
test/testdata/cfg/retry.rb
test/testdata/desugar/hash.rb.desugar-tree.exp
test/testdata/resolver/resolve_tree_printing.rb
test/testdata/autogen/bare_class.rb
test/cli/suggest-new/suggest-new.sh
gems/sorbet/test/snapshot/partial/db_schema/src/Gemfile
test/testdata/lsp/completion/snippet_block.B.rbedited
test/testdata/cfg/break.rb
test/whitequark/test_send_binary_op_12.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_15.rb
main/lsp/LSPInput.cc
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems
test/testdata/rewriter/t_struct/root.rb
NOTICE
test/whitequark/test_cvar_0.rb
main/lsp/requests/workspace_symbols.cc
gems/sorbet/test/snapshot/partial/ignore_file_table/src
test/testdata/lsp/completion/implicit_self.rb
test/whitequark/test_for_1.rb
test/whitequark/test_space_args_arg_block_0.parse-tree-whitequark.exp
test/cli/rbi-overrides/t1.rbi
test/testdata/lsp/fast_path/method_change_kw_arg_type.1.rbupdate
gems/sorbet-runtime/sorbet-runtime.gemspec
test/testdata/infer/generics/variance_params.rb
gems/sorbet/test/snapshot/partial/use-existing-config/expected/sorbet/config
test/whitequark/test_masgn_attr_2.parse-tree-whitequark.exp
test/testdata/cfg/unreachable.rb
test/cli/print_generics/print_generics.sh
core/serialize/serialize.cc
website/docs/final.md
test/whitequark/test_send_binary_op_13.parse-tree-whitequark.exp
test/testdata/namer/root_private.rb
test/whitequark/test_unless_0.rb
core/TypesAndOrigins.cc
third_party/zlib.BUILD
test/testdata/lsp/code_actions/private.A.rbedited
test/testdata/resolver/optional_block.rb
test/whitequark/test_alias_0.parse-tree-whitequark.exp
test/cli/cli_test.bzl
test/whitequark/test_unary_num_pow_precedence_2.parse-tree-whitequark.exp
test/cli/config-file
third_party/licenses/googletest.txt
test/cli/rbi-overrides/t4.rbi
third_party/ruby/build-ruby.bzl
LICENSE
test/cli/config-file-recursive/sorbet/other-config
test/cli/allowed-extension/allowed-extension.out
rbi/stdlib/pp.rbi
test/whitequark/test_masgn_cmd_0.rb
test/whitequark/test_send_binary_op_13.rb
gems/sorbet-runtime/lib/types/types/simple.rb
test/whitequark/test_and_asgn_1.rb
test/test_corpus.cc
test/whitequark/test_block_arg_combinations_7.parse-tree-whitequark.exp
test/testdata/namer/fuzz_type_template_overwrite.rb
test/whitequark/test_string_plain_0.parse-tree-whitequark.exp
test/cli/suggest-kernel
test/whitequark/test_class_definition_in_while_cond_1.rb
core/serialize/pickler.h
test/whitequark/test_back_ref_0.rb
test/testdata/lsp/completion/sig_redefinition__2.B.rbedited
test/whitequark/test_block_arg_combinations_15.parse-tree-whitequark.exp
test/whitequark/test_bug_lambda_leakage_0.rb
test/whitequark/test_op_asgn_index_cmd_0.rb
test/cli/stop-after-namer
test/whitequark/test_block_arg_combinations_19.parse-tree-whitequark.exp
test/whitequark/test_super_0.parse-tree-whitequark.exp
test/testdata/infer/leaking_exceptions.rb
test/whitequark/test_ruby_bug_11873_a_2.rb
test/testdata/rewriter/minitest.rb.rewrite-tree.exp
website/static/css/overrides.css
test/whitequark/test_kwarg_no_paren_0.parse-tree-whitequark.exp
test/whitequark/test_send_binary_op_1.rb
test/cli/symbol-table-json/symbol-table-json.sh
gems/sorbet/test/snapshot/partial/ignore_file_table/expected/sorbet
test/cli/autogen-classlist/autogen-classlist.sh
test/whitequark/test_kwarg_combinations_0.rb
test/testdata/resolver/linearization/linearization4a.rb.symbol-table-raw.exp
test/testdata/infer/case_untyped.rb
test/testdata/desugar/accidentally_quadratic.rb.desugar-tree.exp
gems/sorbet/test/snapshot/total
test/cli/subprocess-plugin/multi4.rb
test/whitequark/test_lvar_0.parse-tree-whitequark.exp
test/testdata/lsp/code_actions/private.B.rbedited
test/whitequark/test_ruby_bug_10279_0.parse-tree-whitequark.exp
test/testdata/cfg/next_in_junk.rb
test/testdata/infer/fuzz_toplevel_type_member.rb
test/testdata/resolver/implementations.rb
test/cli/ignore-slash/foo/foo.rb
core/LocalVariable.h
test/testdata/autogen/ancestors.rb.autogen.exp
core
test/testdata/namer/ancestors.rb.flatten-tree.exp
test/testdata/infer/control_flow/nil_p.rb
test/testdata/rewriter/attr_bad_string.rb
test/whitequark/test_module_0.parse-tree-whitequark.exp
test/testdata/resolver/resolve_via_ancestors
test/whitequark/test_send_binary_op_9.rb
test/cli/ignore/ignore.out
test/testdata/rewriter/struct_self.rb
test/testdata/resolver/resolve_through_alias.rb.symbol-table-raw.exp
test/testdata/infer/generics/glb2.rb
test/whitequark/test_send_self_block_2.rb
test/cli/folder-input/input/bar.rb
test/testdata/lsp/hover_generics.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi
test/whitequark/test_send_index_0.rb
test/testdata/resolver/field.rb
third_party/parser
test/cli/silence-dev-message/silence-dev-message.out
tools/scripts
test/testdata/desugar/backtick.rb
rbi/core/range.rbi
test/cli/logging/logging.sh
test/whitequark/test_arg_combinations_7.parse-tree-whitequark.exp
test/whitequark/test_unless_else_1.rb
tools/scripts/format_website.sh
test/testdata/infer/infer1.rb.cfg.exp
test/whitequark/test_send_binary_op_8.rb
test/cli/print_to_file/b.rb
test/testdata/lsp/completion/constants_t.rb
test/testdata/lsp/completion/constants_aliases.rb
test/whitequark/test_for_0.parse-tree-whitequark.exp
test/cli/arity-messages/arity-messages.rb
rewriter/Mattr.cc
test/testdata/parser/empty.rb
test/testdata/desugar/constant_error.rb
test/cli/suggest-typed-true/suggest-typed-already-ignore.rb
test/whitequark/test_bang_cmd_0.parse-tree-whitequark.exp
test/helpers/expectations.h
test/testdata/infer/generics/fixed_ordering.rb
test/whitequark/test_def_5.rb
test/testdata/core
test/testdata/infer/nil_noreturn.rb
test/whitequark/test_send_plain_cmd_0.rb
test/cli/no-stdlib/no-stdlib.sh
test/testdata/namer/fuzz_ivar_constant_scope.rb
test/whitequark/test_ruby_bug_12402_1.rb
test/testdata/lsp/completion/constants_all_kinds.rb
test/whitequark/test_ruby_bug_11873_a_17.rb
test/whitequark/test_ruby_bug_12669_1.parse-tree-whitequark.exp
gems/sorbet/lib/gem_loader.rb
test/testdata/desugar/complex.rb.desugar-tree.exp
test/testdata/parser/fuzz_rational.rb
test/whitequark/test_break_3.parse-tree-whitequark.exp
test/testdata/lsp/hover_proc_void.rb
test/cli/suggest-typed-true/suggest-typed-already-autogenerated.rb
test/whitequark/test_send_attr_asgn_conditional_0.rb
test/whitequark/test_array_symbols_interp_1.rb
gems/sorbet/test/snapshot/partial/use-existing-config/src/sorbet/config
test/testdata/infer/control_flow/bug_852.rb
main/autogen/subclasses.h
test/whitequark/test_ruby_bug_12669_2.parse-tree-whitequark.exp
test/testdata/infer/isa_generic.rb.cfg.exp
test/whitequark/test_emit_arg_inside_procarg0_legacy_0.parse-tree-whitequark.exp
rbi/stdlib/set.rbi
test/cli
test/testdata/lsp/errors_clear_after_fixing.rb
test/whitequark/test_parser_bug_525_0.rb
test/testdata/cfg/ivar_assign.rb
test/whitequark/test_ruby_bug_11380_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/config
test/cli/parallel-ordering/1.rb
namer/configatron/BUILD
test/testdata/lsp/code_actions/typo.B.rbedited
main/lsp/requests/signature_help.cc
test/whitequark/test_block_arg_combinations_10.parse-tree-whitequark.exp
website/docs/static.md
test/cli/lsp-requires-input-dir/lsp-requires-input-dir.out
plugin/Plugins.h
test/cli/bad-perm/bad-perm.sh
test/whitequark/test_array_symbols_empty_1.rb
test/testdata/rewriter/prop_computed_by.rb.rewrite-tree.exp
test/whitequark/test_send_block_conditional_0.rb
gems/sorbet-runtime/test/types/final_module.rb
rbi/core/integer.rbi
test/whitequark/test_asgn_cmd_0.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_17.parse-tree-whitequark.exp
test/cli/escaping/escaping.sh
test/whitequark/test_next_block_0.parse-tree-whitequark.exp
test
emscripten/main.cc
rewriter/TypeMembers.cc
gems/sorbet-runtime/lib/types/types/proc.rb
test/testdata/rewriter/minitest.rb
test/whitequark/test_masgn_splat_8.parse-tree-whitequark.exp
parser/Node.cc
third_party/README.md
test/testdata/infer/hashes.rb.cfg.exp
test/testdata/lsp/completion/sig_no_method.B.rbedited
test/testdata/namer/method_redef.rb
test/whitequark/test_marg_combinations_9.parse-tree-whitequark.exp
test/whitequark/test_var_op_asgn_2.rb
test/testdata/deviations/superclass_implicit.rb.symbol-table-raw.exp
test/whitequark/test_defs_3.rb
test/whitequark/test_send_binary_op_18.rb
test/cli/ignore
test/testdata/desugar/line_literal.rb.desugar-tree.exp
test/cli/suggest-object
test/testdata/namer/constants.rb
test/whitequark/test_marg_combinations_7.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/expected/sorbet/rbi/gems
docs/logo/sorbet-logo-purple-sparkles.svg
test/whitequark/test_array_words_empty_0.rb
test/cli/dedup-input-files/dedup-input-files.out
rewriter/Flatfiles.h
test/whitequark/test_space_args_cmd_0.rb
payload/text/nopopulate.cc
test/testdata/namer/fuzz_type_member.rb
test/whitequark/test_array_assocs_1.parse-tree-whitequark.exp
test/cli/version-returns-0/version-returns-0.out
gems/sorbet/test/snapshot/snapshot.bzl
test/testdata/infer/generics/enumerable.rb
test/cli/forgot-typed/forgot-typed.out
test/whitequark/test_on_error_0.rb
test/cli/parse-tree-whitequark/parse-tree-whitequark.sh
common/Timer.cc
test/testdata/resolver/fuzz_multiple_sigs.rb
rbi/core/unbound_method.rbi
test/testdata/rewriter/rails/cattr_accessor.rb.rewrite-tree.exp
gems/sorbet-runtime/lib/types/interface_wrapper.rb
main/pipeline/semantic_extension/BUILD
test/testdata/infer/control_flow/complex_implication_1.rb.cfg.exp
test/testdata/lsp/sig_deletion.1.rbupdate
test/testdata/desugar/nthref.rb.parse-tree.exp
rbi/stdlib/delegator.rbi
common/Subprocess.h
rewriter/Cleanup.cc
website/static/talks/index.html
gems/sorbet/test/snapshot/partial/create-config
test/whitequark/test_casgn_toplevel_0.parse-tree-whitequark.exp
test/lsp/no-trailing-newline
main/autogen/autogen.h
test/testdata/lsp/completion/sig.B.rbedited
rewriter/Flatten.h
test/testdata/resolver/sig_good.rb
test/testdata/rewriter/struct.rb.rewrite-tree.exp
test/cli/file-table-json/file-table-json.out
test/whitequark/test_ruby_bug_11873_a_9.parse-tree-whitequark.exp
main/lsp/requests
test/whitequark/test_send_attr_asgn_conditional_0.parse-tree-whitequark.exp
gems/sorbet/bin/srb
test/testdata/infer/generics/duplicate_members.rb
rewriter/Initializer.h
test/testdata/cfg/reassign_dead_block_bug.rb.cfg.exp
test/testdata/autogen/simple_refs.rb
test/whitequark/test_send_binary_op_6.parse-tree-whitequark.exp
test/whitequark/test_zsuper_0.rb
test/whitequark/test_cond_begin_0.rb
gems/sorbet/test/snapshot/total/empty/expected/err.log
test/testdata/parser/typed_ignore.rb
test/whitequark/test_send_lambda_args_shadow_0.parse-tree-whitequark.exp
parser/tools/generate_ast.cc
test/testdata/namer/alias_method.rb.symbol-table-raw.exp
test/testdata/cfg/examples.rb
test/cli/correct-bare-stdlib-generics/correct-bare-stdlib-generics.out
gems/sorbet-runtime/lib/types/generic.rb
test/testdata/desugar/for.rb
main/lsp/LocalVarFinder.h
test/whitequark/test_ruby_bug_12669_2.rb
test/lsp/multithreaded_protocol_test_corpus.cc
test/whitequark/test_cvasgn_0.parse-tree-whitequark.exp
test/fuzz/fuzz_hover.cc
test/whitequark/test_send_call_1.parse-tree-whitequark.exp
test/whitequark/test_break_2.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_13547_12.rb
gems/sorbet-runtime/lib/types/types/self_type.rb
test/cli/suggest-typed-true/suggest-typed-behaviour-over-multiple-2.rb
test/whitequark/test_if_1.parse-tree-whitequark.exp
test/whitequark/test_class_1.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/types/typed_enumerator.rb
rbi/core/fiber_error.rbi
gems/sorbet/test/snapshot/partial/local_gem/gems/0.0.0/gems
rbi/stdlib/tsort.rbi
rbi/core/true_class.rbi
core/types/printing.cc
gems/sorbet-runtime/bench/getters.rb
test/whitequark/test_space_args_arg_block_1.rb
test/testdata/resolver/type_member_constant_assignment.rb.symbol-table-raw.exp
test/cli/suggest-typed-true/suggest-typed-shabang.rb
gems/sorbet/test/snapshot/partial/fake-rails/src/Gemfile
test/cli/suggest-type-alias/suggest-type-alias.rb
test/cli/bad-plugin-spec/missing-triggers.yaml
test/testdata/desugar/assign_keyword.rb
rewriter/Cleanup.h
rewriter/BUILD
rbi/core/encoding.rbi
test/testdata/lsp/ivars.rb
rewriter/Regexp.h
third_party/gems/BUILD
test/whitequark/test_bug_cmdarg_1.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_singleton.A.rbedited
test/testdata/resolver/resolve_through_alias.rb
test/testdata/infer/splat.rb
test/testdata/infer/assign_self.rb
test/lsp/alias-incremental
test/testdata/lsp/completion/constants_existing.rb
test/backtrace-test-raise.cc
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/gem.rbi
common/os/emscripten.cc
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset/gems/my_gem-0.0.0/lib
test/testdata/namer/class_and_alias.rb
third_party/progressbar.BUILD
rewriter/InterfaceWrapper.h
test/whitequark/test_space_args_arg_block_2.rb
test/testdata/infer/unwrap_locs.rb
test/testdata/autogen/resolving_refs.rb.autogen.exp
gems/sorbet/test/snapshot/partial/bad_gem/src/Gemfile.lock
website/pages
test/testdata/desugar/case.rb
test/cli/suggest-new/suggest-new.rb
test/whitequark/test_ruby_bug_11873_6.parse-tree-whitequark.exp
test/testdata/cfg/do_while.rb.cfg.exp
test/whitequark/test_ruby_bug_12402_2.parse-tree-whitequark.exp
main/lsp/notifications/sorbet_workspace_edit.h
tools/BUILD
test/whitequark/test_bug_interp_single_1.parse-tree-whitequark.exp
test/testdata/rewriter/t_struct/nilable.rb.rewrite-tree.exp
third_party/licenses/statsd-c-client.txt
test/testdata/autogen/bare_alias.rb.autogen.exp
rbi/core/regexp.rbi
test/testdata/lsp/completion/sig_redefinition__1.B.rbedited
test/testdata/rewriter/rails/cattr_reader.rb
test/testdata/desugar/hash.rb
test/whitequark/test_send_self_2.parse-tree-whitequark.exp
third_party/parser/cc/capi.cc
test/testdata/namer/docs_example_1.rb
test/testdata/infer/generics/class_less_than.rb
rbi/core/data.rbi
core/ErrorQueue.cc
test/whitequark/test_gvasgn_0.parse-tree-whitequark.exp
website/static/slack
rbi/sorbet/t.rbi
test/cli/autocorrect-remove-body/pre.rb
test/whitequark/test_send_binary_op_16.rb
website/static/img/sorbet-logo-purple-sparkles.svg
gems/sorbet-runtime/lib/types/types/typed_array.rb
common/backtrace.cc
test/whitequark/test_space_args_block_0.parse-tree-whitequark.exp
cfg/builder/BUILD
test/whitequark/test_ruby_bug_11873_a_19.rb
test/testdata/cfg/array.rb.cfg.exp
sorbet_version/sorbet_version.h
test/testdata/namer/circular_mixin.rb
test/cli/autocorrect-private/autocorrect-private.rb
test/testdata/desugar/sclass.rb
test/testdata/resolver/sealed_module.rb
gems/sorbet/lib/gem-generator-tracepoint.rb
gems/sorbet-runtime/lib/types/types/type_variable.rb
test/testdata/rewriter/prop.rb
test/whitequark/test_multiple_args_with_trailing_comma_0.rb
rbi/core/thread_group.rbi
test/whitequark/test_block_arg_combinations_2.parse-tree-whitequark.exp
core/errors/rewriter.h
test/testdata/resolver/linearization/linearization5.rb
test/cli/rbi-with-code/rbi-with-code.rbi
test/testdata/rewriter/fuzz_qualified_lhs.rb
test/lsp/cache_protocol_test_corpus.cc
test/cli/suggest_autogen/suggest_autogen.rb
test/whitequark/test_bug_447_0.rb
test/cli/cache-doesnt-parse/cache-doesnt-parse.sh
test/whitequark/test_rescue_else_ensure_0.rb
test/whitequark/test_class_invalid_1.rb
gems/sorbet-runtime/lib/types/props/type_validation.rb
test/testdata/rewriter/t_enum_alias.rb
test/cli/autogen-autoloader/autogen-autoloader.sh
core/types/typemaps.cc
test/whitequark/test_ensure_0.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/ignore_file_table/src/bad.rb
gems/sorbet/test/snapshot/sorbet-typed.rev
main/lsp/ShowOperation.cc
website/docs/cli.md
test/whitequark/test_bug_447_1.rb
test/testdata/resolver/recover_from_bad_superclass.rb
test/cli/ignore-slash/foo
test/whitequark/test_ruby_bug_10653_0.parse-tree-whitequark.exp
namer/BUILD
test/testdata/lsp/code_actions/sig_missing__child.A.rbedited
test/whitequark/test_args_args_assocs_0.parse-tree-whitequark.exp
main/lsp/ShowOperation.h
test/cli/autocorrect/autocorrect.sh
test/whitequark/test_bug_452_0.parse-tree-whitequark.exp
test/cli/autogen-autoloader/bar2.rb
test/cli/error-whitelist
test/testdata/infer/non_forcing_is_a_false.rb
rbi/core/gc.rbi
test/whitequark/test_ruby_bug_12686_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_13547_3.rb
test/cli/suggest-typos
payload/text
test/whitequark/test_array_splat_1.rb
website/static/slack/index.html
test/whitequark/test_range_endless_0.parse-tree-whitequark.exp
common/BUILD
test/cli/bad-plugin-spec/ruby-extra-args-not-array.yaml
third_party/parser/BUILD
test/whitequark/test_lvasgn_0.parse-tree-whitequark.exp
test/whitequark/test_masgn_cmd_0.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/method_signature_update.1.rbupdate
test/whitequark/test_casgn_unscoped_0.parse-tree-whitequark.exp
test/testdata/infer/dispatch_call_and.rb
test/whitequark/test_ensure_empty_0.parse-tree-whitequark.exp
test/testdata/resolver/missing_helpers_abstract.rb
docs
test/testdata/rewriter/t_struct/none.rb.rewrite-tree.exp
main/lsp/requests/completion.h
ACKNOWLEDGEMENTS.md
test/testdata/infer/control_flow/truthiness.rb
test/testdata/desugar/top_level_const.rb.desugar-tree-raw.exp
test/testdata/resolver/linearization/linearization2.rb.symbol-table-raw.exp
test/testdata/infer/massign_return_rhs.rb.desugar-tree.exp
test/testdata/lsp/completion/snippet_block.D.rbedited
test/cli/logging
test/testdata/lsp/completion/snippet_keywords.rb
test/whitequark/test_block_kwarg_combinations_0.rb
test/whitequark/test_send_index_legacy_0.rb
test/testdata/cfg/blocks.rb.parse-tree.exp
common/Counters.cc
main/lsp/test/lsp_preprocessor_test.cc
test/whitequark/test_resbody_list_0.rb
website/docs/error-reference.md
test/testdata/autogen/bad_prop.rb.autogen.exp
test/whitequark/test_send_lambda_args_noparen_0.parse-tree-whitequark.exp
test/whitequark/test_procarg0_0.parse-tree-whitequark.exp
test/testdata/parser/error_recovery_missing_fun.rb
common/os/os.h
test/testdata/core/autogenerated.rb
test/cli/autogen-autoloader/errors.rb
test/testdata/lsp/completion/sig_snippet.rb
website/static/img/testimonial_pair_programming.png
ast/ArgParsing.cc
test/testdata/desugar/regexp.rb.desugar-tree.exp
test/whitequark/test_op_asgn_cmd_1.parse-tree-whitequark.exp
test/whitequark/test_class_definition_in_while_cond_1.parse-tree-whitequark.exp
test/testdata/resolver/fuzz_wrong_fun_print.rb
docs/logo/sorbet-logo-white-sparkles-on-purple.svg
test/cli/statsd/statsd.out
test/whitequark/test_op_asgn_invalid_1.rb
test/whitequark/test_case_cond_else_0.parse-tree-whitequark.exp
test/cli/autogen-subclasses-ignore
test/whitequark/test_masgn_nested_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_a_8.rb
main/lsp/notifications/exit.cc
test/cli/line-splitting/line-splitting.sh
test/whitequark/test_break_block_0.parse-tree-whitequark.exp
third_party/licenses/protobufmutator.txt
gems/sorbet/test/snapshot/partial/type_member/src/my_enumerable.rb
test/testdata/namer/nested_class.rb.flatten-tree.exp
test/whitequark/test_bug_lambda_leakage_0.parse-tree-whitequark.exp
test/testdata/rbi/class.rb
test/whitequark/test_yield_1.rb
website/docs/legal/trademark-policy.md
test/cli/progressbar/progressbar.sh
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset/gems/my_gem-0.0.0
test/cli/folder-input-dir-and-file/input/bar.rb
test/testdata/infer/control_flow_in_rescue.rb
test/testdata/lsp/fast_path/parse_errors.3.rbupdate
test/testdata/lsp/no_stdlib.rb
test/testdata/infer/fuzz_static_init_loc.rb
rbi/stdlib/uri.rbi
payload/binary/tools/gen_state_payload.cc
test/whitequark/test_op_asgn_0.rb
test/testdata/namer/circular_mixin.rb.symbol-table-raw.exp
test/whitequark/test_arg_duplicate_ignored_0.parse-tree-whitequark.exp
test/cli/config-file/sorbet/other_config
test/testdata/desugar/fuzz_include_self.rb
test/testdata/cfg/rescue_var_expression.rb.cfg.exp
test/testdata/rewriter/attr_strict.rb
test/testdata/rewriter/default_args_malformed_sig.rb
test/whitequark/test_ruby_bug_11873_a_11.rb
test/cli/symbol-table-json-alias/symbol-table-json-alias.out
test/whitequark/test_masgn_splat_9.parse-tree-whitequark.exp
CODE_OF_CONDUCT.md
test/testdata/parser/anon_blockarg.rb
test/whitequark/test_nil_expression_1.rb
gems/sorbet/test/snapshot/partial/rails-double-require/expected/err.log
test/whitequark/test_arg_combinations_8.parse-tree-whitequark.exp
test/testdata/resolver/sealed_concrete_parent_class.rb
test/cli/at/at.input
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_extend__2.rb
rewriter/SigRewriter.h
test/testdata/rbi/tempfile.rb
website
test/whitequark/test_ruby_bug_13547_2.rb
docs/suggest-sig.md
test/whitequark/test_arg_duplicate_8.rb
test/whitequark/test_case_cond_0.rb
test/whitequark/test_space_args_arg_block_1.parse-tree-whitequark.exp
test/whitequark/test_self_0.parse-tree-whitequark.exp
docs/logo
test/whitequark/test_hash_hashrocket_1.parse-tree-whitequark.exp
gems/sorbet-runtime/test/types/edge_cases.rb
test/whitequark/test_unary_num_pow_precedence_0.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_6.parse-tree-whitequark.exp
test/cli/print_to_file/print_to_file.out
core/lsp/QueryResponse.cc
test/whitequark/test_masgn_splat_9.rb
test/testdata/autogen/bare_module.rb.autogen.exp
test/testdata/lsp/fast_path/method_change_return_type__def.rb
test/whitequark/test_string_plain_1.rb
test/whitequark/test_marg_combinations_3.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_21.rb
test/testdata/lsp/completion/snippet_repeated.A.rbedited
test/whitequark/test_array_symbols_interp_1.parse-tree-whitequark.exp
test/testdata/namer/constant_redefinition/class_then_constant_then_reopen.rb
test/testdata/lsp/completion/non_prefix.rb
test/testdata/parser/heredoc_chomp.rb
test/testdata/lsp/field_edits.1.rbupdate.document-symbols.exp
rbi/core/numeric.rbi
main/lsp/LSPIndexer.cc
test/testdata/resolver/abstract_override.rb
test/testdata/autogen/generator.rb
test/cli/suggest-foreign-lambda/suggest-foreign-lambda.rb
test/testdata/lsp/completion/snippet_keywords.C.rbedited
test/testdata/resolver/let_errors_false.rb
test/testdata/infer/generics/wrong_type_member.rb
third_party/cxxopts.BUILD
test/testdata/cfg/floats.rb.cfg.exp
rbi/stdlib/racc.rbi
common/concurrency/WorkerPool.h
test/cli/rbi-overrides/t3.rb
test/whitequark/test_case_cond_else_0.rb
test/whitequark/test_var_or_asgn_0.rb
test/whitequark/test_marg_combinations_7.parse-tree-whitequark.exp
test/whitequark/test_send_block_conditional_0.parse-tree-whitequark.exp
website/static/docs/rake.html
rbi/stdlib/etc.rbi
tools/scripts/regen-emscripten-cache.sh
test/testdata/resolver/self.rb.symbol-table-raw.exp
payload/payload.h
test/testdata/rbi/io.rb
test/whitequark/test_bug_cmdarg_0.parse-tree-whitequark.exp
gems/sorbet/lib/serialize.rb
test/testdata/lsp/errors_clear_after_fixing.1.rbupdate
test/testdata/infer/all_bug.rb
website/static/img/sorbet-logo.svg
test/testdata/lsp/fast_path/class_change_include_multifile__using.1.rbupdate
test/whitequark/test_send_binary_op_4.rb
test/whitequark/test_const_op_asgn_2.rb
test/whitequark/test_array_words_empty_0.parse-tree-whitequark.exp
test/whitequark/test_send_index_cmd_0.rb
gems/sorbet/test/snapshot/partial/extconf/src
test/testdata/resolver/sealed_class.rb
gems/sorbet-runtime/lib/types/types/typed_range.rb
test/testdata/local_vars/z_super_args.rb.index-tree.exp
test/testdata/desugar/destructure.rb.flatten-tree.exp
payload/BUILD
test/whitequark/test_block_arg_combinations_18.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/local_gem/src/Gemfile.lock
test/whitequark/test_sclass_0.rb
docs/logo/sorbet-logo-45-deg@1x.png
test/whitequark/test_kwbegin_compstmt_0.parse-tree-whitequark.exp
test/whitequark/test_int_2.rb
ast/desugar/Desugar.cc
test/testdata/infer/singleton_methods.rb.cfg.exp
test/whitequark/test_class_super_label_0.rb
gems/sorbet-runtime/lib/types/props/custom_type.rb
main/lsp/requests/sorbet_read_file.cc
test/cli/parse-tree-whitequark
rewriter/Minitest.h
test/lsp/BUILD
gems/sorbet/test/snapshot/partial/fake-object/src
test/whitequark/test_unless_else_0.parse-tree-whitequark.exp
test/whitequark/test_casgn_invalid_1.rb
test/whitequark/test_defs_4.parse-tree-whitequark.exp
test/testdata/tuple_type_size.rb
test/testdata/infer/huge_unions.rb.cfg.exp
common/web_tracer_framework/BUILD
test/testdata/cfg/array.rb
test/whitequark/test_arg_duplicate_ignored_1.rb
test/testdata/cfg/return_type_of_nilable_blk_param.rb
test/testdata/lsp/definitions_and_usages_untyped__untyped.rb
test/testdata/lsp/completion/sig_singleton.rb
test/testdata/resolver/bad_sealed_class__1.rb
test/cli/autogen-autoloader/scripts
test/whitequark/test_class_super_0.rb
test/whitequark/test_defs_0.parse-tree-whitequark.exp
tools/scripts/import_whitequark.rb
test/testdata/infer/dropsubtypesof.rb
test/testdata/call_with_splat_and_block_strict.rb
gems/sorbet/test/snapshot/partial/db_schema/src/Gemfile.lock
test/testdata/lsp/completion/snippet_repeated.D.rbedited
definition_validator/validator.h
rbi/gems/didyoumean.rbi
test/whitequark/test_ruby_bug_11873_a_12.rb
test/whitequark/test_lparenarg_after_lvar_since_25_0.rb
test/testdata/resolver/top_level.rb
test/testdata/lsp/document_symbols_crash.1.rbupdate
test/whitequark/test_send_block_chain_cmd_2.rb
WORKSPACE
test/testdata/infer/casts.rb
gems/sorbet-runtime/test/types/fixtures
test/testdata/autogen/bare_alias.rb
gems/sorbet/test/snapshot/total/sorbet-runtime
test/whitequark/test_marg_combinations_0.parse-tree-whitequark.exp
infer/BUILD
gems/sorbet-runtime/lib/types/private/methods/decl_builder.rb
test/whitequark/test_op_asgn_cmd_2.parse-tree-whitequark.exp
test/testdata/infer/flat_map.rb
test/whitequark/test_block_arg_combinations_14.rb
test/whitequark/test_bug_regex_verification_0.rb
test/cli/lsp-large-message
test/testdata/namer/singleton_class.rb
test/testdata/parser/index_assign.rb.parse-tree.exp
test/whitequark/test_array_words_interp_0.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_5.parse-tree-whitequark.exp
tools/scripts/make_worktree.sh
main/lsp/requests/get_counters.cc
test/whitequark/test_resbody_list_0.parse-tree-whitequark.exp
main/lsp/LSPPreprocessor.cc
test/whitequark/test_unary_num_pow_precedence_2.rb
test/testdata/rewriter/not_prop.rb.rewrite-tree.exp
test/cli/dedup_loc/dedup_loc.rb
test/cli/folder-input-dir-and-file/foo.rb
test/whitequark/test_symbol_interp_0.rb
test/testdata/cfg/examples.rb.symbol-table-raw.exp
test/testdata/infer/module_function_two.rb
parser/test/parser_test.cc
test/whitequark/test_rational_1.rb
main/autogen/autoloader.h
gems/sorbet-runtime/lib/types/private
test/whitequark/test_cond_eflipflop_0.rb
test/cli/autocorrect-bare-stdlib-generics/autocorrect-bare-stdlib-generics.sh
core/Hashing.h
test/whitequark/test_log_asgn_invalid_1.rb
test/cli/suggest-sig-garbage/suggest-sig-garbage.rb
rbi/core/file.rbi
rewriter/Prop.cc
rbi/sorbet/compatibility_patches.rbi
test/cli/config-file-recursive/config-file-recursive.out
test/whitequark/test_unless_else_1.parse-tree-whitequark.exp
test/cli/allowed-extension/lib/a.rb
core/SymbolRef.h
test/cli/suggest-sig-override/suggest-sig-override.rb
test/whitequark/test_arg_1.rb
test/cli/subprocess-plugin/multi.yaml
test/whitequark/test_array_plain_0.parse-tree-whitequark.exp
test/whitequark/test_regex_interp_0.rb
test/testdata/parser/fuzz_ivar.rb
gems/sorbet/test/snapshot/partial/rspec-lots/src/src.rb
test/testdata/namer/simple.rb.flatten-tree.exp
test/whitequark/test_block_arg_combinations_14.parse-tree-whitequark.exp
gems/sorbet-runtime/README.md
test/testdata/lsp/code_actions
tools/scripts/fuzz.sh
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_include__3.rb
test/whitequark/test_block_arg_combinations_22.parse-tree-whitequark.exp
test/cli/suggest-kernel/suggest-kernel.out
test/testdata/infer/generics/isa_with_type_member.rb
test/whitequark/test_send_lambda_0.parse-tree-whitequark.exp
rewriter/DefaultArgs.cc
gems/sorbet-runtime/lib/sorbet-runtime.rb
test/cli/dash-e
gems/sorbet-runtime/test/types/types_to_ruby.rb
test/whitequark/test_op_asgn_2.rb
test/whitequark/test_defs_3.parse-tree-whitequark.exp
test/whitequark/test_ENCODING_0.parse-tree-whitequark.exp
third_party/licenses/typedruby.txt
test/whitequark/test_class_invalid_0.rb
main/lsp/DefLocSaver.h
gems/sorbet-runtime/lib/types/props/private/serde_transform.rb
test/testdata/lsp/code_actions/private.rb
test/testdata/infer/match_operator.rb
test/cli/remove-path-prefix/remove-path-prefix.out
test/cli/correct-bare-stdlib-generics/correct-bare-stdlib-generics.rb
test/testdata/desugar/line_literal.rb
test/whitequark/test_rescue_mod_asgn_0.rb
test/whitequark/test_arg_combinations_9.parse-tree-whitequark.exp
test/testdata/lsp/completion/sig_redefinition__1.A.rbedited
test/testdata/cfg/ensure_after_raise.rb
test/lint
main/lsp/requests/initialize.cc
test/whitequark/test_const_op_asgn_3.rb
test/cli/incremental-resolver/incremental-resolver.sh
test/testdata/infer/generic_methods/infer_bottom.rb
test/testdata/lsp/fast_path/method_add_keyword_arg.1.rbupdate
test/whitequark/test_arg_duplicate_proc_0.rb
cfg/builder/builder_finalize.cc
test/whitequark/test_args_star_0.parse-tree-whitequark.exp
core/TypePtr.h
test/cli/allowed-extension
rbi/gems/configatron.rbi
test/testdata/rewriter/prop_default.rb
test/whitequark/test_return_0.rb
third_party/parser/cc/context.cc
test/cli/subprocess-plugin/ruby_extra_args.yaml
gems/sorbet/test/snapshot/partial/missing-instance-methods
test/testdata/lsp/completion/constants.rb
test/whitequark/test_ruby_bug_12402_9.rb
gems/sorbet-runtime/test/types/validate_override_shape.rb
test/cli/forbid-autocorrect-with-quiet/input.rb
gems/sorbet/test/snapshot/total/sorbet-runtime/expected/sorbet/rbi/sorbet-typed/lib/ruby/all/resolv.rbi
test/cli/incremental-resolver/type-template.rb
gems/sorbet/lib/constant_cache.rb
gems/sorbet/test/hidden-method-finder/thorough/src/sorbet/config
test/whitequark/test_send_op_asgn_conditional_0.rb
test/whitequark/test_ruby_bug_11873_a_12.parse-tree-whitequark.exp
test/cli/ignore/subfolder/baz.rb
test/testdata/rewriter/flatten_nested.rb.flatten-tree.exp
test/whitequark/test_super_0.rb
test/testdata/cfg/hash.rb
test/whitequark/test_op_asgn_invalid_2.rb
test/whitequark/test_ruby_bug_11873_a_7.parse-tree-whitequark.exp
third_party/parser/cc/grammars/typedruby.ypp
test/cli/suggest-typed-true/suggest-typed-and-type.rb
test/whitequark/test_if_masgn_24_0.rb
test/whitequark/test_ruby_bug_11873_a_13.parse-tree-whitequark.exp
test/testdata/rewriter/t_struct/default.rb
test/whitequark/test_send_attr_asgn_1.parse-tree-whitequark.exp
main/realmain.cc
test/testdata/namer/defs_in_blocks.rb.flatten-tree.exp
test/testdata/lsp/fast_path/method_change_kw_arg_name.1.rbupdate
gems/sorbet/test/snapshot/partial/rails-double-require/src/app
test/whitequark/test_not_masgn_24_0.rb
test/whitequark/test_alias_gvar_0.rb
test/whitequark/test_ruby_bug_10653_1.parse-tree-whitequark.exp
test/whitequark/test_return_block_0.rb
test/whitequark/test_masgn_const_invalid_1.rb
rbi/sorbet/builder.rbi
gems/sorbet/test/snapshot/partial/use-existing-config/expected/sorbet
gems/sorbet/test/hidden-method-finder/thorough/src/thorough.rb
test/testdata/cfg/uaf1.rb.cfg.exp
test/testdata/infer/generics/type_param_is_a.rb
test/whitequark/test_array_assocs_0.rb
test/testdata/lsp/document_symbols_crash.rb.document-symbols.exp
test/fuzz/fuzz_dash_e.cc
test/cli/autogen-autoloader/bar.rb
test/cli/suggest-t-let
test/testdata/infer/constructor_return.rb
test/whitequark/test_send_block_chain_cmd_4.rb
docs/logo/sorbet-logo.svg
test/testdata/infer/strict_dead.rb
test/whitequark/test_arg_0.parse-tree-whitequark.exp
test/whitequark/test_casgn_toplevel_0.rb
test/whitequark/test_string_concat_0.parse-tree-whitequark.exp
test/testdata/namer/arguments.rb.flatten-tree.exp
test/testdata/rewriter/attr_multi.rb
test/whitequark/test_ruby_bug_11873_a_3.parse-tree-whitequark.exp
test/cli/missing-constants/missing-constants.rb
website/static/img/sourcegraph_github.gif
test/testdata/deviations
test/whitequark/test_ruby_bug_11873_6.rb
gems/sorbet/test/snapshot/partial/non-utf-8-file
test/whitequark/test_array_words_interp_1.rb
test/whitequark/test_casgn_invalid_5.rb
test/whitequark/test_lparenarg_after_lvar_since_25_1.rb
test/cli/no-stdlib/no-stdlib.out
test/testdata/namer/superclass_redefinition.rb
gems/sorbet/test/snapshot/partial/db_schema/src/db/schema.rb
test/testdata/infer/restargs.rb
common/web_tracer_framework
test/whitequark/test_bug_cmdarg_0.rb
test/whitequark/test_op_asgn_1.rb
test/testdata/cfg/side_effects2.rb.cfg.exp
test/whitequark/test_array_symbols_0.rb
test/testdata/lsp/workspace_symbols_minitest_scope.rb
test/testdata/lsp/completion/sig.rb
gems/sorbet/test/snapshot
test/test_corpus_forwarder.sh
test/cli/ignore/subfolder2/subfolder
main/lsp/LSPFileUpdates.h
test/testdata/namer/visibility.rb.symbol-table-raw.exp
test/whitequark/test_send_binary_op_17.parse-tree-whitequark.exp
gems/sorbet-runtime/Rakefile
gems/sorbet/BUILD
tools/clang.bzl
test/whitequark/test_masgn_attr_2.rb
test/whitequark/test_arg_combinations_5.rb
tools/scripts/check_using_namespace_std.sh
test/testdata/lsp/hover_method.rb
test/whitequark/test_ruby_bug_12402_4.rb
test/testdata/perf/sealed_registration_perf.rb
docs/JRuby.md
website/static/img/autocompleteAndDocs1.gif
test/testdata/resolver/flatten.rb.flatten-tree.exp
gems/sorbet/test/snapshot/partial/db_schema
test/whitequark/test_ruby_bug_12402_12.rb
test/whitequark/test_arg_combinations_1.rb
test/testdata/namer/fuzz_dynamic_constant.rb
test/cli/cache-dsl/cache-dsl.sh
test/testdata/resolver/type_member_bad_parent.rb
gems/sorbet/test/snapshot/partial/rails-double-require/src/config
test/whitequark/test_const_op_asgn_1.parse-tree-whitequark.exp
test/testdata/rbi/exception.rb
test/whitequark/test_defs_invalid_4.rb
test/cli/suggest-sig/suggest-sig.sh
test/whitequark/test_send_index_asgn_0.rb
test/testdata/rewriter/flatfile_dsl.rb
test/cli/model_mutator_behavior/model_mutator_behavior__2.rb
test/testdata/lsp/workspace_symbols_multiply_defined_2.rb
third_party/parser/include/ruby_parser/lexer.hh
test/whitequark/test_ruby_bug_12073_1.parse-tree-whitequark.exp
test/cli/incremental-resolver/incremental-resolver.out
core/types
test/testdata/infer/generics/two_params.rb
test/whitequark/test_arg_combinations_6.parse-tree-whitequark.exp
tools/scripts/ci_checks.sh
test/testdata/resolver/type_member_missing_then_use.rb
test/whitequark/test_op_asgn_0.parse-tree-whitequark.exp
test/testdata/namer/singleton_class.rb.symbol-table-raw.exp
gems/sorbet/test/snapshot/partial/stupidedi/src/Gemfile.lock
test/cli/autogen-subclasses/autogen-subclasses.out
main/lsp/json_types.cc
test/whitequark/test_array_symbols_interp_0.rb
test/whitequark/test_range_exclusive_0.rb
test/whitequark/test_class_definition_in_while_cond_0.rb
gems/sorbet/test/snapshot/partial/db_schema/expected/sorbet
test/whitequark/test_ruby_bug_11873_5.parse-tree-whitequark.exp
local_vars
test/whitequark/test_ruby_bug_11873_a_15.rb
core/Unfreeze.cc
test/whitequark/test_not_cmd_0.rb
test/testdata/autogen/cbase_const.rb.autogen.exp
third_party/licenses/lmdb.txt
test/whitequark/test_super_2.parse-tree-whitequark.exp
third_party/progressbar/src/progressbar.c
third_party/ruby/ruby.BUILD
tools/scripts/fuzz_minimize_crash.sh
website/static/docs
website/static/img/gusto-logo.jpg
test/whitequark/test_bug_do_block_in_hash_brace_2.rb
website/docs/type-aliases.md
test/whitequark/test_kwarg_combinations_1.rb
main/lsp/test/generate_lsp_messages_test.cc
main/lsp/json_enums.h
test/whitequark/test_send_binary_op_0.parse-tree-whitequark.exp
test/testdata/resolver/defined.rb
test/whitequark/test_not_masgn_24_0.parse-tree-whitequark.exp
common/JSON.cc
rbi/core/math.rbi
main/lsp/requests/hover.h
main/lsp/requests/requests.h
cfg/builder/builder_walk.cc
test/cli/no-error-count/no-error-count.out
test/testdata/lsp/fast_path/class_change_superclass_multifile__child.rb
test/testdata/rewriter/prop_missing.rb.rewrite-tree.exp
website/static/docs/index.html
common/ConstExprStr.h
test/cli/kwarg-loc/kwarg-loc.sh
rewriter/DSLBuilder.h
test/whitequark/test_ternary_0.parse-tree-whitequark.exp
test/cli/suggest_t_must/suggest_t_must.rb
core/serialize/test/serialize_test.cc
test/whitequark/test_masgn_const_0.parse-tree-whitequark.exp
test/hello-test.cc
test/lsp/workspaceSymbol/workspaceSymbol.rec
test/testdata/infer/generics/tuple_decay.rb
gems/sorbet-runtime/lib/types/struct.rb
core/GlobalState.cc
test/testdata/lsp/go_to_type_definition_false.rb
common/Random.cc
test/whitequark/test_op_asgn_cmd_3.parse-tree-whitequark.exp
test/cli/remove-path-prefix-https/remove-path-prefix-https.out
test/cli/print_to_file/d.rb
test/cli/dedup_loc
test/whitequark/test_when_then_0.rb
rbi/stdlib/ripper.rbi
test/testdata/rewriter/fuzz_prop_weird_member.rb
test/testdata/rbi/int_float.rb
namer/configatron
test/testdata/desugar/op_eq.rb.flatten-tree.exp
test/whitequark/test_masgn_splat_5.rb
gems/sorbet/test/snapshot/partial/typed-ignore/src/src.rb
test/testdata/resolver/cbase.rb.symbol-table-raw.exp
test/cli/incremental-resolver/expect-failures/constant_redefinition.rb
website/static/js
test/testdata/lsp/fast_path/parse_errors.2.rbupdate
main/lsp/requests/code_action.cc
common/concurrency/ConcurrentQueue.h
test/whitequark/test_defined_1.rb
test/cli/ignore/ignore.sh
test/whitequark/test_parser_bug_507_0.rb
gems/sorbet/test/snapshot/partial/codecov
test/testdata/infer/generics/use_member_in_body.rb
test/whitequark/test_alias_gvar_0.parse-tree-whitequark.exp
test/testdata/infer/body_not_always_run.rb
test/testdata/autogen/multiple_alias.rb
website/docs/faq.md
gems/sorbet/test/hidden-method-finder/thorough/src
main/lsp/LSPPreprocessor.h
test/whitequark/test_arg_duplicate_4.rb
test/whitequark/test_arg_duplicate_ignored_0.rb
test/testdata/namer/locals.rb.symbol-table-raw.exp
test/cli/override-typed-bad/bad-top-level.yaml
test/testdata/cfg/extra_bb_args.rb
test/whitequark/test_not_1.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/explosive-object/src/Gemfile.lock
gems/sorbet-runtime/bench/deserialize.rb
test/whitequark/test_hash_label_0.parse-tree-whitequark.exp
common/concurrency/WorkerPoolImpl.h
main/lsp/notifications/cancel_request.h
rbi/stdlib/open_struct.rbi
test/whitequark/test_resbody_list_mrhs_0.rb
third_party/parser/cc/driver.cc
test/cli/autocorrect-strict/autocorrect-strict.sh
test/cli/folder-input-dir-and-file/folder-input-dir-and-file.sh
test/whitequark/test_ruby_bug_11873_4.rb
gems/sorbet-runtime/test/types/final_method.rb
test/cli/folder-input/input
test/whitequark/test_block_arg_combinations_11.parse-tree-whitequark.exp
test/whitequark/test_cvasgn_0.rb
test/testdata/infer/case_exhaustive_union_type.rb
test/testdata/lsp/completion/grandchild.rb
gems/sorbet/test/snapshot/partial/stack_master/src
test/testdata/rewriter/attr.rb.flatten-tree.exp
ast/desugar/test/desugar_test.cc
test/cli/override-typed-bad
main/lsp/LocalVarFinder.cc
test/testdata/autogen/no_dsls.rb
test/testdata/namer/simple.rb.symbol-table-raw.exp
gems/sorbet-runtime/lib/types/private/casts.rb
gems/sorbet-runtime/test/types/configuration.rb
common/kvstore/KeyValueStore.h
test/testdata/lsp/symbol_query_filter__helper.rb
namer/test
test/cli/parse-tree-whitequark/parse-tree-whitequark.out
gems/sorbet-runtime/lib/types/props/errors.rb
test/whitequark/test_block_arg_combinations_25.parse-tree-whitequark.exp
gems/sorbet/test/snapshot/partial/non-utf-8-file/src/src.rb
main/lsp/requests/get_counters.h
test/cli/symbol-table-json-alias/symbol-table-json-alias.rb
rewriter/Struct.cc
test/whitequark/test_send_block_chain_cmd_1.parse-tree-whitequark.exp
test/testdata/infer/control_flow/normalize_params.rb
test/whitequark/test_ruby_bug_12402_10.rb
test/testdata/resolver/abstract_validation.rb
test/whitequark/test_ruby_bug_11873_10.rb
test/testdata/parser/error_recovery_constant_only_scope.rb
test/cli/license
test/testdata/namer/payload_name.rb
test/whitequark/test_send_index_asgn_legacy_0.rb
test/testdata/parser/ruby_25.rb.parse-tree.exp
test/whitequark/test_kwrestarg_named_0.parse-tree-whitequark.exp
test/cli/suggest-sig-override-edge
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/src/Gemfile.lock
test/whitequark/test_next_3.rb
third_party/parser/codegen
tools/scripts/generate_compdb_targets.sh
test/testdata/lsp/field_edits.rb
test/testdata/desugar/defs_not_self.rb.desugar-tree.exp
common/statsd/statsd.cc
test/testdata/infer/meta_types.rb.cfg.exp
test/cli/subprocess-plugin/multi5.rb
test/testdata/substitutions
test/whitequark/test_def_5.parse-tree-whitequark.exp
website/docs/tstruct.md
test/testdata/desugar/fuzz_break_do_end.rb
main/lsp/requests/sorbet_read_file.h
test/whitequark/test_complex_2.rb
test/testdata/namer/alias_cross_file.flatten-tree.exp
rbi/core/fiber.rbi
test/testdata/desugar/integers.rb.desugar-tree.exp
gems/sorbet-runtime
test/whitequark/test_method_definition_in_while_cond_2.rb
test/testdata/cfg/rescue_with_return.rb.cfg.exp
test/testdata/cfg/blocks.rb
test/testdata/resolver/type_member_out_of_order.rb
test/whitequark/test_if_nl_then_0.rb
parser/Parser.cc
resolver/type_syntax.h
test/whitequark/test_var_op_asgn_0.rb
test/testdata/resolver/stub_missing_class_alias.rb.symbol-table-raw.exp
website/docs/nilable-types.md
test/cli/config-file/config-file.rb
test/whitequark/test_if_0.rb
test/testdata/lsp/workspace_symbols_boundaries.rb
test/whitequark/test_asgn_mrhs_2.parse-tree-whitequark.exp
test/whitequark/test_ambiuous_quoted_label_in_ternary_operator_1.rb
gems/sorbet/test/snapshot/partial/db_schema/src
test/cli/suggest-not-stub/suggest-not-stub.rb
gems/sorbet-runtime/lib/types/configuration.rb
website/static/img/coinbase-logo.png
test/testdata/resolver/type_alias.rb
gems/sorbet-runtime/test/types/abstract_validation.rb
test/cli/symbol-table-json-alias
test/whitequark/test_super_block_0.rb
gems/sorbet/test/snapshot/partial/ignore_file_table/src/Gemfile.lock
test/whitequark/test_ruby_bug_11873_a_5.rb
test/cli/parser-error
test/whitequark/test_resbody_list_mrhs_0.parse-tree-whitequark.exp
test/cli/counters/counters.out
main/cache/BUILD
test/whitequark/test_array_splat_1.parse-tree-whitequark.exp
test/cli/autocorrect-private
main/lsp/requests/document_symbol.cc
test/testdata/infer/fuzz_disallow_overloading.rb
test/testdata/autogen/hidden_include.rb.autogen.exp
test/whitequark/test_ruby_bug_13547_8.rb
test/whitequark/test_ruby_bug_11873_3.parse-tree-whitequark.exp
test/cli/suppress-non-critical/suppress-non-critical.sh
rbi/sorbet
test/whitequark/test_parser_bug_604_0.rb
test/whitequark/test_cond_iflipflop_0.parse-tree-whitequark.exp
test/testdata/cfg/break_in_while.rb
test/testdata/resolver/sig_generated.rb
test/testdata/namer/root_private.rb.symbol-table-raw.exp
test/cli/phases/phases.out
test/testdata/cfg/fuzz_dedup_exit_edges.rb
test/testdata/resolver/resolve_through_type_alias.rb
gems/sorbet/test/snapshot/partial/fake-object/src/Gemfile.lock
third_party/progressbar/progressbar
ast/Helpers.cc
test/whitequark/test_preexe_0.rb
test/whitequark/test_method_definition_in_while_cond_1.rb
test/cli/correct-bare-stdlib-generics/correct-bare-stdlib-generics.sh
test/whitequark/test_space_args_arg_newline_0.parse-tree-whitequark.exp
test/whitequark/test_float_1.parse-tree-whitequark.exp
test/whitequark/test_bug_heredoc_do_0.rb
test/whitequark/test_send_unary_op_0.parse-tree-whitequark.exp
test/testdata/desugar/and_self.rb
test/testdata/lsp/bad_field_edits.1.rbupdate.document-symbols.exp
test/testdata/resolver/linearization/includes_class.rb
gems/sorbet/test/hidden-method-finder/thorough/src/sorbet
test/whitequark/test_block_arg_combinations_5.rb
test/cli/subprocess-plugin/gen.rb
test/testdata/namer/locals.rb.flatten-tree.exp
test/whitequark/test_send_self_block_1.rb
test/cli/suggest-new/suggest-new.out
third_party/parser/include/ruby_parser/state_stack.hh
test/whitequark/test_or_1.rb
website/docs/class-of.md
test/whitequark/test_ruby_bug_12073_1.rb
test/whitequark/test_hash_label_end_2.rb
test/whitequark/test_ruby_bug_12402_6.parse-tree-whitequark.exp
website/docs/talks
test/cli/cache-keeps-print-options
test/testdata/todo/block_in_class.rb.symbol-table-raw.exp
test/testdata/resolver/rbi_final_no_sig__1.rb
tools/toolchain/webasm-linux/em_cache_existing.tar.gz
test/backtrace-test.sh
test/whitequark/test_postexe_0.parse-tree-whitequark.exp
test/whitequark
test/testdata/cfg/side_effects.rb.cfg.exp
test/testdata/infer/boolean.rb
test/testdata/rewriter/fuzz_chalk_bad_type.rb
test/whitequark/test_ruby_bug_10279_0.rb
test/whitequark/test_args_args_assocs_comma_0.parse-tree-whitequark.exp
test/testdata/lsp/go_to_type_definition_applied.rb
test/lsp/alias-incremental/alias-incremental.rec
website/static/docs/bad-rbi.html
gems/sorbet/test/snapshot/partial/db_schema/expected/sorbet/foo.txt
gems/sorbet/test/snapshot/partial/create-config/expected/sorbet/config
test/testdata/infer/control_flow/complex_implications_2.rb.cfg.exp
test/testdata/lsp/code_actions/typo.C.rbedited
test/whitequark/test_block_arg_combinations_0.parse-tree-whitequark.exp
gems/sorbet/bin
test/whitequark/test_array_words_0.rb
resolver/CorrectTypeAlias.cc
rbi/stdlib/fileutils.rbi
test/testdata/resolver/resolve_via_ancestors/mixin.rb
gems/sorbet/test/snapshot/partial/create-config/expected
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed
gems/sorbet/test/snapshot/partial/missing-instance-methods/src/src.rb
core/serialize/BUILD
tools/toolchain
test/testdata/rewriter/t_struct/override.rb.rewrite-tree.exp
test/whitequark/test_masgn_splat_1.rb
test/whitequark/test_ivasgn_0.parse-tree-whitequark.exp
test/testdata/lsp/hover_method_not_found.rb
test/whitequark/test_block_arg_combinations_18.rb
test/whitequark/test_defs_invalid_2.rb
test/cli/config-file/config-file.sh
rbi/sorbet/ttypes.rbi
test/cli/forbid-autocorrect-with-quiet/forbid-autocorrect-with-quiet.out
common/FileSystem.h
main/autogen/subclasses.cc
test/whitequark/test_args_assocs_0.rb
third_party/progressbar/progressbar/progressbar.h
test/cli/suggest-kernel/suggest-kernel.sh
test/whitequark/test_send_binary_op_5.parse-tree-whitequark.exp
test/testdata/infer/t_all_type_member_bug.rb
test/testdata/lsp/completion/constants_type_members.rb
test/cli/no-error-count
test/testdata/namer/yield.rb
test/testdata/desugar/regexp_strict.rb
test/whitequark/test_for_mlhs_0.rb
gems/sorbet/test/hidden-method-finder/hidden_methods_bazel.sh
test/testdata/infer/intrinsics_generics.rb
test/testdata/resolver/class_instance_vars.rb.flatten-tree.exp
test/whitequark/test_send_lambda_args_shadow_0.rb
test/testdata/resolver/generic_class_alias.rb
test/whitequark/test_resbody_var_1.rb
rbi/core/warning.rbi
test/whitequark/test_masgn_splat_7.parse-tree-whitequark.exp
test/whitequark/test_defs_0.rb
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/ruby/all
test/testdata/namer/fuzz_type_template_overwrite.rb.symbol-table-raw.exp
test/whitequark/test_procarg0_legacy_0.parse-tree-whitequark.exp
test/whitequark/test_args_assocs_1.rb
test/cli/constant-fuzzy/constant-fuzzy.rb
resolver/GlobalPass.cc
test/cli/suggest-sig-override-edge/suggest-sig-override-edge.rb
test/cli/override-typed-bad/override-typed-bad.sh
test/cli/allowed-extension/lib
test/testdata/resolver/no_runtime_sig.rb
test/testdata/lsp/good_field_edits.1.rbupdate.document-symbols.exp
test/testdata/infer/tuples.rb
gems/sorbet/test/snapshot/total/empty/expected/sorbet/rbi/sorbet-typed/lib/bundler/all
test/whitequark/test_super_block_0.parse-tree-whitequark.exp
test/testdata/namer/docs_example_2.rb
test/whitequark/test_range_endless_0.rb
test/testdata/rewriter/rails/cattr_reader.rb.rewrite-tree.exp
test/whitequark/test_non_lvar_injecting_match_0.rb
test/cli/missing-constants/missing-constants.out
test/testdata/resolver/sealed_with_rbi__2.rb
gems/sorbet/test/snapshot/partial/local_gem
test/testdata/rewriter/protobuf_descriptor_pool.rb.rewrite-tree.exp
third_party/progressbar/src/statusbar.c
third_party/openssl/darwin.BUILD
test/whitequark/test_return_3.rb
test/whitequark/test_kwarg_invalid_1.rb
parser/BUILD
website/static/img/czi-logo.svg
test/testdata/cfg/examples.rb.cfg.exp
test/testdata/rewriter/prop_proc_regression.rb
common/test
test/cli/subprocess-plugin
test/whitequark/test_marg_combinations_2.parse-tree-whitequark.exp
common/Random.h
main/lsp/test/lsp_file_updates_test.cc
main/lsp/requests/signature_help.h
test/testdata/infer/rebind.rb
test/testdata/infer/assignment_call.rb
main/lsp/requests/workspace_symbols.h
test/testdata/cfg/rescue_var_expression.rb
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_module_include__2.rb
website/static/img/kickstarter-logo.png
test/whitequark/test_if_elsif_0.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_10653_0.rb
test/cli/parser-error/parser-error.out
test/cli/subprocess-plugin/echo_argv.rb
tools/toolchain/webasm-darwin/cc_toolchain_config.bzl
rbi/stdlib/singleton.rbi
test/testdata/infer/control_flow/dead_condition.rb
test/whitequark/test_block_arg_combinations_16.parse-tree-whitequark.exp
core/StrictLevel.h
rbi/stdlib/forwardable.rbi
test/whitequark/test_send_attr_asgn_1.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem
test/testdata/namer/conflicting_names.rb.symbol-table-raw.exp
test/testdata/lsp/completion/sig_no_defs.rb
test/testdata/rewriter/t_enum_exhaustiveness.rb
test/cli/module-redefinition/module-redefinition-3.rb
main/lsp/requests/document_highlight.cc
test/testdata/infer/aliases__1.rb
plugin/SubprocessTextPlugin.cc
test/whitequark/test_ruby_bug_12686_0.rb
test/testdata/resolver/reveal_type.rb
website/static/css/PageSection.css
test/cli/autocorrect-remove-body/autocorrect-remove-body.sh
test/whitequark/test_defs_invalid_7.rb
test/cli/autogen-subclasses-ignore/autogen-subclasses-ignore.out
test/testdata/todo/class_of.rb
test/testdata/desugar/ensure_without_rescue.rb.cfg.exp
test/whitequark/test_blockarg_0.parse-tree-whitequark.exp
tools/toolchain/webasm-darwin/emcc.sh
test/cli/make_accessible/make_accessible.out
test/testdata/desugar
core/serialize/serialize.h
test/cli/constant-fuzzy/constant-fuzzy.sh
test/whitequark/test_kwarg_0.parse-tree-whitequark.exp
rewriter/TEnum.h
test/cli/errors/errors.out
test/whitequark/test_false_0.parse-tree-whitequark.exp
test/testdata/lsp/completion/alias_method.rb
test/testdata/namer/defs_in_blocks.rb
test/cli/suggest-sig-garbage
core/errors/errors.h
test/whitequark/test_parser_bug_525_0.parse-tree-whitequark.exp
test/testdata/lsp/completion/snippet_arity.A.rbedited
test/whitequark/test_send_plain_2.rb
test/whitequark/test_if_elsif_0.rb
test/cli/metrics-file/test.rb
test/testdata/resolver/sealed_stdlib.rb
gems/sorbet/test/snapshot/partial/local_gem/expected/sorbet/rbi/gems/my_gem.rbi
test/whitequark/test_var_op_asgn_1.rb
rbi/stdlib/pathname.rbi
test/testdata/cfg/next_in_junk.rb.cfg.exp
test/testdata/cfg/raw_test.rb.cfg-raw.exp
test/whitequark/test_send_lambda_0.rb
test/testdata/infer/flatten_methods.rb.flatten-tree.exp
test/whitequark/test_method_definition_in_while_cond_2.parse-tree-whitequark.exp
test/whitequark/test_bug_do_block_in_hash_brace_2.parse-tree-whitequark.exp
test/testdata/lsp/fast_path/method_change_argument_type__def.1.rbupdate
test/testdata/lsp/completion/simple.rb
test/cli/error-url/error-url.sh
gems/sorbet-runtime/test/pay-server-shims.rbi
test/testdata/resolver/bare_stdlib_generics.rb
test/whitequark/test_ruby_bug_11107_0.parse-tree-whitequark.exp
test/whitequark/test_block_arg_combinations_3.parse-tree-whitequark.exp
test/cli/folder-input-dir-and-file/input/file_with_no_dot
test/whitequark/test_marg_combinations_8.parse-tree-whitequark.exp
test/cli/metrics-file/sorbet
website/docs/talks/ruby-kaigi-2019.md
test/whitequark/test_bug_466_0.parse-tree-whitequark.exp
main/lsp/LSPMessage.h
test/testdata/resolver/optional_nil.rb.flatten-tree.exp
third_party/cpp_subprocess.BUILD
payload/binary/tools
definition_validator/variance.cc
test/cli/suggest-typed-true/empty.rb
test/testdata/rewriter/fuzz_attr_bare.rb
core/Unfreeze.h
test/testdata/lsp/fast_path/method_change_argument_kind.rb
test/whitequark/test_complex_0.parse-tree-whitequark.exp
test/whitequark/test_block_kwarg_combinations_1.rb
test/whitequark/test_ruby_bug_13547_9.rb
test/whitequark/test_send_binary_op_11.rb
test/cli/symbol-table-json/symbol-table-json.rb
test/testdata/desugar/assign_empty_stmts.rb
test/cli/file-table-json
test/testdata/infer/casts.rb.flatten-tree.exp
test/testdata/rewriter/prop.rb.rewrite-tree.exp
test/whitequark/test_super_block_1.parse-tree-whitequark.exp
test/testdata/desugar/top_level_const.rb
test/lsp/incremental-lsp-changes
rbi/stdlib/abbrev.rbi
test/whitequark/test_cvar_0.parse-tree-whitequark.exp
test/testdata/resolver/sealed_with_rbi__1.rb
test/whitequark/test_block_arg_combinations_12.parse-tree-whitequark.exp
test/cli/silence-dev-message/silence-dev-message.sh
test/whitequark/test_send_block_blockarg_0.rb
test/helpers/MockFileSystem.cc
test/cli/empty-file
test/cli/parallel-ordering/2.rb
test/whitequark/test_ruby_bug_12402_3.parse-tree-whitequark.exp
test/testdata/infer/generics/lub_with_raw.rb
test/cli/autocorrect-remove-body/post.rb
test/testdata/infer/control_flow/simple.rb.cfg.exp
test/testdata/rewriter/rails/cattr_writer.rb.rewrite-tree.exp
gems/sorbet-runtime/lib/types/enum.rb
test/testdata/desugar/hash_two_args.rb
test/whitequark/test_begin_cmdarg_0.rb
test/testdata/parser/ruby_25.rb
test/testdata/infer/align_child_as_parent.rb
test/testdata/rewriter/attr_parameters.rb
gems/sorbet-runtime/test/types/fixtures/sealed_module/forbid_sealed_class__2.rb
test/whitequark/test_bug_do_block_in_hash_brace_1.parse-tree-whitequark.exp
test/whitequark/test_masgn_2.rb
test/testdata/autogen/defines_behavior.rb
test/testdata/rewriter/t_struct/some_default.rb.rewrite-tree.exp
gems/sorbet/lib/real_stdlib.rb
rbi/core/errno.rbi
test/whitequark/test_arg_combinations_2.parse-tree-whitequark.exp
website/docs
test/testdata/namer/fuzz_shared_singletons.rb
test/testdata/lsp/fast_path/method_change_argument_type__usage.rb
test/testdata/infer/exhaustiveness.rb
test/testdata/infer/generics/variance_neutral.rb
test/testdata/rewriter/default_args.rb
test/testdata/autogen/bare_casgn.rb.autogen.exp
test/whitequark/test_array_assocs_1.rb
test/testdata/infer/infer_types.rb
common/kvstore/test
test/testdata/cfg/rescue.rb.desugar-tree-raw.exp
test/whitequark/test_var_op_asgn_3.rb
test/testdata/infer/lub_tuples.rb
core/NameHash.cc
test/whitequark/test_nil_expression_0.parse-tree-whitequark.exp
gems/sorbet-runtime/lib/types/props/generated_code_validation.rb
rbi/core/basic_object.rbi
test/testdata/rewriter/prop_missing.rb
test/whitequark/test_ruby_bug_12402_5.parse-tree-whitequark.exp
test/testdata/resolver/fuzz_top_level_abstract.rb
core/NameHash.h
main/realmain.h
test/whitequark/test_ruby_bug_12402_1.parse-tree-whitequark.exp
test/cli/suggest-not-stub/suggest-not-stub.sh
test/whitequark/test_rescue_else_useless_0.rb
class_flatten/class_flatten.cc
test/whitequark/test_defs_invalid_0.rb
local_vars/local_vars.cc
test/testdata/resolver/linearization/linearization4a.rb
test/testdata/infer/arg_matching.rb
test/testdata/rewriter/command.rb
test/cli/folder-input/foo.rb
gems/sorbet-runtime/bench
rbi/gems/msgpack.rbi
test/testdata/resolver/alias.rb
test/whitequark/test_string_FILE_0.parse-tree-whitequark.exp
test/cli/suggest-sig-literal
test/testdata/rewriter/chalk_odm_document.rb.rewrite-tree.exp
test/testdata/desugar/destructure_rest.rb
main/BUILD
infer/test/infer_test.cc
test/whitequark/test_method_definition_in_while_cond_3.parse-tree-whitequark.exp
namer/configatron/configatron.cc
test/testdata/resolver/resolve_via_ancestors/two_mixins.rb
test/whitequark/test_cpath_1.parse-tree-whitequark.exp
main/pipeline/ProgressIndicator.cc
test/testdata/rewriter/rails/cattr_writer.rb
test/whitequark/test_if_else_0.rb
test/testdata/rewriter/t_enum_snapshot.rb
payload
test/whitequark/test_bug_cmdarg_2.parse-tree-whitequark.exp
main/lsp/requests/type_definition.h
gems/sorbet-runtime/lib/types/props/private/serializer_generator.rb
test/cli/error-blacklist/error-blacklist.sh
test/cli/non-existing-option
test/whitequark/test_marg_combinations_2.rb
website/docs/tuples.md
main/lsp/notifications/exit.h
rbi/core/string.rbi
test/whitequark/test_lbrace_arg_after_command_args_0.parse-tree-whitequark.exp
test/whitequark/test_arg_combinations_1.parse-tree-whitequark.exp
test/testdata/lsp/completion/snippet_block_arity.B.rbedited
test/whitequark/test_const_op_asgn_4.parse-tree-whitequark.exp
test/whitequark/test_ruby_bug_11873_1.parse-tree-whitequark.exp
tools/scripts/format_build_files.sh
test/whitequark/test_ruby_bug_13547_10.rb
website/static/blog/2019/05
test/cli/license/license.out
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/src/Gemfile
test/whitequark/test_ruby_bug_12402_6.rb
test/cli/config-file-recursive/sorbet
third_party/licenses/blake2.txt
gems/sorbet-runtime/test/types/props/optional.rb
test/testdata/parser/heredoc_chomp.rb.desugar-tree.exp
test/testdata/cfg/raw_test.rb
test/testdata/lsp/definitions_and_usages_untyped__typed.rb
test/testdata/desugar/constant_error.rb.symbol-table-raw.exp
test/testdata/lsp/completion/union.rb
test/whitequark/test_super_1.parse-tree-whitequark.exp
test/whitequark/test_send_unary_op_2.parse-tree-whitequark.exp
test/testdata/infer/void_final.rb
test/cli/suggest-typed-true/suggest-typed-true.sh
test/cli/config-file/config-file.out
test/cli/file-table-json/file-table-json.rb
rewriter/AttrReader.cc
test/cli/suggest-typed-true/suggest-typed-with-too-low.rb
test/cli/kwarg-loc/kwarg-loc.rb
test/cli/rbi-overrides/t2.rb
website/docs/tenum.md
test/whitequark/test_kwarg_combinations_3.parse-tree-whitequark.exp
test/whitequark/test_bug_do_block_in_cmdarg_0.rb
test/cli/print_to_file/c.rb
test/whitequark/test_cpath_0.rb
main/lsp/notifications/initialized.cc
gems/sorbet-runtime/lib/types/props/has_lazily_specialized_methods.rb
test/whitequark/test_arg_invalid_1.rb
test/whitequark/test_block_arg_combinations_1.rb
test/testdata/rewriter/t_struct
test/cli/configatron-yaml-error
test/cli/folder-input-not-dir/folder-input-not-dir.sh
rbi/core/file_test.rbi
test/testdata/rewriter/minitest_tables.rb
gems/sorbet/lib/gem_loader.rbi
test/testdata/lsp/completion/snippet_keywords.D.rbedited
test/cli/make_accessible
test/testdata/resolver/fuzz_empty_type_alias.rb
test/whitequark/test_send_plain_0.rb
test/whitequark/test_return_1.rb
test/whitequark/test_cond_match_current_line_0.rb
test/testdata/namer/next_break.rb
rbi/core/complex.rbi
gems/sorbet-runtime/lib/types/types/attached_class.rb
test/testdata/cfg/examples.rb.flatten-tree.exp
test/whitequark/test_ruby_bug_11873_4.parse-tree-whitequark.exp
test/whitequark/test_array_words_empty_1.rb
test/testdata/rewriter/generic_module_function.rb
test/whitequark/test_ruby_bug_11873_a_19.parse-tree-whitequark.exp
test/testdata/namer/visibility.rb
rewriter/rewriter.cc
test/testdata/lsp/fast_path/method_change_kw_arg_type.rb
test/testdata/rewriter/t_enum_override_initialize.rb
gems/sorbet/test/snapshot/partial/rails-double-require/src/Gemfile.lock
docs/img/chrome-tracing-load-button.png
test/testdata/infer/constructors.rb
test/cli/lsp-requires-input-dir
test/cli/suggest_t_must
test/testdata/resolver/top_level_include.rb
test/testdata/namer/fuzz_repeated_kwarg.rb
gems/sorbet/test/snapshot/partial/local_rvm_gemset_gem/gems/ruby-0.0.0@gemset/gems
test/cli/override-typed-bad/bad-list.yaml
test/testdata/desugar/strings.rb
test/cli/override-typed/override-typed.out
test/testdata/lsp/completion/snippet_types.rb
infer/inference.cc
test/whitequark/test_block_arg_combinations_23.parse-tree-whitequark.exp
test/testdata/desugar/backtick.rb.desugar-tree.exp
test/whitequark/test_masgn_2.parse-tree-whitequark.exp
test/testdata/resolver/linearization/includes_class.rb.symbol-table-raw.exp
local_vars/local_vars.h
test/whitequark/test_send_self_block_3.rb
test/testdata/resolver/fuzz_type_member_scope.rb
test/cli/folder-input
core/Error.h
website/static/blog/2019/05/16/State-of-Sorbet-May-2019.html
test/testdata/rewriter/stringy_struct.rb
";
    #[cfg(not(target_os = "windows"))]
    let expected = "\
.
├── ACKNOWLEDGEMENTS.md
├── BUILD
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE
├── NOTICE
├── README.md
├── Rakefile
├── WORKSPACE
├── ast
│   ├── ArgParsing.cc
│   ├── ArgParsing.h
│   ├── BUILD
│   ├── Helpers.cc
│   ├── Helpers.h
│   ├── TreeCopying.cc
│   ├── TreeSanityChecks.cc
│   ├── Trees.cc
│   ├── Trees.h
│   ├── ast.h
│   ├── desugar
│   │   ├── BUILD
│   │   ├── Desugar.cc
│   │   ├── Desugar.h
│   │   └── test
│   │       └── desugar_test.cc
│   ├── substitute
│   │   ├── BUILD
│   │   ├── Substitute.cc
│   │   └── substitute.h
│   ├── treemap
│   │   ├── BUILD
│   │   └── treemap.h
│   └── verifier
│       ├── BUILD
│       ├── Verifier.cc
│       └── verifier.h
├── bazel
├── cfg
│   ├── BUILD
│   ├── CFG.cc
│   ├── CFG.h
│   ├── Instructions.cc
│   ├── Instructions.h
│   └── builder
│       ├── BUILD
│       ├── builder.h
│       ├── builder_entry.cc
│       ├── builder_finalize.cc
│       └── builder_walk.cc
├── class_flatten
│   ├── BUILD
│   ├── class_flatten.cc
│   └── class_flatten.h
├── common
│   ├── BUILD
│   ├── ConstExprStr.h
│   ├── Counters.cc
│   ├── Counters.h
│   ├── Counters_impl.h
│   ├── Exception.h
│   ├── FileOps.h
│   ├── FileSystem.cc
│   ├── FileSystem.h
│   ├── JSON.cc
│   ├── JSON.h
│   ├── Levenstein.cc
│   ├── Levenstein.h
│   ├── Random.cc
│   ├── Random.h
│   ├── Subprocess.cc
│   ├── Subprocess.h
│   ├── Timer.cc
│   ├── Timer.h
│   ├── backtrace.cc
│   ├── common.cc
│   ├── common.h
│   ├── concurrency
│   │   ├── BUILD
│   │   ├── ConcurrentQueue.h
│   │   ├── WorkerPool.h
│   │   ├── WorkerPoolImpl.cc
│   │   └── WorkerPoolImpl.h
│   ├── crypto_hashing
│   │   ├── BUILD
│   │   └── crypto_hashing.h
│   ├── formatting.h
│   ├── kvstore
│   │   ├── BUILD
│   │   ├── KeyValueStore-emscripten.cc
│   │   ├── KeyValueStore.cc
│   │   ├── KeyValueStore.h
│   │   └── test
│   │       └── kvstore_test.cc
│   ├── os
│   │   ├── emscripten.cc
│   │   ├── linux.cc
│   │   ├── mac.cc
│   │   ├── os.cc
│   │   └── os.h
│   ├── sort.h
│   ├── statsd
│   │   ├── BUILD
│   │   ├── statsd-emscripten.cc
│   │   ├── statsd.cc
│   │   └── statsd.h
│   ├── test
│   │   └── common_test.cc
│   ├── typecase.h
│   └── web_tracer_framework
│       ├── BUILD
│       ├── tracing.cc
│       └── tracing.h
├── core
│   ├── AutocorrectSuggestion.cc
│   ├── AutocorrectSuggestion.h
│   ├── BUILD
│   ├── Context.cc
│   ├── Context.h
│   ├── DebugOnlyCheck.h
│   ├── Error.cc
│   ├── Error.h
│   ├── ErrorFlusher.cc
│   ├── ErrorFlusher.h
│   ├── ErrorQueue.cc
│   ├── ErrorQueue.h
│   ├── ErrorQueueMessage.h
│   ├── Files.cc
│   ├── Files.h
│   ├── GlobalState.cc
│   ├── GlobalState.h
│   ├── GlobalSubstitution.h
│   ├── Hashing.h
│   ├── Loc.cc
│   ├── Loc.h
│   ├── LocalVariable.cc
│   ├── LocalVariable.h
│   ├── NameHash.cc
│   ├── NameHash.h
│   ├── NameRef.h
│   ├── Names.cc
│   ├── Names.h
│   ├── StrictLevel.h
│   ├── SymbolRef.h
│   ├── Symbols.cc
│   ├── Symbols.h
│   ├── TypeConstraint.cc
│   ├── TypeConstraint.h
│   ├── TypePtr.h
│   ├── Types.h
│   ├── TypesAndOrigins.cc
│   ├── Unfreeze.cc
│   ├── Unfreeze.h
│   ├── core.h
│   ├── errors
│   │   ├── cfg.h
│   │   ├── desugar.h
│   │   ├── errors.h
│   │   ├── infer.h
│   │   ├── internal.h
│   │   ├── namer.h
│   │   ├── parser.h
│   │   ├── plugin.h
│   │   ├── resolver.h
│   │   └── rewriter.h
│   ├── lsp
│   │   ├── PreemptionTaskManager.cc
│   │   ├── PreemptionTaskManager.h
│   │   ├── Query.cc
│   │   ├── Query.h
│   │   ├── QueryResponse.cc
│   │   ├── QueryResponse.h
│   │   ├── Task.h
│   │   ├── TypecheckEpochManager.cc
│   │   └── TypecheckEpochManager.h
│   ├── proto
│   │   ├── BUILD
│   │   ├── proto.cc
│   │   └── proto.h
│   ├── serialize
│   │   ├── BUILD
│   │   ├── pickler.h
│   │   ├── serialize.cc
│   │   ├── serialize.h
│   │   └── test
│   │       └── serialize_test.cc
│   ├── test
│   │   └── core_test.cc
│   ├── tools
│   │   └── generate_names.cc
│   └── types
│       ├── calls.cc
│       ├── printing.cc
│       ├── subtyping.cc
│       ├── typemaps.cc
│       └── types.cc
├── definition_validator
│   ├── BUILD
│   ├── validator.cc
│   ├── validator.h
│   ├── variance.cc
│   └── variance.h
├── docs
│   ├── JRuby.md
│   ├── README.md
│   ├── compressors.md
│   ├── img
│   │   ├── cfg-example.svg
│   │   ├── chrome-tracing-load-button.png
│   │   ├── chrome-tracing-loaded.png
│   │   ├── chrome-tracing-pipeline.png
│   │   ├── chrome-tracing-scrolled.png
│   │   ├── chrome-tracing-typecheck-one.png
│   │   └── sorbet-pipeline.monopic
│   ├── internals.md
│   ├── logo
│   │   ├── README.md
│   │   ├── sorbet-logo-45-deg@1x.png
│   │   ├── sorbet-logo-45-deg@2x.png
│   │   ├── sorbet-logo-monochrome.svg
│   │   ├── sorbet-logo-purple-sparkles.svg
│   │   ├── sorbet-logo-white-sparkles-on-purple.svg
│   │   ├── sorbet-logo-white-sparkles.svg
│   │   ├── sorbet-logo.sketch
│   │   ├── sorbet-logo.svg
│   │   └── sorbet-logo@2x.png
│   ├── pipeline.md
│   ├── suggest-sig.md
│   └── tracing.md
├── emscripten
│   ├── BUILD
│   └── main.cc
├── gems
│   ├── sorbet
│   │   ├── BUILD
│   │   ├── Gemfile
│   │   ├── README.md
│   │   ├── Rakefile
│   │   ├── bin
│   │   │   ├── srb
│   │   │   └── srb-rbi
│   │   ├── lib
│   │   │   ├── constant_cache.rb
│   │   │   ├── create-config.rb
│   │   │   ├── fetch-rbis.rb
│   │   │   ├── find-gem-rbis.rb
│   │   │   ├── gem-generator-tracepoint
│   │   │   │   ├── tracepoint_serializer.rb
│   │   │   │   └── tracer.rb
│   │   │   ├── gem-generator-tracepoint.rb
│   │   │   ├── gem_loader.rb
│   │   │   ├── gem_loader.rbi
│   │   │   ├── hidden-definition-finder.rb
│   │   │   ├── real_stdlib.rb
│   │   │   ├── require_everything.rb
│   │   │   ├── serialize.rb
│   │   │   ├── status.rb
│   │   │   ├── step_interface.rb
│   │   │   ├── suggest-typed.rb
│   │   │   ├── t.rb
│   │   │   └── todo-rbi.rb
│   │   ├── sorbet.gemspec
│   │   └── test
│   │       ├── hidden-method-finder
│   │       │   ├── BUILD
│   │       │   ├── check_one_bazel.sh
│   │       │   ├── hidden-method-tests.rb
│   │       │   ├── hidden_methods.bzl
│   │       │   ├── hidden_methods_bazel.sh
│   │       │   ├── logging.sh
│   │       │   ├── shims.rb.source
│   │       │   ├── simple
│   │       │   │   ├── expectations.json
│   │       │   │   ├── ruby_2_4_hidden.rbi.exp
│   │       │   │   ├── ruby_2_6_hidden.rbi.exp
│   │       │   │   └── src
│   │       │   │       ├── Gemfile
│   │       │   │       ├── simple.rb
│   │       │   │       └── sorbet
│   │       │   │           └── config
│   │       │   ├── thorough
│   │       │   │   ├── expectations.json
│   │       │   │   ├── ruby_2_4_hidden.rbi.exp
│   │       │   │   ├── ruby_2_6_hidden.rbi.exp
│   │       │   │   └── src
│   │       │   │       ├── Gemfile
│   │       │   │       ├── sorbet
│   │       │   │       │   ├── config
│   │       │   │       │   └── rbi/hidden-definitions
│   │       │   │       │       └── errors.txt
│   │       │   │       └── thorough.rb
│   │       │   └── update_hidden_methods_exp.sh
│   │       └── snapshot
│   │           ├── BUILD
│   │           ├── check_one.sh
│   │           ├── hermetic_tar.sh
│   │           ├── logging.sh
│   │           ├── partial
│   │           │   ├── bad-hash
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── src.rb
│   │           │   ├── bad-t/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── bad_gem
│   │           │   │   ├── expected
│   │           │   │   │   ├── out.log
│   │           │   │   │   └── sorbet
│   │           │   │   │       └── config
│   │           │   │   ├── sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── bad-gem.gemspec
│   │           │   │       ├── lib
│   │           │   │       │   └── bad-gem.rb
│   │           │   │       └── src.rb
│   │           │   ├── codecov/src
│   │           │   │   ├── Gemfile
│   │           │   │   └── Gemfile.lock
│   │           │   ├── create-config
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── db_schema
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   └── foo.txt
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── db
│   │           │   │       │   └── schema.rb
│   │           │   │       └── sorbet
│   │           │   │           └── foo.txt
│   │           │   ├── explosive-object/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── extconf
│   │           │   │   ├── expected
│   │           │   │   │   └── out.log
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── lib
│   │           │   │           └── extconf.rb
│   │           │   ├── fake-object/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── fake-rails/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── config
│   │           │   │       └── application.rb
│   │           │   ├── ignore_file_table
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   ├── config
│   │           │   │   │   └── important_file.txt
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── bad.rb
│   │           │   │       └── sorbet
│   │           │   │           ├── config
│   │           │   │           └── important_file.txt
│   │           │   ├── local_gem
│   │           │   │   ├── expected/sorbet/rbi/gems
│   │           │   │   │   └── my_gem.rbi
│   │           │   │   ├── gems/0.0.0/gems/my_gem-0.0.0
│   │           │   │   │   ├── lib
│   │           │   │   │   │   └── my_gem.rb
│   │           │   │   │   └── my_gem.gemspec
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── local_rvm_gemset_gem
│   │           │   │   ├── expected/sorbet/rbi/gems
│   │           │   │   │   └── my_gem.rbi
│   │           │   │   ├── gems/ruby-0.0.0@gemset/gems/my_gem-0.0.0
│   │           │   │   │   ├── lib
│   │           │   │   │   │   └── my_gem.rb
│   │           │   │   │   └── my_gem.gemspec
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── missing-instance-methods/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── non-utf-8-file
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── src.rb
│   │           │   ├── rails-double-require
│   │           │   │   ├── expected
│   │           │   │   │   └── err.log
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── app/models
│   │           │   │       │   └── article.rb
│   │           │   │       └── config
│   │           │   │           ├── application.rb
│   │           │   │           └── database.yml
│   │           │   ├── rails6/src
│   │           │   │   ├── Gemfile
│   │           │   │   └── Gemfile.lock
│   │           │   ├── real_singleton_class/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── rspec-lots/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── stack_master/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── stupidedi/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── type_member/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── my_enumerable.rb
│   │           │   ├── typed-ignore/src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── use-existing-config
│   │           │   │   ├── expected/sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── foo.rb
│   │           │   │       └── sorbet
│   │           │   │           └── config
│   │           │   └── webmock/src
│   │           │       ├── Gemfile
│   │           │       ├── Gemfile.lock
│   │           │       └── webmock.rb
│   │           ├── run_one.sh
│   │           ├── snapshot.bzl
│   │           ├── sorbet-typed.rev
│   │           ├── total
│   │           │   ├── empty
│   │           │   │   ├── expected
│   │           │   │   │   ├── err.log
│   │           │   │   │   ├── out.log
│   │           │   │   │   └── sorbet
│   │           │   │   │       ├── config
│   │           │   │   │       └── rbi/sorbet-typed/lib
│   │           │   │   │           ├── bundler/all
│   │           │   │   │           │   └── bundler.rbi
│   │           │   │   │           └── ruby/all
│   │           │   │   │               ├── gem.rbi
│   │           │   │   │               ├── open3.rbi
│   │           │   │   │               └── resolv.rbi
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   └── sorbet-runtime
│   │           │       ├── expected
│   │           │       │   ├── err.log
│   │           │       │   ├── out.log
│   │           │       │   └── sorbet
│   │           │       │       ├── config
│   │           │       │       └── rbi/sorbet-typed/lib
│   │           │       │           ├── bundler/all
│   │           │       │           │   └── bundler.rbi
│   │           │       │           └── ruby/all
│   │           │       │               ├── gem.rbi
│   │           │       │               ├── open3.rbi
│   │           │       │               └── resolv.rbi
│   │           │       └── src
│   │           │           ├── Gemfile
│   │           │           └── Gemfile.lock
│   │           ├── update_one.sh
│   │           └── validate_utils.sh
│   ├── sorbet-runtime
│   │   ├── BUILD
│   │   ├── Gemfile
│   │   ├── README.md
│   │   ├── Rakefile
│   │   ├── bench
│   │   │   ├── constructor.rb
│   │   │   ├── deserialize.rb
│   │   │   ├── getters.rb
│   │   │   ├── prop_definition.rb
│   │   │   ├── setters.rb
│   │   │   └── tasks.rb
│   │   ├── lib
│   │   │   ├── sorbet-runtime.rb
│   │   │   └── types
│   │   │       ├── _types.rb
│   │   │       ├── abstract_utils.rb
│   │   │       ├── boolean.rb
│   │   │       ├── compatibility_patches.rb
│   │   │       ├── configuration.rb
│   │   │       ├── enum.rb
│   │   │       ├── generic.rb
│   │   │       ├── helpers.rb
│   │   │       ├── interface_wrapper.rb
│   │   │       ├── non_forcing_constants.rb
│   │   │       ├── private
│   │   │       │   ├── abstract
│   │   │       │   │   ├── data.rb
│   │   │       │   │   ├── declare.rb
│   │   │       │   │   ├── hooks.rb
│   │   │       │   │   └── validate.rb
│   │   │       │   ├── casts.rb
│   │   │       │   ├── class_utils.rb
│   │   │       │   ├── decl_state.rb
│   │   │       │   ├── final.rb
│   │   │       │   ├── methods
│   │   │       │   │   ├── _methods.rb
│   │   │       │   │   ├── call_validation.rb
│   │   │       │   │   ├── decl_builder.rb
│   │   │       │   │   ├── modes.rb
│   │   │       │   │   ├── signature.rb
│   │   │       │   │   └── signature_validation.rb
│   │   │       │   ├── mixins
│   │   │       │   │   └── mixins.rb
│   │   │       │   ├── runtime_levels.rb
│   │   │       │   ├── sealed.rb
│   │   │       │   └── types
│   │   │       │       ├── not_typed.rb
│   │   │       │       ├── string_holder.rb
│   │   │       │       ├── type_alias.rb
│   │   │       │       └── void.rb
│   │   │       ├── profile.rb
│   │   │       ├── props
│   │   │       │   ├── _props.rb
│   │   │       │   ├── constructor.rb
│   │   │       │   ├── custom_type.rb
│   │   │       │   ├── decorator.rb
│   │   │       │   ├── errors.rb
│   │   │       │   ├── generated_code_validation.rb
│   │   │       │   ├── has_lazily_specialized_methods.rb
│   │   │       │   ├── optional.rb
│   │   │       │   ├── plugin.rb
│   │   │       │   ├── pretty_printable.rb
│   │   │       │   ├── private
│   │   │       │   │   ├── apply_default.rb
│   │   │       │   │   ├── deserializer_generator.rb
│   │   │       │   │   ├── parser.rb
│   │   │       │   │   ├── serde_transform.rb
│   │   │       │   │   ├── serializer_generator.rb
│   │   │       │   │   └── setter_factory.rb
│   │   │       │   ├── serializable.rb
│   │   │       │   ├── type_validation.rb
│   │   │       │   ├── utils.rb
│   │   │       │   └── weak_constructor.rb
│   │   │       ├── runtime_profiled.rb
│   │   │       ├── sig.rb
│   │   │       ├── struct.rb
│   │   │       ├── types
│   │   │       │   ├── attached_class.rb
│   │   │       │   ├── base.rb
│   │   │       │   ├── class_of.rb
│   │   │       │   ├── enum.rb
│   │   │       │   ├── fixed_array.rb
│   │   │       │   ├── fixed_hash.rb
│   │   │       │   ├── intersection.rb
│   │   │       │   ├── noreturn.rb
│   │   │       │   ├── proc.rb
│   │   │       │   ├── self_type.rb
│   │   │       │   ├── simple.rb
│   │   │       │   ├── t_enum.rb
│   │   │       │   ├── type_member.rb
│   │   │       │   ├── type_parameter.rb
│   │   │       │   ├── type_template.rb
│   │   │       │   ├── type_variable.rb
│   │   │       │   ├── typed_array.rb
│   │   │       │   ├── typed_enumerable.rb
│   │   │       │   ├── typed_enumerator.rb
│   │   │       │   ├── typed_hash.rb
│   │   │       │   ├── typed_range.rb
│   │   │       │   ├── typed_set.rb
│   │   │       │   ├── union.rb
│   │   │       │   └── untyped.rb
│   │   │       └── utils.rb
│   │   ├── sorbet-runtime.gemspec
│   │   └── test
│   │       ├── pay-server-shims.rbi
│   │       ├── test_helper.rb
│   │       ├── typecheck-runtime.sh
│   │       └── types
│   │           ├── abstract_validation.rb
│   │           ├── absurd.rb
│   │           ├── attached_class.rb
│   │           ├── builder_syntax.rb
│   │           ├── casts.rb
│   │           ├── configuration.rb
│   │           ├── edge_cases.rb
│   │           ├── enum.rb
│   │           ├── final_method.rb
│   │           ├── final_module.rb
│   │           ├── fixtures
│   │           │   ├── always_raise.rb
│   │           │   └── sealed_module
│   │           │       ├── forbid_sealed_class__1.rb
│   │           │       ├── forbid_sealed_class__2.rb
│   │           │       ├── forbid_sealed_class__3.rb
│   │           │       ├── forbid_sealed_module_extend__1.rb
│   │           │       ├── forbid_sealed_module_extend__2.rb
│   │           │       ├── forbid_sealed_module_extend__3.rb
│   │           │       ├── forbid_sealed_module_include__1.rb
│   │           │       ├── forbid_sealed_module_include__2.rb
│   │           │       ├── forbid_sealed_module_include__3.rb
│   │           │       ├── sealed_abstract__1.rb
│   │           │       ├── sealed_abstract__2.rb
│   │           │       ├── sealed_abstract__3.rb
│   │           │       ├── whitelist_violation__1.rb
│   │           │       └── whitelist_violation__2.rb
│   │           ├── interface_validation.rb
│   │           ├── interface_wrapper.rb
│   │           ├── method_modes.rb
│   │           ├── method_patches.rb
│   │           ├── method_validation.rb
│   │           ├── mixins.rb
│   │           ├── must.rb
│   │           ├── non_forcing_constants.rb
│   │           ├── props
│   │           │   ├── _props.rb
│   │           │   ├── constructor.rb
│   │           │   ├── decorator.rb
│   │           │   ├── optional.rb
│   │           │   ├── private
│   │           │   │   └── setter_factory.rb
│   │           │   ├── serializable.rb
│   │           │   └── struct.rb
│   │           ├── returns.rb
│   │           ├── sealed_module.rb
│   │           ├── sig.rb
│   │           ├── struct.rb
│   │           ├── types.rb
│   │           ├── types_to_ruby.rb
│   │           ├── validate_override_shape.rb
│   │           └── validate_override_types.rb
│   └── sorbet-static
│       └── sorbet-static.gemspec
├── infer
│   ├── BUILD
│   ├── SigSuggestion.cc
│   ├── SigSuggestion.h
│   ├── environment.cc
│   ├── environment.h
│   ├── infer.h
│   ├── inference.cc
│   ├── inference.h
│   └── test
│       └── infer_test.cc
├── local_vars
│   ├── BUILD
│   ├── local_vars.cc
│   └── local_vars.h
├── main
│   ├── BUILD
│   ├── autogen
│   │   ├── BUILD
│   │   ├── autogen.cc
│   │   ├── autogen.h
│   │   ├── autoloader.cc
│   │   ├── autoloader.h
│   │   ├── subclasses.cc
│   │   └── subclasses.h
│   ├── cache
│   │   ├── BUILD
│   │   ├── cache-orig.cc
│   │   ├── cache.cc
│   │   └── cache.h
│   ├── lsp
│   │   ├── BUILD
│   │   ├── DefLocSaver.cc
│   │   ├── DefLocSaver.h
│   │   ├── ErrorReporter.cc
│   │   ├── ErrorReporter.h
│   │   ├── LSPConfiguration.cc
│   │   ├── LSPConfiguration.h
│   │   ├── LSPFileUpdates.cc
│   │   ├── LSPFileUpdates.h
│   │   ├── LSPIndexer.cc
│   │   ├── LSPIndexer.h
│   │   ├── LSPInput.cc
│   │   ├── LSPInput.h
│   │   ├── LSPMessage.cc
│   │   ├── LSPMessage.h
│   │   ├── LSPOutput.cc
│   │   ├── LSPOutput.h
│   │   ├── LSPPreprocessor.cc
│   │   ├── LSPPreprocessor.h
│   │   ├── LSPTask.cc
│   │   ├── LSPTask.h
│   │   ├── LSPTypechecker.cc
│   │   ├── LSPTypechecker.h
│   │   ├── LSPTypecheckerCoordinator.cc
│   │   ├── LSPTypecheckerCoordinator.h
│   │   ├── LocalVarFinder.cc
│   │   ├── LocalVarFinder.h
│   │   ├── LocalVarSaver.cc
│   │   ├── LocalVarSaver.h
│   │   ├── NextMethodFinder.cc
│   │   ├── NextMethodFinder.h
│   │   ├── ShowOperation.cc
│   │   ├── ShowOperation.h
│   │   ├── UndoState.cc
│   │   ├── UndoState.h
│   │   ├── json_enums.h
│   │   ├── json_types.cc
│   │   ├── json_types.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── lsp_helpers.cc
│   │   ├── lsp_messages_gen_helpers.cc
│   │   ├── lsp_messages_gen_helpers.h
│   │   ├── notifications
│   │   │   ├── cancel_request.cc
│   │   │   ├── cancel_request.h
│   │   │   ├── exit.cc
│   │   │   ├── exit.h
│   │   │   ├── initialized.cc
│   │   │   ├── initialized.h
│   │   │   ├── notifications.h
│   │   │   ├── sorbet_fence.cc
│   │   │   ├── sorbet_fence.h
│   │   │   ├── sorbet_pause.cc
│   │   │   ├── sorbet_pause.h
│   │   │   ├── sorbet_resume.cc
│   │   │   ├── sorbet_resume.h
│   │   │   ├── sorbet_workspace_edit.cc
│   │   │   └── sorbet_workspace_edit.h
│   │   ├── protocol.cc
│   │   ├── request_dispatch.cc
│   │   ├── requests
│   │   │   ├── code_action.cc
│   │   │   ├── code_action.h
│   │   │   ├── completion.cc
│   │   │   ├── completion.h
│   │   │   ├── definition.cc
│   │   │   ├── definition.h
│   │   │   ├── document_highlight.cc
│   │   │   ├── document_highlight.h
│   │   │   ├── document_symbol.cc
│   │   │   ├── document_symbol.h
│   │   │   ├── get_counters.cc
│   │   │   ├── get_counters.h
│   │   │   ├── hover.cc
│   │   │   ├── hover.h
│   │   │   ├── initialize.cc
│   │   │   ├── initialize.h
│   │   │   ├── references.cc
│   │   │   ├── references.h
│   │   │   ├── requests.h
│   │   │   ├── shutdown.cc
│   │   │   ├── shutdown.h
│   │   │   ├── signature_help.cc
│   │   │   ├── signature_help.h
│   │   │   ├── sorbet_error.cc
│   │   │   ├── sorbet_error.h
│   │   │   ├── sorbet_read_file.cc
│   │   │   ├── sorbet_read_file.h
│   │   │   ├── type_definition.cc
│   │   │   ├── type_definition.h
│   │   │   ├── workspace_symbols.cc
│   │   │   └── workspace_symbols.h
│   │   ├── test
│   │   │   ├── error_reporter_test.cc
│   │   │   ├── generate_lsp_messages_test.cc
│   │   │   ├── lsp_file_updates_test.cc
│   │   │   ├── lsp_preprocessor_test.cc
│   │   │   └── lsp_test.cc
│   │   ├── tools
│   │   │   ├── generate_lsp_messages.cc
│   │   │   ├── generate_lsp_messages.h
│   │   │   ├── make_lsp_types.cc
│   │   │   └── make_lsp_types.h
│   │   ├── watchman
│   │   │   ├── WatchmanProcess.cc
│   │   │   └── WatchmanProcess.h
│   │   ├── wrapper.cc
│   │   └── wrapper.h
│   ├── main.cc
│   ├── options
│   │   ├── BUILD
│   │   ├── ConfigParser.cc
│   │   ├── ConfigParser.h
│   │   ├── options.cc
│   │   ├── options.h
│   │   └── test
│   │       └── options_test.cc
│   ├── pipeline
│   │   ├── BUILD
│   │   ├── ProgressIndicator.cc
│   │   ├── ProgressIndicator.h
│   │   ├── pipeline.cc
│   │   ├── pipeline.h
│   │   └── semantic_extension
│   │       ├── BUILD
│   │       ├── DefaultImplementation.cc
│   │       └── SemanticExtension.h
│   ├── realmain.cc
│   └── realmain.h
├── namer
│   ├── BUILD
│   ├── configatron
│   │   ├── BUILD
│   │   ├── configatron.cc
│   │   └── configatron.h
│   ├── namer.cc
│   ├── namer.h
│   └── test
│       └── namer_test.cc
├── parser
│   ├── BUILD
│   ├── Builder.cc
│   ├── Builder.h
│   ├── Dedenter.h
│   ├── Node.cc
│   ├── Node.h
│   ├── Parser.cc
│   ├── parser.h
│   ├── test
│   │   └── parser_test.cc
│   └── tools
│       └── generate_ast.cc
├── payload
│   ├── BUILD
│   ├── binary
│   │   ├── BUILD
│   │   ├── binary.h
│   │   ├── empty.cc
│   │   └── tools
│   │       └── gen_state_payload.cc
│   ├── otherwise_compdb_bugs_out.cc
│   ├── payload.cc
│   ├── payload.h
│   └── text
│       ├── BUILD
│       ├── nopopulate.cc
│       ├── populate.cc
│       ├── text.h
│       └── tools
│           └── generate_payload.cc
├── plugin
│   ├── BUILD
│   ├── Plugins.cc
│   ├── Plugins.h
│   ├── SubprocessTextPlugin.cc
│   └── SubprocessTextPlugin.h
├── proto
│   ├── BUILD
│   ├── File.proto
│   ├── Loc.proto
│   ├── Name.proto
│   ├── Symbol.proto
│   ├── Type.proto
│   └── pay-server
│       ├── BUILD
│       ├── README
│       └── SourceMetrics.proto
├── rbi
│   ├── BUILD
│   ├── core
│   │   ├── argf.rbi
│   │   ├── array.rbi
│   │   ├── basic_object.rbi
│   │   ├── binding.rbi
│   │   ├── class.rbi
│   │   ├── comparable.rbi
│   │   ├── complex.rbi
│   │   ├── constants.rbi
│   │   ├── data.rbi
│   │   ├── dir.rbi
│   │   ├── encoding.rbi
│   │   ├── enum.rbi
│   │   ├── enumerable.rbi
│   │   ├── enumerator.rbi
│   │   ├── errno.rbi
│   │   ├── errors.rbi
│   │   ├── exception.rbi
│   │   ├── false_class.rbi
│   │   ├── fiber.rbi
│   │   ├── fiber_error.rbi
│   │   ├── file.rbi
│   │   ├── file_test.rbi
│   │   ├── fixnum.rbi
│   │   ├── float.rbi
│   │   ├── gc.rbi
│   │   ├── hash.rbi
│   │   ├── integer.rbi
│   │   ├── io.rbi
│   │   ├── kernel.rbi
│   │   ├── marshal.rbi
│   │   ├── match_data.rbi
│   │   ├── math.rbi
│   │   ├── method.rbi
│   │   ├── module.rbi
│   │   ├── nil_class.rbi
│   │   ├── numeric.rbi
│   │   ├── object.rbi
│   │   ├── proc.rbi
│   │   ├── process.rbi
│   │   ├── random.rbi
│   │   ├── range.rbi
│   │   ├── rational.rbi
│   │   ├── rb_config.rbi
│   │   ├── regexp.rbi
│   │   ├── ruby_vm.rbi
│   │   ├── signal.rbi
│   │   ├── string.rbi
│   │   ├── struct.rbi
│   │   ├── symbol.rbi
│   │   ├── thread.rbi
│   │   ├── thread_group.rbi
│   │   ├── time.rbi
│   │   ├── trace_point.rbi
│   │   ├── true_class.rbi
│   │   ├── unbound_method.rbi
│   │   └── warning.rbi
│   ├── gems
│   │   ├── bundler.rbi
│   │   ├── chalk.rbi
│   │   ├── configatron.rbi
│   │   ├── didyoumean.rbi
│   │   ├── msgpack.rbi
│   │   └── stringscanner.rbi
│   ├── sorbet
│   │   ├── builder.rbi
│   │   ├── compatibility_patches.rbi
│   │   ├── sorbet.rbi
│   │   ├── t.rbi
│   │   ├── tprivate.rbi
│   │   ├── tprops.rbi
│   │   └── ttypes.rbi
│   ├── stdlib
│   │   ├── abbrev.rbi
│   │   ├── base64.rbi
│   │   ├── benchmark.rbi
│   │   ├── big_math.rbi
│   │   ├── bigdecimal.rbi
│   │   ├── cgi.rbi
│   │   ├── coverage.rbi
│   │   ├── csv.rbi
│   │   ├── date.rbi
│   │   ├── date_time.rbi
│   │   ├── delegator.rbi
│   │   ├── digest.rbi
│   │   ├── dir.rbi
│   │   ├── e2mmap.rbi
│   │   ├── erb.rbi
│   │   ├── etc.rbi
│   │   ├── fileutils.rbi
│   │   ├── forwardable.rbi
│   │   ├── gem.rbi
│   │   ├── json.rbi
│   │   ├── logger.rbi
│   │   ├── monitor.rbi
│   │   ├── mutex_m.rbi
│   │   ├── net.rbi
│   │   ├── objspace.rbi
│   │   ├── open3.rbi
│   │   ├── open_struct.rbi
│   │   ├── openssl.rbi
│   │   ├── optparse.rbi
│   │   ├── pathname.rbi
│   │   ├── pp.rbi
│   │   ├── racc.rbi
│   │   ├── ripper.rbi
│   │   ├── set.rbi
│   │   ├── singleton.rbi
│   │   ├── socket.rbi
│   │   ├── stringio.rbi
│   │   ├── tempfile.rbi
│   │   ├── time.rbi
│   │   ├── timeout.rbi
│   │   ├── tsort.rbi
│   │   ├── uri.rbi
│   │   └── webrick.rbi
│   └── tools
│       ├── generate_procs.cc
│       └── sync-rdoc.rb
├── resolver
│   ├── BUILD
│   ├── CorrectTypeAlias.cc
│   ├── CorrectTypeAlias.h
│   ├── GlobalPass.cc
│   ├── resolver.cc
│   ├── resolver.h
│   ├── type_syntax.cc
│   └── type_syntax.h
├── rewriter
│   ├── AttrReader.cc
│   ├── AttrReader.h
│   ├── BUILD
│   ├── ClassNew.cc
│   ├── ClassNew.h
│   ├── Cleanup.cc
│   ├── Cleanup.h
│   ├── Command.cc
│   ├── Command.h
│   ├── DSLBuilder.cc
│   ├── DSLBuilder.h
│   ├── DefaultArgs.cc
│   ├── DefaultArgs.h
│   ├── Delegate.cc
│   ├── Delegate.h
│   ├── Flatfiles.cc
│   ├── Flatfiles.h
│   ├── Flatten.cc
│   ├── Flatten.h
│   ├── Initializer.cc
│   ├── Initializer.h
│   ├── InterfaceWrapper.cc
│   ├── InterfaceWrapper.h
│   ├── Mattr.cc
│   ├── Mattr.h
│   ├── Minitest.cc
│   ├── Minitest.h
│   ├── MixinEncryptedProp.cc
│   ├── MixinEncryptedProp.h
│   ├── ModuleFunction.cc
│   ├── ModuleFunction.h
│   ├── Private.cc
│   ├── Private.h
│   ├── Prop.cc
│   ├── Prop.h
│   ├── ProtobufDescriptorPool.cc
│   ├── ProtobufDescriptorPool.h
│   ├── Rails.cc
│   ├── Rails.h
│   ├── Regexp.cc
│   ├── Regexp.h
│   ├── SelfNew.cc
│   ├── SelfNew.h
│   ├── SigRewriter.cc
│   ├── SigRewriter.h
│   ├── Struct.cc
│   ├── Struct.h
│   ├── TEnum.cc
│   ├── TEnum.h
│   ├── TypeMembers.cc
│   ├── TypeMembers.h
│   ├── Util.cc
│   ├── Util.h
│   ├── rewriter.cc
│   └── rewriter.h
├── sorbet_version
│   ├── BUILD
│   ├── sorbet_version.c
│   └── sorbet_version.h
├── test
│   ├── BUILD
│   ├── LSPTest.cc
│   ├── LSPTest.h
│   ├── autocorrect-test.cc
│   ├── backtrace-test-error.cc
│   ├── backtrace-test-raise.cc
│   ├── backtrace-test.sh
│   ├── cli
│   │   ├── BUILD
│   │   ├── allowed-extension
│   │   │   ├── allowed-extension.out
│   │   │   ├── allowed-extension.sh
│   │   │   └── lib
│   │   │       ├── a.rb
│   │   │       ├── b.rbi
│   │   │       ├── c.ru
│   │   │       └── d.rake
│   │   ├── arity-messages
│   │   │   ├── arity-messages.out
│   │   │   ├── arity-messages.rb
│   │   │   └── arity-messages.sh
│   │   ├── at
│   │   │   ├── at.input
│   │   │   ├── at.out
│   │   │   ├── at.rb
│   │   │   └── at.sh
│   │   ├── autocorrect
│   │   │   ├── autocorrect.out
│   │   │   ├── autocorrect.rb
│   │   │   └── autocorrect.sh
│   │   ├── autocorrect-abstract
│   │   │   ├── autocorrect-abstract.out
│   │   │   ├── autocorrect-abstract.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autocorrect-attached-class
│   │   │   ├── autocorrect-attached-class.out
│   │   │   ├── autocorrect-attached-class.rb
│   │   │   └── autocorrect-attached-class.sh
│   │   ├── autocorrect-bare-stdlib-generics
│   │   │   ├── autocorrect-bare-stdlib-generics.out
│   │   │   ├── autocorrect-bare-stdlib-generics.rb
│   │   │   └── autocorrect-bare-stdlib-generics.sh
│   │   ├── autocorrect-extend
│   │   │   ├── autocorrect-extend.out
│   │   │   ├── autocorrect-extend.rb
│   │   │   └── autocorrect-extend.sh
│   │   ├── autocorrect-lazy-type-alias
│   │   │   ├── autocorrect-lazy-type-alias.out
│   │   │   ├── autocorrect-lazy-type-alias.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autocorrect-private
│   │   │   ├── autocorrect-private.out
│   │   │   ├── autocorrect-private.rb
│   │   │   └── autocorrect-private.sh
│   │   ├── autocorrect-remove-body
│   │   │   ├── autocorrect-remove-body.out
│   │   │   ├── autocorrect-remove-body.sh
│   │   │   ├── post.rb
│   │   │   ├── post.rbi
│   │   │   ├── pre.rb
│   │   │   └── pre.rbi
│   │   ├── autocorrect-same-loc
│   │   │   ├── autocorrect-same-loc-1.rb
│   │   │   ├── autocorrect-same-loc-2.rb
│   │   │   ├── autocorrect-same-loc.out
│   │   │   └── autocorrect-same-loc.sh
│   │   ├── autocorrect-strict
│   │   │   ├── autocorrect-strict.out
│   │   │   ├── autocorrect-strict.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autogen-autoloader
│   │   │   ├── autogen-autoloader.out
│   │   │   ├── autogen-autoloader.sh
│   │   │   ├── bar.rb
│   │   │   ├── bar2.rb
│   │   │   ├── errors.rb
│   │   │   ├── foo.rb
│   │   │   ├── inplace.rb
│   │   │   └── scripts
│   │   │       └── baz.rb
│   │   ├── autogen-classlist
│   │   │   ├── a.rb
│   │   │   ├── autogen-classlist.out
│   │   │   ├── autogen-classlist.sh
│   │   │   └── b.rb
│   │   ├── autogen-errors
│   │   │   ├── autogen-errors.out
│   │   │   ├── autogen-errors.rb
│   │   │   └── autogen-errors.sh
│   │   ├── autogen-subclasses
│   │   │   ├── a.rb
│   │   │   ├── autogen-subclasses.out
│   │   │   └── autogen-subclasses.sh
│   │   ├── autogen-subclasses-ignore
│   │   │   ├── autogen-subclasses-ignore.out
│   │   │   ├── autogen-subclasses-ignore.sh
│   │   │   ├── ignored
│   │   │   │   └── ignored.rb
│   │   │   └── not-ignored
│   │   │       └── not-ignored.rb
│   │   ├── backtrace
│   │   │   ├── backtrace.out
│   │   │   └── backtrace.sh
│   │   ├── bad-perm
│   │   │   ├── bad-perm.out
│   │   │   └── bad-perm.sh
│   │   ├── bad-plugin-spec
│   │   │   ├── bad-plugin-spec.out
│   │   │   ├── bad-plugin-spec.sh
│   │   │   ├── duplicate-triggers.yaml
│   │   │   ├── missing-triggers.yaml
│   │   │   ├── non-string-in-ruby-extra-args.yaml
│   │   │   ├── not-map.yaml
│   │   │   ├── ruby-extra-args-not-array.yaml
│   │   │   ├── triggers-not-map.yaml
│   │   │   └── values-not-scalar.yaml
│   │   ├── cache-add-typed
│   │   │   ├── cache-add-typed.out
│   │   │   └── cache-add-typed.sh
│   │   ├── cache-doesnt-parse
│   │   │   ├── cache-doesnt-parse.out
│   │   │   └── cache-doesnt-parse.sh
│   │   ├── cache-dsl
│   │   │   ├── attr_accessor.rb
│   │   │   ├── cache-dsl.out
│   │   │   └── cache-dsl.sh
│   │   ├── cache-keeps-print-options
│   │   │   ├── cache-keeps-print-options.out
│   │   │   ├── cache-keeps-print-options.rb
│   │   │   └── cache-keeps-print-options.sh
│   │   ├── cache-reserve-mem
│   │   │   ├── cache-reserve-mem.out
│   │   │   ├── cache-reserve-mem.sh
│   │   │   └── input.rb
│   │   ├── cli_test.bzl
│   │   ├── config-file
│   │   │   ├── config-file.out
│   │   │   ├── config-file.rb
│   │   │   ├── config-file.sh
│   │   │   └── sorbet
│   │   │       ├── bad_no_config
│   │   │       ├── config
│   │   │       └── other_config
│   │   ├── config-file-recursive
│   │   │   ├── config-file-recursive.out
│   │   │   ├── config-file-recursive.rb
│   │   │   ├── config-file-recursive.sh
│   │   │   └── sorbet
│   │   │       ├── config
│   │   │       └── other-config
│   │   ├── configatron
│   │   │   ├── configatron.out
│   │   │   ├── configatron.rb
│   │   │   ├── configatron.sh
│   │   │   └── configatron.yaml
│   │   ├── configatron-yaml-error
│   │   │   ├── configatron-yaml-error.out
│   │   │   ├── configatron-yaml-error.rb
│   │   │   ├── configatron-yaml-error.sh
│   │   │   └── configatron-yaml-error.yaml
│   │   ├── conflicting-definition
│   │   │   ├── a.rb
│   │   │   ├── b.rb
│   │   │   ├── conflicting-definition.out
│   │   │   └── conflicting-definition.sh
│   │   ├── constant-fuzzy
│   │   │   ├── constant-fuzzy.out
│   │   │   ├── constant-fuzzy.rb
│   │   │   └── constant-fuzzy.sh
│   │   ├── correct-bare-stdlib-generics
│   │   │   ├── correct-bare-stdlib-generics.out
│   │   │   ├── correct-bare-stdlib-generics.rb
│   │   │   └── correct-bare-stdlib-generics.sh
│   │   ├── counters
│   │   │   ├── counters.out
│   │   │   └── counters.sh
│   │   ├── dash-e
│   │   │   ├── dash-e.out
│   │   │   └── dash-e.sh
│   │   ├── dedup-input-files
│   │   │   ├── dedup-input-files.out
│   │   │   └── dedup-input-files.sh
│   │   ├── dedup_loc
│   │   │   ├── dedup_loc.out
│   │   │   ├── dedup_loc.rb
│   │   │   └── dedup_loc.sh
│   │   ├── empty-file
│   │   │   ├── empty-file.out
│   │   │   ├── empty-file.sh
│   │   │   └── empty.rb
│   │   ├── error-blacklist
│   │   │   ├── error-blacklist.out
│   │   │   ├── error-blacklist.rb
│   │   │   └── error-blacklist.sh
│   │   ├── error-url
│   │   │   ├── error-url.out
│   │   │   ├── error-url.rb
│   │   │   └── error-url.sh
│   │   ├── error-whitelist
│   │   │   ├── error-whitelist.out
│   │   │   ├── error-whitelist.rb
│   │   │   └── error-whitelist.sh
│   │   ├── errors
│   │   │   ├── errors.out
│   │   │   ├── errors.rb
│   │   │   └── errors.sh
│   │   ├── escaping
│   │   │   ├── escaping.out
│   │   │   ├── escaping.rb
│   │   │   └── escaping.sh
│   │   ├── file-table-json
│   │   │   ├── file-table-json.out
│   │   │   ├── file-table-json.rb
│   │   │   └── file-table-json.sh
│   │   ├── folder-input
│   │   │   ├── folder-input.out
│   │   │   ├── folder-input.sh
│   │   │   ├── foo.rb
│   │   │   └── input
│   │   │       ├── bar.rb
│   │   │       ├── file_with_no_dot
│   │   │       └── subfolder
│   │   │           └── baz.rb
│   │   ├── folder-input-dir-and-file
│   │   │   ├── folder-input-dir-and-file.out
│   │   │   ├── folder-input-dir-and-file.sh
│   │   │   ├── foo.rb
│   │   │   └── input
│   │   │       ├── bar.rb
│   │   │       ├── file_with_no_dot
│   │   │       └── subfolder
│   │   │           └── baz.rb
│   │   ├── folder-input-not-dir
│   │   │   ├── folder-input-not-dir.out
│   │   │   ├── folder-input-not-dir.sh
│   │   │   └── foo.rb
│   │   ├── folder-input-not-found
│   │   │   ├── folder-input-not-found.out
│   │   │   └── folder-input-not-found.sh
│   │   ├── forbid-autocorrect-with-e
│   │   │   ├── forbid-autocorrect-with-e.out
│   │   │   └── forbid-autocorrect-with-e.sh
│   │   ├── forbid-autocorrect-with-quiet
│   │   │   ├── forbid-autocorrect-with-quiet.out
│   │   │   ├── forbid-autocorrect-with-quiet.sh
│   │   │   └── input.rb
│   │   ├── forgot-typed
│   │   │   ├── forgot-typed.out
│   │   │   ├── forgot-typed.rb
│   │   │   ├── forgot-typed.sh
│   │   │   └── permit-dsl-sig.rb
│   │   ├── help
│   │   │   ├── help.out
│   │   │   └── help.sh
│   │   ├── hup-hack
│   │   │   ├── hup-hack.out
│   │   │   └── hup-hack.sh
│   │   ├── ignore
│   │   │   ├── bar.rb
│   │   │   ├── foo.rb
│   │   │   ├── ignore.out
│   │   │   ├── ignore.sh
│   │   │   ├── subfolder
│   │   │   │   └── baz.rb
│   │   │   └── subfolder2
│   │   │       ├── foo.rb
│   │   │       └── subfolder
│   │   │           └── bar.rb
│   │   ├── ignore-slash
│   │   │   ├── bar.rb
│   │   │   ├── foo
│   │   │   │   └── foo.rb
│   │   │   ├── ignore-slash.out
│   │   │   └── ignore-slash.sh
│   │   ├── incremental-resolver
│   │   │   ├── expect-failures
│   │   │   │   ├── abstract_impl.rb
│   │   │   │   ├── constant_override.rb
│   │   │   │   ├── constant_redefinition.rb
│   │   │   │   └── multiple_sigs.rb
│   │   │   ├── incremental-resolver.out
│   │   │   ├── incremental-resolver.sh
│   │   │   ├── type-member.rb
│   │   │   └── type-template.rb
│   │   ├── index-cache-invalidation
│   │   │   ├── index-cache-invalidation.out
│   │   │   └── index-cache-invalidation.sh
│   │   ├── kwarg-loc
│   │   │   ├── kwarg-loc.out
│   │   │   ├── kwarg-loc.rb
│   │   │   └── kwarg-loc.sh
│   │   ├── license
│   │   │   ├── license.out
│   │   │   └── license.sh
│   │   ├── line-splitting
│   │   │   ├── line-splitting.out
│   │   │   └── line-splitting.sh
│   │   ├── logging
│   │   │   ├── logging.out
│   │   │   └── logging.sh
│   │   ├── lsp-common-case-exit
│   │   │   ├── lsp-common-case-exit.out
│   │   │   └── lsp-common-case-exit.sh
│   │   ├── lsp-invalid-json-and-exit
│   │   │   ├── lsp-invalid-json-and-exit.out
│   │   │   └── lsp-invalid-json-and-exit.sh
│   │   ├── lsp-large-message
│   │   │   ├── lsp-large-message.out
│   │   │   └── lsp-large-message.sh
│   │   ├── lsp-requires-input-dir
│   │   │   ├── lsp-requires-input-dir.out
│   │   │   └── lsp-requires-input-dir.sh
│   │   ├── make_accessible
│   │   │   ├── make_accessible.out
│   │   │   ├── make_accessible.sh
│   │   │   └── suit.rb
│   │   ├── metrics-file
│   │   │   ├── metrics-file.out
│   │   │   ├── metrics-file.sh
│   │   │   ├── sorbet
│   │   │   │   ├── plugin.rb
│   │   │   │   └── triggers.yml
│   │   │   ├── test.rb
│   │   │   └── with-error-branching.rb
│   │   ├── missing-constants
│   │   │   ├── missing-constants.out
│   │   │   ├── missing-constants.rb
│   │   │   └── missing-constants.sh
│   │   ├── model_mutator_behavior
│   │   │   ├── model_mutator_behavior.out
│   │   │   ├── model_mutator_behavior.sh
│   │   │   ├── model_mutator_behavior__1.rb
│   │   │   └── model_mutator_behavior__2.rb
│   │   ├── module-redefinition
│   │   │   ├── module-redefinition-1.rb
│   │   │   ├── module-redefinition-2.rb
│   │   │   ├── module-redefinition-3.rb
│   │   │   ├── module-redefinition.out
│   │   │   └── module-redefinition.sh
│   │   ├── no-did-you-mean
│   │   │   ├── no-did-you-mean.out
│   │   │   ├── no-did-you-mean.rb
│   │   │   └── no-did-you-mean.sh
│   │   ├── no-error-count
│   │   │   ├── no-error-count.out
│   │   │   └── no-error-count.sh
│   │   ├── no-stdlib
│   │   │   ├── no-stdlib.out
│   │   │   └── no-stdlib.sh
│   │   ├── non-existing-option
│   │   │   ├── non-existing-option.out
│   │   │   └── non-existing-option.sh
│   │   ├── override-typed
│   │   │   ├── override-typed.out
│   │   │   ├── override-typed.rb
│   │   │   ├── override-typed.sh
│   │   │   └── override-typed.yaml
│   │   ├── override-typed-bad
│   │   │   ├── bad-filename.yaml
│   │   │   ├── bad-list.yaml
│   │   │   ├── bad-strictness.yaml
│   │   │   ├── bad-top-level.yaml
│   │   │   ├── override-typed-bad.out
│   │   │   ├── override-typed-bad.rb
│   │   │   ├── override-typed-bad.sh
│   │   │   └── override-typed-bad.yaml
│   │   ├── parallel-ordering
│   │   │   ├── 1.rb
│   │   │   ├── 2.rb
│   │   │   ├── 3.rb
│   │   │   ├── parallel-ordering.out
│   │   │   └── parallel-ordering.sh
│   │   ├── parse-tree-whitequark
│   │   │   ├── parse-tree-whitequark.out
│   │   │   ├── parse-tree-whitequark.rb
│   │   │   └── parse-tree-whitequark.sh
│   │   ├── parser-error
│   │   │   ├── parser-error-1.rb
│   │   │   ├── parser-error-2.rb
│   │   │   ├── parser-error-3.rb
│   │   │   ├── parser-error-4.rb
│   │   │   ├── parser-error.out
│   │   │   └── parser-error.sh
│   │   ├── phases
│   │   │   ├── phases.out
│   │   │   └── phases.sh
│   │   ├── print-procs
│   │   │   ├── print-procs.out
│   │   │   ├── print-procs.rb
│   │   │   └── print-procs.sh
│   │   ├── print_generics
│   │   │   ├── print_generics.out
│   │   │   ├── print_generics.rb
│   │   │   └── print_generics.sh
│   │   ├── print_to_file
│   │   │   ├── a.rb
│   │   │   ├── b.rb
│   │   │   ├── c.rb
│   │   │   ├── d.rb
│   │   │   ├── print_to_file.out
│   │   │   └── print_to_file.sh
│   │   ├── progressbar
│   │   │   ├── progressbar.out
│   │   │   └── progressbar.sh
│   │   ├── rbi-overrides
│   │   │   ├── rbi-overrides.out
│   │   │   ├── rbi-overrides.sh
│   │   │   ├── t1.rb
│   │   │   ├── t1.rbi
│   │   │   ├── t2.rb
│   │   │   ├── t2.rbi
│   │   │   ├── t3.rb
│   │   │   ├── t3.rbi
│   │   │   ├── t4.rb
│   │   │   └── t4.rbi
│   │   ├── rbi-with-code
│   │   │   ├── rbi-with-code.out
│   │   │   ├── rbi-with-code.rbi
│   │   │   └── rbi-with-code.sh
│   │   ├── remove-path-prefix
│   │   │   ├── remove-path-prefix.out
│   │   │   ├── remove-path-prefix.rb
│   │   │   └── remove-path-prefix.sh
│   │   ├── remove-path-prefix-https
│   │   │   ├── remove-path-prefix-https.out
│   │   │   ├── remove-path-prefix-https.rb
│   │   │   └── remove-path-prefix-https.sh
│   │   ├── remove-path-prefix-no-match
│   │   │   ├── remove-path-prefix-no-match.out
│   │   │   ├── remove-path-prefix-no-match.rb
│   │   │   └── remove-path-prefix-no-match.sh
│   │   ├── sigil-rbi
│   │   │   ├── abstract.rbi
│   │   │   ├── multiple_definition.rbi
│   │   │   ├── no_type.rbi
│   │   │   ├── overrides.rbi
│   │   │   ├── sigil-rbi.out
│   │   │   ├── sigil-rbi.sh
│   │   │   ├── strict.rbi
│   │   │   └── typed.rbi
│   │   ├── silence-dev-message
│   │   │   ├── silence-dev-message.out
│   │   │   └── silence-dev-message.sh
│   │   ├── statsd
│   │   │   ├── statsd.out
│   │   │   └── statsd.sh
│   │   ├── stop-after
│   │   │   ├── stop-after.out
│   │   │   └── stop-after.sh
│   │   ├── stop-after-namer
│   │   │   ├── stop-after-namer.out
│   │   │   ├── stop-after-namer.rb
│   │   │   └── stop-after-namer.sh
│   │   ├── store-state
│   │   │   ├── store-state.out
│   │   │   └── store-state.sh
│   │   ├── subprocess-plugin
│   │   │   ├── bad_plugin.rb
│   │   │   ├── bad_plugin.yaml
│   │   │   ├── bar_gen.rb
│   │   │   ├── baz_gen.rb
│   │   │   ├── echo_argv.rb
│   │   │   ├── echo_argv.yaml
│   │   │   ├── foo_gen.rb
│   │   │   ├── gen.rb
│   │   │   ├── multi.yaml
│   │   │   ├── multi1.rb
│   │   │   ├── multi2.rb
│   │   │   ├── multi3.rb
│   │   │   ├── multi4.rb
│   │   │   ├── multi5.rb
│   │   │   ├── multi6.rb
│   │   │   ├── multi_empty.yaml
│   │   │   ├── no_output.rb
│   │   │   ├── permute.rb
│   │   │   ├── ruby_extra_args.yaml
│   │   │   ├── subprocess-plugin.out
│   │   │   ├── subprocess-plugin.sh
│   │   │   ├── trigger_bad_plugin.rb
│   │   │   └── verify_ruby_options.rb
│   │   ├── suggest-constant-type
│   │   │   ├── suggest-constant-type.out
│   │   │   ├── suggest-constant-type.rb
│   │   │   └── suggest-constant-type.sh
│   │   ├── suggest-foreign-lambda
│   │   │   ├── suggest-foreign-lambda.out
│   │   │   ├── suggest-foreign-lambda.rb
│   │   │   └── suggest-foreign-lambda.sh
│   │   ├── suggest-kernel
│   │   │   ├── suggest-kernel.out
│   │   │   ├── suggest-kernel.rb
│   │   │   └── suggest-kernel.sh
│   │   ├── suggest-new
│   │   │   ├── suggest-new.out
│   │   │   ├── suggest-new.rb
│   │   │   └── suggest-new.sh
│   │   ├── suggest-not-stub
│   │   │   ├── suggest-not-stub.out
│   │   │   ├── suggest-not-stub.rb
│   │   │   └── suggest-not-stub.sh
│   │   ├── suggest-object
│   │   │   ├── suggest-object.out
│   │   │   ├── suggest-object.rb
│   │   │   └── suggest-object.sh
│   │   ├── suggest-sig
│   │   │   ├── suggest-sig.out
│   │   │   ├── suggest-sig.rb
│   │   │   └── suggest-sig.sh
│   │   ├── suggest-sig-garbage
│   │   │   ├── suggest-sig-garbage.out
│   │   │   ├── suggest-sig-garbage.rb
│   │   │   └── suggest-sig-garbage.sh
│   │   ├── suggest-sig-literal
│   │   │   ├── suggest-sig-literal.out
│   │   │   ├── suggest-sig-literal.rb
│   │   │   └── suggest-sig-literal.sh
│   │   ├── suggest-sig-override
│   │   │   ├── suggest-sig-override.out
│   │   │   ├── suggest-sig-override.rb
│   │   │   └── suggest-sig-override.sh
│   │   ├── suggest-sig-override-edge
│   │   │   ├── suggest-sig-override-edge.out
│   │   │   ├── suggest-sig-override-edge.rb
│   │   │   └── suggest-sig-override-edge.sh
│   │   ├── suggest-singleton
│   │   │   ├── suggest-singleton.out
│   │   │   ├── suggest-singleton.rb
│   │   │   └── suggest-singleton.sh
│   │   ├── suggest-t-let
│   │   │   ├── suggest-t-let.out
│   │   │   ├── suggest-t-let.rb
│   │   │   └── suggest-t-let.sh
│   │   ├── suggest-type-alias
│   │   │   ├── suggest-type-alias.out
│   │   │   ├── suggest-type-alias.rb
│   │   │   └── suggest-type-alias.sh
│   │   ├── suggest-typed-true
│   │   │   ├── empty.rb
│   │   │   ├── suggest-typed-already-autogenerated.rb
│   │   │   ├── suggest-typed-already-ignore.rb
│   │   │   ├── suggest-typed-and-type.rb
│   │   │   ├── suggest-typed-behaviour-over-multiple-1.rb
│   │   │   ├── suggest-typed-behaviour-over-multiple-2.rb
│   │   │   ├── suggest-typed-false.rb
│   │   │   ├── suggest-typed-ignore.rb
│   │   │   ├── suggest-typed-shabang.rb
│   │   │   ├── suggest-typed-strict.rb
│   │   │   ├── suggest-typed-strong.rb
│   │   │   ├── suggest-typed-true.out
│   │   │   ├── suggest-typed-true.rb
│   │   │   ├── suggest-typed-true.sh
│   │   │   └── suggest-typed-with-too-low.rb
│   │   ├── suggest-typos
│   │   │   ├── suggest-typos.out
│   │   │   └── suggest-typos.sh
│   │   ├── suggest_autogen
│   │   │   ├── suggest_autogen.out
│   │   │   ├── suggest_autogen.rb
│   │   │   └── suggest_autogen.sh
│   │   ├── suggest_static
│   │   │   ├── suggest_static.out
│   │   │   ├── suggest_static.rb
│   │   │   └── suggest_static.sh
│   │   ├── suggest_t_must
│   │   │   ├── suggest_t_must.out
│   │   │   ├── suggest_t_must.rb
│   │   │   └── suggest_t_must.sh
│   │   ├── suppress-non-critical
│   │   │   ├── suppress-non-critical.out
│   │   │   └── suppress-non-critical.sh
│   │   ├── symbol-table-json
│   │   │   ├── symbol-table-json.out
│   │   │   ├── symbol-table-json.rb
│   │   │   └── symbol-table-json.sh
│   │   ├── symbol-table-json-alias
│   │   │   ├── symbol-table-json-alias.out
│   │   │   ├── symbol-table-json-alias.rb
│   │   │   └── symbol-table-json-alias.sh
│   │   ├── test_one.sh
│   │   ├── typed-src
│   │   │   └── typed-src.out
│   │   ├── update_one.sh
│   │   ├── version-returns-0
│   │   │   ├── version-returns-0.out
│   │   │   └── version-returns-0.sh
│   │   ├── web-trace-file
│   │   │   ├── web-trace-file.out
│   │   │   └── web-trace-file.sh
│   │   └── web-trace-file-non-ascii
│   │       ├── web-trace-file-non-ascii.out
│   │       ├── web-trace-file-non-ascii.rb
│   │       └── web-trace-file-non-ascii.sh
│   ├── error-check-test.cc
│   ├── fuzz
│   │   ├── BUILD
│   │   ├── TextDocumentPositionParamsWithoutTextDocumentIdentifier.proto
│   │   ├── empty.cc
│   │   ├── fuzz_dash_e.cc
│   │   ├── fuzz_doc_symbols.cc
│   │   ├── fuzz_hover.cc
│   │   └── ruby.dict
│   ├── hello-test.cc
│   ├── helpers
│   │   ├── BUILD
│   │   ├── MockFileSystem.cc
│   │   ├── MockFileSystem.h
│   │   ├── expectations.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── position_assertions.cc
│   │   └── position_assertions.h
│   ├── lint/buildifier
│   │   └── BUILD
│   ├── lsp
│   │   ├── BUILD
│   │   ├── ProtocolTest.cc
│   │   ├── ProtocolTest.h
│   │   ├── alias-incremental
│   │   │   └── alias-incremental.rec
│   │   ├── cache_protocol_test_corpus.cc
│   │   ├── incremental-lsp-changes
│   │   │   └── incremental-lsp-changes.rec
│   │   ├── lsp_test.bzl
│   │   ├── lsp_test_runner.sh
│   │   ├── multithreaded_protocol_test_corpus.cc
│   │   ├── no-trailing-newline
│   │   │   └── no-trailing-newline.rec
│   │   ├── protocol_test_corpus.cc
│   │   ├── update_one.sh
│   │   ├── watchman_test_corpus.cc
│   │   └── workspaceSymbol
│   │       └── workspaceSymbol.rec
│   ├── pipeline_test.bzl
│   ├── print_document_symbols.cc
│   ├── test_corpus.cc
│   ├── test_corpus_forwarder.sh
│   ├── testdata
│   │   ├── autogen
│   │   │   ├── aliased_ancestors.rb
│   │   │   ├── aliased_ancestors.rb.autogen.exp
│   │   │   ├── ancestors.rb
│   │   │   ├── ancestors.rb.autogen.exp
│   │   │   ├── ancestors_ref.rb
│   │   │   ├── ancestors_ref.rb.autogen.exp
│   │   │   ├── bad_prop.rb
│   │   │   ├── bad_prop.rb.autogen.exp
│   │   │   ├── bare_alias.rb
│   │   │   ├── bare_alias.rb.autogen.exp
│   │   │   ├── bare_casgn.rb
│   │   │   ├── bare_casgn.rb.autogen.exp
│   │   │   ├── bare_class.rb
│   │   │   ├── bare_class.rb.autogen.exp
│   │   │   ├── bare_module.rb
│   │   │   ├── bare_module.rb.autogen.exp
│   │   │   ├── cbase_const.rb
│   │   │   ├── cbase_const.rb.autogen.exp
│   │   │   ├── defines_behavior.rb
│   │   │   ├── defines_behavior.rb.autogen.exp
│   │   │   ├── dynamic_superclass.rb
│   │   │   ├── dynamic_superclass.rb.autogen.exp
│   │   │   ├── foo.fixturerb
│   │   │   ├── foo.fixturerb.autogen.exp
│   │   │   ├── generator.rb
│   │   │   ├── generator.rb.autogen.exp
│   │   │   ├── hidden_include.rb
│   │   │   ├── hidden_include.rb.autogen.exp
│   │   │   ├── multiple_alias.rb
│   │   │   ├── multiple_alias.rb.autogen.exp
│   │   │   ├── nested_defs.rb
│   │   │   ├── nested_defs.rb.autogen.exp
│   │   │   ├── no_dsls.rb
│   │   │   ├── no_dsls.rb.autogen.exp
│   │   │   ├── resolving_refs.rb
│   │   │   ├── resolving_refs.rb.autogen.exp
│   │   │   ├── simple_refs.rb
│   │   │   ├── simple_refs.rb.autogen.exp
│   │   │   ├── strong_sigil.rb
│   │   │   └── strong_sigil.rb.autogen.exp
│   │   ├── call_with_block.rb
│   │   ├── call_with_block_strict.rb
│   │   ├── call_with_splat_and_block.rb
│   │   ├── call_with_splat_and_block_strict.rb
│   │   ├── cfg
│   │   │   ├── array.rb
│   │   │   ├── array.rb.cfg.exp
│   │   │   ├── block_in_deadcode.rb
│   │   │   ├── block_in_deadcode.rb.cfg.exp
│   │   │   ├── block_return_bug.rb
│   │   │   ├── blocks.rb
│   │   │   ├── blocks.rb.cfg.exp
│   │   │   ├── blocks.rb.desugar-tree.exp
│   │   │   ├── blocks.rb.parse-tree.exp
│   │   │   ├── blocks.rb.symbol-table-raw.exp
│   │   │   ├── break.rb
│   │   │   ├── break.rb.cfg.exp
│   │   │   ├── break_in_junk.rb
│   │   │   ├── break_in_junk.rb.cfg.exp
│   │   │   ├── break_in_while.rb
│   │   │   ├── break_in_while.rb.cfg.exp
│   │   │   ├── class_static_field.rb
│   │   │   ├── class_static_field_children.rb
│   │   │   ├── dealias_with_return.rb
│   │   │   ├── dealias_with_return.rb.cfg.exp
│   │   │   ├── do_while.rb
│   │   │   ├── do_while.rb.cfg.exp
│   │   │   ├── ensure_after_raise.rb
│   │   │   ├── examples.rb
│   │   │   ├── examples.rb.cfg.exp
│   │   │   ├── examples.rb.desugar-tree.exp
│   │   │   ├── examples.rb.flatten-tree.exp
│   │   │   ├── examples.rb.parse-tree.exp
│   │   │   ├── examples.rb.symbol-table-raw.exp
│   │   │   ├── extra_bb_args.rb
│   │   │   ├── extra_bb_args.rb.cfg.exp
│   │   │   ├── floats.rb
│   │   │   ├── floats.rb.cfg.exp
│   │   │   ├── floats.rb.desugar-tree.exp
│   │   │   ├── fuzz_dedup_exit_edges.rb
│   │   │   ├── fuzz_recursive_alias.rb
│   │   │   ├── hash.rb
│   │   │   ├── hash.rb.cfg.exp
│   │   │   ├── ivar_assign.rb
│   │   │   ├── ivar_assign.rb.cfg.exp
│   │   │   ├── next.rb
│   │   │   ├── next.rb.cfg.exp
│   │   │   ├── next_in_junk.rb
│   │   │   ├── next_in_junk.rb.cfg.exp
│   │   │   ├── next_in_while.rb
│   │   │   ├── next_in_while.rb.cfg.exp
│   │   │   ├── override_bang.rb
│   │   │   ├── raw_test.rb
│   │   │   ├── raw_test.rb.cfg-raw.exp
│   │   │   ├── reassign_dead_block_bug.rb
│   │   │   ├── reassign_dead_block_bug.rb.cfg.exp
│   │   │   ├── rescue.rb
│   │   │   ├── rescue.rb.cfg.exp
│   │   │   ├── rescue.rb.desugar-tree-raw.exp
│   │   │   ├── rescue.rb.desugar-tree.exp
│   │   │   ├── rescue_bad_class.rb
│   │   │   ├── rescue_bad_class.rb.flatten-tree.exp
│   │   │   ├── rescue_complex.rb
│   │   │   ├── rescue_complex.rb.cfg.exp
│   │   │   ├── rescue_complex.rb.desugar-tree.exp
│   │   │   ├── rescue_else_block.rb
│   │   │   ├── rescue_else_block.rb.cfg.exp
│   │   │   ├── rescue_expression.rb
│   │   │   ├── rescue_expression.rb.cfg.exp
│   │   │   ├── rescue_two_return.rb
│   │   │   ├── rescue_two_return.rb.cfg.exp
│   │   │   ├── rescue_var_expression.rb
│   │   │   ├── rescue_var_expression.rb.cfg.exp
│   │   │   ├── rescue_with_return.rb
│   │   │   ├── rescue_with_return.rb.cfg.exp
│   │   │   ├── retry.rb
│   │   │   ├── retry.rb.cfg.exp
│   │   │   ├── retry.rb.desugar-tree-raw.exp
│   │   │   ├── retry.rb.desugar-tree.exp
│   │   │   ├── retry_invalid.rb
│   │   │   ├── retry_multiple.rb
│   │   │   ├── retry_multiple.rb.cfg.exp
│   │   │   ├── retry_nested.rb
│   │   │   ├── retry_nested.rb.cfg.exp
│   │   │   ├── return_type_of_nilable_blk_param.rb
│   │   │   ├── side_effects.rb
│   │   │   ├── side_effects.rb.cfg.exp
│   │   │   ├── side_effects2.rb
│   │   │   ├── side_effects2.rb.cfg.exp
│   │   │   ├── singleton_lazy.rb
│   │   │   ├── singleton_lazy.rb.symbol-table-raw.exp
│   │   │   ├── triple_eq.rb
│   │   │   ├── uaf1.rb
│   │   │   ├── uaf1.rb.cfg.exp
│   │   │   ├── undeclared_variable.rb
│   │   │   ├── unreachable.rb
│   │   │   └── wtf_ensure.rb
│   │   ├── class_not_class_of.rb
│   │   ├── core
│   │   │   ├── autogenerated.rb
│   │   │   ├── fuzz_bad_subtyping.rb
│   │   │   └── fuzz_unparseable.rb
│   │   ├── desugar
│   │   │   ├── accidentally_quadratic.rb
│   │   │   ├── accidentally_quadratic.rb.desugar-tree.exp
│   │   │   ├── and_self.rb
│   │   │   ├── assign_empty_parens.rb
│   │   │   ├── assign_empty_stmts.rb
│   │   │   ├── assign_keyword.rb
│   │   │   ├── backtick.rb
│   │   │   ├── backtick.rb.desugar-tree.exp
│   │   │   ├── blockpass.rb
│   │   │   ├── blockpass.rb.desugar-tree.exp
│   │   │   ├── case.rb
│   │   │   ├── class_def_kind.rb
│   │   │   ├── class_def_kind.rb.desugar-tree-raw.exp
│   │   │   ├── complex.rb
│   │   │   ├── complex.rb.desugar-tree.exp
│   │   │   ├── constant_error.rb
│   │   │   ├── constant_error.rb.symbol-table-raw.exp
│   │   │   ├── constant_reassignment.rb
│   │   │   ├── csend.rb
│   │   │   ├── csend.rb.desugar-tree.exp
│   │   │   ├── defined.rb
│   │   │   ├── defined.rb.desugar-tree.exp
│   │   │   ├── defs_not_self.rb
│   │   │   ├── defs_not_self.rb.desugar-tree.exp
│   │   │   ├── destructure.rb
│   │   │   ├── destructure.rb.flatten-tree.exp
│   │   │   ├── destructure.rb.symbol-table-raw.exp
│   │   │   ├── destructure_rest.rb
│   │   │   ├── dsymbol.rb
│   │   │   ├── dsymbol.rb.desugar-tree.exp
│   │   │   ├── empty_string_concatenation.rb
│   │   │   ├── ensure_without_rescue.rb
│   │   │   ├── ensure_without_rescue.rb.cfg.exp
│   │   │   ├── ensure_without_rescue.rb.desugar-tree.exp
│   │   │   ├── file.rb
│   │   │   ├── file.rb.desugar-tree.exp
│   │   │   ├── file.rb.parse-tree.exp
│   │   │   ├── for.rb
│   │   │   ├── for.rb.cfg.exp
│   │   │   ├── for.rb.desugar-tree.exp
│   │   │   ├── fuzz_bad_loc_dstring.rb
│   │   │   ├── fuzz_block_pass.rb
│   │   │   ├── fuzz_break_do_end.rb
│   │   │   ├── fuzz_include_self.rb
│   │   │   ├── hash.rb
│   │   │   ├── hash.rb.desugar-tree.exp
│   │   │   ├── hash_two_args.rb
│   │   │   ├── integers.rb
│   │   │   ├── integers.rb.desugar-tree.exp
│   │   │   ├── lambda.rb
│   │   │   ├── lambda.rb.desugar-tree.exp
│   │   │   ├── line_literal.rb
│   │   │   ├── line_literal.rb.desugar-tree.exp
│   │   │   ├── multi_assign.rb
│   │   │   ├── multi_assign.rb.desugar-tree.exp
│   │   │   ├── multi_assign.rb.symbol-table-raw.exp
│   │   │   ├── nthref.rb
│   │   │   ├── nthref.rb.desugar-tree-raw.exp
│   │   │   ├── nthref.rb.parse-tree.exp
│   │   │   ├── op_eq.rb
│   │   │   ├── op_eq.rb.desugar-tree.exp
│   │   │   ├── op_eq.rb.flatten-tree.exp
│   │   │   ├── opasgn.rb
│   │   │   ├── opasgn.rb.desugar-tree.exp
│   │   │   ├── ops.rb
│   │   │   ├── ops.rb.desugar-tree.exp
│   │   │   ├── range.rb
│   │   │   ├── range.rb.desugar-tree-raw.exp
│   │   │   ├── regexp.rb
│   │   │   ├── regexp.rb.desugar-tree.exp
│   │   │   ├── regexp_strict.rb
│   │   │   ├── sclass.rb
│   │   │   ├── sclass.rb.desugar-tree.exp
│   │   │   ├── sclass.rb.flatten-tree.exp
│   │   │   ├── sclass.rb.symbol-table-raw.exp
│   │   │   ├── sclass_inheritance.rb
│   │   │   ├── sclass_inheritance.rb.desugar-tree.exp
│   │   │   ├── sclass_inheritance.rb.flatten-tree.exp
│   │   │   ├── sclass_inheritance.rb.symbol-table-raw.exp
│   │   │   ├── splat.rb
│   │   │   ├── splat.rb.desugar-tree.exp
│   │   │   ├── star_in_block_arg.rb
│   │   │   ├── star_in_block_arg.rb.desugar-tree.exp
│   │   │   ├── strings.rb
│   │   │   ├── strings.rb.desugar-tree.exp
│   │   │   ├── top_level_const.rb
│   │   │   ├── top_level_const.rb.desugar-tree-raw.exp
│   │   │   ├── top_level_const.rb.desugar-tree.exp
│   │   │   ├── top_level_const.rb.symbol-table-raw.exp
│   │   │   ├── undef.rb
│   │   │   └── undef_strict.rb
│   │   ├── deviations
│   │   │   ├── keyword_method_names.rb
│   │   │   ├── keyword_method_names.rb.parse-tree.exp
│   │   │   ├── non_ruby_names.rb
│   │   │   ├── non_ruby_names.rb.desugar-tree.exp
│   │   │   ├── non_ruby_names.rb.flatten-tree.exp
│   │   │   ├── non_ruby_names.rb.parse-tree.exp
│   │   │   ├── non_ruby_names.rb.symbol-table-raw.exp
│   │   │   ├── superclass_implicit.rb
│   │   │   └── superclass_implicit.rb.symbol-table-raw.exp
│   │   ├── disabled/whitequark
│   │   │   ├── parse_dedenting_heredoc_13.rb
│   │   │   └── parse_encoding_.rb
│   │   ├── error_recovery_send_after_end.rb
│   │   ├── error_recovery_send_after_end.rb.parse-tree.exp
│   │   ├── infer
│   │   │   ├── absurd_false.rb
│   │   │   ├── aliases__1.rb
│   │   │   ├── aliases__2.rb
│   │   │   ├── align_child_as_parent.rb
│   │   │   ├── all_bug.rb
│   │   │   ├── arg_matching.rb
│   │   │   ├── arity.rb
│   │   │   ├── assign_self.rb
│   │   │   ├── assignment_call.rb
│   │   │   ├── attached_class.rb
│   │   │   ├── attached_class_factory_example.rb
│   │   │   ├── attached_class_params.rb
│   │   │   ├── attached_class_printing.rb
│   │   │   ├── attached_class_reopen.rb
│   │   │   ├── attached_class_self_new.rb
│   │   │   ├── bad_child_class.rb
│   │   │   ├── bad_suggest.rb
│   │   │   ├── block_arg.rb
│   │   │   ├── block_given.rb
│   │   │   ├── block_parameter.rb
│   │   │   ├── blocks2.rb
│   │   │   ├── blocks2.rb.cfg.exp
│   │   │   ├── body_not_always_run.rb
│   │   │   ├── boolean.rb
│   │   │   ├── case.rb
│   │   │   ├── case_all.rb
│   │   │   ├── case_exhaustive_union_type.rb
│   │   │   ├── case_subclass.rb
│   │   │   ├── case_untyped.rb
│   │   │   ├── case_when_any.rb
│   │   │   ├── casts.rb
│   │   │   ├── casts.rb.cfg.exp
│   │   │   ├── casts.rb.flatten-tree.exp
│   │   │   ├── class.rb
│   │   │   ├── class_new.rb
│   │   │   ├── compact.rb
│   │   │   ├── constructor_return.rb
│   │   │   ├── constructors.rb
│   │   │   ├── control_flow
│   │   │   │   ├── blank_p.rb
│   │   │   │   ├── bug_852.rb
│   │   │   │   ├── class_less_than.rb
│   │   │   │   ├── complex_implication_1.rb
│   │   │   │   ├── complex_implication_1.rb.cfg.exp
│   │   │   │   ├── complex_implications_2.rb
│   │   │   │   ├── complex_implications_2.rb.cfg.exp
│   │   │   │   ├── csend_and.rb
│   │   │   │   ├── dead_condition.rb
│   │   │   │   ├── dynamic.rb
│   │   │   │   ├── isa_module.rb
│   │   │   │   ├── nil_p.rb
│   │   │   │   ├── normalize_params.rb
│   │   │   │   ├── normalize_params.rb.cfg.exp
│   │   │   │   ├── present_p.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── simple.rb.cfg.exp
│   │   │   │   ├── singletons.rb
│   │   │   │   ├── truthiness.rb
│   │   │   │   └── truthiness_bug.rb
│   │   │   ├── control_flow_in_ensure.rb
│   │   │   ├── control_flow_in_rescue.rb
│   │   │   ├── crash_after_parse_errors.rb
│   │   │   ├── dead_after_return.rb
│   │   │   ├── dispatch_call_and.rb
│   │   │   ├── dropsubtypesof.rb
│   │   │   ├── exhaustiveness.rb
│   │   │   ├── false_dead_code.rb
│   │   │   ├── fields.rb
│   │   │   ├── fields.rb.cfg.exp
│   │   │   ├── flat_map.rb
│   │   │   ├── flatten.rb
│   │   │   ├── flatten_methods.rb
│   │   │   ├── flatten_methods.rb.flatten-tree.exp
│   │   │   ├── flatten_private.rb
│   │   │   ├── flatten_private.rb.flatten-tree.exp
│   │   │   ├── forward_proc.rb
│   │   │   ├── fuzz_dangling_type_parameter.rb
│   │   │   ├── fuzz_disallow_overloading.rb
│   │   │   ├── fuzz_nonexistant_method.rb
│   │   │   ├── fuzz_oneline_conditional.rb
│   │   │   ├── fuzz_static_init_loc.rb
│   │   │   ├── fuzz_toplevel_type_member.rb
│   │   │   ├── fuzz_tripleeq_sig_suggestion.rb
│   │   │   ├── fuzz_uninitialized_vars.rb
│   │   │   ├── generic_methods
│   │   │   │   ├── countraints_crosstalk.rb
│   │   │   │   ├── genericMethods1.rb
│   │   │   │   ├── genericMethods2.rb
│   │   │   │   ├── genericMethodsErrors.rb
│   │   │   │   ├── infer_bottom.rb
│   │   │   │   ├── two_bounds.rb
│   │   │   │   └── untyped_in_container.rb
│   │   │   ├── generic_type_template_arg.rb
│   │   │   ├── generics
│   │   │   │   ├── Boxes.rb
│   │   │   │   ├── TypeSyntax.rb
│   │   │   │   ├── adapt_child_to_parent.rb
│   │   │   │   ├── all_bottom.rb
│   │   │   │   ├── arity_mismatch.rb
│   │   │   │   ├── bad_bound_typed_false.rb
│   │   │   │   ├── bounds.rb
│   │   │   │   ├── bounds_super.rb
│   │   │   │   ├── class_less_than.rb
│   │   │   │   ├── duplicate_members.rb
│   │   │   │   ├── enumerable.rb
│   │   │   │   ├── fixed_ordering.rb
│   │   │   │   ├── generic_self_method.rb
│   │   │   │   ├── generics_class_of.rb
│   │   │   │   ├── glb.rb
│   │   │   │   ├── glb2.rb
│   │   │   │   ├── isa_with_type_member.rb
│   │   │   │   ├── lub.rb
│   │   │   │   ├── lub_lambda_param.rb
│   │   │   │   ├── lub_with_raw.rb
│   │   │   │   ├── others_type_members.rb
│   │   │   │   ├── self_params.rb
│   │   │   │   ├── specified.rb
│   │   │   │   ├── t_magic.rb
│   │   │   │   ├── tuple_decay.rb
│   │   │   │   ├── two_params.rb
│   │   │   │   ├── type_param_is_a.rb
│   │   │   │   ├── use_member_in_body.rb
│   │   │   │   ├── variance_methods.rb
│   │   │   │   ├── variance_neutral.rb
│   │   │   │   ├── variance_params.rb
│   │   │   │   ├── variance_user.rb
│   │   │   │   └── wrong_type_member.rb
│   │   │   ├── glb_corner_case.rb
│   │   │   ├── glb_generic_with_class.rb
│   │   │   ├── hashes.rb
│   │   │   ├── hashes.rb.cfg.exp
│   │   │   ├── huge_unions.rb
│   │   │   ├── huge_unions.rb.cfg.exp
│   │   │   ├── i1438.rb
│   │   │   ├── infer1.rb
│   │   │   ├── infer1.rb.cfg.exp
│   │   │   ├── infer1.rb.desugar-tree.exp
│   │   │   ├── infer1.rb.flatten-tree.exp
│   │   │   ├── infer1.rb.symbol-table-raw.exp
│   │   │   ├── infer_types.rb
│   │   │   ├── intrinsics_generics.rb
│   │   │   ├── isa_bug.rb
│   │   │   ├── isa_generic.rb
│   │   │   ├── isa_generic.rb.cfg.exp
│   │   │   ├── kwargs.rb
│   │   │   ├── kwargs_generics.rb
│   │   │   ├── leaking_exceptions.rb
│   │   │   ├── let.rb
│   │   │   ├── literal_is_array.rb
│   │   │   ├── loop_in_self_reassignments.rb
│   │   │   ├── loop_with_rescue_next.rb
│   │   │   ├── loops.rb
│   │   │   ├── loops.rb.cfg.exp
│   │   │   ├── lub_and_glb_corner_cases.rb
│   │   │   ├── lub_between_self_params.rb
│   │   │   ├── lub_generics.rb
│   │   │   ├── lub_tuples.rb
│   │   │   ├── lub_tuples.rb.symbol-table-raw.exp
│   │   │   ├── magic_dead.rb
│   │   │   ├── massign_return_rhs.rb
│   │   │   ├── massign_return_rhs.rb.desugar-tree.exp
│   │   │   ├── match_operator.rb
│   │   │   ├── meta_types.rb
│   │   │   ├── meta_types.rb.cfg.exp
│   │   │   ├── metatype_in_lub.rb
│   │   │   ├── metatype_instantiation.rb
│   │   │   ├── module_function_two.rb
│   │   │   ├── more_after_return.rb
│   │   │   ├── multi_assign.rb
│   │   │   ├── must.rb
│   │   │   ├── next_in_while.rb
│   │   │   ├── nil_noreturn.rb
│   │   │   ├── nilable_and.rb
│   │   │   ├── non_existent_method.rb
│   │   │   ├── non_forcing_is_a.rb
│   │   │   ├── non_forcing_is_a_dead.rb
│   │   │   ├── non_forcing_is_a_false.rb
│   │   │   ├── or_and_and_or.rb
│   │   │   ├── overload_block.rb
│   │   │   ├── overloads_test.rb
│   │   │   ├── pinned_control_flow.rb
│   │   │   ├── pinned_control_flow1.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc_args.rb
│   │   │   ├── product.rb
│   │   │   ├── rebind.rb
│   │   │   ├── rebind.rb.cfg.exp
│   │   │   ├── ref_eq.rb
│   │   │   ├── restargs.rb
│   │   │   ├── restargsbox.rb
│   │   │   ├── return_in_if.rb
│   │   │   ├── reveal_type.rb
│   │   │   ├── sealed_singleton_class.rb
│   │   │   ├── segfault_generic.rb
│   │   │   ├── self_type.rb
│   │   │   ├── self_type.rb.cfg.exp
│   │   │   ├── self_type_param.rb
│   │   │   ├── self_type_param_bounded.rb
│   │   │   ├── setters.rb
│   │   │   ├── shape_merge.rb
│   │   │   ├── show.rb
│   │   │   ├── sigil.rb
│   │   │   ├── sigil.rb.cfg.exp
│   │   │   ├── singleton_case_exhaustiveness.rb
│   │   │   ├── singleton_classes.rb
│   │   │   ├── singleton_enum_case.rb
│   │   │   ├── singleton_enum_eqeq.rb
│   │   │   ├── singleton_if_exhaustiveness.rb
│   │   │   ├── singleton_methods.rb
│   │   │   ├── singleton_methods.rb.cfg.exp
│   │   │   ├── singleton_non_final_enum.rb
│   │   │   ├── singleton_of_singleton.rb
│   │   │   ├── singleton_of_singleton.rb.cfg.exp
│   │   │   ├── splat.rb
│   │   │   ├── star_star_args.rb
│   │   │   ├── strict_dead.rb
│   │   │   ├── stubs_are_dynamic.rb
│   │   │   ├── subtype_symbol.rb
│   │   │   ├── t_all_type_member_bug.rb
│   │   │   ├── toplevel.rb
│   │   │   ├── toplevel.rb.flatten-tree.exp
│   │   │   ├── toplevel_var.rb
│   │   │   ├── transitive.rb
│   │   │   ├── transitive.rb.cfg.exp
│   │   │   ├── tuples.rb
│   │   │   ├── type_substraction.rb
│   │   │   ├── unwrap_locs.rb
│   │   │   ├── void_final.rb
│   │   │   ├── void_proc.rb
│   │   │   ├── yield_inside_block.rb
│   │   │   ├── yield_multiple.rb
│   │   │   ├── yield_multiple.rb.desugar-tree-raw.exp
│   │   │   ├── zsuper.rb
│   │   │   └── zsuper.rb.cfg.exp
│   │   ├── intrinsics
│   │   │   ├── shapes.rb
│   │   │   └── to_h.rb
│   │   ├── local_vars
│   │   │   ├── z_super_args.rb
│   │   │   └── z_super_args.rb.index-tree.exp
│   │   ├── lsp
│   │   │   ├── bad_field_edits.1.rbupdate
│   │   │   ├── bad_field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── bad_field_edits.rb
│   │   │   ├── bad_field_edits.rb.document-symbols.exp
│   │   │   ├── code_actions
│   │   │   │   ├── loop_type_change.A.rbedited
│   │   │   │   ├── loop_type_change.rb
│   │   │   │   ├── private.A.rbedited
│   │   │   │   ├── private.B.rbedited
│   │   │   │   ├── private.rb
│   │   │   │   ├── sig_missing__child.A.rbedited
│   │   │   │   ├── sig_missing__child.rb
│   │   │   │   ├── sig_missing__parent.A.rbedited
│   │   │   │   ├── sig_missing__parent.rb
│   │   │   │   ├── typo.A.rbedited
│   │   │   │   ├── typo.B.rbedited
│   │   │   │   ├── typo.C.rbedited
│   │   │   │   ├── typo.D.rbedited
│   │   │   │   └── typo.rb
│   │   │   ├── completion
│   │   │   │   ├── alias_method.rb
│   │   │   │   ├── angle_bracket_names.rb
│   │   │   │   ├── class_and_module.rb
│   │   │   │   ├── constants.A.rbedited
│   │   │   │   ├── constants.B.rbedited
│   │   │   │   ├── constants.C.rbedited
│   │   │   │   ├── constants.rb
│   │   │   │   ├── constants_aliases.rb
│   │   │   │   ├── constants_all_kinds.rb
│   │   │   │   ├── constants_existing.rb
│   │   │   │   ├── constants_magic.rb
│   │   │   │   ├── constants_root.rb
│   │   │   │   ├── constants_t.rb
│   │   │   │   ├── constants_type_members.rb
│   │   │   │   ├── constants_via_inherit.rb
│   │   │   │   ├── constants_via_mixins.rb
│   │   │   │   ├── constants_via_nesting.rb
│   │   │   │   ├── depth.rb
│   │   │   │   ├── duplicate_locals.rb
│   │   │   │   ├── grandchild.rb
│   │   │   │   ├── implicit_self.rb
│   │   │   │   ├── intersection.rb
│   │   │   │   ├── keywords.rb
│   │   │   │   ├── locals.rb
│   │   │   │   ├── locals_and_methods.rb
│   │   │   │   ├── no_parens.A.rbedited
│   │   │   │   ├── no_parens.rb
│   │   │   │   ├── non_prefix.rb
│   │   │   │   ├── overloads_test.A.rbedited
│   │   │   │   ├── overloads_test.B.rbedited
│   │   │   │   ├── overloads_test.rb
│   │   │   │   ├── private.rb
│   │   │   │   ├── redefinition.rb
│   │   │   │   ├── sig.A.rbedited
│   │   │   │   ├── sig.B.rbedited
│   │   │   │   ├── sig.rb
│   │   │   │   ├── sig_all_untyped.A.rbedited
│   │   │   │   ├── sig_all_untyped.rb
│   │   │   │   ├── sig_many_defs.A.rbedited
│   │   │   │   ├── sig_many_defs.rb
│   │   │   │   ├── sig_no_defs.A.rbedited
│   │   │   │   ├── sig_no_defs.rb
│   │   │   │   ├── sig_no_method.A.rbedited
│   │   │   │   ├── sig_no_method.B.rbedited
│   │   │   │   ├── sig_no_method.rb
│   │   │   │   ├── sig_redefinition__1.A.rbedited
│   │   │   │   ├── sig_redefinition__1.B.rbedited
│   │   │   │   ├── sig_redefinition__1.rb
│   │   │   │   ├── sig_redefinition__2.A.rbedited
│   │   │   │   ├── sig_redefinition__2.B.rbedited
│   │   │   │   ├── sig_redefinition__2.rb
│   │   │   │   ├── sig_root.A.rbedited
│   │   │   │   ├── sig_root.rb
│   │   │   │   ├── sig_singleton.A.rbedited
│   │   │   │   ├── sig_singleton.B.rbedited
│   │   │   │   ├── sig_singleton.rb
│   │   │   │   ├── sig_snippet.A.rbedited
│   │   │   │   ├── sig_snippet.B.rbedited
│   │   │   │   ├── sig_snippet.C.rbedited
│   │   │   │   ├── sig_snippet.D.rbedited
│   │   │   │   ├── sig_snippet.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── snippet_arity.A.rbedited
│   │   │   │   ├── snippet_arity.B.rbedited
│   │   │   │   ├── snippet_arity.C.rbedited
│   │   │   │   ├── snippet_arity.D.rbedited
│   │   │   │   ├── snippet_arity.rb
│   │   │   │   ├── snippet_block.A.rbedited
│   │   │   │   ├── snippet_block.B.rbedited
│   │   │   │   ├── snippet_block.C.rbedited
│   │   │   │   ├── snippet_block.D.rbedited
│   │   │   │   ├── snippet_block.rb
│   │   │   │   ├── snippet_block_arity.A.rbedited
│   │   │   │   ├── snippet_block_arity.B.rbedited
│   │   │   │   ├── snippet_block_arity.C.rbedited
│   │   │   │   ├── snippet_block_arity.rb
│   │   │   │   ├── snippet_default.A.rbedited
│   │   │   │   ├── snippet_default.rb
│   │   │   │   ├── snippet_keywords.A.rbedited
│   │   │   │   ├── snippet_keywords.B.rbedited
│   │   │   │   ├── snippet_keywords.C.rbedited
│   │   │   │   ├── snippet_keywords.D.rbedited
│   │   │   │   ├── snippet_keywords.E.rbedited
│   │   │   │   ├── snippet_keywords.rb
│   │   │   │   ├── snippet_repeated.A.rbedited
│   │   │   │   ├── snippet_repeated.B.rbedited
│   │   │   │   ├── snippet_repeated.C.rbedited
│   │   │   │   ├── snippet_repeated.D.rbedited
│   │   │   │   ├── snippet_repeated.rb
│   │   │   │   ├── snippet_types.A.rbedited
│   │   │   │   ├── snippet_types.B.rbedited
│   │   │   │   ├── snippet_types.rb
│   │   │   │   └── union.rb
│   │   │   ├── cvar__definition.rb
│   │   │   ├── cvar__usage.rb
│   │   │   ├── definition_untyped.rb
│   │   │   ├── definitions_and_usages.rb
│   │   │   ├── definitions_and_usages_2.rb
│   │   │   ├── definitions_and_usages_untyped__typed.rb
│   │   │   ├── definitions_and_usages_untyped__untyped.rb
│   │   │   ├── document_symbols.rb
│   │   │   ├── document_symbols.rb.document-symbols.exp
│   │   │   ├── document_symbols_crash.1.rbupdate
│   │   │   ├── document_symbols_crash.1.rbupdate.document-symbols.exp
│   │   │   ├── document_symbols_crash.rb
│   │   │   ├── document_symbols_crash.rb.document-symbols.exp
│   │   │   ├── errors_clear_after_fixing.1.rbupdate
│   │   │   ├── errors_clear_after_fixing.rb
│   │   │   ├── fast_path
│   │   │   │   ├── class_add_member.1.rbupdate
│   │   │   │   ├── class_add_member.rb
│   │   │   │   ├── class_change_include_multifile__included.rb
│   │   │   │   ├── class_change_include_multifile__using.1.rbupdate
│   │   │   │   ├── class_change_include_multifile__using.rb
│   │   │   │   ├── class_change_superclass.1.rbupdate
│   │   │   │   ├── class_change_superclass.rb
│   │   │   │   ├── class_change_superclass_multifile__child.1.rbupdate
│   │   │   │   ├── class_change_superclass_multifile__child.rb
│   │   │   │   ├── class_change_superclass_multifile__super.rb
│   │   │   │   ├── class_remove_member.1.rbupdate
│   │   │   │   ├── class_remove_member.rb
│   │   │   │   ├── method_add_argument.1.rbupdate
│   │   │   │   ├── method_add_argument.rb
│   │   │   │   ├── method_add_keyword_arg.1.rbupdate
│   │   │   │   ├── method_add_keyword_arg.rb
│   │   │   │   ├── method_add_sig.1.rbupdate
│   │   │   │   ├── method_add_sig.rb
│   │   │   │   ├── method_change_argument_kind.1.rbupdate
│   │   │   │   ├── method_change_argument_kind.rb
│   │   │   │   ├── method_change_argument_type__def.1.rbupdate
│   │   │   │   ├── method_change_argument_type__def.rb
│   │   │   │   ├── method_change_argument_type__usage.1.rbupdate
│   │   │   │   ├── method_change_argument_type__usage.rb
│   │   │   │   ├── method_change_kw_arg_name.1.rbupdate
│   │   │   │   ├── method_change_kw_arg_name.rb
│   │   │   │   ├── method_change_kw_arg_type.1.rbupdate
│   │   │   │   ├── method_change_kw_arg_type.rb
│   │   │   │   ├── method_change_return_type__def.1.rbupdate
│   │   │   │   ├── method_change_return_type__def.rb
│   │   │   │   ├── method_change_return_type__usage.1.rbupdate
│   │   │   │   ├── method_change_return_type__usage.rb
│   │   │   │   ├── method_rename_argument.1.rbupdate
│   │   │   │   ├── method_rename_argument.rb
│   │   │   │   ├── method_signature_update.1.rbupdate
│   │   │   │   ├── method_signature_update.rb
│   │   │   │   ├── method_signature_update_generics__def.1.rbupdate
│   │   │   │   ├── method_signature_update_generics__def.rb
│   │   │   │   ├── method_signature_update_generics__usage.rb
│   │   │   │   ├── method_signature_update_static__def.1.rbupdate
│   │   │   │   ├── method_signature_update_static__def.rb
│   │   │   │   ├── method_signature_update_static__usage.rb
│   │   │   │   ├── parse_errors.1.rbupdate
│   │   │   │   ├── parse_errors.2.rbupdate
│   │   │   │   ├── parse_errors.3.rbupdate
│   │   │   │   ├── parse_errors.rb
│   │   │   │   ├── string_literal_change.1.rbupdate
│   │   │   │   ├── string_literal_change.rb
│   │   │   │   ├── undefined_constant.1.rbupdate
│   │   │   │   └── undefined_constant.rb
│   │   │   ├── field_edits.1.rbupdate
│   │   │   ├── field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── field_edits.2.rbupdate
│   │   │   ├── field_edits.2.rbupdate.document-symbols.exp
│   │   │   ├── field_edits.rb
│   │   │   ├── field_edits.rb.document-symbols.exp
│   │   │   ├── genericMethods.rb
│   │   │   ├── go_to_type_definition.rb
│   │   │   ├── go_to_type_definition_applied.rb
│   │   │   ├── go_to_type_definition_false.rb
│   │   │   ├── good_field_edits.1.rbupdate
│   │   │   ├── good_field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── good_field_edits.rb
│   │   │   ├── good_field_edits.rb.document-symbols.exp
│   │   │   ├── highlight.rb
│   │   │   ├── highlight_simple.rb
│   │   │   ├── hover.rb
│   │   │   ├── hover_abstract.rb
│   │   │   ├── hover_ampersand_operations.rb
│   │   │   ├── hover_ancestors.rb
│   │   │   ├── hover_conditional_type_narrowing.rb
│   │   │   ├── hover_constants.rb
│   │   │   ├── hover_generics.rb
│   │   │   ├── hover_method.rb
│   │   │   ├── hover_method_for_building_arrays_and_hashes.rb
│   │   │   ├── hover_method_includes_defs.rb
│   │   │   ├── hover_method_not_found.rb
│   │   │   ├── hover_operator_overload.rb
│   │   │   ├── hover_override.rb
│   │   │   ├── hover_proc_void.rb
│   │   │   ├── hover_untyped.rb
│   │   │   ├── hover_untyped_keyword_arg.rb
│   │   │   ├── hover_untyped_proc_arg.rb
│   │   │   ├── include_and_extend.rb
│   │   │   ├── ivars.rb
│   │   │   ├── missing_typed_sigil.A.rbedited
│   │   │   ├── missing_typed_sigil.rb
│   │   │   ├── no_stdlib.rb
│   │   │   ├── sig_deletion.1.rbupdate
│   │   │   ├── sig_deletion.rb
│   │   │   ├── struct_fuzz.rb
│   │   │   ├── symbol_query_filter__helper.rb
│   │   │   ├── symbol_query_filter__main.rb
│   │   │   ├── type_aliases.rb
│   │   │   ├── untyped_field_reference__1.rb
│   │   │   ├── untyped_field_reference__2.rb
│   │   │   ├── workspace_symbols_boundaries.rb
│   │   │   ├── workspace_symbols_fullname.rb
│   │   │   ├── workspace_symbols_minitest.rb
│   │   │   ├── workspace_symbols_minitest_scope.rb
│   │   │   ├── workspace_symbols_multiply_defined.rb
│   │   │   ├── workspace_symbols_multiply_defined_2.rb
│   │   │   ├── workspace_symbols_namespaces.rb
│   │   │   ├── workspace_symbols_shortname.rb
│   │   │   ├── workspace_symbols_sparse.rb
│   │   │   └── workspace_symbols_stdlib.rb
│   │   ├── namer
│   │   │   ├── alias_cross_file.flatten-tree.exp
│   │   │   ├── alias_cross_file.symbol-table-raw.exp
│   │   │   ├── alias_cross_file__01.rb
│   │   │   ├── alias_cross_file__02.rb
│   │   │   ├── alias_method.rb
│   │   │   ├── alias_method.rb.symbol-table-raw.exp
│   │   │   ├── alias_method_redefinition.rb
│   │   │   ├── ancestors.rb
│   │   │   ├── ancestors.rb.flatten-tree.exp
│   │   │   ├── ancestors.rb.symbol-table-raw.exp
│   │   │   ├── arguments.rb
│   │   │   ├── arguments.rb.desugar-tree-raw.exp
│   │   │   ├── arguments.rb.desugar-tree.exp
│   │   │   ├── arguments.rb.flatten-tree-raw.exp
│   │   │   ├── arguments.rb.flatten-tree.exp
│   │   │   ├── arguments.rb.symbol-table-raw.exp
│   │   │   ├── array_sum.rb
│   │   │   ├── block_in_class.rb
│   │   │   ├── blocks_in_reopened_class.rb
│   │   │   ├── bug_1425.rb
│   │   │   ├── circular_mixin.rb
│   │   │   ├── circular_mixin.rb.symbol-table-raw.exp
│   │   │   ├── class_and_alias.rb
│   │   │   ├── class_and_alias.rb.flatten-tree.exp
│   │   │   ├── class_and_alias.rb.symbol-table-raw.exp
│   │   │   ├── class_module.rb
│   │   │   ├── conflicting_names.rb
│   │   │   ├── conflicting_names.rb.flatten-tree.exp
│   │   │   ├── conflicting_names.rb.symbol-table-raw.exp
│   │   │   ├── constant_in_block.rb
│   │   │   ├── constant_in_block.rb.symbol-table-raw.exp
│   │   │   ├── constant_redefinition
│   │   │   │   ├── class_then_constant.rb
│   │   │   │   ├── class_then_constant_then_reopen.rb
│   │   │   │   ├── constant_then_class.rb
│   │   │   │   ├── constant_then_module.rb
│   │   │   │   ├── module_then_constant.rb
│   │   │   │   └── module_then_constant_then_reopen.rb
│   │   │   ├── constant_types.rb
│   │   │   ├── constant_types.rb.symbol-table-raw.exp
│   │   │   ├── constants.rb
│   │   │   ├── constants.rb.flatten-tree.exp
│   │   │   ├── constants.rb.symbol-table-raw.exp
│   │   │   ├── defs_in_blocks.rb
│   │   │   ├── defs_in_blocks.rb.flatten-tree.exp
│   │   │   ├── defs_in_blocks.rb.symbol-table-raw.exp
│   │   │   ├── docs_example_1.rb
│   │   │   ├── docs_example_2.rb
│   │   │   ├── duplicate_scope.rb
│   │   │   ├── dynamic_constant.rb
│   │   │   ├── dynamic_method_with_class.rb
│   │   │   ├── dynamic_method_with_class.rb.symbol-table-raw.exp
│   │   │   ├── extend.rb
│   │   │   ├── extend.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_class_in_field.rb
│   │   │   ├── fuzz_dynamic_constant.rb
│   │   │   ├── fuzz_ivar_constant_scope.rb
│   │   │   ├── fuzz_repeated_kwarg.rb
│   │   │   ├── fuzz_shared_singletons.rb
│   │   │   ├── fuzz_type_member.rb
│   │   │   ├── fuzz_type_template_overwrite.rb
│   │   │   ├── fuzz_type_template_overwrite.rb.symbol-table-raw.exp
│   │   │   ├── gvar.rb
│   │   │   ├── gvar.rb.flatten-tree.exp
│   │   │   ├── gvar.rb.symbol-table-raw.exp
│   │   │   ├── include_noarg.rb
│   │   │   ├── locals.rb
│   │   │   ├── locals.rb.flatten-tree.exp
│   │   │   ├── locals.rb.symbol-table-raw.exp
│   │   │   ├── method_redef.rb
│   │   │   ├── module_function.rb
│   │   │   ├── module_function.rb.cfg.exp
│   │   │   ├── module_function.rb.symbol-table-raw.exp
│   │   │   ├── multiple_stubs.rb
│   │   │   ├── multiple_stubs.rb.flatten-tree.exp
│   │   │   ├── multiple_stubs.rb.symbol-table-raw.exp
│   │   │   ├── nested_class.rb
│   │   │   ├── nested_class.rb.flatten-tree.exp
│   │   │   ├── nested_static_field.rb
│   │   │   ├── net_imap.rb
│   │   │   ├── next_break.rb
│   │   │   ├── next_break.rb.flatten-tree.exp
│   │   │   ├── payload_name.rb
│   │   │   ├── redefine.rb
│   │   │   ├── redefines_object.rb
│   │   │   ├── redefines_object.rb.cfg.exp
│   │   │   ├── redefinition_method.rb
│   │   │   ├── redefinition_method.rb.flatten-tree.exp
│   │   │   ├── root_private.rb
│   │   │   ├── root_private.rb.symbol-table-raw.exp
│   │   │   ├── self_constant.rb
│   │   │   ├── self_disallowed.rb
│   │   │   ├── simple.rb
│   │   │   ├── simple.rb.desugar-tree.exp
│   │   │   ├── simple.rb.flatten-tree.exp
│   │   │   ├── simple.rb.parse-tree.exp
│   │   │   ├── simple.rb.symbol-table-raw.exp
│   │   │   ├── singleton_class.rb
│   │   │   ├── singleton_class.rb.symbol-table-raw.exp
│   │   │   ├── super.rb
│   │   │   ├── superclass_redefinition.rb
│   │   │   ├── superclass_redefinition.rb.symbol-table-raw.exp
│   │   │   ├── type_alias.rb
│   │   │   ├── type_alias.rb.symbol-table-raw.exp
│   │   │   ├── type_member_redefs__1.rb
│   │   │   ├── type_member_redefs__2.rb
│   │   │   ├── visibility.rb
│   │   │   ├── visibility.rb.symbol-table-raw.exp
│   │   │   ├── yield.rb
│   │   │   ├── yield.rb.cfg.exp
│   │   │   ├── yield.rb.flatten-tree.exp
│   │   │   └── yield.rb.symbol-table-raw.exp
│   │   ├── parser
│   │   │   ├── and_and_bug.rb
│   │   │   ├── and_and_bug.rb.parse-tree.exp
│   │   │   ├── anon_blockarg.rb
│   │   │   ├── chained_sends_lots.rb
│   │   │   ├── chained_sends_lots.rb.desugar-tree.exp
│   │   │   ├── compare_overload_parse_error.rb
│   │   │   ├── complement_literal.rb
│   │   │   ├── complement_literal.rb.desugar-tree.exp
│   │   │   ├── complement_literal.rb.parse-tree.exp
│   │   │   ├── empty.rb
│   │   │   ├── empty.rb.parse-tree.exp
│   │   │   ├── encoding.rb
│   │   │   ├── error_recovery_assign.rb
│   │   │   ├── error_recovery_assign.rb.parse-tree.exp
│   │   │   ├── error_recovery_const_case.rb
│   │   │   ├── error_recovery_const_case.rb.parse-tree.exp
│   │   │   ├── error_recovery_constant_only_scope.rb
│   │   │   ├── error_recovery_constant_only_scope.rb.parse-tree.exp
│   │   │   ├── error_recovery_if_no_end.rb
│   │   │   ├── error_recovery_if_no_end.rb.parse-tree.exp
│   │   │   ├── error_recovery_missing_fun.rb
│   │   │   ├── error_recovery_missing_fun.rb.parse-tree.exp
│   │   │   ├── error_recovery_multiple_stmts.rb
│   │   │   ├── error_recovery_multiple_stmts.rb.parse-tree.exp
│   │   │   ├── fuzz_def_begin.rb
│   │   │   ├── fuzz_ivar.rb
│   │   │   ├── fuzz_rational.rb
│   │   │   ├── heredoc_chomp.rb
│   │   │   ├── heredoc_chomp.rb.desugar-tree.exp
│   │   │   ├── index_assign.rb
│   │   │   ├── index_assign.rb.parse-tree.exp
│   │   │   ├── invalid_fatal.rb
│   │   │   ├── invalid_fatal.rb.parse-tree.exp
│   │   │   ├── invalid_syntax_error.rb
│   │   │   ├── invalid_syntax_error.rb.parse-tree.exp
│   │   │   ├── invalid_trailing_in_number.rb
│   │   │   ├── invalid_trailing_in_number.rb.parse-tree.exp
│   │   │   ├── long_string.rb
│   │   │   ├── long_string.rb.parse-tree.exp
│   │   │   ├── misc.rb
│   │   │   ├── misc.rb.desugar-tree.exp
│   │   │   ├── misc.rb.parse-tree.exp
│   │   │   ├── offset0.rb
│   │   │   ├── offset0.rb.parse-tree.exp
│   │   │   ├── ruby_25.rb
│   │   │   ├── ruby_25.rb.parse-tree.exp
│   │   │   ├── trailing_comas.rb
│   │   │   ├── trailing_comas.rb.parse-tree-json.exp
│   │   │   ├── typed_ignore.rb
│   │   │   ├── unary_num.rb
│   │   │   └── unary_num.rb.parse-tree.exp
│   │   ├── perf
│   │   │   ├── enum_canBeTruthy_regression.rb
│   │   │   └── sealed_registration_perf.rb
│   │   ├── proc_variance.rb
│   │   ├── rbi
│   │   │   ├── argf.rb
│   │   │   ├── array.rb
│   │   │   ├── basicobject_instance_eval.rb
│   │   │   ├── basicobject_instance_exec.rb
│   │   │   ├── bigdecimal.rb
│   │   │   ├── class.rb
│   │   │   ├── date.rb
│   │   │   ├── each_with_object.rb
│   │   │   ├── enumerable.rb
│   │   │   ├── enumerable_flat_map.rb
│   │   │   ├── etc.rb
│   │   │   ├── exception.rb
│   │   │   ├── falseclass.rb
│   │   │   ├── hash.rb
│   │   │   ├── int_float.rb
│   │   │   ├── io.rb
│   │   │   ├── json.rb
│   │   │   ├── kernel.rb
│   │   │   ├── kernel_array.rb
│   │   │   ├── kernel_class.rb
│   │   │   ├── lazy_enumerator.rb
│   │   │   ├── module.rb
│   │   │   ├── object.rb
│   │   │   ├── object_constant.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc_sig_strong.rb
│   │   │   ├── process.rb
│   │   │   ├── random.rb
│   │   │   ├── range.rb
│   │   │   ├── regexp.rb
│   │   │   ├── ruby_typer.rb
│   │   │   ├── sorbet.rb
│   │   │   ├── string.rb
│   │   │   ├── t.rb
│   │   │   ├── tempfile.rb
│   │   │   ├── time.rb
│   │   │   ├── to_sym.rb
│   │   │   ├── trueclass.rb
│   │   │   ├── uri.rb
│   │   │   ├── with_without__1.rbi
│   │   │   ├── with_without__2.rbi
│   │   │   └── with_without__3.rb
│   │   ├── resolver
│   │   │   ├── abstract.rb
│   │   │   ├── abstract_out_of_order.rb
│   │   │   ├── abstract_override.rb
│   │   │   ├── abstract_types.rb
│   │   │   ├── abstract_validation.rb
│   │   │   ├── alias.rb
│   │   │   ├── alias.rb.symbol-table-raw.exp
│   │   │   ├── alias_define_method__01.rb
│   │   │   ├── alias_define_method__02.rb
│   │   │   ├── alias_without_alias.rb
│   │   │   ├── ancestor_scope.rb
│   │   │   ├── ancestor_scope.rb.flatten-tree.exp
│   │   │   ├── bad_alias.rb
│   │   │   ├── bad_final_sig.rb
│   │   │   ├── bad_hash.rb
│   │   │   ├── bad_param_ordering.rb
│   │   │   ├── bad_sealed_class__1.rb
│   │   │   ├── bad_sealed_class__2.rb
│   │   │   ├── bad_sealed_class_absurd__1.rb
│   │   │   ├── bad_sealed_class_absurd__2.rb
│   │   │   ├── bad_sealed_module__1.rb
│   │   │   ├── bad_sealed_module__2.rb
│   │   │   ├── bad_synthesize.rb
│   │   │   ├── bare_generics.rb
│   │   │   ├── bare_generics_strict.rb
│   │   │   ├── bare_stdlib_generics.rb
│   │   │   ├── bind.rb
│   │   │   ├── bind_class_of.rb
│   │   │   ├── bind_class_of.rb.symbol-table-raw.exp
│   │   │   ├── bool_alias.rb
│   │   │   ├── cbase.rb
│   │   │   ├── cbase.rb.symbol-table-raw.exp
│   │   │   ├── choose_sig.rb
│   │   │   ├── circle_of_itself.rb
│   │   │   ├── class_instance_vars.rb
│   │   │   ├── class_instance_vars.rb.flatten-tree.exp
│   │   │   ├── class_instance_vars.rb.symbol-table-raw.exp
│   │   │   ├── class_ivar.rb
│   │   │   ├── constant_in_typealias.rb
│   │   │   ├── crash.rb
│   │   │   ├── default_arg_in_block.rb
│   │   │   ├── defined.rb
│   │   │   ├── empty_sealed.rb
│   │   │   ├── enumerable_strict.rb
│   │   │   ├── field.rb
│   │   │   ├── field.rb.flatten-tree-raw.exp
│   │   │   ├── field_across_file__01.rb
│   │   │   ├── field_across_file__02.rb
│   │   │   ├── final_method.rb
│   │   │   ├── final_module.rb
│   │   │   ├── flatten.rb
│   │   │   ├── flatten.rb.flatten-tree.exp
│   │   │   ├── flatten.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_alias_to_type_alias.rb
│   │   │   ├── fuzz_ancester.rb
│   │   │   ├── fuzz_class_of_static_field.rb
│   │   │   ├── fuzz_empty_type_alias.rb
│   │   │   ├── fuzz_include_infinite_typealias.rb
│   │   │   ├── fuzz_include_type_alias.rb
│   │   │   ├── fuzz_infinite_type.rb
│   │   │   ├── fuzz_mixes_in_class_methods.rb
│   │   │   ├── fuzz_multiple_sigs.rb
│   │   │   ├── fuzz_sig_generic_field.rb
│   │   │   ├── fuzz_sig_parsing.rb
│   │   │   ├── fuzz_sig_weird.rb
│   │   │   ├── fuzz_suggest.rb
│   │   │   ├── fuzz_top_level_abstract.rb
│   │   │   ├── fuzz_type_member_forget.rb
│   │   │   ├── fuzz_type_member_scope.rb
│   │   │   ├── fuzz_type_member_scope.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_wrong_fun_print.rb
│   │   │   ├── generic_class_alias.rb
│   │   │   ├── implementations.rb
│   │   │   ├── infinite.rb
│   │   │   ├── inherit_alias.rb
│   │   │   ├── inherit_alias.rb.symbol-table-raw.exp
│   │   │   ├── invalid_alias.rb
│   │   │   ├── invalid_alias.rb.symbol-table-raw.exp
│   │   │   ├── lazy_field.rb
│   │   │   ├── let_errors.rb
│   │   │   ├── let_errors.rb.symbol-table-raw.exp
│   │   │   ├── let_errors_false.rb
│   │   │   ├── let_errors_nilable.rb
│   │   │   ├── let_var.rb
│   │   │   ├── let_var.rb.symbol-table-raw.exp
│   │   │   ├── linearization
│   │   │   │   ├── constant_resolution_before_linearization.rb
│   │   │   │   ├── includes_class.rb
│   │   │   │   ├── includes_class.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization1.rb
│   │   │   │   ├── linearization1.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization2.rb
│   │   │   │   ├── linearization2.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization3.rb
│   │   │   │   ├── linearization3.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization4.rb
│   │   │   │   ├── linearization4.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization4a.rb
│   │   │   │   ├── linearization4a.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization5.rb
│   │   │   │   ├── linearization5.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization6.rb
│   │   │   │   └── linearization6.rb.symbol-table-raw.exp
│   │   │   ├── missing_helpers_abstract.rb
│   │   │   ├── missing_helpers_interface.rb
│   │   │   ├── missing_type_combinator_args.rb
│   │   │   ├── missing_type_combinator_args.rb.flatten-tree.exp
│   │   │   ├── mixes_in_class_methods.rb
│   │   │   ├── mixes_in_class_methods.rb.symbol-table-raw.exp
│   │   │   ├── mixes_in_class_methods_twice.rb
│   │   │   ├── nested_sealed.rb
│   │   │   ├── no_runtime_sig.rb
│   │   │   ├── non_builder_sig.rb
│   │   │   ├── object_include_stub.rb
│   │   │   ├── optional_block.rb
│   │   │   ├── optional_cyclic.rb
│   │   │   ├── optional_nested.rb
│   │   │   ├── optional_nil.rb
│   │   │   ├── optional_nil.rb.flatten-tree.exp
│   │   │   ├── optional_nil.rb.name-tree.exp
│   │   │   ├── overloads_test.rb
│   │   │   ├── override_shapes.rb
│   │   │   ├── overrides.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc.rb.symbol-table-raw.exp
│   │   │   ├── rbi_final_no_sig__1.rb
│   │   │   ├── rbi_final_no_sig__2.rb
│   │   │   ├── rbi_final_re_sig__1.rb
│   │   │   ├── rbi_final_re_sig__2.rb
│   │   │   ├── recover_from_bad_superclass.rb
│   │   │   ├── recover_from_bad_superclass.rb.symbol-table-raw.exp
│   │   │   ├── redefinition_of_subclass_type_member.rb
│   │   │   ├── resolution_order.rb
│   │   │   ├── resolution_order.rb.symbol-table-raw.exp
│   │   │   ├── resolution_scoping.rb
│   │   │   ├── resolution_scoping.rb.symbol-table-raw.exp
│   │   │   ├── resolve_errors.rb
│   │   │   ├── resolve_through_alias.rb
│   │   │   ├── resolve_through_alias.rb.symbol-table-raw.exp
│   │   │   ├── resolve_through_type_alias.rb
│   │   │   ├── resolve_tree_printing.rb
│   │   │   ├── resolve_tree_printing.rb.flatten-tree-raw.exp
│   │   │   ├── resolve_via_ancestors
│   │   │   │   ├── mixin.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── superclass_three_pass.rb
│   │   │   │   └── two_mixins.rb
│   │   │   ├── reveal_type.rb
│   │   │   ├── sealed_aliases.rb
│   │   │   ├── sealed_class.rb
│   │   │   ├── sealed_concrete_parent_class.rb
│   │   │   ├── sealed_module.rb
│   │   │   ├── sealed_stdlib.rb
│   │   │   ├── sealed_with_rbi__1.rb
│   │   │   ├── sealed_with_rbi__2.rb
│   │   │   ├── sealed_with_rbi__3.rbi
│   │   │   ├── self.rb
│   │   │   ├── self.rb.symbol-table-raw.exp
│   │   │   ├── self_ancestor.rb
│   │   │   ├── sig_bad.rb
│   │   │   ├── sig_bad.rb.symbol-table-raw.exp
│   │   │   ├── sig_compat.rb
│   │   │   ├── sig_compat.rb.symbol-table-raw.exp
│   │   │   ├── sig_generated.rb
│   │   │   ├── sig_good.rb
│   │   │   ├── sig_good.rb.symbol-table-raw.exp
│   │   │   ├── sig_in_block.rb
│   │   │   ├── sig_misc.rb
│   │   │   ├── sig_misc.rb.symbol-table-raw.exp
│   │   │   ├── sig_on_failure.rb
│   │   │   ├── sig_returns_nil.rb
│   │   │   ├── sig_void.rb
│   │   │   ├── simple.rb
│   │   │   ├── simple.rb.flatten-tree.exp
│   │   │   ├── simple.rb.symbol-table-raw.exp
│   │   │   ├── strict.rb
│   │   │   ├── stub_assign.rb
│   │   │   ├── stub_missing_class_alias.rb
│   │   │   ├── stub_missing_class_alias.rb.symbol-table-raw.exp
│   │   │   ├── stubs_typed_untyped.flatten-tree.exp
│   │   │   ├── stubs_typed_untyped__1.rb
│   │   │   ├── stubs_typed_untyped__2.rb
│   │   │   ├── t_struct_subclass.rb
│   │   │   ├── top_level.rb
│   │   │   ├── top_level_abstract_typed_true.rb
│   │   │   ├── top_level_include.rb
│   │   │   ├── top_level_sig.rb
│   │   │   ├── type_alias.rb
│   │   │   ├── type_alias_order.rb
│   │   │   ├── type_arguments.rb
│   │   │   ├── type_arguments.rb.symbol-table-raw.exp
│   │   │   ├── type_member_bad_parent.rb
│   │   │   ├── type_member_constant_assignment.rb
│   │   │   ├── type_member_constant_assignment.rb.symbol-table-raw.exp
│   │   │   ├── type_member_cycle.rb
│   │   │   ├── type_member_fixed_order.rb
│   │   │   ├── type_member_in_method.rb
│   │   │   ├── type_member_missing.rb
│   │   │   ├── type_member_missing.rb.symbol-table-raw.exp
│   │   │   ├── type_member_missing_then_use.rb
│   │   │   ├── type_member_out_of_order.rb
│   │   │   ├── type_member_singleton_members.rb
│   │   │   ├── type_member_singleton_members.rb.symbol-table-raw.exp
│   │   │   ├── type_members.rb
│   │   │   ├── unnamed_proc_arguments.rb
│   │   │   ├── unsigged_default.rb
│   │   │   ├── untyped_generics.rb
│   │   │   └── void.rb
│   │   ├── rewriter
│   │   │   ├── attr.rb
│   │   │   ├── attr.rb.flatten-tree.exp
│   │   │   ├── attr.rb.symbol-table-raw.exp
│   │   │   ├── attr_bad_string.rb
│   │   │   ├── attr_multi.rb
│   │   │   ├── attr_multi.rb.rewrite-tree.exp
│   │   │   ├── attr_multi.rb.symbol-table-raw.exp
│   │   │   ├── attr_parameters.rb
│   │   │   ├── attr_strict.rb
│   │   │   ├── chalk_odm_document.rb
│   │   │   ├── chalk_odm_document.rb.rewrite-tree.exp
│   │   │   ├── class_new.rb
│   │   │   ├── class_new.rb.rewrite-tree.exp
│   │   │   ├── coerce_in_prop.rb
│   │   │   ├── command.rb
│   │   │   ├── command.rb.rewrite-tree.exp
│   │   │   ├── default_args.rb
│   │   │   ├── default_args.rb.rewrite-tree.exp
│   │   │   ├── default_args_edge.rb
│   │   │   ├── default_args_malformed_sig.rb
│   │   │   ├── dsl_builder.rb
│   │   │   ├── dsl_builder.rb.rewrite-tree.exp
│   │   │   ├── empty_until.rb
│   │   │   ├── flatfile_dsl.rb
│   │   │   ├── flatfile_dsl.rb.rewrite-tree.exp
│   │   │   ├── flatten_module_private_class_method.rb
│   │   │   ├── flatten_module_private_class_method.rb.symbol-table-raw.exp
│   │   │   ├── flatten_nested.rb
│   │   │   ├── flatten_nested.rb.flatten-tree.exp
│   │   │   ├── flatten_nested_sclass.rb
│   │   │   ├── flatten_nested_sclass.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_attr_bare.rb
│   │   │   ├── fuzz_attr_no_args.rb
│   │   │   ├── fuzz_chalk_bad_type.rb
│   │   │   ├── fuzz_dup_type_not_constant.rb
│   │   │   ├── fuzz_encrypted_prop.rb
│   │   │   ├── fuzz_optinal_crash.rb
│   │   │   ├── fuzz_prop_weird_member.rb
│   │   │   ├── fuzz_qualified_lhs.rb
│   │   │   ├── generic_module_function.rb
│   │   │   ├── initializer.rb
│   │   │   ├── interface_wrapper.rb
│   │   │   ├── interface_wrapper.rb.rewrite-tree.exp
│   │   │   ├── minitest.rb
│   │   │   ├── minitest.rb.rewrite-tree.exp
│   │   │   ├── minitest_tables.rb
│   │   │   ├── minitest_tables.rb.rewrite-tree.exp
│   │   │   ├── nesting.rb
│   │   │   ├── not_prop.rb
│   │   │   ├── not_prop.rb.rewrite-tree.exp
│   │   │   ├── not_prop.rb.symbol-table.exp
│   │   │   ├── prop.rb
│   │   │   ├── prop.rb.rewrite-tree-raw.exp
│   │   │   ├── prop.rb.rewrite-tree.exp
│   │   │   ├── prop.rb.symbol-table-raw.exp
│   │   │   ├── prop_computed_by.rb
│   │   │   ├── prop_computed_by.rb.rewrite-tree.exp
│   │   │   ├── prop_default.rb
│   │   │   ├── prop_enum.rb
│   │   │   ├── prop_get_logic.rb
│   │   │   ├── prop_in_module.rb
│   │   │   ├── prop_in_module.rb.rewrite-tree.exp
│   │   │   ├── prop_missing.rb
│   │   │   ├── prop_missing.rb.rewrite-tree.exp
│   │   │   ├── prop_mutator.rb
│   │   │   ├── prop_proc_regression.rb
│   │   │   ├── prop_prohibit_shapes_and_tuples.rb
│   │   │   ├── prop_skipped.rb
│   │   │   ├── protobuf_descriptor_pool.rb
│   │   │   ├── protobuf_descriptor_pool.rb.rewrite-tree.exp
│   │   │   ├── rails
│   │   │   │   ├── cattr_accessor.rb
│   │   │   │   ├── cattr_accessor.rb.rewrite-tree.exp
│   │   │   │   ├── cattr_reader.rb
│   │   │   │   ├── cattr_reader.rb.rewrite-tree.exp
│   │   │   │   ├── cattr_writer.rb
│   │   │   │   ├── cattr_writer.rb.rewrite-tree.exp
│   │   │   │   ├── class_attribute.rb
│   │   │   │   ├── class_attribute.rb.rewrite-tree.exp
│   │   │   │   ├── delegate.rb
│   │   │   │   ├── delegate.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_accessor.rb
│   │   │   │   ├── mattr_accessor.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_reader.rb
│   │   │   │   ├── mattr_reader.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_writer.rb
│   │   │   │   ├── mattr_writer.rb.rewrite-tree.exp
│   │   │   │   └── migration.rb
│   │   │   ├── stringy_struct.rb
│   │   │   ├── struct.rb
│   │   │   ├── struct.rb.rewrite-tree.exp
│   │   │   ├── struct.rb.symbol-table-raw.exp
│   │   │   ├── struct_initialize.rb
│   │   │   ├── struct_self.rb
│   │   │   ├── t_enum.rb
│   │   │   ├── t_enum_alias.rb
│   │   │   ├── t_enum_all.rb
│   │   │   ├── t_enum_exhaustiveness.rb
│   │   │   ├── t_enum_override_initialize.rb
│   │   │   ├── t_enum_snapshot.rb
│   │   │   ├── t_enum_snapshot.rb.rewrite-tree.exp
│   │   │   ├── t_enum_type_syntax.rb
│   │   │   └── t_struct
│   │   │       ├── custom_initialize.rb
│   │   │       ├── default.rb
│   │   │       ├── default.rb.rewrite-tree.exp
│   │   │       ├── default_bad.rb
│   │   │       ├── inexact.rb
│   │   │       ├── inexact.rb.rewrite-tree.exp
│   │   │       ├── nilable.rb
│   │   │       ├── nilable.rb.rewrite-tree.exp
│   │   │       ├── none.rb
│   │   │       ├── none.rb.rewrite-tree.exp
│   │   │       ├── normal.rb
│   │   │       ├── normal.rb.rewrite-tree.exp
│   │   │       ├── override.rb
│   │   │       ├── override.rb.rewrite-tree.exp
│   │   │       ├── override_bad.rb
│   │   │       ├── param_order.rb
│   │   │       ├── param_order.rb.rewrite-tree.exp
│   │   │       ├── root.rb
│   │   │       ├── root.rb.rewrite-tree.exp
│   │   │       ├── some_default.rb
│   │   │       └── some_default.rb.rewrite-tree.exp
│   │   ├── substitutions
│   │   │   └── double_subsitutions.rb
│   │   ├── testrunner
│   │   │   └── pos.rb
│   │   ├── todo
│   │   │   ├── block_in_class.rb
│   │   │   ├── block_in_class.rb.flatten-tree.exp
│   │   │   ├── block_in_class.rb.symbol-table-raw.exp
│   │   │   ├── class_of.rb
│   │   │   ├── const_in_block.rb
│   │   │   ├── const_in_block.rb.symbol-table-raw.exp
│   │   │   └── generics_should_fail.rb
│   │   ├── tuple_type_size.rb
│   │   └── union_method_arity_error.rb
│   └── whitequark
│       ├── test_ENCODING_0.parse-tree-whitequark.exp
│       ├── test_ENCODING_0.rb
│       ├── test_alias_0.parse-tree-whitequark.exp
│       ├── test_alias_0.rb
│       ├── test_alias_gvar_0.parse-tree-whitequark.exp
│       ├── test_alias_gvar_0.rb
│       ├── test_alias_gvar_1.parse-tree-whitequark.exp
│       ├── test_alias_gvar_1.rb
│       ├── test_alias_nth_ref_0.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_0.parse-tree-whitequark.exp
│       ├── test_ambiuous_quoted_label_in_ternary_operator_0.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_1.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_2.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_3.rb
│       ├── test_and_0.parse-tree-whitequark.exp
│       ├── test_and_0.rb
│       ├── test_and_1.parse-tree-whitequark.exp
│       ├── test_and_1.rb
│       ├── test_and_asgn_0.parse-tree-whitequark.exp
│       ├── test_and_asgn_0.rb
│       ├── test_and_asgn_1.parse-tree-whitequark.exp
│       ├── test_and_asgn_1.rb
│       ├── test_and_or_masgn_0.parse-tree-whitequark.exp
│       ├── test_and_or_masgn_0.rb
│       ├── test_and_or_masgn_1.parse-tree-whitequark.exp
│       ├── test_and_or_masgn_1.rb
│       ├── test_arg_0.parse-tree-whitequark.exp
│       ├── test_arg_0.rb
│       ├── test_arg_1.parse-tree-whitequark.exp
│       ├── test_arg_1.rb
│       ├── test_arg_combinations_0.parse-tree-whitequark.exp
│       ├── test_arg_combinations_0.rb
│       ├── test_arg_combinations_1.parse-tree-whitequark.exp
│       ├── test_arg_combinations_1.rb
│       ├── test_arg_combinations_10.parse-tree-whitequark.exp
│       ├── test_arg_combinations_10.rb
│       ├── test_arg_combinations_11.parse-tree-whitequark.exp
│       ├── test_arg_combinations_11.rb
│       ├── test_arg_combinations_12.parse-tree-whitequark.exp
│       ├── test_arg_combinations_12.rb
│       ├── test_arg_combinations_13.parse-tree-whitequark.exp
│       ├── test_arg_combinations_13.rb
│       ├── test_arg_combinations_14.parse-tree-whitequark.exp
│       ├── test_arg_combinations_14.rb
│       ├── test_arg_combinations_2.parse-tree-whitequark.exp
│       ├── test_arg_combinations_2.rb
│       ├── test_arg_combinations_3.parse-tree-whitequark.exp
│       ├── test_arg_combinations_3.rb
│       ├── test_arg_combinations_4.parse-tree-whitequark.exp
│       ├── test_arg_combinations_4.rb
│       ├── test_arg_combinations_5.parse-tree-whitequark.exp
│       ├── test_arg_combinations_5.rb
│       ├── test_arg_combinations_6.parse-tree-whitequark.exp
│       ├── test_arg_combinations_6.rb
│       ├── test_arg_combinations_7.parse-tree-whitequark.exp
│       ├── test_arg_combinations_7.rb
│       ├── test_arg_combinations_8.parse-tree-whitequark.exp
│       ├── test_arg_combinations_8.rb
│       ├── test_arg_combinations_9.parse-tree-whitequark.exp
│       ├── test_arg_combinations_9.rb
│       ├── test_arg_duplicate_0.rb
│       ├── test_arg_duplicate_1.rb
│       ├── test_arg_duplicate_2.rb
│       ├── test_arg_duplicate_3.rb
│       ├── test_arg_duplicate_4.rb
│       ├── test_arg_duplicate_5.rb
│       ├── test_arg_duplicate_6.rb
│       ├── test_arg_duplicate_7.rb
│       ├── test_arg_duplicate_8.rb
│       ├── test_arg_duplicate_9.rb
│       ├── test_arg_duplicate_ignored_0.parse-tree-whitequark.exp
│       ├── test_arg_duplicate_ignored_0.rb
│       ├── test_arg_duplicate_ignored_1.parse-tree-whitequark.exp
│       ├── test_arg_duplicate_ignored_1.rb
│       ├── test_arg_duplicate_proc_0.rb
│       ├── test_arg_invalid_0.rb
│       ├── test_arg_invalid_1.rb
│       ├── test_arg_invalid_2.rb
│       ├── test_arg_invalid_3.rb
│       ├── test_arg_label_0.parse-tree-whitequark.exp
│       ├── test_arg_label_0.rb
│       ├── test_arg_label_1.parse-tree-whitequark.exp
│       ├── test_arg_label_1.rb
│       ├── test_arg_label_2.parse-tree-whitequark.exp
│       ├── test_arg_label_2.rb
│       ├── test_arg_scope_0.parse-tree-whitequark.exp
│       ├── test_arg_scope_0.rb
│       ├── test_args_args_assocs_0.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_0.rb
│       ├── test_args_args_assocs_1.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_1.rb
│       ├── test_args_args_assocs_comma_0.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_comma_0.rb
│       ├── test_args_args_comma_0.parse-tree-whitequark.exp
│       ├── test_args_args_comma_0.rb
│       ├── test_args_args_star_0.parse-tree-whitequark.exp
│       ├── test_args_args_star_0.rb
│       ├── test_args_args_star_1.parse-tree-whitequark.exp
│       ├── test_args_args_star_1.rb
│       ├── test_args_assocs_0.parse-tree-whitequark.exp
│       ├── test_args_assocs_0.rb
│       ├── test_args_assocs_1.parse-tree-whitequark.exp
│       ├── test_args_assocs_1.rb
│       ├── test_args_assocs_comma_0.parse-tree-whitequark.exp
│       ├── test_args_assocs_comma_0.rb
│       ├── test_args_block_pass_0.parse-tree-whitequark.exp
│       ├── test_args_block_pass_0.rb
│       ├── test_args_cmd_0.parse-tree-whitequark.exp
│       ├── test_args_cmd_0.rb
│       ├── test_args_star_0.parse-tree-whitequark.exp
│       ├── test_args_star_0.rb
│       ├── test_args_star_1.parse-tree-whitequark.exp
│       ├── test_args_star_1.rb
│       ├── test_array_assocs_0.parse-tree-whitequark.exp
│       ├── test_array_assocs_0.rb
│       ├── test_array_assocs_1.parse-tree-whitequark.exp
│       ├── test_array_assocs_1.rb
│       ├── test_array_plain_0.parse-tree-whitequark.exp
│       ├── test_array_plain_0.rb
│       ├── test_array_splat_0.parse-tree-whitequark.exp
│       ├── test_array_splat_0.rb
│       ├── test_array_splat_1.parse-tree-whitequark.exp
│       ├── test_array_splat_1.rb
│       ├── test_array_splat_2.parse-tree-whitequark.exp
│       ├── test_array_splat_2.rb
│       ├── test_array_symbols_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_0.rb
│       ├── test_array_symbols_empty_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_empty_0.rb
│       ├── test_array_symbols_empty_1.parse-tree-whitequark.exp
│       ├── test_array_symbols_empty_1.rb
│       ├── test_array_symbols_interp_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_interp_0.rb
│       ├── test_array_symbols_interp_1.parse-tree-whitequark.exp
│       ├── test_array_symbols_interp_1.rb
│       ├── test_array_words_0.parse-tree-whitequark.exp
│       ├── test_array_words_0.rb
│       ├── test_array_words_empty_0.parse-tree-whitequark.exp
│       ├── test_array_words_empty_0.rb
│       ├── test_array_words_empty_1.parse-tree-whitequark.exp
│       ├── test_array_words_empty_1.rb
│       ├── test_array_words_interp_0.parse-tree-whitequark.exp
│       ├── test_array_words_interp_0.rb
│       ├── test_array_words_interp_1.parse-tree-whitequark.exp
│       ├── test_array_words_interp_1.rb
│       ├── test_asgn_backref_invalid_0.rb
│       ├── test_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_asgn_cmd_0.rb
│       ├── test_asgn_cmd_1.parse-tree-whitequark.exp
│       ├── test_asgn_cmd_1.rb
│       ├── test_asgn_keyword_invalid_0.rb
│       ├── test_asgn_keyword_invalid_1.rb
│       ├── test_asgn_keyword_invalid_2.rb
│       ├── test_asgn_keyword_invalid_3.rb
│       ├── test_asgn_keyword_invalid_4.rb
│       ├── test_asgn_keyword_invalid_5.rb
│       ├── test_asgn_mrhs_0.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_0.rb
│       ├── test_asgn_mrhs_1.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_1.rb
│       ├── test_asgn_mrhs_2.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_2.rb
│       ├── test_back_ref_0.parse-tree-whitequark.exp
│       ├── test_back_ref_0.rb
│       ├── test_bang_0.parse-tree-whitequark.exp
│       ├── test_bang_0.rb
│       ├── test_bang_cmd_0.parse-tree-whitequark.exp
│       ├── test_bang_cmd_0.rb
│       ├── test_begin_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_begin_cmdarg_0.rb
│       ├── test_beginless_range_before_27_0.rb
│       ├── test_beginless_range_before_27_1.rb
│       ├── test_block_arg_combinations_0.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_0.rb
│       ├── test_block_arg_combinations_1.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_1.rb
│       ├── test_block_arg_combinations_10.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_10.rb
│       ├── test_block_arg_combinations_11.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_11.rb
│       ├── test_block_arg_combinations_12.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_12.rb
│       ├── test_block_arg_combinations_13.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_13.rb
│       ├── test_block_arg_combinations_14.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_14.rb
│       ├── test_block_arg_combinations_15.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_15.rb
│       ├── test_block_arg_combinations_16.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_16.rb
│       ├── test_block_arg_combinations_17.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_17.rb
│       ├── test_block_arg_combinations_18.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_18.rb
│       ├── test_block_arg_combinations_19.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_19.rb
│       ├── test_block_arg_combinations_2.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_2.rb
│       ├── test_block_arg_combinations_20.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_20.rb
│       ├── test_block_arg_combinations_21.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_21.rb
│       ├── test_block_arg_combinations_22.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_22.rb
│       ├── test_block_arg_combinations_23.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_23.rb
│       ├── test_block_arg_combinations_24.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_24.rb
│       ├── test_block_arg_combinations_25.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_25.rb
│       ├── test_block_arg_combinations_26.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_26.rb
│       ├── test_block_arg_combinations_27.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_27.rb
│       ├── test_block_arg_combinations_3.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_3.rb
│       ├── test_block_arg_combinations_4.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_4.rb
│       ├── test_block_arg_combinations_5.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_5.rb
│       ├── test_block_arg_combinations_6.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_6.rb
│       ├── test_block_arg_combinations_7.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_7.rb
│       ├── test_block_arg_combinations_8.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_8.rb
│       ├── test_block_arg_combinations_9.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_9.rb
│       ├── test_block_kwarg_0.parse-tree-whitequark.exp
│       ├── test_block_kwarg_0.rb
│       ├── test_block_kwarg_combinations_0.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_0.rb
│       ├── test_block_kwarg_combinations_1.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_1.rb
│       ├── test_block_kwarg_combinations_2.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_2.rb
│       ├── test_blockarg_0.parse-tree-whitequark.exp
│       ├── test_blockarg_0.rb
│       ├── test_break_0.parse-tree-whitequark.exp
│       ├── test_break_0.rb
│       ├── test_break_1.parse-tree-whitequark.exp
│       ├── test_break_1.rb
│       ├── test_break_2.parse-tree-whitequark.exp
│       ├── test_break_2.rb
│       ├── test_break_3.parse-tree-whitequark.exp
│       ├── test_break_3.rb
│       ├── test_break_block_0.parse-tree-whitequark.exp
│       ├── test_break_block_0.rb
│       ├── test_bug_435_0.parse-tree-whitequark.exp
│       ├── test_bug_435_0.rb
│       ├── test_bug_447_0.parse-tree-whitequark.exp
│       ├── test_bug_447_0.rb
│       ├── test_bug_447_1.parse-tree-whitequark.exp
│       ├── test_bug_447_1.rb
│       ├── test_bug_452_0.parse-tree-whitequark.exp
│       ├── test_bug_452_0.rb
│       ├── test_bug_466_0.parse-tree-whitequark.exp
│       ├── test_bug_466_0.rb
│       ├── test_bug_473_0.parse-tree-whitequark.exp
│       ├── test_bug_473_0.rb
│       ├── test_bug_480_0.parse-tree-whitequark.exp
│       ├── test_bug_480_0.rb
│       ├── test_bug_481_0.parse-tree-whitequark.exp
│       ├── test_bug_481_0.rb
│       ├── test_bug_cmd_string_lookahead_0.parse-tree-whitequark.exp
│       ├── test_bug_cmd_string_lookahead_0.rb
│       ├── test_bug_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_0.rb
│       ├── test_bug_cmdarg_1.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_1.rb
│       ├── test_bug_cmdarg_2.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_2.rb
│       ├── test_bug_def_no_paren_eql_begin_0.parse-tree-whitequark.exp
│       ├── test_bug_def_no_paren_eql_begin_0.rb
│       ├── test_bug_do_block_in_call_args_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_call_args_0.rb
│       ├── test_bug_do_block_in_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_cmdarg_0.rb
│       ├── test_bug_do_block_in_hash_brace_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_0.rb
│       ├── test_bug_do_block_in_hash_brace_1.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_1.rb
│       ├── test_bug_do_block_in_hash_brace_2.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_2.rb
│       ├── test_bug_do_block_in_hash_brace_3.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_3.rb
│       ├── test_bug_do_block_in_hash_brace_4.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_4.rb
│       ├── test_bug_heredoc_do_0.parse-tree-whitequark.exp
│       ├── test_bug_heredoc_do_0.rb
│       ├── test_bug_heredoc_xstring_0.rb
│       ├── test_bug_interp_single_0.parse-tree-whitequark.exp
│       ├── test_bug_interp_single_0.rb
│       ├── test_bug_interp_single_1.parse-tree-whitequark.exp
│       ├── test_bug_interp_single_1.rb
│       ├── test_bug_lambda_leakage_0.parse-tree-whitequark.exp
│       ├── test_bug_lambda_leakage_0.rb
│       ├── test_bug_regex_verification_0.parse-tree-whitequark.exp
│       ├── test_bug_regex_verification_0.rb
│       ├── test_bug_rescue_empty_else_0.parse-tree-whitequark.exp
│       ├── test_bug_rescue_empty_else_0.rb
│       ├── test_bug_while_not_parens_do_0.parse-tree-whitequark.exp
│       ├── test_bug_while_not_parens_do_0.rb
│       ├── test_case_cond_0.parse-tree-whitequark.exp
│       ├── test_case_cond_0.rb
│       ├── test_case_cond_else_0.parse-tree-whitequark.exp
│       ├── test_case_cond_else_0.rb
│       ├── test_case_expr_0.parse-tree-whitequark.exp
│       ├── test_case_expr_0.rb
│       ├── test_case_expr_else_0.parse-tree-whitequark.exp
│       ├── test_case_expr_else_0.rb
│       ├── test_casgn_invalid_0.rb
│       ├── test_casgn_invalid_1.rb
│       ├── test_casgn_invalid_2.rb
│       ├── test_casgn_invalid_3.rb
│       ├── test_casgn_invalid_4.rb
│       ├── test_casgn_invalid_5.rb
│       ├── test_casgn_scoped_0.parse-tree-whitequark.exp
│       ├── test_casgn_scoped_0.rb
│       ├── test_casgn_toplevel_0.parse-tree-whitequark.exp
│       ├── test_casgn_toplevel_0.rb
│       ├── test_casgn_unscoped_0.parse-tree-whitequark.exp
│       ├── test_casgn_unscoped_0.rb
│       ├── test_character_0.parse-tree-whitequark.exp
│       ├── test_character_0.rb
│       ├── test_class_0.parse-tree-whitequark.exp
│       ├── test_class_0.rb
│       ├── test_class_1.parse-tree-whitequark.exp
│       ├── test_class_1.rb
│       ├── test_class_definition_in_while_cond_0.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_0.rb
│       ├── test_class_definition_in_while_cond_1.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_1.rb
│       ├── test_class_definition_in_while_cond_2.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_2.rb
│       ├── test_class_definition_in_while_cond_3.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_3.rb
│       ├── test_class_invalid_0.rb
│       ├── test_class_invalid_1.rb
│       ├── test_class_super_0.parse-tree-whitequark.exp
│       ├── test_class_super_0.rb
│       ├── test_class_super_label_0.parse-tree-whitequark.exp
│       ├── test_class_super_label_0.rb
│       ├── test_codepoint_too_large_0.rb
│       ├── test_complex_0.parse-tree-whitequark.exp
│       ├── test_complex_0.rb
│       ├── test_complex_1.parse-tree-whitequark.exp
│       ├── test_complex_1.rb
│       ├── test_complex_2.parse-tree-whitequark.exp
│       ├── test_complex_2.rb
│       ├── test_complex_3.parse-tree-whitequark.exp
│       ├── test_complex_3.rb
│       ├── test_cond_begin_0.parse-tree-whitequark.exp
│       ├── test_cond_begin_0.rb
│       ├── test_cond_eflipflop_0.parse-tree-whitequark.exp
│       ├── test_cond_eflipflop_0.rb
│       ├── test_cond_eflipflop_1.parse-tree-whitequark.exp
│       ├── test_cond_eflipflop_1.rb
│       ├── test_cond_iflipflop_0.parse-tree-whitequark.exp
│       ├── test_cond_iflipflop_0.rb
│       ├── test_cond_iflipflop_1.parse-tree-whitequark.exp
│       ├── test_cond_iflipflop_1.rb
│       ├── test_cond_match_current_line_0.parse-tree-whitequark.exp
│       ├── test_cond_match_current_line_0.rb
│       ├── test_cond_match_current_line_1.parse-tree-whitequark.exp
│       ├── test_cond_match_current_line_1.rb
│       ├── test_const_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_0.rb
│       ├── test_const_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_1.rb
│       ├── test_const_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_2.rb
│       ├── test_const_op_asgn_3.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_3.rb
│       ├── test_const_op_asgn_4.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_4.rb
│       ├── test_const_scoped_0.parse-tree-whitequark.exp
│       ├── test_const_scoped_0.rb
│       ├── test_const_toplevel_0.parse-tree-whitequark.exp
│       ├── test_const_toplevel_0.rb
│       ├── test_const_unscoped_0.parse-tree-whitequark.exp
│       ├── test_const_unscoped_0.rb
│       ├── test_cpath_0.parse-tree-whitequark.exp
│       ├── test_cpath_0.rb
│       ├── test_cpath_1.parse-tree-whitequark.exp
│       ├── test_cpath_1.rb
│       ├── test_cpath_invalid_0.rb
│       ├── test_cvar_0.parse-tree-whitequark.exp
│       ├── test_cvar_0.rb
│       ├── test_cvasgn_0.parse-tree-whitequark.exp
│       ├── test_cvasgn_0.rb
│       ├── test_def_0.parse-tree-whitequark.exp
│       ├── test_def_0.rb
│       ├── test_def_1.parse-tree-whitequark.exp
│       ├── test_def_1.rb
│       ├── test_def_2.parse-tree-whitequark.exp
│       ├── test_def_2.rb
│       ├── test_def_3.parse-tree-whitequark.exp
│       ├── test_def_3.rb
│       ├── test_def_4.parse-tree-whitequark.exp
│       ├── test_def_4.rb
│       ├── test_def_5.parse-tree-whitequark.exp
│       ├── test_def_5.rb
│       ├── test_defined_0.parse-tree-whitequark.exp
│       ├── test_defined_0.rb
│       ├── test_defined_1.parse-tree-whitequark.exp
│       ├── test_defined_1.rb
│       ├── test_defined_2.parse-tree-whitequark.exp
│       ├── test_defined_2.rb
│       ├── test_defs_0.parse-tree-whitequark.exp
│       ├── test_defs_0.rb
│       ├── test_defs_1.parse-tree-whitequark.exp
│       ├── test_defs_1.rb
│       ├── test_defs_2.parse-tree-whitequark.exp
│       ├── test_defs_2.rb
│       ├── test_defs_3.parse-tree-whitequark.exp
│       ├── test_defs_3.rb
│       ├── test_defs_4.parse-tree-whitequark.exp
│       ├── test_defs_4.rb
│       ├── test_defs_invalid_0.rb
│       ├── test_defs_invalid_1.rb
│       ├── test_defs_invalid_2.rb
│       ├── test_defs_invalid_3.rb
│       ├── test_defs_invalid_4.rb
│       ├── test_defs_invalid_5.rb
│       ├── test_defs_invalid_6.rb
│       ├── test_defs_invalid_7.rb
│       ├── test_emit_arg_inside_procarg0_legacy_0.parse-tree-whitequark.exp
│       ├── test_emit_arg_inside_procarg0_legacy_0.rb
│       ├── test_ensure_0.parse-tree-whitequark.exp
│       ├── test_ensure_0.rb
│       ├── test_ensure_empty_0.parse-tree-whitequark.exp
│       ├── test_ensure_empty_0.rb
│       ├── test_false_0.parse-tree-whitequark.exp
│       ├── test_false_0.rb
│       ├── test_float_0.parse-tree-whitequark.exp
│       ├── test_float_0.rb
│       ├── test_float_1.parse-tree-whitequark.exp
│       ├── test_float_1.rb
│       ├── test_for_0.parse-tree-whitequark.exp
│       ├── test_for_0.rb
│       ├── test_for_1.parse-tree-whitequark.exp
│       ├── test_for_1.rb
│       ├── test_for_mlhs_0.parse-tree-whitequark.exp
│       ├── test_for_mlhs_0.rb
│       ├── test_gvar_0.parse-tree-whitequark.exp
│       ├── test_gvar_0.rb
│       ├── test_gvasgn_0.parse-tree-whitequark.exp
│       ├── test_gvasgn_0.rb
│       ├── test_hash_empty_0.parse-tree-whitequark.exp
│       ├── test_hash_empty_0.rb
│       ├── test_hash_hashrocket_0.parse-tree-whitequark.exp
│       ├── test_hash_hashrocket_0.rb
│       ├── test_hash_hashrocket_1.parse-tree-whitequark.exp
│       ├── test_hash_hashrocket_1.rb
│       ├── test_hash_kwsplat_0.parse-tree-whitequark.exp
│       ├── test_hash_kwsplat_0.rb
│       ├── test_hash_label_0.parse-tree-whitequark.exp
│       ├── test_hash_label_0.rb
│       ├── test_hash_label_end_0.parse-tree-whitequark.exp
│       ├── test_hash_label_end_0.rb
│       ├── test_hash_label_end_1.parse-tree-whitequark.exp
│       ├── test_hash_label_end_1.rb
│       ├── test_hash_label_end_2.parse-tree-whitequark.exp
│       ├── test_hash_label_end_2.rb
│       ├── test_if_0.parse-tree-whitequark.exp
│       ├── test_if_0.rb
│       ├── test_if_1.parse-tree-whitequark.exp
│       ├── test_if_1.rb
│       ├── test_if_else_0.parse-tree-whitequark.exp
│       ├── test_if_else_0.rb
│       ├── test_if_else_1.parse-tree-whitequark.exp
│       ├── test_if_else_1.rb
│       ├── test_if_elsif_0.parse-tree-whitequark.exp
│       ├── test_if_elsif_0.rb
│       ├── test_if_masgn_24_0.parse-tree-whitequark.exp
│       ├── test_if_masgn_24_0.rb
│       ├── test_if_mod_0.parse-tree-whitequark.exp
│       ├── test_if_mod_0.rb
│       ├── test_if_nl_then_0.parse-tree-whitequark.exp
│       ├── test_if_nl_then_0.rb
│       ├── test_int_0.parse-tree-whitequark.exp
│       ├── test_int_0.rb
│       ├── test_int_1.parse-tree-whitequark.exp
│       ├── test_int_1.rb
│       ├── test_int_2.parse-tree-whitequark.exp
│       ├── test_int_2.rb
│       ├── test_int_LINE_0.parse-tree-whitequark.exp
│       ├── test_int_LINE_0.rb
│       ├── test_ivar_0.parse-tree-whitequark.exp
│       ├── test_ivar_0.rb
│       ├── test_ivasgn_0.parse-tree-whitequark.exp
│       ├── test_ivasgn_0.rb
│       ├── test_kwarg_0.parse-tree-whitequark.exp
│       ├── test_kwarg_0.rb
│       ├── test_kwarg_combinations_0.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_0.rb
│       ├── test_kwarg_combinations_1.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_1.rb
│       ├── test_kwarg_combinations_2.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_2.rb
│       ├── test_kwarg_combinations_3.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_3.rb
│       ├── test_kwarg_invalid_0.rb
│       ├── test_kwarg_invalid_1.rb
│       ├── test_kwarg_no_paren_0.parse-tree-whitequark.exp
│       ├── test_kwarg_no_paren_0.rb
│       ├── test_kwarg_no_paren_1.parse-tree-whitequark.exp
│       ├── test_kwarg_no_paren_1.rb
│       ├── test_kwbegin_compstmt_0.parse-tree-whitequark.exp
│       ├── test_kwbegin_compstmt_0.rb
│       ├── test_kwoptarg_0.parse-tree-whitequark.exp
│       ├── test_kwoptarg_0.rb
│       ├── test_kwrestarg_named_0.parse-tree-whitequark.exp
│       ├── test_kwrestarg_named_0.rb
│       ├── test_kwrestarg_unnamed_0.parse-tree-whitequark.exp
│       ├── test_kwrestarg_unnamed_0.rb
│       ├── test_lbrace_arg_after_command_args_0.parse-tree-whitequark.exp
│       ├── test_lbrace_arg_after_command_args_0.rb
│       ├── test_log_asgn_invalid_0.rb
│       ├── test_log_asgn_invalid_1.rb
│       ├── test_lparenarg_after_lvar_since_25_0.parse-tree-whitequark.exp
│       ├── test_lparenarg_after_lvar_since_25_0.rb
│       ├── test_lparenarg_after_lvar_since_25_1.parse-tree-whitequark.exp
│       ├── test_lparenarg_after_lvar_since_25_1.rb
│       ├── test_lvar_0.parse-tree-whitequark.exp
│       ├── test_lvar_0.rb
│       ├── test_lvasgn_0.parse-tree-whitequark.exp
│       ├── test_lvasgn_0.rb
│       ├── test_marg_combinations_0.parse-tree-whitequark.exp
│       ├── test_marg_combinations_0.rb
│       ├── test_marg_combinations_1.parse-tree-whitequark.exp
│       ├── test_marg_combinations_1.rb
│       ├── test_marg_combinations_2.parse-tree-whitequark.exp
│       ├── test_marg_combinations_2.rb
│       ├── test_marg_combinations_3.parse-tree-whitequark.exp
│       ├── test_marg_combinations_3.rb
│       ├── test_marg_combinations_4.parse-tree-whitequark.exp
│       ├── test_marg_combinations_4.rb
│       ├── test_marg_combinations_5.parse-tree-whitequark.exp
│       ├── test_marg_combinations_5.rb
│       ├── test_marg_combinations_6.parse-tree-whitequark.exp
│       ├── test_marg_combinations_6.rb
│       ├── test_marg_combinations_7.parse-tree-whitequark.exp
│       ├── test_marg_combinations_7.rb
│       ├── test_marg_combinations_8.parse-tree-whitequark.exp
│       ├── test_marg_combinations_8.rb
│       ├── test_marg_combinations_9.parse-tree-whitequark.exp
│       ├── test_marg_combinations_9.rb
│       ├── test_masgn_0.parse-tree-whitequark.exp
│       ├── test_masgn_0.rb
│       ├── test_masgn_1.parse-tree-whitequark.exp
│       ├── test_masgn_1.rb
│       ├── test_masgn_2.parse-tree-whitequark.exp
│       ├── test_masgn_2.rb
│       ├── test_masgn_attr_0.parse-tree-whitequark.exp
│       ├── test_masgn_attr_0.rb
│       ├── test_masgn_attr_1.parse-tree-whitequark.exp
│       ├── test_masgn_attr_1.rb
│       ├── test_masgn_attr_2.parse-tree-whitequark.exp
│       ├── test_masgn_attr_2.rb
│       ├── test_masgn_backref_invalid_0.rb
│       ├── test_masgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_masgn_cmd_0.rb
│       ├── test_masgn_const_0.parse-tree-whitequark.exp
│       ├── test_masgn_const_0.rb
│       ├── test_masgn_const_1.parse-tree-whitequark.exp
│       ├── test_masgn_const_1.rb
│       ├── test_masgn_const_invalid_0.rb
│       ├── test_masgn_const_invalid_1.rb
│       ├── test_masgn_keyword_invalid_0.rb
│       ├── test_masgn_nested_0.parse-tree-whitequark.exp
│       ├── test_masgn_nested_0.rb
│       ├── test_masgn_nested_1.parse-tree-whitequark.exp
│       ├── test_masgn_nested_1.rb
│       ├── test_masgn_splat_0.parse-tree-whitequark.exp
│       ├── test_masgn_splat_0.rb
│       ├── test_masgn_splat_1.parse-tree-whitequark.exp
│       ├── test_masgn_splat_1.rb
│       ├── test_masgn_splat_2.parse-tree-whitequark.exp
│       ├── test_masgn_splat_2.rb
│       ├── test_masgn_splat_3.parse-tree-whitequark.exp
│       ├── test_masgn_splat_3.rb
│       ├── test_masgn_splat_4.parse-tree-whitequark.exp
│       ├── test_masgn_splat_4.rb
│       ├── test_masgn_splat_5.parse-tree-whitequark.exp
│       ├── test_masgn_splat_5.rb
│       ├── test_masgn_splat_6.parse-tree-whitequark.exp
│       ├── test_masgn_splat_6.rb
│       ├── test_masgn_splat_7.parse-tree-whitequark.exp
│       ├── test_masgn_splat_7.rb
│       ├── test_masgn_splat_8.parse-tree-whitequark.exp
│       ├── test_masgn_splat_8.rb
│       ├── test_masgn_splat_9.parse-tree-whitequark.exp
│       ├── test_masgn_splat_9.rb
│       ├── test_method_definition_in_while_cond_0.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_0.rb
│       ├── test_method_definition_in_while_cond_1.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_1.rb
│       ├── test_method_definition_in_while_cond_2.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_2.rb
│       ├── test_method_definition_in_while_cond_3.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_3.rb
│       ├── test_module_0.parse-tree-whitequark.exp
│       ├── test_module_0.rb
│       ├── test_module_invalid_0.rb
│       ├── test_multiple_args_with_trailing_comma_0.parse-tree-whitequark.exp
│       ├── test_multiple_args_with_trailing_comma_0.rb
│       ├── test_next_0.parse-tree-whitequark.exp
│       ├── test_next_0.rb
│       ├── test_next_1.parse-tree-whitequark.exp
│       ├── test_next_1.rb
│       ├── test_next_2.parse-tree-whitequark.exp
│       ├── test_next_2.rb
│       ├── test_next_3.parse-tree-whitequark.exp
│       ├── test_next_3.rb
│       ├── test_next_block_0.parse-tree-whitequark.exp
│       ├── test_next_block_0.rb
│       ├── test_nil_0.parse-tree-whitequark.exp
│       ├── test_nil_0.rb
│       ├── test_nil_expression_0.parse-tree-whitequark.exp
│       ├── test_nil_expression_0.rb
│       ├── test_nil_expression_1.parse-tree-whitequark.exp
│       ├── test_nil_expression_1.rb
│       ├── test_non_lvar_injecting_match_0.parse-tree-whitequark.exp
│       ├── test_non_lvar_injecting_match_0.rb
│       ├── test_not_0.parse-tree-whitequark.exp
│       ├── test_not_0.rb
│       ├── test_not_1.parse-tree-whitequark.exp
│       ├── test_not_1.rb
│       ├── test_not_2.parse-tree-whitequark.exp
│       ├── test_not_2.rb
│       ├── test_not_cmd_0.parse-tree-whitequark.exp
│       ├── test_not_cmd_0.rb
│       ├── test_not_masgn_24_0.parse-tree-whitequark.exp
│       ├── test_not_masgn_24_0.rb
│       ├── test_nth_ref_0.parse-tree-whitequark.exp
│       ├── test_nth_ref_0.rb
│       ├── test_on_error_0.rb
│       ├── test_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_0.rb
│       ├── test_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_op_asgn_1.rb
│       ├── test_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_op_asgn_2.rb
│       ├── test_op_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_0.rb
│       ├── test_op_asgn_cmd_1.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_1.rb
│       ├── test_op_asgn_cmd_2.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_2.rb
│       ├── test_op_asgn_cmd_3.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_3.rb
│       ├── test_op_asgn_index_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_index_0.rb
│       ├── test_op_asgn_index_cmd_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_index_cmd_0.rb
│       ├── test_op_asgn_invalid_0.rb
│       ├── test_op_asgn_invalid_1.rb
│       ├── test_op_asgn_invalid_2.rb
│       ├── test_optarg_0.parse-tree-whitequark.exp
│       ├── test_optarg_0.rb
│       ├── test_optarg_1.parse-tree-whitequark.exp
│       ├── test_optarg_1.rb
│       ├── test_or_0.parse-tree-whitequark.exp
│       ├── test_or_0.rb
│       ├── test_or_1.parse-tree-whitequark.exp
│       ├── test_or_1.rb
│       ├── test_or_asgn_0.parse-tree-whitequark.exp
│       ├── test_or_asgn_0.rb
│       ├── test_or_asgn_1.parse-tree-whitequark.exp
│       ├── test_or_asgn_1.rb
│       ├── test_parser_bug_272_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_272_0.rb
│       ├── test_parser_bug_490_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_0.rb
│       ├── test_parser_bug_490_1.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_1.rb
│       ├── test_parser_bug_490_2.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_2.rb
│       ├── test_parser_bug_507_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_507_0.rb
│       ├── test_parser_bug_518_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_518_0.rb
│       ├── test_parser_bug_525_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_525_0.rb
│       ├── test_parser_bug_604_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_604_0.rb
│       ├── test_postexe_0.parse-tree-whitequark.exp
│       ├── test_postexe_0.rb
│       ├── test_preexe_0.parse-tree-whitequark.exp
│       ├── test_preexe_0.rb
│       ├── test_preexe_invalid_0.rb
│       ├── test_procarg0_0.parse-tree-whitequark.exp
│       ├── test_procarg0_0.rb
│       ├── test_procarg0_1.parse-tree-whitequark.exp
│       ├── test_procarg0_1.rb
│       ├── test_procarg0_legacy_0.parse-tree-whitequark.exp
│       ├── test_procarg0_legacy_0.rb
│       ├── test_range_endless_0.parse-tree-whitequark.exp
│       ├── test_range_endless_0.rb
│       ├── test_range_endless_1.parse-tree-whitequark.exp
│       ├── test_range_endless_1.rb
│       ├── test_range_exclusive_0.parse-tree-whitequark.exp
│       ├── test_range_exclusive_0.rb
│       ├── test_range_inclusive_0.parse-tree-whitequark.exp
│       ├── test_range_inclusive_0.rb
│       ├── test_rational_0.parse-tree-whitequark.exp
│       ├── test_rational_0.rb
│       ├── test_rational_1.parse-tree-whitequark.exp
│       ├── test_rational_1.rb
│       ├── test_redo_0.parse-tree-whitequark.exp
│       ├── test_redo_0.rb
│       ├── test_regex_interp_0.parse-tree-whitequark.exp
│       ├── test_regex_interp_0.rb
│       ├── test_regex_plain_0.parse-tree-whitequark.exp
│       ├── test_regex_plain_0.rb
│       ├── test_resbody_list_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_0.rb
│       ├── test_resbody_list_mrhs_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_mrhs_0.rb
│       ├── test_resbody_list_var_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_var_0.rb
│       ├── test_resbody_var_0.parse-tree-whitequark.exp
│       ├── test_resbody_var_0.rb
│       ├── test_resbody_var_1.parse-tree-whitequark.exp
│       ├── test_resbody_var_1.rb
│       ├── test_rescue_0.parse-tree-whitequark.exp
│       ├── test_rescue_0.rb
│       ├── test_rescue_else_0.parse-tree-whitequark.exp
│       ├── test_rescue_else_0.rb
│       ├── test_rescue_else_ensure_0.parse-tree-whitequark.exp
│       ├── test_rescue_else_ensure_0.rb
│       ├── test_rescue_else_useless_0.rb
│       ├── test_rescue_else_useless_1.rb
│       ├── test_rescue_ensure_0.parse-tree-whitequark.exp
│       ├── test_rescue_ensure_0.rb
│       ├── test_rescue_in_lambda_block_0.parse-tree-whitequark.exp
│       ├── test_rescue_in_lambda_block_0.rb
│       ├── test_rescue_in_lambda_block_1.rb
│       ├── test_rescue_mod_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_0.rb
│       ├── test_rescue_mod_asgn_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_asgn_0.rb
│       ├── test_rescue_mod_op_assign_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_op_assign_0.rb
│       ├── test_rescue_without_begin_end_0.parse-tree-whitequark.exp
│       ├── test_rescue_without_begin_end_0.rb
│       ├── test_restarg_named_0.parse-tree-whitequark.exp
│       ├── test_restarg_named_0.rb
│       ├── test_restarg_unnamed_0.parse-tree-whitequark.exp
│       ├── test_restarg_unnamed_0.rb
│       ├── test_retry_0.parse-tree-whitequark.exp
│       ├── test_retry_0.rb
│       ├── test_return_0.parse-tree-whitequark.exp
│       ├── test_return_0.rb
│       ├── test_return_1.parse-tree-whitequark.exp
│       ├── test_return_1.rb
│       ├── test_return_2.parse-tree-whitequark.exp
│       ├── test_return_2.rb
│       ├── test_return_3.parse-tree-whitequark.exp
│       ├── test_return_3.rb
│       ├── test_return_block_0.parse-tree-whitequark.exp
│       ├── test_return_block_0.rb
│       ├── test_return_in_class_0.rb
│       ├── test_ruby_bug_10279_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10279_0.rb
│       ├── test_ruby_bug_10653_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_0.rb
│       ├── test_ruby_bug_10653_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_1.rb
│       ├── test_ruby_bug_10653_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_2.rb
│       ├── test_ruby_bug_11107_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11107_0.rb
│       ├── test_ruby_bug_11380_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11380_0.rb
│       ├── test_ruby_bug_11873_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_0.rb
│       ├── test_ruby_bug_11873_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_1.rb
│       ├── test_ruby_bug_11873_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_10.rb
│       ├── test_ruby_bug_11873_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_11.rb
│       ├── test_ruby_bug_11873_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_2.rb
│       ├── test_ruby_bug_11873_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_3.rb
│       ├── test_ruby_bug_11873_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_4.rb
│       ├── test_ruby_bug_11873_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_5.rb
│       ├── test_ruby_bug_11873_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_6.rb
│       ├── test_ruby_bug_11873_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_7.rb
│       ├── test_ruby_bug_11873_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_8.rb
│       ├── test_ruby_bug_11873_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_9.rb
│       ├── test_ruby_bug_11873_a_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_0.rb
│       ├── test_ruby_bug_11873_a_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_1.rb
│       ├── test_ruby_bug_11873_a_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_10.rb
│       ├── test_ruby_bug_11873_a_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_11.rb
│       ├── test_ruby_bug_11873_a_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_12.rb
│       ├── test_ruby_bug_11873_a_13.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_13.rb
│       ├── test_ruby_bug_11873_a_14.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_14.rb
│       ├── test_ruby_bug_11873_a_15.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_15.rb
│       ├── test_ruby_bug_11873_a_16.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_16.rb
│       ├── test_ruby_bug_11873_a_17.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_17.rb
│       ├── test_ruby_bug_11873_a_18.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_18.rb
│       ├── test_ruby_bug_11873_a_19.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_19.rb
│       ├── test_ruby_bug_11873_a_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_2.rb
│       ├── test_ruby_bug_11873_a_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_3.rb
│       ├── test_ruby_bug_11873_a_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_4.rb
│       ├── test_ruby_bug_11873_a_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_5.rb
│       ├── test_ruby_bug_11873_a_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_6.rb
│       ├── test_ruby_bug_11873_a_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_7.rb
│       ├── test_ruby_bug_11873_a_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_8.rb
│       ├── test_ruby_bug_11873_a_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_9.rb
│       ├── test_ruby_bug_11873_b_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_b_0.rb
│       ├── test_ruby_bug_12073_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12073_0.rb
│       ├── test_ruby_bug_12073_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12073_1.rb
│       ├── test_ruby_bug_12402_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_0.rb
│       ├── test_ruby_bug_12402_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_1.rb
│       ├── test_ruby_bug_12402_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_10.rb
│       ├── test_ruby_bug_12402_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_11.rb
│       ├── test_ruby_bug_12402_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_12.rb
│       ├── test_ruby_bug_12402_13.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_13.rb
│       ├── test_ruby_bug_12402_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_2.rb
│       ├── test_ruby_bug_12402_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_3.rb
│       ├── test_ruby_bug_12402_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_4.rb
│       ├── test_ruby_bug_12402_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_5.rb
│       ├── test_ruby_bug_12402_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_6.rb
│       ├── test_ruby_bug_12402_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_7.rb
│       ├── test_ruby_bug_12402_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_8.rb
│       ├── test_ruby_bug_12402_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_9.rb
│       ├── test_ruby_bug_12669_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_0.rb
│       ├── test_ruby_bug_12669_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_1.rb
│       ├── test_ruby_bug_12669_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_2.rb
│       ├── test_ruby_bug_12669_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_3.rb
│       ├── test_ruby_bug_12686_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12686_0.rb
│       ├── test_ruby_bug_12686_1.rb
│       ├── test_ruby_bug_13547_0.rb
│       ├── test_ruby_bug_13547_1.rb
│       ├── test_ruby_bug_13547_10.rb
│       ├── test_ruby_bug_13547_11.rb
│       ├── test_ruby_bug_13547_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_13547_12.rb
│       ├── test_ruby_bug_13547_2.rb
│       ├── test_ruby_bug_13547_3.rb
│       ├── test_ruby_bug_13547_4.rb
│       ├── test_ruby_bug_13547_5.rb
│       ├── test_ruby_bug_13547_6.rb
│       ├── test_ruby_bug_13547_7.rb
│       ├── test_ruby_bug_13547_8.rb
│       ├── test_ruby_bug_13547_9.rb
│       ├── test_ruby_bug_14690_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_14690_0.rb
│       ├── test_ruby_bug_9669_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_9669_0.rb
│       ├── test_ruby_bug_9669_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_9669_1.rb
│       ├── test_sclass_0.parse-tree-whitequark.exp
│       ├── test_sclass_0.rb
│       ├── test_self_0.parse-tree-whitequark.exp
│       ├── test_self_0.rb
│       ├── test_send_attr_asgn_0.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_0.rb
│       ├── test_send_attr_asgn_1.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_1.rb
│       ├── test_send_attr_asgn_2.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_2.rb
│       ├── test_send_attr_asgn_3.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_3.rb
│       ├── test_send_attr_asgn_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_conditional_0.rb
│       ├── test_send_binary_op_0.parse-tree-whitequark.exp
│       ├── test_send_binary_op_0.rb
│       ├── test_send_binary_op_1.parse-tree-whitequark.exp
│       ├── test_send_binary_op_1.rb
│       ├── test_send_binary_op_10.parse-tree-whitequark.exp
│       ├── test_send_binary_op_10.rb
│       ├── test_send_binary_op_11.parse-tree-whitequark.exp
│       ├── test_send_binary_op_11.rb
│       ├── test_send_binary_op_12.parse-tree-whitequark.exp
│       ├── test_send_binary_op_12.rb
│       ├── test_send_binary_op_13.parse-tree-whitequark.exp
│       ├── test_send_binary_op_13.rb
│       ├── test_send_binary_op_14.parse-tree-whitequark.exp
│       ├── test_send_binary_op_14.rb
│       ├── test_send_binary_op_15.parse-tree-whitequark.exp
│       ├── test_send_binary_op_15.rb
│       ├── test_send_binary_op_16.parse-tree-whitequark.exp
│       ├── test_send_binary_op_16.rb
│       ├── test_send_binary_op_17.parse-tree-whitequark.exp
│       ├── test_send_binary_op_17.rb
│       ├── test_send_binary_op_18.parse-tree-whitequark.exp
│       ├── test_send_binary_op_18.rb
│       ├── test_send_binary_op_19.parse-tree-whitequark.exp
│       ├── test_send_binary_op_19.rb
│       ├── test_send_binary_op_2.parse-tree-whitequark.exp
│       ├── test_send_binary_op_2.rb
│       ├── test_send_binary_op_20.parse-tree-whitequark.exp
│       ├── test_send_binary_op_20.rb
│       ├── test_send_binary_op_3.parse-tree-whitequark.exp
│       ├── test_send_binary_op_3.rb
│       ├── test_send_binary_op_4.parse-tree-whitequark.exp
│       ├── test_send_binary_op_4.rb
│       ├── test_send_binary_op_5.parse-tree-whitequark.exp
│       ├── test_send_binary_op_5.rb
│       ├── test_send_binary_op_6.parse-tree-whitequark.exp
│       ├── test_send_binary_op_6.rb
│       ├── test_send_binary_op_7.parse-tree-whitequark.exp
│       ├── test_send_binary_op_7.rb
│       ├── test_send_binary_op_8.parse-tree-whitequark.exp
│       ├── test_send_binary_op_8.rb
│       ├── test_send_binary_op_9.parse-tree-whitequark.exp
│       ├── test_send_binary_op_9.rb
│       ├── test_send_block_blockarg_0.rb
│       ├── test_send_block_chain_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_0.rb
│       ├── test_send_block_chain_cmd_1.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_1.rb
│       ├── test_send_block_chain_cmd_2.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_2.rb
│       ├── test_send_block_chain_cmd_3.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_3.rb
│       ├── test_send_block_chain_cmd_4.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_4.rb
│       ├── test_send_block_chain_cmd_5.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_5.rb
│       ├── test_send_block_chain_cmd_6.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_6.rb
│       ├── test_send_block_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_block_conditional_0.rb
│       ├── test_send_call_0.parse-tree-whitequark.exp
│       ├── test_send_call_0.rb
│       ├── test_send_call_1.parse-tree-whitequark.exp
│       ├── test_send_call_1.rb
│       ├── test_send_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_conditional_0.rb
│       ├── test_send_index_0.parse-tree-whitequark.exp
│       ├── test_send_index_0.rb
│       ├── test_send_index_asgn_0.parse-tree-whitequark.exp
│       ├── test_send_index_asgn_0.rb
│       ├── test_send_index_asgn_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_index_asgn_legacy_0.rb
│       ├── test_send_index_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_index_cmd_0.rb
│       ├── test_send_index_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_index_legacy_0.rb
│       ├── test_send_lambda_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_0.rb
│       ├── test_send_lambda_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_1.rb
│       ├── test_send_lambda_2.parse-tree-whitequark.exp
│       ├── test_send_lambda_2.rb
│       ├── test_send_lambda_args_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_0.rb
│       ├── test_send_lambda_args_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_1.rb
│       ├── test_send_lambda_args_noparen_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_noparen_0.rb
│       ├── test_send_lambda_args_noparen_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_noparen_1.rb
│       ├── test_send_lambda_args_shadow_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_shadow_0.rb
│       ├── test_send_lambda_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_legacy_0.rb
│       ├── test_send_op_asgn_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_op_asgn_conditional_0.rb
│       ├── test_send_plain_0.parse-tree-whitequark.exp
│       ├── test_send_plain_0.rb
│       ├── test_send_plain_1.parse-tree-whitequark.exp
│       ├── test_send_plain_1.rb
│       ├── test_send_plain_2.parse-tree-whitequark.exp
│       ├── test_send_plain_2.rb
│       ├── test_send_plain_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_0.rb
│       ├── test_send_plain_cmd_1.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_1.rb
│       ├── test_send_plain_cmd_2.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_2.rb
│       ├── test_send_self_0.parse-tree-whitequark.exp
│       ├── test_send_self_0.rb
│       ├── test_send_self_1.parse-tree-whitequark.exp
│       ├── test_send_self_1.rb
│       ├── test_send_self_2.parse-tree-whitequark.exp
│       ├── test_send_self_2.rb
│       ├── test_send_self_block_0.parse-tree-whitequark.exp
│       ├── test_send_self_block_0.rb
│       ├── test_send_self_block_1.parse-tree-whitequark.exp
│       ├── test_send_self_block_1.rb
│       ├── test_send_self_block_2.parse-tree-whitequark.exp
│       ├── test_send_self_block_2.rb
│       ├── test_send_self_block_3.parse-tree-whitequark.exp
│       ├── test_send_self_block_3.rb
│       ├── test_send_unary_op_0.parse-tree-whitequark.exp
│       ├── test_send_unary_op_0.rb
│       ├── test_send_unary_op_1.parse-tree-whitequark.exp
│       ├── test_send_unary_op_1.rb
│       ├── test_send_unary_op_2.parse-tree-whitequark.exp
│       ├── test_send_unary_op_2.rb
│       ├── test_space_args_arg_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_0.rb
│       ├── test_space_args_arg_block_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_0.rb
│       ├── test_space_args_arg_block_1.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_1.rb
│       ├── test_space_args_arg_block_2.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_2.rb
│       ├── test_space_args_arg_call_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_call_0.rb
│       ├── test_space_args_arg_newline_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_newline_0.rb
│       ├── test_space_args_block_0.parse-tree-whitequark.exp
│       ├── test_space_args_block_0.rb
│       ├── test_space_args_cmd_0.parse-tree-whitequark.exp
│       ├── test_space_args_cmd_0.rb
│       ├── test_string_FILE_0.parse-tree-whitequark.exp
│       ├── test_string_FILE_0.rb
│       ├── test_string_concat_0.parse-tree-whitequark.exp
│       ├── test_string_concat_0.rb
│       ├── test_string_dvar_0.parse-tree-whitequark.exp
│       ├── test_string_dvar_0.rb
│       ├── test_string_interp_0.parse-tree-whitequark.exp
│       ├── test_string_interp_0.rb
│       ├── test_string_plain_0.parse-tree-whitequark.exp
│       ├── test_string_plain_0.rb
│       ├── test_string_plain_1.parse-tree-whitequark.exp
│       ├── test_string_plain_1.rb
│       ├── test_super_0.parse-tree-whitequark.exp
│       ├── test_super_0.rb
│       ├── test_super_1.parse-tree-whitequark.exp
│       ├── test_super_1.rb
│       ├── test_super_2.parse-tree-whitequark.exp
│       ├── test_super_2.rb
│       ├── test_super_block_0.parse-tree-whitequark.exp
│       ├── test_super_block_0.rb
│       ├── test_super_block_1.parse-tree-whitequark.exp
│       ├── test_super_block_1.rb
│       ├── test_symbol_interp_0.parse-tree-whitequark.exp
│       ├── test_symbol_interp_0.rb
│       ├── test_symbol_plain_0.parse-tree-whitequark.exp
│       ├── test_symbol_plain_0.rb
│       ├── test_symbol_plain_1.parse-tree-whitequark.exp
│       ├── test_symbol_plain_1.rb
│       ├── test_ternary_0.parse-tree-whitequark.exp
│       ├── test_ternary_0.rb
│       ├── test_ternary_ambiguous_symbol_0.parse-tree-whitequark.exp
│       ├── test_ternary_ambiguous_symbol_0.rb
│       ├── test_true_0.parse-tree-whitequark.exp
│       ├── test_true_0.rb
│       ├── test_unary_num_pow_precedence_0.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_0.rb
│       ├── test_unary_num_pow_precedence_1.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_1.rb
│       ├── test_unary_num_pow_precedence_2.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_2.rb
│       ├── test_undef_0.parse-tree-whitequark.exp
│       ├── test_undef_0.rb
│       ├── test_unknown_percent_str_0.rb
│       ├── test_unless_0.parse-tree-whitequark.exp
│       ├── test_unless_0.rb
│       ├── test_unless_1.parse-tree-whitequark.exp
│       ├── test_unless_1.rb
│       ├── test_unless_else_0.parse-tree-whitequark.exp
│       ├── test_unless_else_0.rb
│       ├── test_unless_else_1.parse-tree-whitequark.exp
│       ├── test_unless_else_1.rb
│       ├── test_unless_mod_0.parse-tree-whitequark.exp
│       ├── test_unless_mod_0.rb
│       ├── test_until_0.parse-tree-whitequark.exp
│       ├── test_until_0.rb
│       ├── test_until_1.parse-tree-whitequark.exp
│       ├── test_until_1.rb
│       ├── test_until_mod_0.parse-tree-whitequark.exp
│       ├── test_until_mod_0.rb
│       ├── test_until_post_0.parse-tree-whitequark.exp
│       ├── test_until_post_0.rb
│       ├── test_var_and_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_and_asgn_0.rb
│       ├── test_var_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_0.rb
│       ├── test_var_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_1.rb
│       ├── test_var_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_2.rb
│       ├── test_var_op_asgn_3.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_3.rb
│       ├── test_var_op_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_cmd_0.rb
│       ├── test_var_op_asgn_keyword_invalid_0.rb
│       ├── test_var_or_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_or_asgn_0.rb
│       ├── test_when_multi_0.parse-tree-whitequark.exp
│       ├── test_when_multi_0.rb
│       ├── test_when_splat_0.parse-tree-whitequark.exp
│       ├── test_when_splat_0.rb
│       ├── test_when_then_0.parse-tree-whitequark.exp
│       ├── test_when_then_0.rb
│       ├── test_while_0.parse-tree-whitequark.exp
│       ├── test_while_0.rb
│       ├── test_while_1.parse-tree-whitequark.exp
│       ├── test_while_1.rb
│       ├── test_while_mod_0.parse-tree-whitequark.exp
│       ├── test_while_mod_0.rb
│       ├── test_while_post_0.parse-tree-whitequark.exp
│       ├── test_while_post_0.rb
│       ├── test_xstring_interp_0.parse-tree-whitequark.exp
│       ├── test_xstring_interp_0.rb
│       ├── test_xstring_plain_0.parse-tree-whitequark.exp
│       ├── test_xstring_plain_0.rb
│       ├── test_yield_0.parse-tree-whitequark.exp
│       ├── test_yield_0.rb
│       ├── test_yield_1.parse-tree-whitequark.exp
│       ├── test_yield_1.rb
│       ├── test_yield_2.parse-tree-whitequark.exp
│       ├── test_yield_2.rb
│       ├── test_yield_3.parse-tree-whitequark.exp
│       ├── test_yield_3.rb
│       ├── test_yield_block_0.rb
│       ├── test_yield_block_1.rb
│       ├── test_zsuper_0.parse-tree-whitequark.exp
│       └── test_zsuper_0.rb
├── third_party
│   ├── BUILD
│   ├── README.md
│   ├── blake2.BUILD
│   ├── clang.BUILD
│   ├── concurrentqueue.BUILD
│   ├── cpp_subprocess.BUILD
│   ├── crcpp.BUILD
│   ├── cxxopts.BUILD
│   ├── emscripten-clang.BUILD
│   ├── emscripten-toolchain.BUILD
│   ├── externals.bzl
│   ├── gems
│   │   ├── BUILD
│   │   ├── build_defs.BUILD
│   │   ├── gemfile.bzl
│   │   ├── gems.BUILD
│   │   ├── known_gems.bzl
│   │   └── rules.bzl
│   ├── gtest.BUILD
│   ├── jemalloc.BUILD
│   ├── libb2.BUILD
│   ├── libprotobuf-mutator.BUILD
│   ├── licenses
│   │   ├── BUILD
│   │   ├── README
│   │   ├── abseil.txt
│   │   ├── blake2.txt
│   │   ├── crcpp.txt
│   │   ├── cxxopts.txt
│   │   ├── googletest.txt
│   │   ├── jemalloc.txt
│   │   ├── libb2.txt
│   │   ├── licenses.h
│   │   ├── lizard.txt
│   │   ├── lmdb.txt
│   │   ├── msgpack-c.txt
│   │   ├── pdqsort.txt
│   │   ├── progressbar.txt
│   │   ├── protobuf.txt
│   │   ├── protobufmutator.txt
│   │   ├── rang.txt
│   │   ├── rapidjson.txt
│   │   ├── spdlog.txt
│   │   ├── statsd-c-client.txt
│   │   ├── tools
│   │   │   └── generate_licenses.cc
│   │   ├── typedruby.txt
│   │   └── yamlcpp.txt
│   ├── lizard.BUILD
│   ├── lmdb.BUILD
│   ├── msgpack.BUILD
│   ├── openssl
│   │   ├── BUILD
│   │   ├── darwin.BUILD
│   │   └── linux.BUILD
│   ├── parser
│   │   ├── BUILD
│   │   ├── README.md
│   │   ├── cc
│   │   │   ├── capi.cc
│   │   │   ├── context.cc
│   │   │   ├── driver.cc
│   │   │   ├── grammars
│   │   │   │   └── typedruby.ypp
│   │   │   ├── lexer.rl
│   │   │   ├── literal.cc
│   │   │   ├── state_stack.cc
│   │   │   └── token.cc
│   │   ├── codegen
│   │   │   ├── builder.rb
│   │   │   └── generate_diagnostics.cc
│   │   └── include/ruby_parser
│   │       ├── builder.hh
│   │       ├── capi.hh
│   │       ├── context.hh
│   │       ├── diagnostic.hh
│   │       ├── driver.hh
│   │       ├── lexer.hh
│   │       ├── literal.hh
│   │       ├── node.hh
│   │       ├── pool.hh
│   │       ├── state_stack.hh
│   │       └── token.hh
│   ├── pdqsort.BUILD
│   ├── progressbar
│   │   ├── BUILD
│   │   ├── README
│   │   ├── progressbar
│   │   │   ├── progressbar.h
│   │   │   └── statusbar.h
│   │   └── src
│   │       ├── progressbar.c
│   │       └── statusbar.c
│   ├── progressbar.BUILD
│   ├── rang.BUILD
│   ├── rapidjson.BUILD
│   ├── ruby
│   │   ├── BUILD
│   │   ├── build-ruby.bzl
│   │   └── ruby.BUILD
│   ├── spdlog.BUILD
│   ├── statsd.BUILD
│   ├── yaml_cpp.BUILD
│   └── zlib.BUILD
├── tools
│   ├── BUILD
│   ├── bazel
│   ├── buildstamp
│   │   └── get_workspace_status
│   ├── clang.bzl
│   ├── config
│   │   └── BUILD
│   ├── scripts
│   │   ├── build_compilation_db.sh
│   │   ├── cfg-view.sh
│   │   ├── check_using_namespace_std.sh
│   │   ├── ci_checks.sh
│   │   ├── format_build_files.sh
│   │   ├── format_cxx.sh
│   │   ├── format_website.sh
│   │   ├── fuzz.sh
│   │   ├── fuzz_minimize_all.sh
│   │   ├── fuzz_minimize_crash.sh
│   │   ├── generate_compdb_targets.sh
│   │   ├── import_whitequark.rb
│   │   ├── import_whitequark.sh
│   │   ├── lint_cxx.sh
│   │   ├── lint_sh.sh
│   │   ├── make_worktree.sh
│   │   ├── regen-emscripten-cache.sh
│   │   ├── try_fast_path_tests.sh
│   │   ├── update-sorbet.run.sh
│   │   ├── update_exp_files.sh
│   │   └── update_testdata_exp.sh
│   └── toolchain
│       ├── webasm-darwin
│       │   ├── BUILD
│       │   ├── ar.sh
│       │   ├── cc_toolchain_config.bzl
│       │   ├── em_cache_existing.tar.gz
│       │   └── emcc.sh
│       └── webasm-linux
│           ├── BUILD
│           ├── ar.sh
│           ├── cc_toolchain_config.bzl
│           ├── em_cache_existing.tar.gz
│           └── emcc.sh
└── website
    ├── README.md
    ├── blog
    │   ├── 2019-05-16-state-of-sorbet-spring-2019.md
    │   ├── 2019-06-20-open-sourcing-sorbet.md
    │   └── 2019-12-20-announcing-sorbet-0.5.md
    ├── core
    │   ├── Footer.js
    │   └── PageSection.js
    ├── docs
    │   ├── abstract.md
    │   ├── adopting.md
    │   ├── attached-class.md
    │   ├── class-of.md
    │   ├── class-types.md
    │   ├── cli.md
    │   ├── error-reference.md
    │   ├── exhaustiveness.md
    │   ├── faq.md
    │   ├── final.md
    │   ├── flow-sensitive.md
    │   ├── gradual.md
    │   ├── intersection-types.md
    │   ├── legal
    │   │   └── trademark-policy.md
    │   ├── metaprogramming-plugins.md
    │   ├── metrics.md
    │   ├── nilable-types.md
    │   ├── non-forcing-constants.md
    │   ├── noreturn.md
    │   ├── override-checking.md
    │   ├── overview.md
    │   ├── procs.md
    │   ├── quickref.md
    │   ├── rbi.md
    │   ├── runtime.md
    │   ├── sealed.md
    │   ├── self-type.md
    │   ├── shapes.md
    │   ├── sigs.md
    │   ├── static.md
    │   ├── stdlib-generics.md
    │   ├── talks
    │   │   ├── curry-on-2019.md
    │   │   ├── jvm-ls-2019.md
    │   │   ├── ruby-conf-2019.md
    │   │   ├── ruby-kaigi-2018.md
    │   │   ├── ruby-kaigi-2019.md
    │   │   └── strange-loop-2018.md
    │   ├── tconfiguration.md
    │   ├── tenum.md
    │   ├── troubleshooting.md
    │   ├── tstruct.md
    │   ├── tuples.md
    │   ├── type-aliases.md
    │   ├── type-annotations.md
    │   ├── type-assertions.md
    │   ├── union-types.md
    │   └── untyped.md
    ├── i18n
    ├── package.json
    ├── pages/en
    │   ├── community.js
    │   └── index.js
    ├── sidebars.json
    ├── siteConfig.js
    ├── static
    │   ├── blog/2019/05/16
    │   │   ├── State-of-Sorbet-May-2019
    │   │   │   └── index.html
    │   │   └── State-of-Sorbet-May-2019.html
    │   ├── css
    │   │   ├── PageSection.css
    │   │   ├── custom.css
    │   │   └── overrides.css
    │   ├── docs
    │   │   ├── attr_reader.html
    │   │   ├── bad-rbi.html
    │   │   ├── include-kernel.html
    │   │   ├── index.html
    │   │   ├── rake.html
    │   │   ├── ruby-3.html
    │   │   └── why-nil.html
    │   ├── img
    │   │   ├── atrium-logo.jpg
    │   │   ├── autocompleteAndDocs1.gif
    │   │   ├── coinbase-logo.png
    │   │   ├── czi-logo.svg
    │   │   ├── editor_autocomplete.gif
    │   │   ├── editor_error_squiggles.gif
    │   │   ├── editor_go_to_definition.gif
    │   │   ├── factorial-logo.png
    │   │   ├── favicon.ico
    │   │   ├── gusto-logo.jpg
    │   │   ├── kickstarter-logo.png
    │   │   ├── shopify-logo.svg
    │   │   ├── sorbet-logo-card@2x.png
    │   │   ├── sorbet-logo-purple-sparkles.svg
    │   │   ├── sorbet-logo-white-sparkles.svg
    │   │   ├── sorbet-logo.svg
    │   │   ├── sourcegraph_github.gif
    │   │   ├── stripe-logo.svg
    │   │   ├── talk-thumb.png
    │   │   ├── testimonial_once_every_never.png
    │   │   ├── testimonial_pair_programming.png
    │   │   └── vonage-logo.png
    │   ├── js
    │   │   └── error-reference.js
    │   ├── slack
    │   │   └── index.html
    │   └── talks
    │       └── index.html
    └── style-guide.md
";
    #[cfg(target_os = "windows")]
    let expected = "\
.
├── ACKNOWLEDGEMENTS.md
├── BUILD
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE
├── NOTICE
├── README.md
├── Rakefile
├── WORKSPACE
├── ast
│   ├── ArgParsing.cc
│   ├── ArgParsing.h
│   ├── BUILD
│   ├── Helpers.cc
│   ├── Helpers.h
│   ├── TreeCopying.cc
│   ├── TreeSanityChecks.cc
│   ├── Trees.cc
│   ├── Trees.h
│   ├── ast.h
│   ├── desugar
│   │   ├── BUILD
│   │   ├── Desugar.cc
│   │   ├── Desugar.h
│   │   └── test
│   │       └── desugar_test.cc
│   ├── substitute
│   │   ├── BUILD
│   │   ├── Substitute.cc
│   │   └── substitute.h
│   ├── treemap
│   │   ├── BUILD
│   │   └── treemap.h
│   └── verifier
│       ├── BUILD
│       ├── Verifier.cc
│       └── verifier.h
├── bazel
├── cfg
│   ├── BUILD
│   ├── CFG.cc
│   ├── CFG.h
│   ├── Instructions.cc
│   ├── Instructions.h
│   └── builder
│       ├── BUILD
│       ├── builder.h
│       ├── builder_entry.cc
│       ├── builder_finalize.cc
│       └── builder_walk.cc
├── class_flatten
│   ├── BUILD
│   ├── class_flatten.cc
│   └── class_flatten.h
├── common
│   ├── BUILD
│   ├── ConstExprStr.h
│   ├── Counters.cc
│   ├── Counters.h
│   ├── Counters_impl.h
│   ├── Exception.h
│   ├── FileOps.h
│   ├── FileSystem.cc
│   ├── FileSystem.h
│   ├── JSON.cc
│   ├── JSON.h
│   ├── Levenstein.cc
│   ├── Levenstein.h
│   ├── Random.cc
│   ├── Random.h
│   ├── Subprocess.cc
│   ├── Subprocess.h
│   ├── Timer.cc
│   ├── Timer.h
│   ├── backtrace.cc
│   ├── common.cc
│   ├── common.h
│   ├── concurrency
│   │   ├── BUILD
│   │   ├── ConcurrentQueue.h
│   │   ├── WorkerPool.h
│   │   ├── WorkerPoolImpl.cc
│   │   └── WorkerPoolImpl.h
│   ├── crypto_hashing
│   │   ├── BUILD
│   │   └── crypto_hashing.h
│   ├── formatting.h
│   ├── kvstore
│   │   ├── BUILD
│   │   ├── KeyValueStore-emscripten.cc
│   │   ├── KeyValueStore.cc
│   │   ├── KeyValueStore.h
│   │   └── test
│   │       └── kvstore_test.cc
│   ├── os
│   │   ├── emscripten.cc
│   │   ├── linux.cc
│   │   ├── mac.cc
│   │   ├── os.cc
│   │   └── os.h
│   ├── sort.h
│   ├── statsd
│   │   ├── BUILD
│   │   ├── statsd-emscripten.cc
│   │   ├── statsd.cc
│   │   └── statsd.h
│   ├── test
│   │   └── common_test.cc
│   ├── typecase.h
│   └── web_tracer_framework
│       ├── BUILD
│       ├── tracing.cc
│       └── tracing.h
├── core
│   ├── AutocorrectSuggestion.cc
│   ├── AutocorrectSuggestion.h
│   ├── BUILD
│   ├── Context.cc
│   ├── Context.h
│   ├── DebugOnlyCheck.h
│   ├── Error.cc
│   ├── Error.h
│   ├── ErrorFlusher.cc
│   ├── ErrorFlusher.h
│   ├── ErrorQueue.cc
│   ├── ErrorQueue.h
│   ├── ErrorQueueMessage.h
│   ├── Files.cc
│   ├── Files.h
│   ├── GlobalState.cc
│   ├── GlobalState.h
│   ├── GlobalSubstitution.h
│   ├── Hashing.h
│   ├── Loc.cc
│   ├── Loc.h
│   ├── LocalVariable.cc
│   ├── LocalVariable.h
│   ├── NameHash.cc
│   ├── NameHash.h
│   ├── NameRef.h
│   ├── Names.cc
│   ├── Names.h
│   ├── StrictLevel.h
│   ├── SymbolRef.h
│   ├── Symbols.cc
│   ├── Symbols.h
│   ├── TypeConstraint.cc
│   ├── TypeConstraint.h
│   ├── TypePtr.h
│   ├── Types.h
│   ├── TypesAndOrigins.cc
│   ├── Unfreeze.cc
│   ├── Unfreeze.h
│   ├── core.h
│   ├── errors
│   │   ├── cfg.h
│   │   ├── desugar.h
│   │   ├── errors.h
│   │   ├── infer.h
│   │   ├── internal.h
│   │   ├── namer.h
│   │   ├── parser.h
│   │   ├── plugin.h
│   │   ├── resolver.h
│   │   └── rewriter.h
│   ├── lsp
│   │   ├── PreemptionTaskManager.cc
│   │   ├── PreemptionTaskManager.h
│   │   ├── Query.cc
│   │   ├── Query.h
│   │   ├── QueryResponse.cc
│   │   ├── QueryResponse.h
│   │   ├── Task.h
│   │   ├── TypecheckEpochManager.cc
│   │   └── TypecheckEpochManager.h
│   ├── proto
│   │   ├── BUILD
│   │   ├── proto.cc
│   │   └── proto.h
│   ├── serialize
│   │   ├── BUILD
│   │   ├── pickler.h
│   │   ├── serialize.cc
│   │   ├── serialize.h
│   │   └── test
│   │       └── serialize_test.cc
│   ├── test
│   │   └── core_test.cc
│   ├── tools
│   │   └── generate_names.cc
│   └── types
│       ├── calls.cc
│       ├── printing.cc
│       ├── subtyping.cc
│       ├── typemaps.cc
│       └── types.cc
├── definition_validator
│   ├── BUILD
│   ├── validator.cc
│   ├── validator.h
│   ├── variance.cc
│   └── variance.h
├── docs
│   ├── JRuby.md
│   ├── README.md
│   ├── compressors.md
│   ├── img
│   │   ├── cfg-example.svg
│   │   ├── chrome-tracing-load-button.png
│   │   ├── chrome-tracing-loaded.png
│   │   ├── chrome-tracing-pipeline.png
│   │   ├── chrome-tracing-scrolled.png
│   │   ├── chrome-tracing-typecheck-one.png
│   │   └── sorbet-pipeline.monopic
│   ├── internals.md
│   ├── logo
│   │   ├── README.md
│   │   ├── sorbet-logo-45-deg@1x.png
│   │   ├── sorbet-logo-45-deg@2x.png
│   │   ├── sorbet-logo-monochrome.svg
│   │   ├── sorbet-logo-purple-sparkles.svg
│   │   ├── sorbet-logo-white-sparkles-on-purple.svg
│   │   ├── sorbet-logo-white-sparkles.svg
│   │   ├── sorbet-logo.sketch
│   │   ├── sorbet-logo.svg
│   │   └── sorbet-logo@2x.png
│   ├── pipeline.md
│   ├── suggest-sig.md
│   └── tracing.md
├── emscripten
│   ├── BUILD
│   └── main.cc
├── gems
│   ├── sorbet
│   │   ├── BUILD
│   │   ├── Gemfile
│   │   ├── README.md
│   │   ├── Rakefile
│   │   ├── bin
│   │   │   ├── srb
│   │   │   └── srb-rbi
│   │   ├── lib
│   │   │   ├── constant_cache.rb
│   │   │   ├── create-config.rb
│   │   │   ├── fetch-rbis.rb
│   │   │   ├── find-gem-rbis.rb
│   │   │   ├── gem-generator-tracepoint
│   │   │   │   ├── tracepoint_serializer.rb
│   │   │   │   └── tracer.rb
│   │   │   ├── gem-generator-tracepoint.rb
│   │   │   ├── gem_loader.rb
│   │   │   ├── gem_loader.rbi
│   │   │   ├── hidden-definition-finder.rb
│   │   │   ├── real_stdlib.rb
│   │   │   ├── require_everything.rb
│   │   │   ├── serialize.rb
│   │   │   ├── status.rb
│   │   │   ├── step_interface.rb
│   │   │   ├── suggest-typed.rb
│   │   │   ├── t.rb
│   │   │   └── todo-rbi.rb
│   │   ├── sorbet.gemspec
│   │   └── test
│   │       ├── hidden-method-finder
│   │       │   ├── BUILD
│   │       │   ├── check_one_bazel.sh
│   │       │   ├── hidden-method-tests.rb
│   │       │   ├── hidden_methods.bzl
│   │       │   ├── hidden_methods_bazel.sh
│   │       │   ├── logging.sh
│   │       │   ├── shims.rb.source
│   │       │   ├── simple
│   │       │   │   ├── expectations.json
│   │       │   │   ├── ruby_2_4_hidden.rbi.exp
│   │       │   │   ├── ruby_2_6_hidden.rbi.exp
│   │       │   │   └── src
│   │       │   │       ├── Gemfile
│   │       │   │       ├── simple.rb
│   │       │   │       └── sorbet
│   │       │   │           └── config
│   │       │   ├── thorough
│   │       │   │   ├── expectations.json
│   │       │   │   ├── ruby_2_4_hidden.rbi.exp
│   │       │   │   ├── ruby_2_6_hidden.rbi.exp
│   │       │   │   └── src
│   │       │   │       ├── Gemfile
│   │       │   │       ├── sorbet
│   │       │   │       │   ├── config
│   │       │   │       │   └── rbi\\hidden-definitions
│   │       │   │       │       └── errors.txt
│   │       │   │       └── thorough.rb
│   │       │   └── update_hidden_methods_exp.sh
│   │       └── snapshot
│   │           ├── BUILD
│   │           ├── check_one.sh
│   │           ├── hermetic_tar.sh
│   │           ├── logging.sh
│   │           ├── partial
│   │           │   ├── bad-hash
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── src.rb
│   │           │   ├── bad-t\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── bad_gem
│   │           │   │   ├── expected
│   │           │   │   │   ├── out.log
│   │           │   │   │   └── sorbet
│   │           │   │   │       └── config
│   │           │   │   ├── sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── bad-gem.gemspec
│   │           │   │       ├── lib
│   │           │   │       │   └── bad-gem.rb
│   │           │   │       └── src.rb
│   │           │   ├── codecov\\src
│   │           │   │   ├── Gemfile
│   │           │   │   └── Gemfile.lock
│   │           │   ├── create-config
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── db_schema
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   └── foo.txt
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── db
│   │           │   │       │   └── schema.rb
│   │           │   │       └── sorbet
│   │           │   │           └── foo.txt
│   │           │   ├── explosive-object\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── extconf
│   │           │   │   ├── expected
│   │           │   │   │   └── out.log
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── lib
│   │           │   │           └── extconf.rb
│   │           │   ├── fake-object\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── fake-rails\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── config
│   │           │   │       └── application.rb
│   │           │   ├── ignore_file_table
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   ├── config
│   │           │   │   │   └── important_file.txt
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── bad.rb
│   │           │   │       └── sorbet
│   │           │   │           ├── config
│   │           │   │           └── important_file.txt
│   │           │   ├── local_gem
│   │           │   │   ├── expected\\sorbet\\rbi\\gems
│   │           │   │   │   └── my_gem.rbi
│   │           │   │   ├── gems\\0.0.0\\gems\\my_gem-0.0.0
│   │           │   │   │   ├── lib
│   │           │   │   │   │   └── my_gem.rb
│   │           │   │   │   └── my_gem.gemspec
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── local_rvm_gemset_gem
│   │           │   │   ├── expected\\sorbet\\rbi\\gems
│   │           │   │   │   └── my_gem.rbi
│   │           │   │   ├── gems\\ruby-0.0.0@gemset\\gems\\my_gem-0.0.0
│   │           │   │   │   ├── lib
│   │           │   │   │   │   └── my_gem.rb
│   │           │   │   │   └── my_gem.gemspec
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   ├── missing-instance-methods\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── non-utf-8-file
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       └── src.rb
│   │           │   ├── rails-double-require
│   │           │   │   ├── expected
│   │           │   │   │   └── err.log
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── app\\models
│   │           │   │       │   └── article.rb
│   │           │   │       └── config
│   │           │   │           ├── application.rb
│   │           │   │           └── database.yml
│   │           │   ├── rails6\\src
│   │           │   │   ├── Gemfile
│   │           │   │   └── Gemfile.lock
│   │           │   ├── real_singleton_class\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── rspec-lots\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── stack_master\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── stupidedi\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── type_member\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── my_enumerable.rb
│   │           │   ├── typed-ignore\\src
│   │           │   │   ├── Gemfile
│   │           │   │   ├── Gemfile.lock
│   │           │   │   └── src.rb
│   │           │   ├── use-existing-config
│   │           │   │   ├── expected\\sorbet
│   │           │   │   │   └── config
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       ├── Gemfile.lock
│   │           │   │       ├── foo.rb
│   │           │   │       └── sorbet
│   │           │   │           └── config
│   │           │   └── webmock\\src
│   │           │       ├── Gemfile
│   │           │       ├── Gemfile.lock
│   │           │       └── webmock.rb
│   │           ├── run_one.sh
│   │           ├── snapshot.bzl
│   │           ├── sorbet-typed.rev
│   │           ├── total
│   │           │   ├── empty
│   │           │   │   ├── expected
│   │           │   │   │   ├── err.log
│   │           │   │   │   ├── out.log
│   │           │   │   │   └── sorbet
│   │           │   │   │       ├── config
│   │           │   │   │       └── rbi\\sorbet-typed\\lib
│   │           │   │   │           ├── bundler\\all
│   │           │   │   │           │   └── bundler.rbi
│   │           │   │   │           └── ruby\\all
│   │           │   │   │               ├── gem.rbi
│   │           │   │   │               ├── open3.rbi
│   │           │   │   │               └── resolv.rbi
│   │           │   │   └── src
│   │           │   │       ├── Gemfile
│   │           │   │       └── Gemfile.lock
│   │           │   └── sorbet-runtime
│   │           │       ├── expected
│   │           │       │   ├── err.log
│   │           │       │   ├── out.log
│   │           │       │   └── sorbet
│   │           │       │       ├── config
│   │           │       │       └── rbi\\sorbet-typed\\lib
│   │           │       │           ├── bundler\\all
│   │           │       │           │   └── bundler.rbi
│   │           │       │           └── ruby\\all
│   │           │       │               ├── gem.rbi
│   │           │       │               ├── open3.rbi
│   │           │       │               └── resolv.rbi
│   │           │       └── src
│   │           │           ├── Gemfile
│   │           │           └── Gemfile.lock
│   │           ├── update_one.sh
│   │           └── validate_utils.sh
│   ├── sorbet-runtime
│   │   ├── BUILD
│   │   ├── Gemfile
│   │   ├── README.md
│   │   ├── Rakefile
│   │   ├── bench
│   │   │   ├── constructor.rb
│   │   │   ├── deserialize.rb
│   │   │   ├── getters.rb
│   │   │   ├── prop_definition.rb
│   │   │   ├── setters.rb
│   │   │   └── tasks.rb
│   │   ├── lib
│   │   │   ├── sorbet-runtime.rb
│   │   │   └── types
│   │   │       ├── _types.rb
│   │   │       ├── abstract_utils.rb
│   │   │       ├── boolean.rb
│   │   │       ├── compatibility_patches.rb
│   │   │       ├── configuration.rb
│   │   │       ├── enum.rb
│   │   │       ├── generic.rb
│   │   │       ├── helpers.rb
│   │   │       ├── interface_wrapper.rb
│   │   │       ├── non_forcing_constants.rb
│   │   │       ├── private
│   │   │       │   ├── abstract
│   │   │       │   │   ├── data.rb
│   │   │       │   │   ├── declare.rb
│   │   │       │   │   ├── hooks.rb
│   │   │       │   │   └── validate.rb
│   │   │       │   ├── casts.rb
│   │   │       │   ├── class_utils.rb
│   │   │       │   ├── decl_state.rb
│   │   │       │   ├── final.rb
│   │   │       │   ├── methods
│   │   │       │   │   ├── _methods.rb
│   │   │       │   │   ├── call_validation.rb
│   │   │       │   │   ├── decl_builder.rb
│   │   │       │   │   ├── modes.rb
│   │   │       │   │   ├── signature.rb
│   │   │       │   │   └── signature_validation.rb
│   │   │       │   ├── mixins
│   │   │       │   │   └── mixins.rb
│   │   │       │   ├── runtime_levels.rb
│   │   │       │   ├── sealed.rb
│   │   │       │   └── types
│   │   │       │       ├── not_typed.rb
│   │   │       │       ├── string_holder.rb
│   │   │       │       ├── type_alias.rb
│   │   │       │       └── void.rb
│   │   │       ├── profile.rb
│   │   │       ├── props
│   │   │       │   ├── _props.rb
│   │   │       │   ├── constructor.rb
│   │   │       │   ├── custom_type.rb
│   │   │       │   ├── decorator.rb
│   │   │       │   ├── errors.rb
│   │   │       │   ├── generated_code_validation.rb
│   │   │       │   ├── has_lazily_specialized_methods.rb
│   │   │       │   ├── optional.rb
│   │   │       │   ├── plugin.rb
│   │   │       │   ├── pretty_printable.rb
│   │   │       │   ├── private
│   │   │       │   │   ├── apply_default.rb
│   │   │       │   │   ├── deserializer_generator.rb
│   │   │       │   │   ├── parser.rb
│   │   │       │   │   ├── serde_transform.rb
│   │   │       │   │   ├── serializer_generator.rb
│   │   │       │   │   └── setter_factory.rb
│   │   │       │   ├── serializable.rb
│   │   │       │   ├── type_validation.rb
│   │   │       │   ├── utils.rb
│   │   │       │   └── weak_constructor.rb
│   │   │       ├── runtime_profiled.rb
│   │   │       ├── sig.rb
│   │   │       ├── struct.rb
│   │   │       ├── types
│   │   │       │   ├── attached_class.rb
│   │   │       │   ├── base.rb
│   │   │       │   ├── class_of.rb
│   │   │       │   ├── enum.rb
│   │   │       │   ├── fixed_array.rb
│   │   │       │   ├── fixed_hash.rb
│   │   │       │   ├── intersection.rb
│   │   │       │   ├── noreturn.rb
│   │   │       │   ├── proc.rb
│   │   │       │   ├── self_type.rb
│   │   │       │   ├── simple.rb
│   │   │       │   ├── t_enum.rb
│   │   │       │   ├── type_member.rb
│   │   │       │   ├── type_parameter.rb
│   │   │       │   ├── type_template.rb
│   │   │       │   ├── type_variable.rb
│   │   │       │   ├── typed_array.rb
│   │   │       │   ├── typed_enumerable.rb
│   │   │       │   ├── typed_enumerator.rb
│   │   │       │   ├── typed_hash.rb
│   │   │       │   ├── typed_range.rb
│   │   │       │   ├── typed_set.rb
│   │   │       │   ├── union.rb
│   │   │       │   └── untyped.rb
│   │   │       └── utils.rb
│   │   ├── sorbet-runtime.gemspec
│   │   └── test
│   │       ├── pay-server-shims.rbi
│   │       ├── test_helper.rb
│   │       ├── typecheck-runtime.sh
│   │       └── types
│   │           ├── abstract_validation.rb
│   │           ├── absurd.rb
│   │           ├── attached_class.rb
│   │           ├── builder_syntax.rb
│   │           ├── casts.rb
│   │           ├── configuration.rb
│   │           ├── edge_cases.rb
│   │           ├── enum.rb
│   │           ├── final_method.rb
│   │           ├── final_module.rb
│   │           ├── fixtures
│   │           │   ├── always_raise.rb
│   │           │   └── sealed_module
│   │           │       ├── forbid_sealed_class__1.rb
│   │           │       ├── forbid_sealed_class__2.rb
│   │           │       ├── forbid_sealed_class__3.rb
│   │           │       ├── forbid_sealed_module_extend__1.rb
│   │           │       ├── forbid_sealed_module_extend__2.rb
│   │           │       ├── forbid_sealed_module_extend__3.rb
│   │           │       ├── forbid_sealed_module_include__1.rb
│   │           │       ├── forbid_sealed_module_include__2.rb
│   │           │       ├── forbid_sealed_module_include__3.rb
│   │           │       ├── sealed_abstract__1.rb
│   │           │       ├── sealed_abstract__2.rb
│   │           │       ├── sealed_abstract__3.rb
│   │           │       ├── whitelist_violation__1.rb
│   │           │       └── whitelist_violation__2.rb
│   │           ├── interface_validation.rb
│   │           ├── interface_wrapper.rb
│   │           ├── method_modes.rb
│   │           ├── method_patches.rb
│   │           ├── method_validation.rb
│   │           ├── mixins.rb
│   │           ├── must.rb
│   │           ├── non_forcing_constants.rb
│   │           ├── props
│   │           │   ├── _props.rb
│   │           │   ├── constructor.rb
│   │           │   ├── decorator.rb
│   │           │   ├── optional.rb
│   │           │   ├── private
│   │           │   │   └── setter_factory.rb
│   │           │   ├── serializable.rb
│   │           │   └── struct.rb
│   │           ├── returns.rb
│   │           ├── sealed_module.rb
│   │           ├── sig.rb
│   │           ├── struct.rb
│   │           ├── types.rb
│   │           ├── types_to_ruby.rb
│   │           ├── validate_override_shape.rb
│   │           └── validate_override_types.rb
│   └── sorbet-static
│       └── sorbet-static.gemspec
├── infer
│   ├── BUILD
│   ├── SigSuggestion.cc
│   ├── SigSuggestion.h
│   ├── environment.cc
│   ├── environment.h
│   ├── infer.h
│   ├── inference.cc
│   ├── inference.h
│   └── test
│       └── infer_test.cc
├── local_vars
│   ├── BUILD
│   ├── local_vars.cc
│   └── local_vars.h
├── main
│   ├── BUILD
│   ├── autogen
│   │   ├── BUILD
│   │   ├── autogen.cc
│   │   ├── autogen.h
│   │   ├── autoloader.cc
│   │   ├── autoloader.h
│   │   ├── subclasses.cc
│   │   └── subclasses.h
│   ├── cache
│   │   ├── BUILD
│   │   ├── cache-orig.cc
│   │   ├── cache.cc
│   │   └── cache.h
│   ├── lsp
│   │   ├── BUILD
│   │   ├── DefLocSaver.cc
│   │   ├── DefLocSaver.h
│   │   ├── ErrorReporter.cc
│   │   ├── ErrorReporter.h
│   │   ├── LSPConfiguration.cc
│   │   ├── LSPConfiguration.h
│   │   ├── LSPFileUpdates.cc
│   │   ├── LSPFileUpdates.h
│   │   ├── LSPIndexer.cc
│   │   ├── LSPIndexer.h
│   │   ├── LSPInput.cc
│   │   ├── LSPInput.h
│   │   ├── LSPMessage.cc
│   │   ├── LSPMessage.h
│   │   ├── LSPOutput.cc
│   │   ├── LSPOutput.h
│   │   ├── LSPPreprocessor.cc
│   │   ├── LSPPreprocessor.h
│   │   ├── LSPTask.cc
│   │   ├── LSPTask.h
│   │   ├── LSPTypechecker.cc
│   │   ├── LSPTypechecker.h
│   │   ├── LSPTypecheckerCoordinator.cc
│   │   ├── LSPTypecheckerCoordinator.h
│   │   ├── LocalVarFinder.cc
│   │   ├── LocalVarFinder.h
│   │   ├── LocalVarSaver.cc
│   │   ├── LocalVarSaver.h
│   │   ├── NextMethodFinder.cc
│   │   ├── NextMethodFinder.h
│   │   ├── ShowOperation.cc
│   │   ├── ShowOperation.h
│   │   ├── UndoState.cc
│   │   ├── UndoState.h
│   │   ├── json_enums.h
│   │   ├── json_types.cc
│   │   ├── json_types.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── lsp_helpers.cc
│   │   ├── lsp_messages_gen_helpers.cc
│   │   ├── lsp_messages_gen_helpers.h
│   │   ├── notifications
│   │   │   ├── cancel_request.cc
│   │   │   ├── cancel_request.h
│   │   │   ├── exit.cc
│   │   │   ├── exit.h
│   │   │   ├── initialized.cc
│   │   │   ├── initialized.h
│   │   │   ├── notifications.h
│   │   │   ├── sorbet_fence.cc
│   │   │   ├── sorbet_fence.h
│   │   │   ├── sorbet_pause.cc
│   │   │   ├── sorbet_pause.h
│   │   │   ├── sorbet_resume.cc
│   │   │   ├── sorbet_resume.h
│   │   │   ├── sorbet_workspace_edit.cc
│   │   │   └── sorbet_workspace_edit.h
│   │   ├── protocol.cc
│   │   ├── request_dispatch.cc
│   │   ├── requests
│   │   │   ├── code_action.cc
│   │   │   ├── code_action.h
│   │   │   ├── completion.cc
│   │   │   ├── completion.h
│   │   │   ├── definition.cc
│   │   │   ├── definition.h
│   │   │   ├── document_highlight.cc
│   │   │   ├── document_highlight.h
│   │   │   ├── document_symbol.cc
│   │   │   ├── document_symbol.h
│   │   │   ├── get_counters.cc
│   │   │   ├── get_counters.h
│   │   │   ├── hover.cc
│   │   │   ├── hover.h
│   │   │   ├── initialize.cc
│   │   │   ├── initialize.h
│   │   │   ├── references.cc
│   │   │   ├── references.h
│   │   │   ├── requests.h
│   │   │   ├── shutdown.cc
│   │   │   ├── shutdown.h
│   │   │   ├── signature_help.cc
│   │   │   ├── signature_help.h
│   │   │   ├── sorbet_error.cc
│   │   │   ├── sorbet_error.h
│   │   │   ├── sorbet_read_file.cc
│   │   │   ├── sorbet_read_file.h
│   │   │   ├── type_definition.cc
│   │   │   ├── type_definition.h
│   │   │   ├── workspace_symbols.cc
│   │   │   └── workspace_symbols.h
│   │   ├── test
│   │   │   ├── error_reporter_test.cc
│   │   │   ├── generate_lsp_messages_test.cc
│   │   │   ├── lsp_file_updates_test.cc
│   │   │   ├── lsp_preprocessor_test.cc
│   │   │   └── lsp_test.cc
│   │   ├── tools
│   │   │   ├── generate_lsp_messages.cc
│   │   │   ├── generate_lsp_messages.h
│   │   │   ├── make_lsp_types.cc
│   │   │   └── make_lsp_types.h
│   │   ├── watchman
│   │   │   ├── WatchmanProcess.cc
│   │   │   └── WatchmanProcess.h
│   │   ├── wrapper.cc
│   │   └── wrapper.h
│   ├── main.cc
│   ├── options
│   │   ├── BUILD
│   │   ├── ConfigParser.cc
│   │   ├── ConfigParser.h
│   │   ├── options.cc
│   │   ├── options.h
│   │   └── test
│   │       └── options_test.cc
│   ├── pipeline
│   │   ├── BUILD
│   │   ├── ProgressIndicator.cc
│   │   ├── ProgressIndicator.h
│   │   ├── pipeline.cc
│   │   ├── pipeline.h
│   │   └── semantic_extension
│   │       ├── BUILD
│   │       ├── DefaultImplementation.cc
│   │       └── SemanticExtension.h
│   ├── realmain.cc
│   └── realmain.h
├── namer
│   ├── BUILD
│   ├── configatron
│   │   ├── BUILD
│   │   ├── configatron.cc
│   │   └── configatron.h
│   ├── namer.cc
│   ├── namer.h
│   └── test
│       └── namer_test.cc
├── parser
│   ├── BUILD
│   ├── Builder.cc
│   ├── Builder.h
│   ├── Dedenter.h
│   ├── Node.cc
│   ├── Node.h
│   ├── Parser.cc
│   ├── parser.h
│   ├── test
│   │   └── parser_test.cc
│   └── tools
│       └── generate_ast.cc
├── payload
│   ├── BUILD
│   ├── binary
│   │   ├── BUILD
│   │   ├── binary.h
│   │   ├── empty.cc
│   │   └── tools
│   │       └── gen_state_payload.cc
│   ├── otherwise_compdb_bugs_out.cc
│   ├── payload.cc
│   ├── payload.h
│   └── text
│       ├── BUILD
│       ├── nopopulate.cc
│       ├── populate.cc
│       ├── text.h
│       └── tools
│           └── generate_payload.cc
├── plugin
│   ├── BUILD
│   ├── Plugins.cc
│   ├── Plugins.h
│   ├── SubprocessTextPlugin.cc
│   └── SubprocessTextPlugin.h
├── proto
│   ├── BUILD
│   ├── File.proto
│   ├── Loc.proto
│   ├── Name.proto
│   ├── Symbol.proto
│   ├── Type.proto
│   └── pay-server
│       ├── BUILD
│       ├── README
│       └── SourceMetrics.proto
├── rbi
│   ├── BUILD
│   ├── core
│   │   ├── argf.rbi
│   │   ├── array.rbi
│   │   ├── basic_object.rbi
│   │   ├── binding.rbi
│   │   ├── class.rbi
│   │   ├── comparable.rbi
│   │   ├── complex.rbi
│   │   ├── constants.rbi
│   │   ├── data.rbi
│   │   ├── dir.rbi
│   │   ├── encoding.rbi
│   │   ├── enum.rbi
│   │   ├── enumerable.rbi
│   │   ├── enumerator.rbi
│   │   ├── errno.rbi
│   │   ├── errors.rbi
│   │   ├── exception.rbi
│   │   ├── false_class.rbi
│   │   ├── fiber.rbi
│   │   ├── fiber_error.rbi
│   │   ├── file.rbi
│   │   ├── file_test.rbi
│   │   ├── fixnum.rbi
│   │   ├── float.rbi
│   │   ├── gc.rbi
│   │   ├── hash.rbi
│   │   ├── integer.rbi
│   │   ├── io.rbi
│   │   ├── kernel.rbi
│   │   ├── marshal.rbi
│   │   ├── match_data.rbi
│   │   ├── math.rbi
│   │   ├── method.rbi
│   │   ├── module.rbi
│   │   ├── nil_class.rbi
│   │   ├── numeric.rbi
│   │   ├── object.rbi
│   │   ├── proc.rbi
│   │   ├── process.rbi
│   │   ├── random.rbi
│   │   ├── range.rbi
│   │   ├── rational.rbi
│   │   ├── rb_config.rbi
│   │   ├── regexp.rbi
│   │   ├── ruby_vm.rbi
│   │   ├── signal.rbi
│   │   ├── string.rbi
│   │   ├── struct.rbi
│   │   ├── symbol.rbi
│   │   ├── thread.rbi
│   │   ├── thread_group.rbi
│   │   ├── time.rbi
│   │   ├── trace_point.rbi
│   │   ├── true_class.rbi
│   │   ├── unbound_method.rbi
│   │   └── warning.rbi
│   ├── gems
│   │   ├── bundler.rbi
│   │   ├── chalk.rbi
│   │   ├── configatron.rbi
│   │   ├── didyoumean.rbi
│   │   ├── msgpack.rbi
│   │   └── stringscanner.rbi
│   ├── sorbet
│   │   ├── builder.rbi
│   │   ├── compatibility_patches.rbi
│   │   ├── sorbet.rbi
│   │   ├── t.rbi
│   │   ├── tprivate.rbi
│   │   ├── tprops.rbi
│   │   └── ttypes.rbi
│   ├── stdlib
│   │   ├── abbrev.rbi
│   │   ├── base64.rbi
│   │   ├── benchmark.rbi
│   │   ├── big_math.rbi
│   │   ├── bigdecimal.rbi
│   │   ├── cgi.rbi
│   │   ├── coverage.rbi
│   │   ├── csv.rbi
│   │   ├── date.rbi
│   │   ├── date_time.rbi
│   │   ├── delegator.rbi
│   │   ├── digest.rbi
│   │   ├── dir.rbi
│   │   ├── e2mmap.rbi
│   │   ├── erb.rbi
│   │   ├── etc.rbi
│   │   ├── fileutils.rbi
│   │   ├── forwardable.rbi
│   │   ├── gem.rbi
│   │   ├── json.rbi
│   │   ├── logger.rbi
│   │   ├── monitor.rbi
│   │   ├── mutex_m.rbi
│   │   ├── net.rbi
│   │   ├── objspace.rbi
│   │   ├── open3.rbi
│   │   ├── open_struct.rbi
│   │   ├── openssl.rbi
│   │   ├── optparse.rbi
│   │   ├── pathname.rbi
│   │   ├── pp.rbi
│   │   ├── racc.rbi
│   │   ├── ripper.rbi
│   │   ├── set.rbi
│   │   ├── singleton.rbi
│   │   ├── socket.rbi
│   │   ├── stringio.rbi
│   │   ├── tempfile.rbi
│   │   ├── time.rbi
│   │   ├── timeout.rbi
│   │   ├── tsort.rbi
│   │   ├── uri.rbi
│   │   └── webrick.rbi
│   └── tools
│       ├── generate_procs.cc
│       └── sync-rdoc.rb
├── resolver
│   ├── BUILD
│   ├── CorrectTypeAlias.cc
│   ├── CorrectTypeAlias.h
│   ├── GlobalPass.cc
│   ├── resolver.cc
│   ├── resolver.h
│   ├── type_syntax.cc
│   └── type_syntax.h
├── rewriter
│   ├── AttrReader.cc
│   ├── AttrReader.h
│   ├── BUILD
│   ├── ClassNew.cc
│   ├── ClassNew.h
│   ├── Cleanup.cc
│   ├── Cleanup.h
│   ├── Command.cc
│   ├── Command.h
│   ├── DSLBuilder.cc
│   ├── DSLBuilder.h
│   ├── DefaultArgs.cc
│   ├── DefaultArgs.h
│   ├── Delegate.cc
│   ├── Delegate.h
│   ├── Flatfiles.cc
│   ├── Flatfiles.h
│   ├── Flatten.cc
│   ├── Flatten.h
│   ├── Initializer.cc
│   ├── Initializer.h
│   ├── InterfaceWrapper.cc
│   ├── InterfaceWrapper.h
│   ├── Mattr.cc
│   ├── Mattr.h
│   ├── Minitest.cc
│   ├── Minitest.h
│   ├── MixinEncryptedProp.cc
│   ├── MixinEncryptedProp.h
│   ├── ModuleFunction.cc
│   ├── ModuleFunction.h
│   ├── Private.cc
│   ├── Private.h
│   ├── Prop.cc
│   ├── Prop.h
│   ├── ProtobufDescriptorPool.cc
│   ├── ProtobufDescriptorPool.h
│   ├── Rails.cc
│   ├── Rails.h
│   ├── Regexp.cc
│   ├── Regexp.h
│   ├── SelfNew.cc
│   ├── SelfNew.h
│   ├── SigRewriter.cc
│   ├── SigRewriter.h
│   ├── Struct.cc
│   ├── Struct.h
│   ├── TEnum.cc
│   ├── TEnum.h
│   ├── TypeMembers.cc
│   ├── TypeMembers.h
│   ├── Util.cc
│   ├── Util.h
│   ├── rewriter.cc
│   └── rewriter.h
├── sorbet_version
│   ├── BUILD
│   ├── sorbet_version.c
│   └── sorbet_version.h
├── test
│   ├── BUILD
│   ├── LSPTest.cc
│   ├── LSPTest.h
│   ├── autocorrect-test.cc
│   ├── backtrace-test-error.cc
│   ├── backtrace-test-raise.cc
│   ├── backtrace-test.sh
│   ├── cli
│   │   ├── BUILD
│   │   ├── allowed-extension
│   │   │   ├── allowed-extension.out
│   │   │   ├── allowed-extension.sh
│   │   │   └── lib
│   │   │       ├── a.rb
│   │   │       ├── b.rbi
│   │   │       ├── c.ru
│   │   │       └── d.rake
│   │   ├── arity-messages
│   │   │   ├── arity-messages.out
│   │   │   ├── arity-messages.rb
│   │   │   └── arity-messages.sh
│   │   ├── at
│   │   │   ├── at.input
│   │   │   ├── at.out
│   │   │   ├── at.rb
│   │   │   └── at.sh
│   │   ├── autocorrect
│   │   │   ├── autocorrect.out
│   │   │   ├── autocorrect.rb
│   │   │   └── autocorrect.sh
│   │   ├── autocorrect-abstract
│   │   │   ├── autocorrect-abstract.out
│   │   │   ├── autocorrect-abstract.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autocorrect-attached-class
│   │   │   ├── autocorrect-attached-class.out
│   │   │   ├── autocorrect-attached-class.rb
│   │   │   └── autocorrect-attached-class.sh
│   │   ├── autocorrect-bare-stdlib-generics
│   │   │   ├── autocorrect-bare-stdlib-generics.out
│   │   │   ├── autocorrect-bare-stdlib-generics.rb
│   │   │   └── autocorrect-bare-stdlib-generics.sh
│   │   ├── autocorrect-extend
│   │   │   ├── autocorrect-extend.out
│   │   │   ├── autocorrect-extend.rb
│   │   │   └── autocorrect-extend.sh
│   │   ├── autocorrect-lazy-type-alias
│   │   │   ├── autocorrect-lazy-type-alias.out
│   │   │   ├── autocorrect-lazy-type-alias.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autocorrect-private
│   │   │   ├── autocorrect-private.out
│   │   │   ├── autocorrect-private.rb
│   │   │   └── autocorrect-private.sh
│   │   ├── autocorrect-remove-body
│   │   │   ├── autocorrect-remove-body.out
│   │   │   ├── autocorrect-remove-body.sh
│   │   │   ├── post.rb
│   │   │   ├── post.rbi
│   │   │   ├── pre.rb
│   │   │   └── pre.rbi
│   │   ├── autocorrect-same-loc
│   │   │   ├── autocorrect-same-loc-1.rb
│   │   │   ├── autocorrect-same-loc-2.rb
│   │   │   ├── autocorrect-same-loc.out
│   │   │   └── autocorrect-same-loc.sh
│   │   ├── autocorrect-strict
│   │   │   ├── autocorrect-strict.out
│   │   │   ├── autocorrect-strict.sh
│   │   │   ├── post.rb
│   │   │   └── pre.rb
│   │   ├── autogen-autoloader
│   │   │   ├── autogen-autoloader.out
│   │   │   ├── autogen-autoloader.sh
│   │   │   ├── bar.rb
│   │   │   ├── bar2.rb
│   │   │   ├── errors.rb
│   │   │   ├── foo.rb
│   │   │   ├── inplace.rb
│   │   │   └── scripts
│   │   │       └── baz.rb
│   │   ├── autogen-classlist
│   │   │   ├── a.rb
│   │   │   ├── autogen-classlist.out
│   │   │   ├── autogen-classlist.sh
│   │   │   └── b.rb
│   │   ├── autogen-errors
│   │   │   ├── autogen-errors.out
│   │   │   ├── autogen-errors.rb
│   │   │   └── autogen-errors.sh
│   │   ├── autogen-subclasses
│   │   │   ├── a.rb
│   │   │   ├── autogen-subclasses.out
│   │   │   └── autogen-subclasses.sh
│   │   ├── autogen-subclasses-ignore
│   │   │   ├── autogen-subclasses-ignore.out
│   │   │   ├── autogen-subclasses-ignore.sh
│   │   │   ├── ignored
│   │   │   │   └── ignored.rb
│   │   │   └── not-ignored
│   │   │       └── not-ignored.rb
│   │   ├── backtrace
│   │   │   ├── backtrace.out
│   │   │   └── backtrace.sh
│   │   ├── bad-perm
│   │   │   ├── bad-perm.out
│   │   │   └── bad-perm.sh
│   │   ├── bad-plugin-spec
│   │   │   ├── bad-plugin-spec.out
│   │   │   ├── bad-plugin-spec.sh
│   │   │   ├── duplicate-triggers.yaml
│   │   │   ├── missing-triggers.yaml
│   │   │   ├── non-string-in-ruby-extra-args.yaml
│   │   │   ├── not-map.yaml
│   │   │   ├── ruby-extra-args-not-array.yaml
│   │   │   ├── triggers-not-map.yaml
│   │   │   └── values-not-scalar.yaml
│   │   ├── cache-add-typed
│   │   │   ├── cache-add-typed.out
│   │   │   └── cache-add-typed.sh
│   │   ├── cache-doesnt-parse
│   │   │   ├── cache-doesnt-parse.out
│   │   │   └── cache-doesnt-parse.sh
│   │   ├── cache-dsl
│   │   │   ├── attr_accessor.rb
│   │   │   ├── cache-dsl.out
│   │   │   └── cache-dsl.sh
│   │   ├── cache-keeps-print-options
│   │   │   ├── cache-keeps-print-options.out
│   │   │   ├── cache-keeps-print-options.rb
│   │   │   └── cache-keeps-print-options.sh
│   │   ├── cache-reserve-mem
│   │   │   ├── cache-reserve-mem.out
│   │   │   ├── cache-reserve-mem.sh
│   │   │   └── input.rb
│   │   ├── cli_test.bzl
│   │   ├── config-file
│   │   │   ├── config-file.out
│   │   │   ├── config-file.rb
│   │   │   ├── config-file.sh
│   │   │   └── sorbet
│   │   │       ├── bad_no_config
│   │   │       ├── config
│   │   │       └── other_config
│   │   ├── config-file-recursive
│   │   │   ├── config-file-recursive.out
│   │   │   ├── config-file-recursive.rb
│   │   │   ├── config-file-recursive.sh
│   │   │   └── sorbet
│   │   │       ├── config
│   │   │       └── other-config
│   │   ├── configatron
│   │   │   ├── configatron.out
│   │   │   ├── configatron.rb
│   │   │   ├── configatron.sh
│   │   │   └── configatron.yaml
│   │   ├── configatron-yaml-error
│   │   │   ├── configatron-yaml-error.out
│   │   │   ├── configatron-yaml-error.rb
│   │   │   ├── configatron-yaml-error.sh
│   │   │   └── configatron-yaml-error.yaml
│   │   ├── conflicting-definition
│   │   │   ├── a.rb
│   │   │   ├── b.rb
│   │   │   ├── conflicting-definition.out
│   │   │   └── conflicting-definition.sh
│   │   ├── constant-fuzzy
│   │   │   ├── constant-fuzzy.out
│   │   │   ├── constant-fuzzy.rb
│   │   │   └── constant-fuzzy.sh
│   │   ├── correct-bare-stdlib-generics
│   │   │   ├── correct-bare-stdlib-generics.out
│   │   │   ├── correct-bare-stdlib-generics.rb
│   │   │   └── correct-bare-stdlib-generics.sh
│   │   ├── counters
│   │   │   ├── counters.out
│   │   │   └── counters.sh
│   │   ├── dash-e
│   │   │   ├── dash-e.out
│   │   │   └── dash-e.sh
│   │   ├── dedup-input-files
│   │   │   ├── dedup-input-files.out
│   │   │   └── dedup-input-files.sh
│   │   ├── dedup_loc
│   │   │   ├── dedup_loc.out
│   │   │   ├── dedup_loc.rb
│   │   │   └── dedup_loc.sh
│   │   ├── empty-file
│   │   │   ├── empty-file.out
│   │   │   ├── empty-file.sh
│   │   │   └── empty.rb
│   │   ├── error-blacklist
│   │   │   ├── error-blacklist.out
│   │   │   ├── error-blacklist.rb
│   │   │   └── error-blacklist.sh
│   │   ├── error-url
│   │   │   ├── error-url.out
│   │   │   ├── error-url.rb
│   │   │   └── error-url.sh
│   │   ├── error-whitelist
│   │   │   ├── error-whitelist.out
│   │   │   ├── error-whitelist.rb
│   │   │   └── error-whitelist.sh
│   │   ├── errors
│   │   │   ├── errors.out
│   │   │   ├── errors.rb
│   │   │   └── errors.sh
│   │   ├── escaping
│   │   │   ├── escaping.out
│   │   │   ├── escaping.rb
│   │   │   └── escaping.sh
│   │   ├── file-table-json
│   │   │   ├── file-table-json.out
│   │   │   ├── file-table-json.rb
│   │   │   └── file-table-json.sh
│   │   ├── folder-input
│   │   │   ├── folder-input.out
│   │   │   ├── folder-input.sh
│   │   │   ├── foo.rb
│   │   │   └── input
│   │   │       ├── bar.rb
│   │   │       ├── file_with_no_dot
│   │   │       └── subfolder
│   │   │           └── baz.rb
│   │   ├── folder-input-dir-and-file
│   │   │   ├── folder-input-dir-and-file.out
│   │   │   ├── folder-input-dir-and-file.sh
│   │   │   ├── foo.rb
│   │   │   └── input
│   │   │       ├── bar.rb
│   │   │       ├── file_with_no_dot
│   │   │       └── subfolder
│   │   │           └── baz.rb
│   │   ├── folder-input-not-dir
│   │   │   ├── folder-input-not-dir.out
│   │   │   ├── folder-input-not-dir.sh
│   │   │   └── foo.rb
│   │   ├── folder-input-not-found
│   │   │   ├── folder-input-not-found.out
│   │   │   └── folder-input-not-found.sh
│   │   ├── forbid-autocorrect-with-e
│   │   │   ├── forbid-autocorrect-with-e.out
│   │   │   └── forbid-autocorrect-with-e.sh
│   │   ├── forbid-autocorrect-with-quiet
│   │   │   ├── forbid-autocorrect-with-quiet.out
│   │   │   ├── forbid-autocorrect-with-quiet.sh
│   │   │   └── input.rb
│   │   ├── forgot-typed
│   │   │   ├── forgot-typed.out
│   │   │   ├── forgot-typed.rb
│   │   │   ├── forgot-typed.sh
│   │   │   └── permit-dsl-sig.rb
│   │   ├── help
│   │   │   ├── help.out
│   │   │   └── help.sh
│   │   ├── hup-hack
│   │   │   ├── hup-hack.out
│   │   │   └── hup-hack.sh
│   │   ├── ignore
│   │   │   ├── bar.rb
│   │   │   ├── foo.rb
│   │   │   ├── ignore.out
│   │   │   ├── ignore.sh
│   │   │   ├── subfolder
│   │   │   │   └── baz.rb
│   │   │   └── subfolder2
│   │   │       ├── foo.rb
│   │   │       └── subfolder
│   │   │           └── bar.rb
│   │   ├── ignore-slash
│   │   │   ├── bar.rb
│   │   │   ├── foo
│   │   │   │   └── foo.rb
│   │   │   ├── ignore-slash.out
│   │   │   └── ignore-slash.sh
│   │   ├── incremental-resolver
│   │   │   ├── expect-failures
│   │   │   │   ├── abstract_impl.rb
│   │   │   │   ├── constant_override.rb
│   │   │   │   ├── constant_redefinition.rb
│   │   │   │   └── multiple_sigs.rb
│   │   │   ├── incremental-resolver.out
│   │   │   ├── incremental-resolver.sh
│   │   │   ├── type-member.rb
│   │   │   └── type-template.rb
│   │   ├── index-cache-invalidation
│   │   │   ├── index-cache-invalidation.out
│   │   │   └── index-cache-invalidation.sh
│   │   ├── kwarg-loc
│   │   │   ├── kwarg-loc.out
│   │   │   ├── kwarg-loc.rb
│   │   │   └── kwarg-loc.sh
│   │   ├── license
│   │   │   ├── license.out
│   │   │   └── license.sh
│   │   ├── line-splitting
│   │   │   ├── line-splitting.out
│   │   │   └── line-splitting.sh
│   │   ├── logging
│   │   │   ├── logging.out
│   │   │   └── logging.sh
│   │   ├── lsp-common-case-exit
│   │   │   ├── lsp-common-case-exit.out
│   │   │   └── lsp-common-case-exit.sh
│   │   ├── lsp-invalid-json-and-exit
│   │   │   ├── lsp-invalid-json-and-exit.out
│   │   │   └── lsp-invalid-json-and-exit.sh
│   │   ├── lsp-large-message
│   │   │   ├── lsp-large-message.out
│   │   │   └── lsp-large-message.sh
│   │   ├── lsp-requires-input-dir
│   │   │   ├── lsp-requires-input-dir.out
│   │   │   └── lsp-requires-input-dir.sh
│   │   ├── make_accessible
│   │   │   ├── make_accessible.out
│   │   │   ├── make_accessible.sh
│   │   │   └── suit.rb
│   │   ├── metrics-file
│   │   │   ├── metrics-file.out
│   │   │   ├── metrics-file.sh
│   │   │   ├── sorbet
│   │   │   │   ├── plugin.rb
│   │   │   │   └── triggers.yml
│   │   │   ├── test.rb
│   │   │   └── with-error-branching.rb
│   │   ├── missing-constants
│   │   │   ├── missing-constants.out
│   │   │   ├── missing-constants.rb
│   │   │   └── missing-constants.sh
│   │   ├── model_mutator_behavior
│   │   │   ├── model_mutator_behavior.out
│   │   │   ├── model_mutator_behavior.sh
│   │   │   ├── model_mutator_behavior__1.rb
│   │   │   └── model_mutator_behavior__2.rb
│   │   ├── module-redefinition
│   │   │   ├── module-redefinition-1.rb
│   │   │   ├── module-redefinition-2.rb
│   │   │   ├── module-redefinition-3.rb
│   │   │   ├── module-redefinition.out
│   │   │   └── module-redefinition.sh
│   │   ├── no-did-you-mean
│   │   │   ├── no-did-you-mean.out
│   │   │   ├── no-did-you-mean.rb
│   │   │   └── no-did-you-mean.sh
│   │   ├── no-error-count
│   │   │   ├── no-error-count.out
│   │   │   └── no-error-count.sh
│   │   ├── no-stdlib
│   │   │   ├── no-stdlib.out
│   │   │   └── no-stdlib.sh
│   │   ├── non-existing-option
│   │   │   ├── non-existing-option.out
│   │   │   └── non-existing-option.sh
│   │   ├── override-typed
│   │   │   ├── override-typed.out
│   │   │   ├── override-typed.rb
│   │   │   ├── override-typed.sh
│   │   │   └── override-typed.yaml
│   │   ├── override-typed-bad
│   │   │   ├── bad-filename.yaml
│   │   │   ├── bad-list.yaml
│   │   │   ├── bad-strictness.yaml
│   │   │   ├── bad-top-level.yaml
│   │   │   ├── override-typed-bad.out
│   │   │   ├── override-typed-bad.rb
│   │   │   ├── override-typed-bad.sh
│   │   │   └── override-typed-bad.yaml
│   │   ├── parallel-ordering
│   │   │   ├── 1.rb
│   │   │   ├── 2.rb
│   │   │   ├── 3.rb
│   │   │   ├── parallel-ordering.out
│   │   │   └── parallel-ordering.sh
│   │   ├── parse-tree-whitequark
│   │   │   ├── parse-tree-whitequark.out
│   │   │   ├── parse-tree-whitequark.rb
│   │   │   └── parse-tree-whitequark.sh
│   │   ├── parser-error
│   │   │   ├── parser-error-1.rb
│   │   │   ├── parser-error-2.rb
│   │   │   ├── parser-error-3.rb
│   │   │   ├── parser-error-4.rb
│   │   │   ├── parser-error.out
│   │   │   └── parser-error.sh
│   │   ├── phases
│   │   │   ├── phases.out
│   │   │   └── phases.sh
│   │   ├── print-procs
│   │   │   ├── print-procs.out
│   │   │   ├── print-procs.rb
│   │   │   └── print-procs.sh
│   │   ├── print_generics
│   │   │   ├── print_generics.out
│   │   │   ├── print_generics.rb
│   │   │   └── print_generics.sh
│   │   ├── print_to_file
│   │   │   ├── a.rb
│   │   │   ├── b.rb
│   │   │   ├── c.rb
│   │   │   ├── d.rb
│   │   │   ├── print_to_file.out
│   │   │   └── print_to_file.sh
│   │   ├── progressbar
│   │   │   ├── progressbar.out
│   │   │   └── progressbar.sh
│   │   ├── rbi-overrides
│   │   │   ├── rbi-overrides.out
│   │   │   ├── rbi-overrides.sh
│   │   │   ├── t1.rb
│   │   │   ├── t1.rbi
│   │   │   ├── t2.rb
│   │   │   ├── t2.rbi
│   │   │   ├── t3.rb
│   │   │   ├── t3.rbi
│   │   │   ├── t4.rb
│   │   │   └── t4.rbi
│   │   ├── rbi-with-code
│   │   │   ├── rbi-with-code.out
│   │   │   ├── rbi-with-code.rbi
│   │   │   └── rbi-with-code.sh
│   │   ├── remove-path-prefix
│   │   │   ├── remove-path-prefix.out
│   │   │   ├── remove-path-prefix.rb
│   │   │   └── remove-path-prefix.sh
│   │   ├── remove-path-prefix-https
│   │   │   ├── remove-path-prefix-https.out
│   │   │   ├── remove-path-prefix-https.rb
│   │   │   └── remove-path-prefix-https.sh
│   │   ├── remove-path-prefix-no-match
│   │   │   ├── remove-path-prefix-no-match.out
│   │   │   ├── remove-path-prefix-no-match.rb
│   │   │   └── remove-path-prefix-no-match.sh
│   │   ├── sigil-rbi
│   │   │   ├── abstract.rbi
│   │   │   ├── multiple_definition.rbi
│   │   │   ├── no_type.rbi
│   │   │   ├── overrides.rbi
│   │   │   ├── sigil-rbi.out
│   │   │   ├── sigil-rbi.sh
│   │   │   ├── strict.rbi
│   │   │   └── typed.rbi
│   │   ├── silence-dev-message
│   │   │   ├── silence-dev-message.out
│   │   │   └── silence-dev-message.sh
│   │   ├── statsd
│   │   │   ├── statsd.out
│   │   │   └── statsd.sh
│   │   ├── stop-after
│   │   │   ├── stop-after.out
│   │   │   └── stop-after.sh
│   │   ├── stop-after-namer
│   │   │   ├── stop-after-namer.out
│   │   │   ├── stop-after-namer.rb
│   │   │   └── stop-after-namer.sh
│   │   ├── store-state
│   │   │   ├── store-state.out
│   │   │   └── store-state.sh
│   │   ├── subprocess-plugin
│   │   │   ├── bad_plugin.rb
│   │   │   ├── bad_plugin.yaml
│   │   │   ├── bar_gen.rb
│   │   │   ├── baz_gen.rb
│   │   │   ├── echo_argv.rb
│   │   │   ├── echo_argv.yaml
│   │   │   ├── foo_gen.rb
│   │   │   ├── gen.rb
│   │   │   ├── multi.yaml
│   │   │   ├── multi1.rb
│   │   │   ├── multi2.rb
│   │   │   ├── multi3.rb
│   │   │   ├── multi4.rb
│   │   │   ├── multi5.rb
│   │   │   ├── multi6.rb
│   │   │   ├── multi_empty.yaml
│   │   │   ├── no_output.rb
│   │   │   ├── permute.rb
│   │   │   ├── ruby_extra_args.yaml
│   │   │   ├── subprocess-plugin.out
│   │   │   ├── subprocess-plugin.sh
│   │   │   ├── trigger_bad_plugin.rb
│   │   │   └── verify_ruby_options.rb
│   │   ├── suggest-constant-type
│   │   │   ├── suggest-constant-type.out
│   │   │   ├── suggest-constant-type.rb
│   │   │   └── suggest-constant-type.sh
│   │   ├── suggest-foreign-lambda
│   │   │   ├── suggest-foreign-lambda.out
│   │   │   ├── suggest-foreign-lambda.rb
│   │   │   └── suggest-foreign-lambda.sh
│   │   ├── suggest-kernel
│   │   │   ├── suggest-kernel.out
│   │   │   ├── suggest-kernel.rb
│   │   │   └── suggest-kernel.sh
│   │   ├── suggest-new
│   │   │   ├── suggest-new.out
│   │   │   ├── suggest-new.rb
│   │   │   └── suggest-new.sh
│   │   ├── suggest-not-stub
│   │   │   ├── suggest-not-stub.out
│   │   │   ├── suggest-not-stub.rb
│   │   │   └── suggest-not-stub.sh
│   │   ├── suggest-object
│   │   │   ├── suggest-object.out
│   │   │   ├── suggest-object.rb
│   │   │   └── suggest-object.sh
│   │   ├── suggest-sig
│   │   │   ├── suggest-sig.out
│   │   │   ├── suggest-sig.rb
│   │   │   └── suggest-sig.sh
│   │   ├── suggest-sig-garbage
│   │   │   ├── suggest-sig-garbage.out
│   │   │   ├── suggest-sig-garbage.rb
│   │   │   └── suggest-sig-garbage.sh
│   │   ├── suggest-sig-literal
│   │   │   ├── suggest-sig-literal.out
│   │   │   ├── suggest-sig-literal.rb
│   │   │   └── suggest-sig-literal.sh
│   │   ├── suggest-sig-override
│   │   │   ├── suggest-sig-override.out
│   │   │   ├── suggest-sig-override.rb
│   │   │   └── suggest-sig-override.sh
│   │   ├── suggest-sig-override-edge
│   │   │   ├── suggest-sig-override-edge.out
│   │   │   ├── suggest-sig-override-edge.rb
│   │   │   └── suggest-sig-override-edge.sh
│   │   ├── suggest-singleton
│   │   │   ├── suggest-singleton.out
│   │   │   ├── suggest-singleton.rb
│   │   │   └── suggest-singleton.sh
│   │   ├── suggest-t-let
│   │   │   ├── suggest-t-let.out
│   │   │   ├── suggest-t-let.rb
│   │   │   └── suggest-t-let.sh
│   │   ├── suggest-type-alias
│   │   │   ├── suggest-type-alias.out
│   │   │   ├── suggest-type-alias.rb
│   │   │   └── suggest-type-alias.sh
│   │   ├── suggest-typed-true
│   │   │   ├── empty.rb
│   │   │   ├── suggest-typed-already-autogenerated.rb
│   │   │   ├── suggest-typed-already-ignore.rb
│   │   │   ├── suggest-typed-and-type.rb
│   │   │   ├── suggest-typed-behaviour-over-multiple-1.rb
│   │   │   ├── suggest-typed-behaviour-over-multiple-2.rb
│   │   │   ├── suggest-typed-false.rb
│   │   │   ├── suggest-typed-ignore.rb
│   │   │   ├── suggest-typed-shabang.rb
│   │   │   ├── suggest-typed-strict.rb
│   │   │   ├── suggest-typed-strong.rb
│   │   │   ├── suggest-typed-true.out
│   │   │   ├── suggest-typed-true.rb
│   │   │   ├── suggest-typed-true.sh
│   │   │   └── suggest-typed-with-too-low.rb
│   │   ├── suggest-typos
│   │   │   ├── suggest-typos.out
│   │   │   └── suggest-typos.sh
│   │   ├── suggest_autogen
│   │   │   ├── suggest_autogen.out
│   │   │   ├── suggest_autogen.rb
│   │   │   └── suggest_autogen.sh
│   │   ├── suggest_static
│   │   │   ├── suggest_static.out
│   │   │   ├── suggest_static.rb
│   │   │   └── suggest_static.sh
│   │   ├── suggest_t_must
│   │   │   ├── suggest_t_must.out
│   │   │   ├── suggest_t_must.rb
│   │   │   └── suggest_t_must.sh
│   │   ├── suppress-non-critical
│   │   │   ├── suppress-non-critical.out
│   │   │   └── suppress-non-critical.sh
│   │   ├── symbol-table-json
│   │   │   ├── symbol-table-json.out
│   │   │   ├── symbol-table-json.rb
│   │   │   └── symbol-table-json.sh
│   │   ├── symbol-table-json-alias
│   │   │   ├── symbol-table-json-alias.out
│   │   │   ├── symbol-table-json-alias.rb
│   │   │   └── symbol-table-json-alias.sh
│   │   ├── test_one.sh
│   │   ├── typed-src
│   │   │   └── typed-src.out
│   │   ├── update_one.sh
│   │   ├── version-returns-0
│   │   │   ├── version-returns-0.out
│   │   │   └── version-returns-0.sh
│   │   ├── web-trace-file
│   │   │   ├── web-trace-file.out
│   │   │   └── web-trace-file.sh
│   │   └── web-trace-file-non-ascii
│   │       ├── web-trace-file-non-ascii.out
│   │       ├── web-trace-file-non-ascii.rb
│   │       └── web-trace-file-non-ascii.sh
│   ├── error-check-test.cc
│   ├── fuzz
│   │   ├── BUILD
│   │   ├── TextDocumentPositionParamsWithoutTextDocumentIdentifier.proto
│   │   ├── empty.cc
│   │   ├── fuzz_dash_e.cc
│   │   ├── fuzz_doc_symbols.cc
│   │   ├── fuzz_hover.cc
│   │   └── ruby.dict
│   ├── hello-test.cc
│   ├── helpers
│   │   ├── BUILD
│   │   ├── MockFileSystem.cc
│   │   ├── MockFileSystem.h
│   │   ├── expectations.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── position_assertions.cc
│   │   └── position_assertions.h
│   ├── lint\\buildifier
│   │   └── BUILD
│   ├── lsp
│   │   ├── BUILD
│   │   ├── ProtocolTest.cc
│   │   ├── ProtocolTest.h
│   │   ├── alias-incremental
│   │   │   └── alias-incremental.rec
│   │   ├── cache_protocol_test_corpus.cc
│   │   ├── incremental-lsp-changes
│   │   │   └── incremental-lsp-changes.rec
│   │   ├── lsp_test.bzl
│   │   ├── lsp_test_runner.sh
│   │   ├── multithreaded_protocol_test_corpus.cc
│   │   ├── no-trailing-newline
│   │   │   └── no-trailing-newline.rec
│   │   ├── protocol_test_corpus.cc
│   │   ├── update_one.sh
│   │   ├── watchman_test_corpus.cc
│   │   └── workspaceSymbol
│   │       └── workspaceSymbol.rec
│   ├── pipeline_test.bzl
│   ├── print_document_symbols.cc
│   ├── test_corpus.cc
│   ├── test_corpus_forwarder.sh
│   ├── testdata
│   │   ├── autogen
│   │   │   ├── aliased_ancestors.rb
│   │   │   ├── aliased_ancestors.rb.autogen.exp
│   │   │   ├── ancestors.rb
│   │   │   ├── ancestors.rb.autogen.exp
│   │   │   ├── ancestors_ref.rb
│   │   │   ├── ancestors_ref.rb.autogen.exp
│   │   │   ├── bad_prop.rb
│   │   │   ├── bad_prop.rb.autogen.exp
│   │   │   ├── bare_alias.rb
│   │   │   ├── bare_alias.rb.autogen.exp
│   │   │   ├── bare_casgn.rb
│   │   │   ├── bare_casgn.rb.autogen.exp
│   │   │   ├── bare_class.rb
│   │   │   ├── bare_class.rb.autogen.exp
│   │   │   ├── bare_module.rb
│   │   │   ├── bare_module.rb.autogen.exp
│   │   │   ├── cbase_const.rb
│   │   │   ├── cbase_const.rb.autogen.exp
│   │   │   ├── defines_behavior.rb
│   │   │   ├── defines_behavior.rb.autogen.exp
│   │   │   ├── dynamic_superclass.rb
│   │   │   ├── dynamic_superclass.rb.autogen.exp
│   │   │   ├── foo.fixturerb
│   │   │   ├── foo.fixturerb.autogen.exp
│   │   │   ├── generator.rb
│   │   │   ├── generator.rb.autogen.exp
│   │   │   ├── hidden_include.rb
│   │   │   ├── hidden_include.rb.autogen.exp
│   │   │   ├── multiple_alias.rb
│   │   │   ├── multiple_alias.rb.autogen.exp
│   │   │   ├── nested_defs.rb
│   │   │   ├── nested_defs.rb.autogen.exp
│   │   │   ├── no_dsls.rb
│   │   │   ├── no_dsls.rb.autogen.exp
│   │   │   ├── resolving_refs.rb
│   │   │   ├── resolving_refs.rb.autogen.exp
│   │   │   ├── simple_refs.rb
│   │   │   ├── simple_refs.rb.autogen.exp
│   │   │   ├── strong_sigil.rb
│   │   │   └── strong_sigil.rb.autogen.exp
│   │   ├── call_with_block.rb
│   │   ├── call_with_block_strict.rb
│   │   ├── call_with_splat_and_block.rb
│   │   ├── call_with_splat_and_block_strict.rb
│   │   ├── cfg
│   │   │   ├── array.rb
│   │   │   ├── array.rb.cfg.exp
│   │   │   ├── block_in_deadcode.rb
│   │   │   ├── block_in_deadcode.rb.cfg.exp
│   │   │   ├── block_return_bug.rb
│   │   │   ├── blocks.rb
│   │   │   ├── blocks.rb.cfg.exp
│   │   │   ├── blocks.rb.desugar-tree.exp
│   │   │   ├── blocks.rb.parse-tree.exp
│   │   │   ├── blocks.rb.symbol-table-raw.exp
│   │   │   ├── break.rb
│   │   │   ├── break.rb.cfg.exp
│   │   │   ├── break_in_junk.rb
│   │   │   ├── break_in_junk.rb.cfg.exp
│   │   │   ├── break_in_while.rb
│   │   │   ├── break_in_while.rb.cfg.exp
│   │   │   ├── class_static_field.rb
│   │   │   ├── class_static_field_children.rb
│   │   │   ├── dealias_with_return.rb
│   │   │   ├── dealias_with_return.rb.cfg.exp
│   │   │   ├── do_while.rb
│   │   │   ├── do_while.rb.cfg.exp
│   │   │   ├── ensure_after_raise.rb
│   │   │   ├── examples.rb
│   │   │   ├── examples.rb.cfg.exp
│   │   │   ├── examples.rb.desugar-tree.exp
│   │   │   ├── examples.rb.flatten-tree.exp
│   │   │   ├── examples.rb.parse-tree.exp
│   │   │   ├── examples.rb.symbol-table-raw.exp
│   │   │   ├── extra_bb_args.rb
│   │   │   ├── extra_bb_args.rb.cfg.exp
│   │   │   ├── floats.rb
│   │   │   ├── floats.rb.cfg.exp
│   │   │   ├── floats.rb.desugar-tree.exp
│   │   │   ├── fuzz_dedup_exit_edges.rb
│   │   │   ├── fuzz_recursive_alias.rb
│   │   │   ├── hash.rb
│   │   │   ├── hash.rb.cfg.exp
│   │   │   ├── ivar_assign.rb
│   │   │   ├── ivar_assign.rb.cfg.exp
│   │   │   ├── next.rb
│   │   │   ├── next.rb.cfg.exp
│   │   │   ├── next_in_junk.rb
│   │   │   ├── next_in_junk.rb.cfg.exp
│   │   │   ├── next_in_while.rb
│   │   │   ├── next_in_while.rb.cfg.exp
│   │   │   ├── override_bang.rb
│   │   │   ├── raw_test.rb
│   │   │   ├── raw_test.rb.cfg-raw.exp
│   │   │   ├── reassign_dead_block_bug.rb
│   │   │   ├── reassign_dead_block_bug.rb.cfg.exp
│   │   │   ├── rescue.rb
│   │   │   ├── rescue.rb.cfg.exp
│   │   │   ├── rescue.rb.desugar-tree-raw.exp
│   │   │   ├── rescue.rb.desugar-tree.exp
│   │   │   ├── rescue_bad_class.rb
│   │   │   ├── rescue_bad_class.rb.flatten-tree.exp
│   │   │   ├── rescue_complex.rb
│   │   │   ├── rescue_complex.rb.cfg.exp
│   │   │   ├── rescue_complex.rb.desugar-tree.exp
│   │   │   ├── rescue_else_block.rb
│   │   │   ├── rescue_else_block.rb.cfg.exp
│   │   │   ├── rescue_expression.rb
│   │   │   ├── rescue_expression.rb.cfg.exp
│   │   │   ├── rescue_two_return.rb
│   │   │   ├── rescue_two_return.rb.cfg.exp
│   │   │   ├── rescue_var_expression.rb
│   │   │   ├── rescue_var_expression.rb.cfg.exp
│   │   │   ├── rescue_with_return.rb
│   │   │   ├── rescue_with_return.rb.cfg.exp
│   │   │   ├── retry.rb
│   │   │   ├── retry.rb.cfg.exp
│   │   │   ├── retry.rb.desugar-tree-raw.exp
│   │   │   ├── retry.rb.desugar-tree.exp
│   │   │   ├── retry_invalid.rb
│   │   │   ├── retry_multiple.rb
│   │   │   ├── retry_multiple.rb.cfg.exp
│   │   │   ├── retry_nested.rb
│   │   │   ├── retry_nested.rb.cfg.exp
│   │   │   ├── return_type_of_nilable_blk_param.rb
│   │   │   ├── side_effects.rb
│   │   │   ├── side_effects.rb.cfg.exp
│   │   │   ├── side_effects2.rb
│   │   │   ├── side_effects2.rb.cfg.exp
│   │   │   ├── singleton_lazy.rb
│   │   │   ├── singleton_lazy.rb.symbol-table-raw.exp
│   │   │   ├── triple_eq.rb
│   │   │   ├── uaf1.rb
│   │   │   ├── uaf1.rb.cfg.exp
│   │   │   ├── undeclared_variable.rb
│   │   │   ├── unreachable.rb
│   │   │   └── wtf_ensure.rb
│   │   ├── class_not_class_of.rb
│   │   ├── core
│   │   │   ├── autogenerated.rb
│   │   │   ├── fuzz_bad_subtyping.rb
│   │   │   └── fuzz_unparseable.rb
│   │   ├── desugar
│   │   │   ├── accidentally_quadratic.rb
│   │   │   ├── accidentally_quadratic.rb.desugar-tree.exp
│   │   │   ├── and_self.rb
│   │   │   ├── assign_empty_parens.rb
│   │   │   ├── assign_empty_stmts.rb
│   │   │   ├── assign_keyword.rb
│   │   │   ├── backtick.rb
│   │   │   ├── backtick.rb.desugar-tree.exp
│   │   │   ├── blockpass.rb
│   │   │   ├── blockpass.rb.desugar-tree.exp
│   │   │   ├── case.rb
│   │   │   ├── class_def_kind.rb
│   │   │   ├── class_def_kind.rb.desugar-tree-raw.exp
│   │   │   ├── complex.rb
│   │   │   ├── complex.rb.desugar-tree.exp
│   │   │   ├── constant_error.rb
│   │   │   ├── constant_error.rb.symbol-table-raw.exp
│   │   │   ├── constant_reassignment.rb
│   │   │   ├── csend.rb
│   │   │   ├── csend.rb.desugar-tree.exp
│   │   │   ├── defined.rb
│   │   │   ├── defined.rb.desugar-tree.exp
│   │   │   ├── defs_not_self.rb
│   │   │   ├── defs_not_self.rb.desugar-tree.exp
│   │   │   ├── destructure.rb
│   │   │   ├── destructure.rb.flatten-tree.exp
│   │   │   ├── destructure.rb.symbol-table-raw.exp
│   │   │   ├── destructure_rest.rb
│   │   │   ├── dsymbol.rb
│   │   │   ├── dsymbol.rb.desugar-tree.exp
│   │   │   ├── empty_string_concatenation.rb
│   │   │   ├── ensure_without_rescue.rb
│   │   │   ├── ensure_without_rescue.rb.cfg.exp
│   │   │   ├── ensure_without_rescue.rb.desugar-tree.exp
│   │   │   ├── file.rb
│   │   │   ├── file.rb.desugar-tree.exp
│   │   │   ├── file.rb.parse-tree.exp
│   │   │   ├── for.rb
│   │   │   ├── for.rb.cfg.exp
│   │   │   ├── for.rb.desugar-tree.exp
│   │   │   ├── fuzz_bad_loc_dstring.rb
│   │   │   ├── fuzz_block_pass.rb
│   │   │   ├── fuzz_break_do_end.rb
│   │   │   ├── fuzz_include_self.rb
│   │   │   ├── hash.rb
│   │   │   ├── hash.rb.desugar-tree.exp
│   │   │   ├── hash_two_args.rb
│   │   │   ├── integers.rb
│   │   │   ├── integers.rb.desugar-tree.exp
│   │   │   ├── lambda.rb
│   │   │   ├── lambda.rb.desugar-tree.exp
│   │   │   ├── line_literal.rb
│   │   │   ├── line_literal.rb.desugar-tree.exp
│   │   │   ├── multi_assign.rb
│   │   │   ├── multi_assign.rb.desugar-tree.exp
│   │   │   ├── multi_assign.rb.symbol-table-raw.exp
│   │   │   ├── nthref.rb
│   │   │   ├── nthref.rb.desugar-tree-raw.exp
│   │   │   ├── nthref.rb.parse-tree.exp
│   │   │   ├── op_eq.rb
│   │   │   ├── op_eq.rb.desugar-tree.exp
│   │   │   ├── op_eq.rb.flatten-tree.exp
│   │   │   ├── opasgn.rb
│   │   │   ├── opasgn.rb.desugar-tree.exp
│   │   │   ├── ops.rb
│   │   │   ├── ops.rb.desugar-tree.exp
│   │   │   ├── range.rb
│   │   │   ├── range.rb.desugar-tree-raw.exp
│   │   │   ├── regexp.rb
│   │   │   ├── regexp.rb.desugar-tree.exp
│   │   │   ├── regexp_strict.rb
│   │   │   ├── sclass.rb
│   │   │   ├── sclass.rb.desugar-tree.exp
│   │   │   ├── sclass.rb.flatten-tree.exp
│   │   │   ├── sclass.rb.symbol-table-raw.exp
│   │   │   ├── sclass_inheritance.rb
│   │   │   ├── sclass_inheritance.rb.desugar-tree.exp
│   │   │   ├── sclass_inheritance.rb.flatten-tree.exp
│   │   │   ├── sclass_inheritance.rb.symbol-table-raw.exp
│   │   │   ├── splat.rb
│   │   │   ├── splat.rb.desugar-tree.exp
│   │   │   ├── star_in_block_arg.rb
│   │   │   ├── star_in_block_arg.rb.desugar-tree.exp
│   │   │   ├── strings.rb
│   │   │   ├── strings.rb.desugar-tree.exp
│   │   │   ├── top_level_const.rb
│   │   │   ├── top_level_const.rb.desugar-tree-raw.exp
│   │   │   ├── top_level_const.rb.desugar-tree.exp
│   │   │   ├── top_level_const.rb.symbol-table-raw.exp
│   │   │   ├── undef.rb
│   │   │   └── undef_strict.rb
│   │   ├── deviations
│   │   │   ├── keyword_method_names.rb
│   │   │   ├── keyword_method_names.rb.parse-tree.exp
│   │   │   ├── non_ruby_names.rb
│   │   │   ├── non_ruby_names.rb.desugar-tree.exp
│   │   │   ├── non_ruby_names.rb.flatten-tree.exp
│   │   │   ├── non_ruby_names.rb.parse-tree.exp
│   │   │   ├── non_ruby_names.rb.symbol-table-raw.exp
│   │   │   ├── superclass_implicit.rb
│   │   │   └── superclass_implicit.rb.symbol-table-raw.exp
│   │   ├── disabled\\whitequark
│   │   │   ├── parse_dedenting_heredoc_13.rb
│   │   │   └── parse_encoding_.rb
│   │   ├── error_recovery_send_after_end.rb
│   │   ├── error_recovery_send_after_end.rb.parse-tree.exp
│   │   ├── infer
│   │   │   ├── absurd_false.rb
│   │   │   ├── aliases__1.rb
│   │   │   ├── aliases__2.rb
│   │   │   ├── align_child_as_parent.rb
│   │   │   ├── all_bug.rb
│   │   │   ├── arg_matching.rb
│   │   │   ├── arity.rb
│   │   │   ├── assign_self.rb
│   │   │   ├── assignment_call.rb
│   │   │   ├── attached_class.rb
│   │   │   ├── attached_class_factory_example.rb
│   │   │   ├── attached_class_params.rb
│   │   │   ├── attached_class_printing.rb
│   │   │   ├── attached_class_reopen.rb
│   │   │   ├── attached_class_self_new.rb
│   │   │   ├── bad_child_class.rb
│   │   │   ├── bad_suggest.rb
│   │   │   ├── block_arg.rb
│   │   │   ├── block_given.rb
│   │   │   ├── block_parameter.rb
│   │   │   ├── blocks2.rb
│   │   │   ├── blocks2.rb.cfg.exp
│   │   │   ├── body_not_always_run.rb
│   │   │   ├── boolean.rb
│   │   │   ├── case.rb
│   │   │   ├── case_all.rb
│   │   │   ├── case_exhaustive_union_type.rb
│   │   │   ├── case_subclass.rb
│   │   │   ├── case_untyped.rb
│   │   │   ├── case_when_any.rb
│   │   │   ├── casts.rb
│   │   │   ├── casts.rb.cfg.exp
│   │   │   ├── casts.rb.flatten-tree.exp
│   │   │   ├── class.rb
│   │   │   ├── class_new.rb
│   │   │   ├── compact.rb
│   │   │   ├── constructor_return.rb
│   │   │   ├── constructors.rb
│   │   │   ├── control_flow
│   │   │   │   ├── blank_p.rb
│   │   │   │   ├── bug_852.rb
│   │   │   │   ├── class_less_than.rb
│   │   │   │   ├── complex_implication_1.rb
│   │   │   │   ├── complex_implication_1.rb.cfg.exp
│   │   │   │   ├── complex_implications_2.rb
│   │   │   │   ├── complex_implications_2.rb.cfg.exp
│   │   │   │   ├── csend_and.rb
│   │   │   │   ├── dead_condition.rb
│   │   │   │   ├── dynamic.rb
│   │   │   │   ├── isa_module.rb
│   │   │   │   ├── nil_p.rb
│   │   │   │   ├── normalize_params.rb
│   │   │   │   ├── normalize_params.rb.cfg.exp
│   │   │   │   ├── present_p.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── simple.rb.cfg.exp
│   │   │   │   ├── singletons.rb
│   │   │   │   ├── truthiness.rb
│   │   │   │   └── truthiness_bug.rb
│   │   │   ├── control_flow_in_ensure.rb
│   │   │   ├── control_flow_in_rescue.rb
│   │   │   ├── crash_after_parse_errors.rb
│   │   │   ├── dead_after_return.rb
│   │   │   ├── dispatch_call_and.rb
│   │   │   ├── dropsubtypesof.rb
│   │   │   ├── exhaustiveness.rb
│   │   │   ├── false_dead_code.rb
│   │   │   ├── fields.rb
│   │   │   ├── fields.rb.cfg.exp
│   │   │   ├── flat_map.rb
│   │   │   ├── flatten.rb
│   │   │   ├── flatten_methods.rb
│   │   │   ├── flatten_methods.rb.flatten-tree.exp
│   │   │   ├── flatten_private.rb
│   │   │   ├── flatten_private.rb.flatten-tree.exp
│   │   │   ├── forward_proc.rb
│   │   │   ├── fuzz_dangling_type_parameter.rb
│   │   │   ├── fuzz_disallow_overloading.rb
│   │   │   ├── fuzz_nonexistant_method.rb
│   │   │   ├── fuzz_oneline_conditional.rb
│   │   │   ├── fuzz_static_init_loc.rb
│   │   │   ├── fuzz_toplevel_type_member.rb
│   │   │   ├── fuzz_tripleeq_sig_suggestion.rb
│   │   │   ├── fuzz_uninitialized_vars.rb
│   │   │   ├── generic_methods
│   │   │   │   ├── countraints_crosstalk.rb
│   │   │   │   ├── genericMethods1.rb
│   │   │   │   ├── genericMethods2.rb
│   │   │   │   ├── genericMethodsErrors.rb
│   │   │   │   ├── infer_bottom.rb
│   │   │   │   ├── two_bounds.rb
│   │   │   │   └── untyped_in_container.rb
│   │   │   ├── generic_type_template_arg.rb
│   │   │   ├── generics
│   │   │   │   ├── Boxes.rb
│   │   │   │   ├── TypeSyntax.rb
│   │   │   │   ├── adapt_child_to_parent.rb
│   │   │   │   ├── all_bottom.rb
│   │   │   │   ├── arity_mismatch.rb
│   │   │   │   ├── bad_bound_typed_false.rb
│   │   │   │   ├── bounds.rb
│   │   │   │   ├── bounds_super.rb
│   │   │   │   ├── class_less_than.rb
│   │   │   │   ├── duplicate_members.rb
│   │   │   │   ├── enumerable.rb
│   │   │   │   ├── fixed_ordering.rb
│   │   │   │   ├── generic_self_method.rb
│   │   │   │   ├── generics_class_of.rb
│   │   │   │   ├── glb.rb
│   │   │   │   ├── glb2.rb
│   │   │   │   ├── isa_with_type_member.rb
│   │   │   │   ├── lub.rb
│   │   │   │   ├── lub_lambda_param.rb
│   │   │   │   ├── lub_with_raw.rb
│   │   │   │   ├── others_type_members.rb
│   │   │   │   ├── self_params.rb
│   │   │   │   ├── specified.rb
│   │   │   │   ├── t_magic.rb
│   │   │   │   ├── tuple_decay.rb
│   │   │   │   ├── two_params.rb
│   │   │   │   ├── type_param_is_a.rb
│   │   │   │   ├── use_member_in_body.rb
│   │   │   │   ├── variance_methods.rb
│   │   │   │   ├── variance_neutral.rb
│   │   │   │   ├── variance_params.rb
│   │   │   │   ├── variance_user.rb
│   │   │   │   └── wrong_type_member.rb
│   │   │   ├── glb_corner_case.rb
│   │   │   ├── glb_generic_with_class.rb
│   │   │   ├── hashes.rb
│   │   │   ├── hashes.rb.cfg.exp
│   │   │   ├── huge_unions.rb
│   │   │   ├── huge_unions.rb.cfg.exp
│   │   │   ├── i1438.rb
│   │   │   ├── infer1.rb
│   │   │   ├── infer1.rb.cfg.exp
│   │   │   ├── infer1.rb.desugar-tree.exp
│   │   │   ├── infer1.rb.flatten-tree.exp
│   │   │   ├── infer1.rb.symbol-table-raw.exp
│   │   │   ├── infer_types.rb
│   │   │   ├── intrinsics_generics.rb
│   │   │   ├── isa_bug.rb
│   │   │   ├── isa_generic.rb
│   │   │   ├── isa_generic.rb.cfg.exp
│   │   │   ├── kwargs.rb
│   │   │   ├── kwargs_generics.rb
│   │   │   ├── leaking_exceptions.rb
│   │   │   ├── let.rb
│   │   │   ├── literal_is_array.rb
│   │   │   ├── loop_in_self_reassignments.rb
│   │   │   ├── loop_with_rescue_next.rb
│   │   │   ├── loops.rb
│   │   │   ├── loops.rb.cfg.exp
│   │   │   ├── lub_and_glb_corner_cases.rb
│   │   │   ├── lub_between_self_params.rb
│   │   │   ├── lub_generics.rb
│   │   │   ├── lub_tuples.rb
│   │   │   ├── lub_tuples.rb.symbol-table-raw.exp
│   │   │   ├── magic_dead.rb
│   │   │   ├── massign_return_rhs.rb
│   │   │   ├── massign_return_rhs.rb.desugar-tree.exp
│   │   │   ├── match_operator.rb
│   │   │   ├── meta_types.rb
│   │   │   ├── meta_types.rb.cfg.exp
│   │   │   ├── metatype_in_lub.rb
│   │   │   ├── metatype_instantiation.rb
│   │   │   ├── module_function_two.rb
│   │   │   ├── more_after_return.rb
│   │   │   ├── multi_assign.rb
│   │   │   ├── must.rb
│   │   │   ├── next_in_while.rb
│   │   │   ├── nil_noreturn.rb
│   │   │   ├── nilable_and.rb
│   │   │   ├── non_existent_method.rb
│   │   │   ├── non_forcing_is_a.rb
│   │   │   ├── non_forcing_is_a_dead.rb
│   │   │   ├── non_forcing_is_a_false.rb
│   │   │   ├── or_and_and_or.rb
│   │   │   ├── overload_block.rb
│   │   │   ├── overloads_test.rb
│   │   │   ├── pinned_control_flow.rb
│   │   │   ├── pinned_control_flow1.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc_args.rb
│   │   │   ├── product.rb
│   │   │   ├── rebind.rb
│   │   │   ├── rebind.rb.cfg.exp
│   │   │   ├── ref_eq.rb
│   │   │   ├── restargs.rb
│   │   │   ├── restargsbox.rb
│   │   │   ├── return_in_if.rb
│   │   │   ├── reveal_type.rb
│   │   │   ├── sealed_singleton_class.rb
│   │   │   ├── segfault_generic.rb
│   │   │   ├── self_type.rb
│   │   │   ├── self_type.rb.cfg.exp
│   │   │   ├── self_type_param.rb
│   │   │   ├── self_type_param_bounded.rb
│   │   │   ├── setters.rb
│   │   │   ├── shape_merge.rb
│   │   │   ├── show.rb
│   │   │   ├── sigil.rb
│   │   │   ├── sigil.rb.cfg.exp
│   │   │   ├── singleton_case_exhaustiveness.rb
│   │   │   ├── singleton_classes.rb
│   │   │   ├── singleton_enum_case.rb
│   │   │   ├── singleton_enum_eqeq.rb
│   │   │   ├── singleton_if_exhaustiveness.rb
│   │   │   ├── singleton_methods.rb
│   │   │   ├── singleton_methods.rb.cfg.exp
│   │   │   ├── singleton_non_final_enum.rb
│   │   │   ├── singleton_of_singleton.rb
│   │   │   ├── singleton_of_singleton.rb.cfg.exp
│   │   │   ├── splat.rb
│   │   │   ├── star_star_args.rb
│   │   │   ├── strict_dead.rb
│   │   │   ├── stubs_are_dynamic.rb
│   │   │   ├── subtype_symbol.rb
│   │   │   ├── t_all_type_member_bug.rb
│   │   │   ├── toplevel.rb
│   │   │   ├── toplevel.rb.flatten-tree.exp
│   │   │   ├── toplevel_var.rb
│   │   │   ├── transitive.rb
│   │   │   ├── transitive.rb.cfg.exp
│   │   │   ├── tuples.rb
│   │   │   ├── type_substraction.rb
│   │   │   ├── unwrap_locs.rb
│   │   │   ├── void_final.rb
│   │   │   ├── void_proc.rb
│   │   │   ├── yield_inside_block.rb
│   │   │   ├── yield_multiple.rb
│   │   │   ├── yield_multiple.rb.desugar-tree-raw.exp
│   │   │   ├── zsuper.rb
│   │   │   └── zsuper.rb.cfg.exp
│   │   ├── intrinsics
│   │   │   ├── shapes.rb
│   │   │   └── to_h.rb
│   │   ├── local_vars
│   │   │   ├── z_super_args.rb
│   │   │   └── z_super_args.rb.index-tree.exp
│   │   ├── lsp
│   │   │   ├── bad_field_edits.1.rbupdate
│   │   │   ├── bad_field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── bad_field_edits.rb
│   │   │   ├── bad_field_edits.rb.document-symbols.exp
│   │   │   ├── code_actions
│   │   │   │   ├── loop_type_change.A.rbedited
│   │   │   │   ├── loop_type_change.rb
│   │   │   │   ├── private.A.rbedited
│   │   │   │   ├── private.B.rbedited
│   │   │   │   ├── private.rb
│   │   │   │   ├── sig_missing__child.A.rbedited
│   │   │   │   ├── sig_missing__child.rb
│   │   │   │   ├── sig_missing__parent.A.rbedited
│   │   │   │   ├── sig_missing__parent.rb
│   │   │   │   ├── typo.A.rbedited
│   │   │   │   ├── typo.B.rbedited
│   │   │   │   ├── typo.C.rbedited
│   │   │   │   ├── typo.D.rbedited
│   │   │   │   └── typo.rb
│   │   │   ├── completion
│   │   │   │   ├── alias_method.rb
│   │   │   │   ├── angle_bracket_names.rb
│   │   │   │   ├── class_and_module.rb
│   │   │   │   ├── constants.A.rbedited
│   │   │   │   ├── constants.B.rbedited
│   │   │   │   ├── constants.C.rbedited
│   │   │   │   ├── constants.rb
│   │   │   │   ├── constants_aliases.rb
│   │   │   │   ├── constants_all_kinds.rb
│   │   │   │   ├── constants_existing.rb
│   │   │   │   ├── constants_magic.rb
│   │   │   │   ├── constants_root.rb
│   │   │   │   ├── constants_t.rb
│   │   │   │   ├── constants_type_members.rb
│   │   │   │   ├── constants_via_inherit.rb
│   │   │   │   ├── constants_via_mixins.rb
│   │   │   │   ├── constants_via_nesting.rb
│   │   │   │   ├── depth.rb
│   │   │   │   ├── duplicate_locals.rb
│   │   │   │   ├── grandchild.rb
│   │   │   │   ├── implicit_self.rb
│   │   │   │   ├── intersection.rb
│   │   │   │   ├── keywords.rb
│   │   │   │   ├── locals.rb
│   │   │   │   ├── locals_and_methods.rb
│   │   │   │   ├── no_parens.A.rbedited
│   │   │   │   ├── no_parens.rb
│   │   │   │   ├── non_prefix.rb
│   │   │   │   ├── overloads_test.A.rbedited
│   │   │   │   ├── overloads_test.B.rbedited
│   │   │   │   ├── overloads_test.rb
│   │   │   │   ├── private.rb
│   │   │   │   ├── redefinition.rb
│   │   │   │   ├── sig.A.rbedited
│   │   │   │   ├── sig.B.rbedited
│   │   │   │   ├── sig.rb
│   │   │   │   ├── sig_all_untyped.A.rbedited
│   │   │   │   ├── sig_all_untyped.rb
│   │   │   │   ├── sig_many_defs.A.rbedited
│   │   │   │   ├── sig_many_defs.rb
│   │   │   │   ├── sig_no_defs.A.rbedited
│   │   │   │   ├── sig_no_defs.rb
│   │   │   │   ├── sig_no_method.A.rbedited
│   │   │   │   ├── sig_no_method.B.rbedited
│   │   │   │   ├── sig_no_method.rb
│   │   │   │   ├── sig_redefinition__1.A.rbedited
│   │   │   │   ├── sig_redefinition__1.B.rbedited
│   │   │   │   ├── sig_redefinition__1.rb
│   │   │   │   ├── sig_redefinition__2.A.rbedited
│   │   │   │   ├── sig_redefinition__2.B.rbedited
│   │   │   │   ├── sig_redefinition__2.rb
│   │   │   │   ├── sig_root.A.rbedited
│   │   │   │   ├── sig_root.rb
│   │   │   │   ├── sig_singleton.A.rbedited
│   │   │   │   ├── sig_singleton.B.rbedited
│   │   │   │   ├── sig_singleton.rb
│   │   │   │   ├── sig_snippet.A.rbedited
│   │   │   │   ├── sig_snippet.B.rbedited
│   │   │   │   ├── sig_snippet.C.rbedited
│   │   │   │   ├── sig_snippet.D.rbedited
│   │   │   │   ├── sig_snippet.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── snippet_arity.A.rbedited
│   │   │   │   ├── snippet_arity.B.rbedited
│   │   │   │   ├── snippet_arity.C.rbedited
│   │   │   │   ├── snippet_arity.D.rbedited
│   │   │   │   ├── snippet_arity.rb
│   │   │   │   ├── snippet_block.A.rbedited
│   │   │   │   ├── snippet_block.B.rbedited
│   │   │   │   ├── snippet_block.C.rbedited
│   │   │   │   ├── snippet_block.D.rbedited
│   │   │   │   ├── snippet_block.rb
│   │   │   │   ├── snippet_block_arity.A.rbedited
│   │   │   │   ├── snippet_block_arity.B.rbedited
│   │   │   │   ├── snippet_block_arity.C.rbedited
│   │   │   │   ├── snippet_block_arity.rb
│   │   │   │   ├── snippet_default.A.rbedited
│   │   │   │   ├── snippet_default.rb
│   │   │   │   ├── snippet_keywords.A.rbedited
│   │   │   │   ├── snippet_keywords.B.rbedited
│   │   │   │   ├── snippet_keywords.C.rbedited
│   │   │   │   ├── snippet_keywords.D.rbedited
│   │   │   │   ├── snippet_keywords.E.rbedited
│   │   │   │   ├── snippet_keywords.rb
│   │   │   │   ├── snippet_repeated.A.rbedited
│   │   │   │   ├── snippet_repeated.B.rbedited
│   │   │   │   ├── snippet_repeated.C.rbedited
│   │   │   │   ├── snippet_repeated.D.rbedited
│   │   │   │   ├── snippet_repeated.rb
│   │   │   │   ├── snippet_types.A.rbedited
│   │   │   │   ├── snippet_types.B.rbedited
│   │   │   │   ├── snippet_types.rb
│   │   │   │   └── union.rb
│   │   │   ├── cvar__definition.rb
│   │   │   ├── cvar__usage.rb
│   │   │   ├── definition_untyped.rb
│   │   │   ├── definitions_and_usages.rb
│   │   │   ├── definitions_and_usages_2.rb
│   │   │   ├── definitions_and_usages_untyped__typed.rb
│   │   │   ├── definitions_and_usages_untyped__untyped.rb
│   │   │   ├── document_symbols.rb
│   │   │   ├── document_symbols.rb.document-symbols.exp
│   │   │   ├── document_symbols_crash.1.rbupdate
│   │   │   ├── document_symbols_crash.1.rbupdate.document-symbols.exp
│   │   │   ├── document_symbols_crash.rb
│   │   │   ├── document_symbols_crash.rb.document-symbols.exp
│   │   │   ├── errors_clear_after_fixing.1.rbupdate
│   │   │   ├── errors_clear_after_fixing.rb
│   │   │   ├── fast_path
│   │   │   │   ├── class_add_member.1.rbupdate
│   │   │   │   ├── class_add_member.rb
│   │   │   │   ├── class_change_include_multifile__included.rb
│   │   │   │   ├── class_change_include_multifile__using.1.rbupdate
│   │   │   │   ├── class_change_include_multifile__using.rb
│   │   │   │   ├── class_change_superclass.1.rbupdate
│   │   │   │   ├── class_change_superclass.rb
│   │   │   │   ├── class_change_superclass_multifile__child.1.rbupdate
│   │   │   │   ├── class_change_superclass_multifile__child.rb
│   │   │   │   ├── class_change_superclass_multifile__super.rb
│   │   │   │   ├── class_remove_member.1.rbupdate
│   │   │   │   ├── class_remove_member.rb
│   │   │   │   ├── method_add_argument.1.rbupdate
│   │   │   │   ├── method_add_argument.rb
│   │   │   │   ├── method_add_keyword_arg.1.rbupdate
│   │   │   │   ├── method_add_keyword_arg.rb
│   │   │   │   ├── method_add_sig.1.rbupdate
│   │   │   │   ├── method_add_sig.rb
│   │   │   │   ├── method_change_argument_kind.1.rbupdate
│   │   │   │   ├── method_change_argument_kind.rb
│   │   │   │   ├── method_change_argument_type__def.1.rbupdate
│   │   │   │   ├── method_change_argument_type__def.rb
│   │   │   │   ├── method_change_argument_type__usage.1.rbupdate
│   │   │   │   ├── method_change_argument_type__usage.rb
│   │   │   │   ├── method_change_kw_arg_name.1.rbupdate
│   │   │   │   ├── method_change_kw_arg_name.rb
│   │   │   │   ├── method_change_kw_arg_type.1.rbupdate
│   │   │   │   ├── method_change_kw_arg_type.rb
│   │   │   │   ├── method_change_return_type__def.1.rbupdate
│   │   │   │   ├── method_change_return_type__def.rb
│   │   │   │   ├── method_change_return_type__usage.1.rbupdate
│   │   │   │   ├── method_change_return_type__usage.rb
│   │   │   │   ├── method_rename_argument.1.rbupdate
│   │   │   │   ├── method_rename_argument.rb
│   │   │   │   ├── method_signature_update.1.rbupdate
│   │   │   │   ├── method_signature_update.rb
│   │   │   │   ├── method_signature_update_generics__def.1.rbupdate
│   │   │   │   ├── method_signature_update_generics__def.rb
│   │   │   │   ├── method_signature_update_generics__usage.rb
│   │   │   │   ├── method_signature_update_static__def.1.rbupdate
│   │   │   │   ├── method_signature_update_static__def.rb
│   │   │   │   ├── method_signature_update_static__usage.rb
│   │   │   │   ├── parse_errors.1.rbupdate
│   │   │   │   ├── parse_errors.2.rbupdate
│   │   │   │   ├── parse_errors.3.rbupdate
│   │   │   │   ├── parse_errors.rb
│   │   │   │   ├── string_literal_change.1.rbupdate
│   │   │   │   ├── string_literal_change.rb
│   │   │   │   ├── undefined_constant.1.rbupdate
│   │   │   │   └── undefined_constant.rb
│   │   │   ├── field_edits.1.rbupdate
│   │   │   ├── field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── field_edits.2.rbupdate
│   │   │   ├── field_edits.2.rbupdate.document-symbols.exp
│   │   │   ├── field_edits.rb
│   │   │   ├── field_edits.rb.document-symbols.exp
│   │   │   ├── genericMethods.rb
│   │   │   ├── go_to_type_definition.rb
│   │   │   ├── go_to_type_definition_applied.rb
│   │   │   ├── go_to_type_definition_false.rb
│   │   │   ├── good_field_edits.1.rbupdate
│   │   │   ├── good_field_edits.1.rbupdate.document-symbols.exp
│   │   │   ├── good_field_edits.rb
│   │   │   ├── good_field_edits.rb.document-symbols.exp
│   │   │   ├── highlight.rb
│   │   │   ├── highlight_simple.rb
│   │   │   ├── hover.rb
│   │   │   ├── hover_abstract.rb
│   │   │   ├── hover_ampersand_operations.rb
│   │   │   ├── hover_ancestors.rb
│   │   │   ├── hover_conditional_type_narrowing.rb
│   │   │   ├── hover_constants.rb
│   │   │   ├── hover_generics.rb
│   │   │   ├── hover_method.rb
│   │   │   ├── hover_method_for_building_arrays_and_hashes.rb
│   │   │   ├── hover_method_includes_defs.rb
│   │   │   ├── hover_method_not_found.rb
│   │   │   ├── hover_operator_overload.rb
│   │   │   ├── hover_override.rb
│   │   │   ├── hover_proc_void.rb
│   │   │   ├── hover_untyped.rb
│   │   │   ├── hover_untyped_keyword_arg.rb
│   │   │   ├── hover_untyped_proc_arg.rb
│   │   │   ├── include_and_extend.rb
│   │   │   ├── ivars.rb
│   │   │   ├── missing_typed_sigil.A.rbedited
│   │   │   ├── missing_typed_sigil.rb
│   │   │   ├── no_stdlib.rb
│   │   │   ├── sig_deletion.1.rbupdate
│   │   │   ├── sig_deletion.rb
│   │   │   ├── struct_fuzz.rb
│   │   │   ├── symbol_query_filter__helper.rb
│   │   │   ├── symbol_query_filter__main.rb
│   │   │   ├── type_aliases.rb
│   │   │   ├── untyped_field_reference__1.rb
│   │   │   ├── untyped_field_reference__2.rb
│   │   │   ├── workspace_symbols_boundaries.rb
│   │   │   ├── workspace_symbols_fullname.rb
│   │   │   ├── workspace_symbols_minitest.rb
│   │   │   ├── workspace_symbols_minitest_scope.rb
│   │   │   ├── workspace_symbols_multiply_defined.rb
│   │   │   ├── workspace_symbols_multiply_defined_2.rb
│   │   │   ├── workspace_symbols_namespaces.rb
│   │   │   ├── workspace_symbols_shortname.rb
│   │   │   ├── workspace_symbols_sparse.rb
│   │   │   └── workspace_symbols_stdlib.rb
│   │   ├── namer
│   │   │   ├── alias_cross_file.flatten-tree.exp
│   │   │   ├── alias_cross_file.symbol-table-raw.exp
│   │   │   ├── alias_cross_file__01.rb
│   │   │   ├── alias_cross_file__02.rb
│   │   │   ├── alias_method.rb
│   │   │   ├── alias_method.rb.symbol-table-raw.exp
│   │   │   ├── alias_method_redefinition.rb
│   │   │   ├── ancestors.rb
│   │   │   ├── ancestors.rb.flatten-tree.exp
│   │   │   ├── ancestors.rb.symbol-table-raw.exp
│   │   │   ├── arguments.rb
│   │   │   ├── arguments.rb.desugar-tree-raw.exp
│   │   │   ├── arguments.rb.desugar-tree.exp
│   │   │   ├── arguments.rb.flatten-tree-raw.exp
│   │   │   ├── arguments.rb.flatten-tree.exp
│   │   │   ├── arguments.rb.symbol-table-raw.exp
│   │   │   ├── array_sum.rb
│   │   │   ├── block_in_class.rb
│   │   │   ├── blocks_in_reopened_class.rb
│   │   │   ├── bug_1425.rb
│   │   │   ├── circular_mixin.rb
│   │   │   ├── circular_mixin.rb.symbol-table-raw.exp
│   │   │   ├── class_and_alias.rb
│   │   │   ├── class_and_alias.rb.flatten-tree.exp
│   │   │   ├── class_and_alias.rb.symbol-table-raw.exp
│   │   │   ├── class_module.rb
│   │   │   ├── conflicting_names.rb
│   │   │   ├── conflicting_names.rb.flatten-tree.exp
│   │   │   ├── conflicting_names.rb.symbol-table-raw.exp
│   │   │   ├── constant_in_block.rb
│   │   │   ├── constant_in_block.rb.symbol-table-raw.exp
│   │   │   ├── constant_redefinition
│   │   │   │   ├── class_then_constant.rb
│   │   │   │   ├── class_then_constant_then_reopen.rb
│   │   │   │   ├── constant_then_class.rb
│   │   │   │   ├── constant_then_module.rb
│   │   │   │   ├── module_then_constant.rb
│   │   │   │   └── module_then_constant_then_reopen.rb
│   │   │   ├── constant_types.rb
│   │   │   ├── constant_types.rb.symbol-table-raw.exp
│   │   │   ├── constants.rb
│   │   │   ├── constants.rb.flatten-tree.exp
│   │   │   ├── constants.rb.symbol-table-raw.exp
│   │   │   ├── defs_in_blocks.rb
│   │   │   ├── defs_in_blocks.rb.flatten-tree.exp
│   │   │   ├── defs_in_blocks.rb.symbol-table-raw.exp
│   │   │   ├── docs_example_1.rb
│   │   │   ├── docs_example_2.rb
│   │   │   ├── duplicate_scope.rb
│   │   │   ├── dynamic_constant.rb
│   │   │   ├── dynamic_method_with_class.rb
│   │   │   ├── dynamic_method_with_class.rb.symbol-table-raw.exp
│   │   │   ├── extend.rb
│   │   │   ├── extend.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_class_in_field.rb
│   │   │   ├── fuzz_dynamic_constant.rb
│   │   │   ├── fuzz_ivar_constant_scope.rb
│   │   │   ├── fuzz_repeated_kwarg.rb
│   │   │   ├── fuzz_shared_singletons.rb
│   │   │   ├── fuzz_type_member.rb
│   │   │   ├── fuzz_type_template_overwrite.rb
│   │   │   ├── fuzz_type_template_overwrite.rb.symbol-table-raw.exp
│   │   │   ├── gvar.rb
│   │   │   ├── gvar.rb.flatten-tree.exp
│   │   │   ├── gvar.rb.symbol-table-raw.exp
│   │   │   ├── include_noarg.rb
│   │   │   ├── locals.rb
│   │   │   ├── locals.rb.flatten-tree.exp
│   │   │   ├── locals.rb.symbol-table-raw.exp
│   │   │   ├── method_redef.rb
│   │   │   ├── module_function.rb
│   │   │   ├── module_function.rb.cfg.exp
│   │   │   ├── module_function.rb.symbol-table-raw.exp
│   │   │   ├── multiple_stubs.rb
│   │   │   ├── multiple_stubs.rb.flatten-tree.exp
│   │   │   ├── multiple_stubs.rb.symbol-table-raw.exp
│   │   │   ├── nested_class.rb
│   │   │   ├── nested_class.rb.flatten-tree.exp
│   │   │   ├── nested_static_field.rb
│   │   │   ├── net_imap.rb
│   │   │   ├── next_break.rb
│   │   │   ├── next_break.rb.flatten-tree.exp
│   │   │   ├── payload_name.rb
│   │   │   ├── redefine.rb
│   │   │   ├── redefines_object.rb
│   │   │   ├── redefines_object.rb.cfg.exp
│   │   │   ├── redefinition_method.rb
│   │   │   ├── redefinition_method.rb.flatten-tree.exp
│   │   │   ├── root_private.rb
│   │   │   ├── root_private.rb.symbol-table-raw.exp
│   │   │   ├── self_constant.rb
│   │   │   ├── self_disallowed.rb
│   │   │   ├── simple.rb
│   │   │   ├── simple.rb.desugar-tree.exp
│   │   │   ├── simple.rb.flatten-tree.exp
│   │   │   ├── simple.rb.parse-tree.exp
│   │   │   ├── simple.rb.symbol-table-raw.exp
│   │   │   ├── singleton_class.rb
│   │   │   ├── singleton_class.rb.symbol-table-raw.exp
│   │   │   ├── super.rb
│   │   │   ├── superclass_redefinition.rb
│   │   │   ├── superclass_redefinition.rb.symbol-table-raw.exp
│   │   │   ├── type_alias.rb
│   │   │   ├── type_alias.rb.symbol-table-raw.exp
│   │   │   ├── type_member_redefs__1.rb
│   │   │   ├── type_member_redefs__2.rb
│   │   │   ├── visibility.rb
│   │   │   ├── visibility.rb.symbol-table-raw.exp
│   │   │   ├── yield.rb
│   │   │   ├── yield.rb.cfg.exp
│   │   │   ├── yield.rb.flatten-tree.exp
│   │   │   └── yield.rb.symbol-table-raw.exp
│   │   ├── parser
│   │   │   ├── and_and_bug.rb
│   │   │   ├── and_and_bug.rb.parse-tree.exp
│   │   │   ├── anon_blockarg.rb
│   │   │   ├── chained_sends_lots.rb
│   │   │   ├── chained_sends_lots.rb.desugar-tree.exp
│   │   │   ├── compare_overload_parse_error.rb
│   │   │   ├── complement_literal.rb
│   │   │   ├── complement_literal.rb.desugar-tree.exp
│   │   │   ├── complement_literal.rb.parse-tree.exp
│   │   │   ├── empty.rb
│   │   │   ├── empty.rb.parse-tree.exp
│   │   │   ├── encoding.rb
│   │   │   ├── error_recovery_assign.rb
│   │   │   ├── error_recovery_assign.rb.parse-tree.exp
│   │   │   ├── error_recovery_const_case.rb
│   │   │   ├── error_recovery_const_case.rb.parse-tree.exp
│   │   │   ├── error_recovery_constant_only_scope.rb
│   │   │   ├── error_recovery_constant_only_scope.rb.parse-tree.exp
│   │   │   ├── error_recovery_if_no_end.rb
│   │   │   ├── error_recovery_if_no_end.rb.parse-tree.exp
│   │   │   ├── error_recovery_missing_fun.rb
│   │   │   ├── error_recovery_missing_fun.rb.parse-tree.exp
│   │   │   ├── error_recovery_multiple_stmts.rb
│   │   │   ├── error_recovery_multiple_stmts.rb.parse-tree.exp
│   │   │   ├── fuzz_def_begin.rb
│   │   │   ├── fuzz_ivar.rb
│   │   │   ├── fuzz_rational.rb
│   │   │   ├── heredoc_chomp.rb
│   │   │   ├── heredoc_chomp.rb.desugar-tree.exp
│   │   │   ├── index_assign.rb
│   │   │   ├── index_assign.rb.parse-tree.exp
│   │   │   ├── invalid_fatal.rb
│   │   │   ├── invalid_fatal.rb.parse-tree.exp
│   │   │   ├── invalid_syntax_error.rb
│   │   │   ├── invalid_syntax_error.rb.parse-tree.exp
│   │   │   ├── invalid_trailing_in_number.rb
│   │   │   ├── invalid_trailing_in_number.rb.parse-tree.exp
│   │   │   ├── long_string.rb
│   │   │   ├── long_string.rb.parse-tree.exp
│   │   │   ├── misc.rb
│   │   │   ├── misc.rb.desugar-tree.exp
│   │   │   ├── misc.rb.parse-tree.exp
│   │   │   ├── offset0.rb
│   │   │   ├── offset0.rb.parse-tree.exp
│   │   │   ├── ruby_25.rb
│   │   │   ├── ruby_25.rb.parse-tree.exp
│   │   │   ├── trailing_comas.rb
│   │   │   ├── trailing_comas.rb.parse-tree-json.exp
│   │   │   ├── typed_ignore.rb
│   │   │   ├── unary_num.rb
│   │   │   └── unary_num.rb.parse-tree.exp
│   │   ├── perf
│   │   │   ├── enum_canBeTruthy_regression.rb
│   │   │   └── sealed_registration_perf.rb
│   │   ├── proc_variance.rb
│   │   ├── rbi
│   │   │   ├── argf.rb
│   │   │   ├── array.rb
│   │   │   ├── basicobject_instance_eval.rb
│   │   │   ├── basicobject_instance_exec.rb
│   │   │   ├── bigdecimal.rb
│   │   │   ├── class.rb
│   │   │   ├── date.rb
│   │   │   ├── each_with_object.rb
│   │   │   ├── enumerable.rb
│   │   │   ├── enumerable_flat_map.rb
│   │   │   ├── etc.rb
│   │   │   ├── exception.rb
│   │   │   ├── falseclass.rb
│   │   │   ├── hash.rb
│   │   │   ├── int_float.rb
│   │   │   ├── io.rb
│   │   │   ├── json.rb
│   │   │   ├── kernel.rb
│   │   │   ├── kernel_array.rb
│   │   │   ├── kernel_class.rb
│   │   │   ├── lazy_enumerator.rb
│   │   │   ├── module.rb
│   │   │   ├── object.rb
│   │   │   ├── object_constant.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc_sig_strong.rb
│   │   │   ├── process.rb
│   │   │   ├── random.rb
│   │   │   ├── range.rb
│   │   │   ├── regexp.rb
│   │   │   ├── ruby_typer.rb
│   │   │   ├── sorbet.rb
│   │   │   ├── string.rb
│   │   │   ├── t.rb
│   │   │   ├── tempfile.rb
│   │   │   ├── time.rb
│   │   │   ├── to_sym.rb
│   │   │   ├── trueclass.rb
│   │   │   ├── uri.rb
│   │   │   ├── with_without__1.rbi
│   │   │   ├── with_without__2.rbi
│   │   │   └── with_without__3.rb
│   │   ├── resolver
│   │   │   ├── abstract.rb
│   │   │   ├── abstract_out_of_order.rb
│   │   │   ├── abstract_override.rb
│   │   │   ├── abstract_types.rb
│   │   │   ├── abstract_validation.rb
│   │   │   ├── alias.rb
│   │   │   ├── alias.rb.symbol-table-raw.exp
│   │   │   ├── alias_define_method__01.rb
│   │   │   ├── alias_define_method__02.rb
│   │   │   ├── alias_without_alias.rb
│   │   │   ├── ancestor_scope.rb
│   │   │   ├── ancestor_scope.rb.flatten-tree.exp
│   │   │   ├── bad_alias.rb
│   │   │   ├── bad_final_sig.rb
│   │   │   ├── bad_hash.rb
│   │   │   ├── bad_param_ordering.rb
│   │   │   ├── bad_sealed_class__1.rb
│   │   │   ├── bad_sealed_class__2.rb
│   │   │   ├── bad_sealed_class_absurd__1.rb
│   │   │   ├── bad_sealed_class_absurd__2.rb
│   │   │   ├── bad_sealed_module__1.rb
│   │   │   ├── bad_sealed_module__2.rb
│   │   │   ├── bad_synthesize.rb
│   │   │   ├── bare_generics.rb
│   │   │   ├── bare_generics_strict.rb
│   │   │   ├── bare_stdlib_generics.rb
│   │   │   ├── bind.rb
│   │   │   ├── bind_class_of.rb
│   │   │   ├── bind_class_of.rb.symbol-table-raw.exp
│   │   │   ├── bool_alias.rb
│   │   │   ├── cbase.rb
│   │   │   ├── cbase.rb.symbol-table-raw.exp
│   │   │   ├── choose_sig.rb
│   │   │   ├── circle_of_itself.rb
│   │   │   ├── class_instance_vars.rb
│   │   │   ├── class_instance_vars.rb.flatten-tree.exp
│   │   │   ├── class_instance_vars.rb.symbol-table-raw.exp
│   │   │   ├── class_ivar.rb
│   │   │   ├── constant_in_typealias.rb
│   │   │   ├── crash.rb
│   │   │   ├── default_arg_in_block.rb
│   │   │   ├── defined.rb
│   │   │   ├── empty_sealed.rb
│   │   │   ├── enumerable_strict.rb
│   │   │   ├── field.rb
│   │   │   ├── field.rb.flatten-tree-raw.exp
│   │   │   ├── field_across_file__01.rb
│   │   │   ├── field_across_file__02.rb
│   │   │   ├── final_method.rb
│   │   │   ├── final_module.rb
│   │   │   ├── flatten.rb
│   │   │   ├── flatten.rb.flatten-tree.exp
│   │   │   ├── flatten.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_alias_to_type_alias.rb
│   │   │   ├── fuzz_ancester.rb
│   │   │   ├── fuzz_class_of_static_field.rb
│   │   │   ├── fuzz_empty_type_alias.rb
│   │   │   ├── fuzz_include_infinite_typealias.rb
│   │   │   ├── fuzz_include_type_alias.rb
│   │   │   ├── fuzz_infinite_type.rb
│   │   │   ├── fuzz_mixes_in_class_methods.rb
│   │   │   ├── fuzz_multiple_sigs.rb
│   │   │   ├── fuzz_sig_generic_field.rb
│   │   │   ├── fuzz_sig_parsing.rb
│   │   │   ├── fuzz_sig_weird.rb
│   │   │   ├── fuzz_suggest.rb
│   │   │   ├── fuzz_top_level_abstract.rb
│   │   │   ├── fuzz_type_member_forget.rb
│   │   │   ├── fuzz_type_member_scope.rb
│   │   │   ├── fuzz_type_member_scope.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_wrong_fun_print.rb
│   │   │   ├── generic_class_alias.rb
│   │   │   ├── implementations.rb
│   │   │   ├── infinite.rb
│   │   │   ├── inherit_alias.rb
│   │   │   ├── inherit_alias.rb.symbol-table-raw.exp
│   │   │   ├── invalid_alias.rb
│   │   │   ├── invalid_alias.rb.symbol-table-raw.exp
│   │   │   ├── lazy_field.rb
│   │   │   ├── let_errors.rb
│   │   │   ├── let_errors.rb.symbol-table-raw.exp
│   │   │   ├── let_errors_false.rb
│   │   │   ├── let_errors_nilable.rb
│   │   │   ├── let_var.rb
│   │   │   ├── let_var.rb.symbol-table-raw.exp
│   │   │   ├── linearization
│   │   │   │   ├── constant_resolution_before_linearization.rb
│   │   │   │   ├── includes_class.rb
│   │   │   │   ├── includes_class.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization1.rb
│   │   │   │   ├── linearization1.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization2.rb
│   │   │   │   ├── linearization2.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization3.rb
│   │   │   │   ├── linearization3.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization4.rb
│   │   │   │   ├── linearization4.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization4a.rb
│   │   │   │   ├── linearization4a.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization5.rb
│   │   │   │   ├── linearization5.rb.symbol-table-raw.exp
│   │   │   │   ├── linearization6.rb
│   │   │   │   └── linearization6.rb.symbol-table-raw.exp
│   │   │   ├── missing_helpers_abstract.rb
│   │   │   ├── missing_helpers_interface.rb
│   │   │   ├── missing_type_combinator_args.rb
│   │   │   ├── missing_type_combinator_args.rb.flatten-tree.exp
│   │   │   ├── mixes_in_class_methods.rb
│   │   │   ├── mixes_in_class_methods.rb.symbol-table-raw.exp
│   │   │   ├── mixes_in_class_methods_twice.rb
│   │   │   ├── nested_sealed.rb
│   │   │   ├── no_runtime_sig.rb
│   │   │   ├── non_builder_sig.rb
│   │   │   ├── object_include_stub.rb
│   │   │   ├── optional_block.rb
│   │   │   ├── optional_cyclic.rb
│   │   │   ├── optional_nested.rb
│   │   │   ├── optional_nil.rb
│   │   │   ├── optional_nil.rb.flatten-tree.exp
│   │   │   ├── optional_nil.rb.name-tree.exp
│   │   │   ├── overloads_test.rb
│   │   │   ├── override_shapes.rb
│   │   │   ├── overrides.rb
│   │   │   ├── proc.rb
│   │   │   ├── proc.rb.symbol-table-raw.exp
│   │   │   ├── rbi_final_no_sig__1.rb
│   │   │   ├── rbi_final_no_sig__2.rb
│   │   │   ├── rbi_final_re_sig__1.rb
│   │   │   ├── rbi_final_re_sig__2.rb
│   │   │   ├── recover_from_bad_superclass.rb
│   │   │   ├── recover_from_bad_superclass.rb.symbol-table-raw.exp
│   │   │   ├── redefinition_of_subclass_type_member.rb
│   │   │   ├── resolution_order.rb
│   │   │   ├── resolution_order.rb.symbol-table-raw.exp
│   │   │   ├── resolution_scoping.rb
│   │   │   ├── resolution_scoping.rb.symbol-table-raw.exp
│   │   │   ├── resolve_errors.rb
│   │   │   ├── resolve_through_alias.rb
│   │   │   ├── resolve_through_alias.rb.symbol-table-raw.exp
│   │   │   ├── resolve_through_type_alias.rb
│   │   │   ├── resolve_tree_printing.rb
│   │   │   ├── resolve_tree_printing.rb.flatten-tree-raw.exp
│   │   │   ├── resolve_via_ancestors
│   │   │   │   ├── mixin.rb
│   │   │   │   ├── simple.rb
│   │   │   │   ├── superclass_three_pass.rb
│   │   │   │   └── two_mixins.rb
│   │   │   ├── reveal_type.rb
│   │   │   ├── sealed_aliases.rb
│   │   │   ├── sealed_class.rb
│   │   │   ├── sealed_concrete_parent_class.rb
│   │   │   ├── sealed_module.rb
│   │   │   ├── sealed_stdlib.rb
│   │   │   ├── sealed_with_rbi__1.rb
│   │   │   ├── sealed_with_rbi__2.rb
│   │   │   ├── sealed_with_rbi__3.rbi
│   │   │   ├── self.rb
│   │   │   ├── self.rb.symbol-table-raw.exp
│   │   │   ├── self_ancestor.rb
│   │   │   ├── sig_bad.rb
│   │   │   ├── sig_bad.rb.symbol-table-raw.exp
│   │   │   ├── sig_compat.rb
│   │   │   ├── sig_compat.rb.symbol-table-raw.exp
│   │   │   ├── sig_generated.rb
│   │   │   ├── sig_good.rb
│   │   │   ├── sig_good.rb.symbol-table-raw.exp
│   │   │   ├── sig_in_block.rb
│   │   │   ├── sig_misc.rb
│   │   │   ├── sig_misc.rb.symbol-table-raw.exp
│   │   │   ├── sig_on_failure.rb
│   │   │   ├── sig_returns_nil.rb
│   │   │   ├── sig_void.rb
│   │   │   ├── simple.rb
│   │   │   ├── simple.rb.flatten-tree.exp
│   │   │   ├── simple.rb.symbol-table-raw.exp
│   │   │   ├── strict.rb
│   │   │   ├── stub_assign.rb
│   │   │   ├── stub_missing_class_alias.rb
│   │   │   ├── stub_missing_class_alias.rb.symbol-table-raw.exp
│   │   │   ├── stubs_typed_untyped.flatten-tree.exp
│   │   │   ├── stubs_typed_untyped__1.rb
│   │   │   ├── stubs_typed_untyped__2.rb
│   │   │   ├── t_struct_subclass.rb
│   │   │   ├── top_level.rb
│   │   │   ├── top_level_abstract_typed_true.rb
│   │   │   ├── top_level_include.rb
│   │   │   ├── top_level_sig.rb
│   │   │   ├── type_alias.rb
│   │   │   ├── type_alias_order.rb
│   │   │   ├── type_arguments.rb
│   │   │   ├── type_arguments.rb.symbol-table-raw.exp
│   │   │   ├── type_member_bad_parent.rb
│   │   │   ├── type_member_constant_assignment.rb
│   │   │   ├── type_member_constant_assignment.rb.symbol-table-raw.exp
│   │   │   ├── type_member_cycle.rb
│   │   │   ├── type_member_fixed_order.rb
│   │   │   ├── type_member_in_method.rb
│   │   │   ├── type_member_missing.rb
│   │   │   ├── type_member_missing.rb.symbol-table-raw.exp
│   │   │   ├── type_member_missing_then_use.rb
│   │   │   ├── type_member_out_of_order.rb
│   │   │   ├── type_member_singleton_members.rb
│   │   │   ├── type_member_singleton_members.rb.symbol-table-raw.exp
│   │   │   ├── type_members.rb
│   │   │   ├── unnamed_proc_arguments.rb
│   │   │   ├── unsigged_default.rb
│   │   │   ├── untyped_generics.rb
│   │   │   └── void.rb
│   │   ├── rewriter
│   │   │   ├── attr.rb
│   │   │   ├── attr.rb.flatten-tree.exp
│   │   │   ├── attr.rb.symbol-table-raw.exp
│   │   │   ├── attr_bad_string.rb
│   │   │   ├── attr_multi.rb
│   │   │   ├── attr_multi.rb.rewrite-tree.exp
│   │   │   ├── attr_multi.rb.symbol-table-raw.exp
│   │   │   ├── attr_parameters.rb
│   │   │   ├── attr_strict.rb
│   │   │   ├── chalk_odm_document.rb
│   │   │   ├── chalk_odm_document.rb.rewrite-tree.exp
│   │   │   ├── class_new.rb
│   │   │   ├── class_new.rb.rewrite-tree.exp
│   │   │   ├── coerce_in_prop.rb
│   │   │   ├── command.rb
│   │   │   ├── command.rb.rewrite-tree.exp
│   │   │   ├── default_args.rb
│   │   │   ├── default_args.rb.rewrite-tree.exp
│   │   │   ├── default_args_edge.rb
│   │   │   ├── default_args_malformed_sig.rb
│   │   │   ├── dsl_builder.rb
│   │   │   ├── dsl_builder.rb.rewrite-tree.exp
│   │   │   ├── empty_until.rb
│   │   │   ├── flatfile_dsl.rb
│   │   │   ├── flatfile_dsl.rb.rewrite-tree.exp
│   │   │   ├── flatten_module_private_class_method.rb
│   │   │   ├── flatten_module_private_class_method.rb.symbol-table-raw.exp
│   │   │   ├── flatten_nested.rb
│   │   │   ├── flatten_nested.rb.flatten-tree.exp
│   │   │   ├── flatten_nested_sclass.rb
│   │   │   ├── flatten_nested_sclass.rb.symbol-table-raw.exp
│   │   │   ├── fuzz_attr_bare.rb
│   │   │   ├── fuzz_attr_no_args.rb
│   │   │   ├── fuzz_chalk_bad_type.rb
│   │   │   ├── fuzz_dup_type_not_constant.rb
│   │   │   ├── fuzz_encrypted_prop.rb
│   │   │   ├── fuzz_optinal_crash.rb
│   │   │   ├── fuzz_prop_weird_member.rb
│   │   │   ├── fuzz_qualified_lhs.rb
│   │   │   ├── generic_module_function.rb
│   │   │   ├── initializer.rb
│   │   │   ├── interface_wrapper.rb
│   │   │   ├── interface_wrapper.rb.rewrite-tree.exp
│   │   │   ├── minitest.rb
│   │   │   ├── minitest.rb.rewrite-tree.exp
│   │   │   ├── minitest_tables.rb
│   │   │   ├── minitest_tables.rb.rewrite-tree.exp
│   │   │   ├── nesting.rb
│   │   │   ├── not_prop.rb
│   │   │   ├── not_prop.rb.rewrite-tree.exp
│   │   │   ├── not_prop.rb.symbol-table.exp
│   │   │   ├── prop.rb
│   │   │   ├── prop.rb.rewrite-tree-raw.exp
│   │   │   ├── prop.rb.rewrite-tree.exp
│   │   │   ├── prop.rb.symbol-table-raw.exp
│   │   │   ├── prop_computed_by.rb
│   │   │   ├── prop_computed_by.rb.rewrite-tree.exp
│   │   │   ├── prop_default.rb
│   │   │   ├── prop_enum.rb
│   │   │   ├── prop_get_logic.rb
│   │   │   ├── prop_in_module.rb
│   │   │   ├── prop_in_module.rb.rewrite-tree.exp
│   │   │   ├── prop_missing.rb
│   │   │   ├── prop_missing.rb.rewrite-tree.exp
│   │   │   ├── prop_mutator.rb
│   │   │   ├── prop_proc_regression.rb
│   │   │   ├── prop_prohibit_shapes_and_tuples.rb
│   │   │   ├── prop_skipped.rb
│   │   │   ├── protobuf_descriptor_pool.rb
│   │   │   ├── protobuf_descriptor_pool.rb.rewrite-tree.exp
│   │   │   ├── rails
│   │   │   │   ├── cattr_accessor.rb
│   │   │   │   ├── cattr_accessor.rb.rewrite-tree.exp
│   │   │   │   ├── cattr_reader.rb
│   │   │   │   ├── cattr_reader.rb.rewrite-tree.exp
│   │   │   │   ├── cattr_writer.rb
│   │   │   │   ├── cattr_writer.rb.rewrite-tree.exp
│   │   │   │   ├── class_attribute.rb
│   │   │   │   ├── class_attribute.rb.rewrite-tree.exp
│   │   │   │   ├── delegate.rb
│   │   │   │   ├── delegate.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_accessor.rb
│   │   │   │   ├── mattr_accessor.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_reader.rb
│   │   │   │   ├── mattr_reader.rb.rewrite-tree.exp
│   │   │   │   ├── mattr_writer.rb
│   │   │   │   ├── mattr_writer.rb.rewrite-tree.exp
│   │   │   │   └── migration.rb
│   │   │   ├── stringy_struct.rb
│   │   │   ├── struct.rb
│   │   │   ├── struct.rb.rewrite-tree.exp
│   │   │   ├── struct.rb.symbol-table-raw.exp
│   │   │   ├── struct_initialize.rb
│   │   │   ├── struct_self.rb
│   │   │   ├── t_enum.rb
│   │   │   ├── t_enum_alias.rb
│   │   │   ├── t_enum_all.rb
│   │   │   ├── t_enum_exhaustiveness.rb
│   │   │   ├── t_enum_override_initialize.rb
│   │   │   ├── t_enum_snapshot.rb
│   │   │   ├── t_enum_snapshot.rb.rewrite-tree.exp
│   │   │   ├── t_enum_type_syntax.rb
│   │   │   └── t_struct
│   │   │       ├── custom_initialize.rb
│   │   │       ├── default.rb
│   │   │       ├── default.rb.rewrite-tree.exp
│   │   │       ├── default_bad.rb
│   │   │       ├── inexact.rb
│   │   │       ├── inexact.rb.rewrite-tree.exp
│   │   │       ├── nilable.rb
│   │   │       ├── nilable.rb.rewrite-tree.exp
│   │   │       ├── none.rb
│   │   │       ├── none.rb.rewrite-tree.exp
│   │   │       ├── normal.rb
│   │   │       ├── normal.rb.rewrite-tree.exp
│   │   │       ├── override.rb
│   │   │       ├── override.rb.rewrite-tree.exp
│   │   │       ├── override_bad.rb
│   │   │       ├── param_order.rb
│   │   │       ├── param_order.rb.rewrite-tree.exp
│   │   │       ├── root.rb
│   │   │       ├── root.rb.rewrite-tree.exp
│   │   │       ├── some_default.rb
│   │   │       └── some_default.rb.rewrite-tree.exp
│   │   ├── substitutions
│   │   │   └── double_subsitutions.rb
│   │   ├── testrunner
│   │   │   └── pos.rb
│   │   ├── todo
│   │   │   ├── block_in_class.rb
│   │   │   ├── block_in_class.rb.flatten-tree.exp
│   │   │   ├── block_in_class.rb.symbol-table-raw.exp
│   │   │   ├── class_of.rb
│   │   │   ├── const_in_block.rb
│   │   │   ├── const_in_block.rb.symbol-table-raw.exp
│   │   │   └── generics_should_fail.rb
│   │   ├── tuple_type_size.rb
│   │   └── union_method_arity_error.rb
│   └── whitequark
│       ├── test_ENCODING_0.parse-tree-whitequark.exp
│       ├── test_ENCODING_0.rb
│       ├── test_alias_0.parse-tree-whitequark.exp
│       ├── test_alias_0.rb
│       ├── test_alias_gvar_0.parse-tree-whitequark.exp
│       ├── test_alias_gvar_0.rb
│       ├── test_alias_gvar_1.parse-tree-whitequark.exp
│       ├── test_alias_gvar_1.rb
│       ├── test_alias_nth_ref_0.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_0.parse-tree-whitequark.exp
│       ├── test_ambiuous_quoted_label_in_ternary_operator_0.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_1.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_2.rb
│       ├── test_ambiuous_quoted_label_in_ternary_operator_3.rb
│       ├── test_and_0.parse-tree-whitequark.exp
│       ├── test_and_0.rb
│       ├── test_and_1.parse-tree-whitequark.exp
│       ├── test_and_1.rb
│       ├── test_and_asgn_0.parse-tree-whitequark.exp
│       ├── test_and_asgn_0.rb
│       ├── test_and_asgn_1.parse-tree-whitequark.exp
│       ├── test_and_asgn_1.rb
│       ├── test_and_or_masgn_0.parse-tree-whitequark.exp
│       ├── test_and_or_masgn_0.rb
│       ├── test_and_or_masgn_1.parse-tree-whitequark.exp
│       ├── test_and_or_masgn_1.rb
│       ├── test_arg_0.parse-tree-whitequark.exp
│       ├── test_arg_0.rb
│       ├── test_arg_1.parse-tree-whitequark.exp
│       ├── test_arg_1.rb
│       ├── test_arg_combinations_0.parse-tree-whitequark.exp
│       ├── test_arg_combinations_0.rb
│       ├── test_arg_combinations_1.parse-tree-whitequark.exp
│       ├── test_arg_combinations_1.rb
│       ├── test_arg_combinations_10.parse-tree-whitequark.exp
│       ├── test_arg_combinations_10.rb
│       ├── test_arg_combinations_11.parse-tree-whitequark.exp
│       ├── test_arg_combinations_11.rb
│       ├── test_arg_combinations_12.parse-tree-whitequark.exp
│       ├── test_arg_combinations_12.rb
│       ├── test_arg_combinations_13.parse-tree-whitequark.exp
│       ├── test_arg_combinations_13.rb
│       ├── test_arg_combinations_14.parse-tree-whitequark.exp
│       ├── test_arg_combinations_14.rb
│       ├── test_arg_combinations_2.parse-tree-whitequark.exp
│       ├── test_arg_combinations_2.rb
│       ├── test_arg_combinations_3.parse-tree-whitequark.exp
│       ├── test_arg_combinations_3.rb
│       ├── test_arg_combinations_4.parse-tree-whitequark.exp
│       ├── test_arg_combinations_4.rb
│       ├── test_arg_combinations_5.parse-tree-whitequark.exp
│       ├── test_arg_combinations_5.rb
│       ├── test_arg_combinations_6.parse-tree-whitequark.exp
│       ├── test_arg_combinations_6.rb
│       ├── test_arg_combinations_7.parse-tree-whitequark.exp
│       ├── test_arg_combinations_7.rb
│       ├── test_arg_combinations_8.parse-tree-whitequark.exp
│       ├── test_arg_combinations_8.rb
│       ├── test_arg_combinations_9.parse-tree-whitequark.exp
│       ├── test_arg_combinations_9.rb
│       ├── test_arg_duplicate_0.rb
│       ├── test_arg_duplicate_1.rb
│       ├── test_arg_duplicate_2.rb
│       ├── test_arg_duplicate_3.rb
│       ├── test_arg_duplicate_4.rb
│       ├── test_arg_duplicate_5.rb
│       ├── test_arg_duplicate_6.rb
│       ├── test_arg_duplicate_7.rb
│       ├── test_arg_duplicate_8.rb
│       ├── test_arg_duplicate_9.rb
│       ├── test_arg_duplicate_ignored_0.parse-tree-whitequark.exp
│       ├── test_arg_duplicate_ignored_0.rb
│       ├── test_arg_duplicate_ignored_1.parse-tree-whitequark.exp
│       ├── test_arg_duplicate_ignored_1.rb
│       ├── test_arg_duplicate_proc_0.rb
│       ├── test_arg_invalid_0.rb
│       ├── test_arg_invalid_1.rb
│       ├── test_arg_invalid_2.rb
│       ├── test_arg_invalid_3.rb
│       ├── test_arg_label_0.parse-tree-whitequark.exp
│       ├── test_arg_label_0.rb
│       ├── test_arg_label_1.parse-tree-whitequark.exp
│       ├── test_arg_label_1.rb
│       ├── test_arg_label_2.parse-tree-whitequark.exp
│       ├── test_arg_label_2.rb
│       ├── test_arg_scope_0.parse-tree-whitequark.exp
│       ├── test_arg_scope_0.rb
│       ├── test_args_args_assocs_0.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_0.rb
│       ├── test_args_args_assocs_1.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_1.rb
│       ├── test_args_args_assocs_comma_0.parse-tree-whitequark.exp
│       ├── test_args_args_assocs_comma_0.rb
│       ├── test_args_args_comma_0.parse-tree-whitequark.exp
│       ├── test_args_args_comma_0.rb
│       ├── test_args_args_star_0.parse-tree-whitequark.exp
│       ├── test_args_args_star_0.rb
│       ├── test_args_args_star_1.parse-tree-whitequark.exp
│       ├── test_args_args_star_1.rb
│       ├── test_args_assocs_0.parse-tree-whitequark.exp
│       ├── test_args_assocs_0.rb
│       ├── test_args_assocs_1.parse-tree-whitequark.exp
│       ├── test_args_assocs_1.rb
│       ├── test_args_assocs_comma_0.parse-tree-whitequark.exp
│       ├── test_args_assocs_comma_0.rb
│       ├── test_args_block_pass_0.parse-tree-whitequark.exp
│       ├── test_args_block_pass_0.rb
│       ├── test_args_cmd_0.parse-tree-whitequark.exp
│       ├── test_args_cmd_0.rb
│       ├── test_args_star_0.parse-tree-whitequark.exp
│       ├── test_args_star_0.rb
│       ├── test_args_star_1.parse-tree-whitequark.exp
│       ├── test_args_star_1.rb
│       ├── test_array_assocs_0.parse-tree-whitequark.exp
│       ├── test_array_assocs_0.rb
│       ├── test_array_assocs_1.parse-tree-whitequark.exp
│       ├── test_array_assocs_1.rb
│       ├── test_array_plain_0.parse-tree-whitequark.exp
│       ├── test_array_plain_0.rb
│       ├── test_array_splat_0.parse-tree-whitequark.exp
│       ├── test_array_splat_0.rb
│       ├── test_array_splat_1.parse-tree-whitequark.exp
│       ├── test_array_splat_1.rb
│       ├── test_array_splat_2.parse-tree-whitequark.exp
│       ├── test_array_splat_2.rb
│       ├── test_array_symbols_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_0.rb
│       ├── test_array_symbols_empty_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_empty_0.rb
│       ├── test_array_symbols_empty_1.parse-tree-whitequark.exp
│       ├── test_array_symbols_empty_1.rb
│       ├── test_array_symbols_interp_0.parse-tree-whitequark.exp
│       ├── test_array_symbols_interp_0.rb
│       ├── test_array_symbols_interp_1.parse-tree-whitequark.exp
│       ├── test_array_symbols_interp_1.rb
│       ├── test_array_words_0.parse-tree-whitequark.exp
│       ├── test_array_words_0.rb
│       ├── test_array_words_empty_0.parse-tree-whitequark.exp
│       ├── test_array_words_empty_0.rb
│       ├── test_array_words_empty_1.parse-tree-whitequark.exp
│       ├── test_array_words_empty_1.rb
│       ├── test_array_words_interp_0.parse-tree-whitequark.exp
│       ├── test_array_words_interp_0.rb
│       ├── test_array_words_interp_1.parse-tree-whitequark.exp
│       ├── test_array_words_interp_1.rb
│       ├── test_asgn_backref_invalid_0.rb
│       ├── test_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_asgn_cmd_0.rb
│       ├── test_asgn_cmd_1.parse-tree-whitequark.exp
│       ├── test_asgn_cmd_1.rb
│       ├── test_asgn_keyword_invalid_0.rb
│       ├── test_asgn_keyword_invalid_1.rb
│       ├── test_asgn_keyword_invalid_2.rb
│       ├── test_asgn_keyword_invalid_3.rb
│       ├── test_asgn_keyword_invalid_4.rb
│       ├── test_asgn_keyword_invalid_5.rb
│       ├── test_asgn_mrhs_0.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_0.rb
│       ├── test_asgn_mrhs_1.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_1.rb
│       ├── test_asgn_mrhs_2.parse-tree-whitequark.exp
│       ├── test_asgn_mrhs_2.rb
│       ├── test_back_ref_0.parse-tree-whitequark.exp
│       ├── test_back_ref_0.rb
│       ├── test_bang_0.parse-tree-whitequark.exp
│       ├── test_bang_0.rb
│       ├── test_bang_cmd_0.parse-tree-whitequark.exp
│       ├── test_bang_cmd_0.rb
│       ├── test_begin_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_begin_cmdarg_0.rb
│       ├── test_beginless_range_before_27_0.rb
│       ├── test_beginless_range_before_27_1.rb
│       ├── test_block_arg_combinations_0.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_0.rb
│       ├── test_block_arg_combinations_1.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_1.rb
│       ├── test_block_arg_combinations_10.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_10.rb
│       ├── test_block_arg_combinations_11.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_11.rb
│       ├── test_block_arg_combinations_12.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_12.rb
│       ├── test_block_arg_combinations_13.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_13.rb
│       ├── test_block_arg_combinations_14.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_14.rb
│       ├── test_block_arg_combinations_15.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_15.rb
│       ├── test_block_arg_combinations_16.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_16.rb
│       ├── test_block_arg_combinations_17.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_17.rb
│       ├── test_block_arg_combinations_18.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_18.rb
│       ├── test_block_arg_combinations_19.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_19.rb
│       ├── test_block_arg_combinations_2.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_2.rb
│       ├── test_block_arg_combinations_20.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_20.rb
│       ├── test_block_arg_combinations_21.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_21.rb
│       ├── test_block_arg_combinations_22.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_22.rb
│       ├── test_block_arg_combinations_23.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_23.rb
│       ├── test_block_arg_combinations_24.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_24.rb
│       ├── test_block_arg_combinations_25.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_25.rb
│       ├── test_block_arg_combinations_26.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_26.rb
│       ├── test_block_arg_combinations_27.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_27.rb
│       ├── test_block_arg_combinations_3.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_3.rb
│       ├── test_block_arg_combinations_4.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_4.rb
│       ├── test_block_arg_combinations_5.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_5.rb
│       ├── test_block_arg_combinations_6.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_6.rb
│       ├── test_block_arg_combinations_7.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_7.rb
│       ├── test_block_arg_combinations_8.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_8.rb
│       ├── test_block_arg_combinations_9.parse-tree-whitequark.exp
│       ├── test_block_arg_combinations_9.rb
│       ├── test_block_kwarg_0.parse-tree-whitequark.exp
│       ├── test_block_kwarg_0.rb
│       ├── test_block_kwarg_combinations_0.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_0.rb
│       ├── test_block_kwarg_combinations_1.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_1.rb
│       ├── test_block_kwarg_combinations_2.parse-tree-whitequark.exp
│       ├── test_block_kwarg_combinations_2.rb
│       ├── test_blockarg_0.parse-tree-whitequark.exp
│       ├── test_blockarg_0.rb
│       ├── test_break_0.parse-tree-whitequark.exp
│       ├── test_break_0.rb
│       ├── test_break_1.parse-tree-whitequark.exp
│       ├── test_break_1.rb
│       ├── test_break_2.parse-tree-whitequark.exp
│       ├── test_break_2.rb
│       ├── test_break_3.parse-tree-whitequark.exp
│       ├── test_break_3.rb
│       ├── test_break_block_0.parse-tree-whitequark.exp
│       ├── test_break_block_0.rb
│       ├── test_bug_435_0.parse-tree-whitequark.exp
│       ├── test_bug_435_0.rb
│       ├── test_bug_447_0.parse-tree-whitequark.exp
│       ├── test_bug_447_0.rb
│       ├── test_bug_447_1.parse-tree-whitequark.exp
│       ├── test_bug_447_1.rb
│       ├── test_bug_452_0.parse-tree-whitequark.exp
│       ├── test_bug_452_0.rb
│       ├── test_bug_466_0.parse-tree-whitequark.exp
│       ├── test_bug_466_0.rb
│       ├── test_bug_473_0.parse-tree-whitequark.exp
│       ├── test_bug_473_0.rb
│       ├── test_bug_480_0.parse-tree-whitequark.exp
│       ├── test_bug_480_0.rb
│       ├── test_bug_481_0.parse-tree-whitequark.exp
│       ├── test_bug_481_0.rb
│       ├── test_bug_cmd_string_lookahead_0.parse-tree-whitequark.exp
│       ├── test_bug_cmd_string_lookahead_0.rb
│       ├── test_bug_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_0.rb
│       ├── test_bug_cmdarg_1.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_1.rb
│       ├── test_bug_cmdarg_2.parse-tree-whitequark.exp
│       ├── test_bug_cmdarg_2.rb
│       ├── test_bug_def_no_paren_eql_begin_0.parse-tree-whitequark.exp
│       ├── test_bug_def_no_paren_eql_begin_0.rb
│       ├── test_bug_do_block_in_call_args_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_call_args_0.rb
│       ├── test_bug_do_block_in_cmdarg_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_cmdarg_0.rb
│       ├── test_bug_do_block_in_hash_brace_0.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_0.rb
│       ├── test_bug_do_block_in_hash_brace_1.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_1.rb
│       ├── test_bug_do_block_in_hash_brace_2.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_2.rb
│       ├── test_bug_do_block_in_hash_brace_3.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_3.rb
│       ├── test_bug_do_block_in_hash_brace_4.parse-tree-whitequark.exp
│       ├── test_bug_do_block_in_hash_brace_4.rb
│       ├── test_bug_heredoc_do_0.parse-tree-whitequark.exp
│       ├── test_bug_heredoc_do_0.rb
│       ├── test_bug_heredoc_xstring_0.rb
│       ├── test_bug_interp_single_0.parse-tree-whitequark.exp
│       ├── test_bug_interp_single_0.rb
│       ├── test_bug_interp_single_1.parse-tree-whitequark.exp
│       ├── test_bug_interp_single_1.rb
│       ├── test_bug_lambda_leakage_0.parse-tree-whitequark.exp
│       ├── test_bug_lambda_leakage_0.rb
│       ├── test_bug_regex_verification_0.parse-tree-whitequark.exp
│       ├── test_bug_regex_verification_0.rb
│       ├── test_bug_rescue_empty_else_0.parse-tree-whitequark.exp
│       ├── test_bug_rescue_empty_else_0.rb
│       ├── test_bug_while_not_parens_do_0.parse-tree-whitequark.exp
│       ├── test_bug_while_not_parens_do_0.rb
│       ├── test_case_cond_0.parse-tree-whitequark.exp
│       ├── test_case_cond_0.rb
│       ├── test_case_cond_else_0.parse-tree-whitequark.exp
│       ├── test_case_cond_else_0.rb
│       ├── test_case_expr_0.parse-tree-whitequark.exp
│       ├── test_case_expr_0.rb
│       ├── test_case_expr_else_0.parse-tree-whitequark.exp
│       ├── test_case_expr_else_0.rb
│       ├── test_casgn_invalid_0.rb
│       ├── test_casgn_invalid_1.rb
│       ├── test_casgn_invalid_2.rb
│       ├── test_casgn_invalid_3.rb
│       ├── test_casgn_invalid_4.rb
│       ├── test_casgn_invalid_5.rb
│       ├── test_casgn_scoped_0.parse-tree-whitequark.exp
│       ├── test_casgn_scoped_0.rb
│       ├── test_casgn_toplevel_0.parse-tree-whitequark.exp
│       ├── test_casgn_toplevel_0.rb
│       ├── test_casgn_unscoped_0.parse-tree-whitequark.exp
│       ├── test_casgn_unscoped_0.rb
│       ├── test_character_0.parse-tree-whitequark.exp
│       ├── test_character_0.rb
│       ├── test_class_0.parse-tree-whitequark.exp
│       ├── test_class_0.rb
│       ├── test_class_1.parse-tree-whitequark.exp
│       ├── test_class_1.rb
│       ├── test_class_definition_in_while_cond_0.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_0.rb
│       ├── test_class_definition_in_while_cond_1.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_1.rb
│       ├── test_class_definition_in_while_cond_2.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_2.rb
│       ├── test_class_definition_in_while_cond_3.parse-tree-whitequark.exp
│       ├── test_class_definition_in_while_cond_3.rb
│       ├── test_class_invalid_0.rb
│       ├── test_class_invalid_1.rb
│       ├── test_class_super_0.parse-tree-whitequark.exp
│       ├── test_class_super_0.rb
│       ├── test_class_super_label_0.parse-tree-whitequark.exp
│       ├── test_class_super_label_0.rb
│       ├── test_codepoint_too_large_0.rb
│       ├── test_complex_0.parse-tree-whitequark.exp
│       ├── test_complex_0.rb
│       ├── test_complex_1.parse-tree-whitequark.exp
│       ├── test_complex_1.rb
│       ├── test_complex_2.parse-tree-whitequark.exp
│       ├── test_complex_2.rb
│       ├── test_complex_3.parse-tree-whitequark.exp
│       ├── test_complex_3.rb
│       ├── test_cond_begin_0.parse-tree-whitequark.exp
│       ├── test_cond_begin_0.rb
│       ├── test_cond_eflipflop_0.parse-tree-whitequark.exp
│       ├── test_cond_eflipflop_0.rb
│       ├── test_cond_eflipflop_1.parse-tree-whitequark.exp
│       ├── test_cond_eflipflop_1.rb
│       ├── test_cond_iflipflop_0.parse-tree-whitequark.exp
│       ├── test_cond_iflipflop_0.rb
│       ├── test_cond_iflipflop_1.parse-tree-whitequark.exp
│       ├── test_cond_iflipflop_1.rb
│       ├── test_cond_match_current_line_0.parse-tree-whitequark.exp
│       ├── test_cond_match_current_line_0.rb
│       ├── test_cond_match_current_line_1.parse-tree-whitequark.exp
│       ├── test_cond_match_current_line_1.rb
│       ├── test_const_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_0.rb
│       ├── test_const_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_1.rb
│       ├── test_const_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_2.rb
│       ├── test_const_op_asgn_3.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_3.rb
│       ├── test_const_op_asgn_4.parse-tree-whitequark.exp
│       ├── test_const_op_asgn_4.rb
│       ├── test_const_scoped_0.parse-tree-whitequark.exp
│       ├── test_const_scoped_0.rb
│       ├── test_const_toplevel_0.parse-tree-whitequark.exp
│       ├── test_const_toplevel_0.rb
│       ├── test_const_unscoped_0.parse-tree-whitequark.exp
│       ├── test_const_unscoped_0.rb
│       ├── test_cpath_0.parse-tree-whitequark.exp
│       ├── test_cpath_0.rb
│       ├── test_cpath_1.parse-tree-whitequark.exp
│       ├── test_cpath_1.rb
│       ├── test_cpath_invalid_0.rb
│       ├── test_cvar_0.parse-tree-whitequark.exp
│       ├── test_cvar_0.rb
│       ├── test_cvasgn_0.parse-tree-whitequark.exp
│       ├── test_cvasgn_0.rb
│       ├── test_def_0.parse-tree-whitequark.exp
│       ├── test_def_0.rb
│       ├── test_def_1.parse-tree-whitequark.exp
│       ├── test_def_1.rb
│       ├── test_def_2.parse-tree-whitequark.exp
│       ├── test_def_2.rb
│       ├── test_def_3.parse-tree-whitequark.exp
│       ├── test_def_3.rb
│       ├── test_def_4.parse-tree-whitequark.exp
│       ├── test_def_4.rb
│       ├── test_def_5.parse-tree-whitequark.exp
│       ├── test_def_5.rb
│       ├── test_defined_0.parse-tree-whitequark.exp
│       ├── test_defined_0.rb
│       ├── test_defined_1.parse-tree-whitequark.exp
│       ├── test_defined_1.rb
│       ├── test_defined_2.parse-tree-whitequark.exp
│       ├── test_defined_2.rb
│       ├── test_defs_0.parse-tree-whitequark.exp
│       ├── test_defs_0.rb
│       ├── test_defs_1.parse-tree-whitequark.exp
│       ├── test_defs_1.rb
│       ├── test_defs_2.parse-tree-whitequark.exp
│       ├── test_defs_2.rb
│       ├── test_defs_3.parse-tree-whitequark.exp
│       ├── test_defs_3.rb
│       ├── test_defs_4.parse-tree-whitequark.exp
│       ├── test_defs_4.rb
│       ├── test_defs_invalid_0.rb
│       ├── test_defs_invalid_1.rb
│       ├── test_defs_invalid_2.rb
│       ├── test_defs_invalid_3.rb
│       ├── test_defs_invalid_4.rb
│       ├── test_defs_invalid_5.rb
│       ├── test_defs_invalid_6.rb
│       ├── test_defs_invalid_7.rb
│       ├── test_emit_arg_inside_procarg0_legacy_0.parse-tree-whitequark.exp
│       ├── test_emit_arg_inside_procarg0_legacy_0.rb
│       ├── test_ensure_0.parse-tree-whitequark.exp
│       ├── test_ensure_0.rb
│       ├── test_ensure_empty_0.parse-tree-whitequark.exp
│       ├── test_ensure_empty_0.rb
│       ├── test_false_0.parse-tree-whitequark.exp
│       ├── test_false_0.rb
│       ├── test_float_0.parse-tree-whitequark.exp
│       ├── test_float_0.rb
│       ├── test_float_1.parse-tree-whitequark.exp
│       ├── test_float_1.rb
│       ├── test_for_0.parse-tree-whitequark.exp
│       ├── test_for_0.rb
│       ├── test_for_1.parse-tree-whitequark.exp
│       ├── test_for_1.rb
│       ├── test_for_mlhs_0.parse-tree-whitequark.exp
│       ├── test_for_mlhs_0.rb
│       ├── test_gvar_0.parse-tree-whitequark.exp
│       ├── test_gvar_0.rb
│       ├── test_gvasgn_0.parse-tree-whitequark.exp
│       ├── test_gvasgn_0.rb
│       ├── test_hash_empty_0.parse-tree-whitequark.exp
│       ├── test_hash_empty_0.rb
│       ├── test_hash_hashrocket_0.parse-tree-whitequark.exp
│       ├── test_hash_hashrocket_0.rb
│       ├── test_hash_hashrocket_1.parse-tree-whitequark.exp
│       ├── test_hash_hashrocket_1.rb
│       ├── test_hash_kwsplat_0.parse-tree-whitequark.exp
│       ├── test_hash_kwsplat_0.rb
│       ├── test_hash_label_0.parse-tree-whitequark.exp
│       ├── test_hash_label_0.rb
│       ├── test_hash_label_end_0.parse-tree-whitequark.exp
│       ├── test_hash_label_end_0.rb
│       ├── test_hash_label_end_1.parse-tree-whitequark.exp
│       ├── test_hash_label_end_1.rb
│       ├── test_hash_label_end_2.parse-tree-whitequark.exp
│       ├── test_hash_label_end_2.rb
│       ├── test_if_0.parse-tree-whitequark.exp
│       ├── test_if_0.rb
│       ├── test_if_1.parse-tree-whitequark.exp
│       ├── test_if_1.rb
│       ├── test_if_else_0.parse-tree-whitequark.exp
│       ├── test_if_else_0.rb
│       ├── test_if_else_1.parse-tree-whitequark.exp
│       ├── test_if_else_1.rb
│       ├── test_if_elsif_0.parse-tree-whitequark.exp
│       ├── test_if_elsif_0.rb
│       ├── test_if_masgn_24_0.parse-tree-whitequark.exp
│       ├── test_if_masgn_24_0.rb
│       ├── test_if_mod_0.parse-tree-whitequark.exp
│       ├── test_if_mod_0.rb
│       ├── test_if_nl_then_0.parse-tree-whitequark.exp
│       ├── test_if_nl_then_0.rb
│       ├── test_int_0.parse-tree-whitequark.exp
│       ├── test_int_0.rb
│       ├── test_int_1.parse-tree-whitequark.exp
│       ├── test_int_1.rb
│       ├── test_int_2.parse-tree-whitequark.exp
│       ├── test_int_2.rb
│       ├── test_int_LINE_0.parse-tree-whitequark.exp
│       ├── test_int_LINE_0.rb
│       ├── test_ivar_0.parse-tree-whitequark.exp
│       ├── test_ivar_0.rb
│       ├── test_ivasgn_0.parse-tree-whitequark.exp
│       ├── test_ivasgn_0.rb
│       ├── test_kwarg_0.parse-tree-whitequark.exp
│       ├── test_kwarg_0.rb
│       ├── test_kwarg_combinations_0.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_0.rb
│       ├── test_kwarg_combinations_1.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_1.rb
│       ├── test_kwarg_combinations_2.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_2.rb
│       ├── test_kwarg_combinations_3.parse-tree-whitequark.exp
│       ├── test_kwarg_combinations_3.rb
│       ├── test_kwarg_invalid_0.rb
│       ├── test_kwarg_invalid_1.rb
│       ├── test_kwarg_no_paren_0.parse-tree-whitequark.exp
│       ├── test_kwarg_no_paren_0.rb
│       ├── test_kwarg_no_paren_1.parse-tree-whitequark.exp
│       ├── test_kwarg_no_paren_1.rb
│       ├── test_kwbegin_compstmt_0.parse-tree-whitequark.exp
│       ├── test_kwbegin_compstmt_0.rb
│       ├── test_kwoptarg_0.parse-tree-whitequark.exp
│       ├── test_kwoptarg_0.rb
│       ├── test_kwrestarg_named_0.parse-tree-whitequark.exp
│       ├── test_kwrestarg_named_0.rb
│       ├── test_kwrestarg_unnamed_0.parse-tree-whitequark.exp
│       ├── test_kwrestarg_unnamed_0.rb
│       ├── test_lbrace_arg_after_command_args_0.parse-tree-whitequark.exp
│       ├── test_lbrace_arg_after_command_args_0.rb
│       ├── test_log_asgn_invalid_0.rb
│       ├── test_log_asgn_invalid_1.rb
│       ├── test_lparenarg_after_lvar_since_25_0.parse-tree-whitequark.exp
│       ├── test_lparenarg_after_lvar_since_25_0.rb
│       ├── test_lparenarg_after_lvar_since_25_1.parse-tree-whitequark.exp
│       ├── test_lparenarg_after_lvar_since_25_1.rb
│       ├── test_lvar_0.parse-tree-whitequark.exp
│       ├── test_lvar_0.rb
│       ├── test_lvasgn_0.parse-tree-whitequark.exp
│       ├── test_lvasgn_0.rb
│       ├── test_marg_combinations_0.parse-tree-whitequark.exp
│       ├── test_marg_combinations_0.rb
│       ├── test_marg_combinations_1.parse-tree-whitequark.exp
│       ├── test_marg_combinations_1.rb
│       ├── test_marg_combinations_2.parse-tree-whitequark.exp
│       ├── test_marg_combinations_2.rb
│       ├── test_marg_combinations_3.parse-tree-whitequark.exp
│       ├── test_marg_combinations_3.rb
│       ├── test_marg_combinations_4.parse-tree-whitequark.exp
│       ├── test_marg_combinations_4.rb
│       ├── test_marg_combinations_5.parse-tree-whitequark.exp
│       ├── test_marg_combinations_5.rb
│       ├── test_marg_combinations_6.parse-tree-whitequark.exp
│       ├── test_marg_combinations_6.rb
│       ├── test_marg_combinations_7.parse-tree-whitequark.exp
│       ├── test_marg_combinations_7.rb
│       ├── test_marg_combinations_8.parse-tree-whitequark.exp
│       ├── test_marg_combinations_8.rb
│       ├── test_marg_combinations_9.parse-tree-whitequark.exp
│       ├── test_marg_combinations_9.rb
│       ├── test_masgn_0.parse-tree-whitequark.exp
│       ├── test_masgn_0.rb
│       ├── test_masgn_1.parse-tree-whitequark.exp
│       ├── test_masgn_1.rb
│       ├── test_masgn_2.parse-tree-whitequark.exp
│       ├── test_masgn_2.rb
│       ├── test_masgn_attr_0.parse-tree-whitequark.exp
│       ├── test_masgn_attr_0.rb
│       ├── test_masgn_attr_1.parse-tree-whitequark.exp
│       ├── test_masgn_attr_1.rb
│       ├── test_masgn_attr_2.parse-tree-whitequark.exp
│       ├── test_masgn_attr_2.rb
│       ├── test_masgn_backref_invalid_0.rb
│       ├── test_masgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_masgn_cmd_0.rb
│       ├── test_masgn_const_0.parse-tree-whitequark.exp
│       ├── test_masgn_const_0.rb
│       ├── test_masgn_const_1.parse-tree-whitequark.exp
│       ├── test_masgn_const_1.rb
│       ├── test_masgn_const_invalid_0.rb
│       ├── test_masgn_const_invalid_1.rb
│       ├── test_masgn_keyword_invalid_0.rb
│       ├── test_masgn_nested_0.parse-tree-whitequark.exp
│       ├── test_masgn_nested_0.rb
│       ├── test_masgn_nested_1.parse-tree-whitequark.exp
│       ├── test_masgn_nested_1.rb
│       ├── test_masgn_splat_0.parse-tree-whitequark.exp
│       ├── test_masgn_splat_0.rb
│       ├── test_masgn_splat_1.parse-tree-whitequark.exp
│       ├── test_masgn_splat_1.rb
│       ├── test_masgn_splat_2.parse-tree-whitequark.exp
│       ├── test_masgn_splat_2.rb
│       ├── test_masgn_splat_3.parse-tree-whitequark.exp
│       ├── test_masgn_splat_3.rb
│       ├── test_masgn_splat_4.parse-tree-whitequark.exp
│       ├── test_masgn_splat_4.rb
│       ├── test_masgn_splat_5.parse-tree-whitequark.exp
│       ├── test_masgn_splat_5.rb
│       ├── test_masgn_splat_6.parse-tree-whitequark.exp
│       ├── test_masgn_splat_6.rb
│       ├── test_masgn_splat_7.parse-tree-whitequark.exp
│       ├── test_masgn_splat_7.rb
│       ├── test_masgn_splat_8.parse-tree-whitequark.exp
│       ├── test_masgn_splat_8.rb
│       ├── test_masgn_splat_9.parse-tree-whitequark.exp
│       ├── test_masgn_splat_9.rb
│       ├── test_method_definition_in_while_cond_0.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_0.rb
│       ├── test_method_definition_in_while_cond_1.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_1.rb
│       ├── test_method_definition_in_while_cond_2.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_2.rb
│       ├── test_method_definition_in_while_cond_3.parse-tree-whitequark.exp
│       ├── test_method_definition_in_while_cond_3.rb
│       ├── test_module_0.parse-tree-whitequark.exp
│       ├── test_module_0.rb
│       ├── test_module_invalid_0.rb
│       ├── test_multiple_args_with_trailing_comma_0.parse-tree-whitequark.exp
│       ├── test_multiple_args_with_trailing_comma_0.rb
│       ├── test_next_0.parse-tree-whitequark.exp
│       ├── test_next_0.rb
│       ├── test_next_1.parse-tree-whitequark.exp
│       ├── test_next_1.rb
│       ├── test_next_2.parse-tree-whitequark.exp
│       ├── test_next_2.rb
│       ├── test_next_3.parse-tree-whitequark.exp
│       ├── test_next_3.rb
│       ├── test_next_block_0.parse-tree-whitequark.exp
│       ├── test_next_block_0.rb
│       ├── test_nil_0.parse-tree-whitequark.exp
│       ├── test_nil_0.rb
│       ├── test_nil_expression_0.parse-tree-whitequark.exp
│       ├── test_nil_expression_0.rb
│       ├── test_nil_expression_1.parse-tree-whitequark.exp
│       ├── test_nil_expression_1.rb
│       ├── test_non_lvar_injecting_match_0.parse-tree-whitequark.exp
│       ├── test_non_lvar_injecting_match_0.rb
│       ├── test_not_0.parse-tree-whitequark.exp
│       ├── test_not_0.rb
│       ├── test_not_1.parse-tree-whitequark.exp
│       ├── test_not_1.rb
│       ├── test_not_2.parse-tree-whitequark.exp
│       ├── test_not_2.rb
│       ├── test_not_cmd_0.parse-tree-whitequark.exp
│       ├── test_not_cmd_0.rb
│       ├── test_not_masgn_24_0.parse-tree-whitequark.exp
│       ├── test_not_masgn_24_0.rb
│       ├── test_nth_ref_0.parse-tree-whitequark.exp
│       ├── test_nth_ref_0.rb
│       ├── test_on_error_0.rb
│       ├── test_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_0.rb
│       ├── test_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_op_asgn_1.rb
│       ├── test_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_op_asgn_2.rb
│       ├── test_op_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_0.rb
│       ├── test_op_asgn_cmd_1.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_1.rb
│       ├── test_op_asgn_cmd_2.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_2.rb
│       ├── test_op_asgn_cmd_3.parse-tree-whitequark.exp
│       ├── test_op_asgn_cmd_3.rb
│       ├── test_op_asgn_index_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_index_0.rb
│       ├── test_op_asgn_index_cmd_0.parse-tree-whitequark.exp
│       ├── test_op_asgn_index_cmd_0.rb
│       ├── test_op_asgn_invalid_0.rb
│       ├── test_op_asgn_invalid_1.rb
│       ├── test_op_asgn_invalid_2.rb
│       ├── test_optarg_0.parse-tree-whitequark.exp
│       ├── test_optarg_0.rb
│       ├── test_optarg_1.parse-tree-whitequark.exp
│       ├── test_optarg_1.rb
│       ├── test_or_0.parse-tree-whitequark.exp
│       ├── test_or_0.rb
│       ├── test_or_1.parse-tree-whitequark.exp
│       ├── test_or_1.rb
│       ├── test_or_asgn_0.parse-tree-whitequark.exp
│       ├── test_or_asgn_0.rb
│       ├── test_or_asgn_1.parse-tree-whitequark.exp
│       ├── test_or_asgn_1.rb
│       ├── test_parser_bug_272_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_272_0.rb
│       ├── test_parser_bug_490_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_0.rb
│       ├── test_parser_bug_490_1.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_1.rb
│       ├── test_parser_bug_490_2.parse-tree-whitequark.exp
│       ├── test_parser_bug_490_2.rb
│       ├── test_parser_bug_507_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_507_0.rb
│       ├── test_parser_bug_518_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_518_0.rb
│       ├── test_parser_bug_525_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_525_0.rb
│       ├── test_parser_bug_604_0.parse-tree-whitequark.exp
│       ├── test_parser_bug_604_0.rb
│       ├── test_postexe_0.parse-tree-whitequark.exp
│       ├── test_postexe_0.rb
│       ├── test_preexe_0.parse-tree-whitequark.exp
│       ├── test_preexe_0.rb
│       ├── test_preexe_invalid_0.rb
│       ├── test_procarg0_0.parse-tree-whitequark.exp
│       ├── test_procarg0_0.rb
│       ├── test_procarg0_1.parse-tree-whitequark.exp
│       ├── test_procarg0_1.rb
│       ├── test_procarg0_legacy_0.parse-tree-whitequark.exp
│       ├── test_procarg0_legacy_0.rb
│       ├── test_range_endless_0.parse-tree-whitequark.exp
│       ├── test_range_endless_0.rb
│       ├── test_range_endless_1.parse-tree-whitequark.exp
│       ├── test_range_endless_1.rb
│       ├── test_range_exclusive_0.parse-tree-whitequark.exp
│       ├── test_range_exclusive_0.rb
│       ├── test_range_inclusive_0.parse-tree-whitequark.exp
│       ├── test_range_inclusive_0.rb
│       ├── test_rational_0.parse-tree-whitequark.exp
│       ├── test_rational_0.rb
│       ├── test_rational_1.parse-tree-whitequark.exp
│       ├── test_rational_1.rb
│       ├── test_redo_0.parse-tree-whitequark.exp
│       ├── test_redo_0.rb
│       ├── test_regex_interp_0.parse-tree-whitequark.exp
│       ├── test_regex_interp_0.rb
│       ├── test_regex_plain_0.parse-tree-whitequark.exp
│       ├── test_regex_plain_0.rb
│       ├── test_resbody_list_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_0.rb
│       ├── test_resbody_list_mrhs_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_mrhs_0.rb
│       ├── test_resbody_list_var_0.parse-tree-whitequark.exp
│       ├── test_resbody_list_var_0.rb
│       ├── test_resbody_var_0.parse-tree-whitequark.exp
│       ├── test_resbody_var_0.rb
│       ├── test_resbody_var_1.parse-tree-whitequark.exp
│       ├── test_resbody_var_1.rb
│       ├── test_rescue_0.parse-tree-whitequark.exp
│       ├── test_rescue_0.rb
│       ├── test_rescue_else_0.parse-tree-whitequark.exp
│       ├── test_rescue_else_0.rb
│       ├── test_rescue_else_ensure_0.parse-tree-whitequark.exp
│       ├── test_rescue_else_ensure_0.rb
│       ├── test_rescue_else_useless_0.rb
│       ├── test_rescue_else_useless_1.rb
│       ├── test_rescue_ensure_0.parse-tree-whitequark.exp
│       ├── test_rescue_ensure_0.rb
│       ├── test_rescue_in_lambda_block_0.parse-tree-whitequark.exp
│       ├── test_rescue_in_lambda_block_0.rb
│       ├── test_rescue_in_lambda_block_1.rb
│       ├── test_rescue_mod_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_0.rb
│       ├── test_rescue_mod_asgn_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_asgn_0.rb
│       ├── test_rescue_mod_op_assign_0.parse-tree-whitequark.exp
│       ├── test_rescue_mod_op_assign_0.rb
│       ├── test_rescue_without_begin_end_0.parse-tree-whitequark.exp
│       ├── test_rescue_without_begin_end_0.rb
│       ├── test_restarg_named_0.parse-tree-whitequark.exp
│       ├── test_restarg_named_0.rb
│       ├── test_restarg_unnamed_0.parse-tree-whitequark.exp
│       ├── test_restarg_unnamed_0.rb
│       ├── test_retry_0.parse-tree-whitequark.exp
│       ├── test_retry_0.rb
│       ├── test_return_0.parse-tree-whitequark.exp
│       ├── test_return_0.rb
│       ├── test_return_1.parse-tree-whitequark.exp
│       ├── test_return_1.rb
│       ├── test_return_2.parse-tree-whitequark.exp
│       ├── test_return_2.rb
│       ├── test_return_3.parse-tree-whitequark.exp
│       ├── test_return_3.rb
│       ├── test_return_block_0.parse-tree-whitequark.exp
│       ├── test_return_block_0.rb
│       ├── test_return_in_class_0.rb
│       ├── test_ruby_bug_10279_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10279_0.rb
│       ├── test_ruby_bug_10653_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_0.rb
│       ├── test_ruby_bug_10653_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_1.rb
│       ├── test_ruby_bug_10653_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_10653_2.rb
│       ├── test_ruby_bug_11107_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11107_0.rb
│       ├── test_ruby_bug_11380_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11380_0.rb
│       ├── test_ruby_bug_11873_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_0.rb
│       ├── test_ruby_bug_11873_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_1.rb
│       ├── test_ruby_bug_11873_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_10.rb
│       ├── test_ruby_bug_11873_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_11.rb
│       ├── test_ruby_bug_11873_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_2.rb
│       ├── test_ruby_bug_11873_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_3.rb
│       ├── test_ruby_bug_11873_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_4.rb
│       ├── test_ruby_bug_11873_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_5.rb
│       ├── test_ruby_bug_11873_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_6.rb
│       ├── test_ruby_bug_11873_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_7.rb
│       ├── test_ruby_bug_11873_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_8.rb
│       ├── test_ruby_bug_11873_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_9.rb
│       ├── test_ruby_bug_11873_a_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_0.rb
│       ├── test_ruby_bug_11873_a_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_1.rb
│       ├── test_ruby_bug_11873_a_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_10.rb
│       ├── test_ruby_bug_11873_a_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_11.rb
│       ├── test_ruby_bug_11873_a_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_12.rb
│       ├── test_ruby_bug_11873_a_13.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_13.rb
│       ├── test_ruby_bug_11873_a_14.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_14.rb
│       ├── test_ruby_bug_11873_a_15.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_15.rb
│       ├── test_ruby_bug_11873_a_16.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_16.rb
│       ├── test_ruby_bug_11873_a_17.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_17.rb
│       ├── test_ruby_bug_11873_a_18.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_18.rb
│       ├── test_ruby_bug_11873_a_19.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_19.rb
│       ├── test_ruby_bug_11873_a_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_2.rb
│       ├── test_ruby_bug_11873_a_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_3.rb
│       ├── test_ruby_bug_11873_a_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_4.rb
│       ├── test_ruby_bug_11873_a_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_5.rb
│       ├── test_ruby_bug_11873_a_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_6.rb
│       ├── test_ruby_bug_11873_a_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_7.rb
│       ├── test_ruby_bug_11873_a_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_8.rb
│       ├── test_ruby_bug_11873_a_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_a_9.rb
│       ├── test_ruby_bug_11873_b_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_11873_b_0.rb
│       ├── test_ruby_bug_12073_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12073_0.rb
│       ├── test_ruby_bug_12073_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12073_1.rb
│       ├── test_ruby_bug_12402_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_0.rb
│       ├── test_ruby_bug_12402_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_1.rb
│       ├── test_ruby_bug_12402_10.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_10.rb
│       ├── test_ruby_bug_12402_11.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_11.rb
│       ├── test_ruby_bug_12402_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_12.rb
│       ├── test_ruby_bug_12402_13.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_13.rb
│       ├── test_ruby_bug_12402_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_2.rb
│       ├── test_ruby_bug_12402_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_3.rb
│       ├── test_ruby_bug_12402_4.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_4.rb
│       ├── test_ruby_bug_12402_5.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_5.rb
│       ├── test_ruby_bug_12402_6.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_6.rb
│       ├── test_ruby_bug_12402_7.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_7.rb
│       ├── test_ruby_bug_12402_8.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_8.rb
│       ├── test_ruby_bug_12402_9.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12402_9.rb
│       ├── test_ruby_bug_12669_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_0.rb
│       ├── test_ruby_bug_12669_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_1.rb
│       ├── test_ruby_bug_12669_2.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_2.rb
│       ├── test_ruby_bug_12669_3.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12669_3.rb
│       ├── test_ruby_bug_12686_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_12686_0.rb
│       ├── test_ruby_bug_12686_1.rb
│       ├── test_ruby_bug_13547_0.rb
│       ├── test_ruby_bug_13547_1.rb
│       ├── test_ruby_bug_13547_10.rb
│       ├── test_ruby_bug_13547_11.rb
│       ├── test_ruby_bug_13547_12.parse-tree-whitequark.exp
│       ├── test_ruby_bug_13547_12.rb
│       ├── test_ruby_bug_13547_2.rb
│       ├── test_ruby_bug_13547_3.rb
│       ├── test_ruby_bug_13547_4.rb
│       ├── test_ruby_bug_13547_5.rb
│       ├── test_ruby_bug_13547_6.rb
│       ├── test_ruby_bug_13547_7.rb
│       ├── test_ruby_bug_13547_8.rb
│       ├── test_ruby_bug_13547_9.rb
│       ├── test_ruby_bug_14690_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_14690_0.rb
│       ├── test_ruby_bug_9669_0.parse-tree-whitequark.exp
│       ├── test_ruby_bug_9669_0.rb
│       ├── test_ruby_bug_9669_1.parse-tree-whitequark.exp
│       ├── test_ruby_bug_9669_1.rb
│       ├── test_sclass_0.parse-tree-whitequark.exp
│       ├── test_sclass_0.rb
│       ├── test_self_0.parse-tree-whitequark.exp
│       ├── test_self_0.rb
│       ├── test_send_attr_asgn_0.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_0.rb
│       ├── test_send_attr_asgn_1.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_1.rb
│       ├── test_send_attr_asgn_2.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_2.rb
│       ├── test_send_attr_asgn_3.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_3.rb
│       ├── test_send_attr_asgn_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_attr_asgn_conditional_0.rb
│       ├── test_send_binary_op_0.parse-tree-whitequark.exp
│       ├── test_send_binary_op_0.rb
│       ├── test_send_binary_op_1.parse-tree-whitequark.exp
│       ├── test_send_binary_op_1.rb
│       ├── test_send_binary_op_10.parse-tree-whitequark.exp
│       ├── test_send_binary_op_10.rb
│       ├── test_send_binary_op_11.parse-tree-whitequark.exp
│       ├── test_send_binary_op_11.rb
│       ├── test_send_binary_op_12.parse-tree-whitequark.exp
│       ├── test_send_binary_op_12.rb
│       ├── test_send_binary_op_13.parse-tree-whitequark.exp
│       ├── test_send_binary_op_13.rb
│       ├── test_send_binary_op_14.parse-tree-whitequark.exp
│       ├── test_send_binary_op_14.rb
│       ├── test_send_binary_op_15.parse-tree-whitequark.exp
│       ├── test_send_binary_op_15.rb
│       ├── test_send_binary_op_16.parse-tree-whitequark.exp
│       ├── test_send_binary_op_16.rb
│       ├── test_send_binary_op_17.parse-tree-whitequark.exp
│       ├── test_send_binary_op_17.rb
│       ├── test_send_binary_op_18.parse-tree-whitequark.exp
│       ├── test_send_binary_op_18.rb
│       ├── test_send_binary_op_19.parse-tree-whitequark.exp
│       ├── test_send_binary_op_19.rb
│       ├── test_send_binary_op_2.parse-tree-whitequark.exp
│       ├── test_send_binary_op_2.rb
│       ├── test_send_binary_op_20.parse-tree-whitequark.exp
│       ├── test_send_binary_op_20.rb
│       ├── test_send_binary_op_3.parse-tree-whitequark.exp
│       ├── test_send_binary_op_3.rb
│       ├── test_send_binary_op_4.parse-tree-whitequark.exp
│       ├── test_send_binary_op_4.rb
│       ├── test_send_binary_op_5.parse-tree-whitequark.exp
│       ├── test_send_binary_op_5.rb
│       ├── test_send_binary_op_6.parse-tree-whitequark.exp
│       ├── test_send_binary_op_6.rb
│       ├── test_send_binary_op_7.parse-tree-whitequark.exp
│       ├── test_send_binary_op_7.rb
│       ├── test_send_binary_op_8.parse-tree-whitequark.exp
│       ├── test_send_binary_op_8.rb
│       ├── test_send_binary_op_9.parse-tree-whitequark.exp
│       ├── test_send_binary_op_9.rb
│       ├── test_send_block_blockarg_0.rb
│       ├── test_send_block_chain_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_0.rb
│       ├── test_send_block_chain_cmd_1.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_1.rb
│       ├── test_send_block_chain_cmd_2.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_2.rb
│       ├── test_send_block_chain_cmd_3.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_3.rb
│       ├── test_send_block_chain_cmd_4.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_4.rb
│       ├── test_send_block_chain_cmd_5.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_5.rb
│       ├── test_send_block_chain_cmd_6.parse-tree-whitequark.exp
│       ├── test_send_block_chain_cmd_6.rb
│       ├── test_send_block_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_block_conditional_0.rb
│       ├── test_send_call_0.parse-tree-whitequark.exp
│       ├── test_send_call_0.rb
│       ├── test_send_call_1.parse-tree-whitequark.exp
│       ├── test_send_call_1.rb
│       ├── test_send_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_conditional_0.rb
│       ├── test_send_index_0.parse-tree-whitequark.exp
│       ├── test_send_index_0.rb
│       ├── test_send_index_asgn_0.parse-tree-whitequark.exp
│       ├── test_send_index_asgn_0.rb
│       ├── test_send_index_asgn_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_index_asgn_legacy_0.rb
│       ├── test_send_index_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_index_cmd_0.rb
│       ├── test_send_index_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_index_legacy_0.rb
│       ├── test_send_lambda_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_0.rb
│       ├── test_send_lambda_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_1.rb
│       ├── test_send_lambda_2.parse-tree-whitequark.exp
│       ├── test_send_lambda_2.rb
│       ├── test_send_lambda_args_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_0.rb
│       ├── test_send_lambda_args_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_1.rb
│       ├── test_send_lambda_args_noparen_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_noparen_0.rb
│       ├── test_send_lambda_args_noparen_1.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_noparen_1.rb
│       ├── test_send_lambda_args_shadow_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_args_shadow_0.rb
│       ├── test_send_lambda_legacy_0.parse-tree-whitequark.exp
│       ├── test_send_lambda_legacy_0.rb
│       ├── test_send_op_asgn_conditional_0.parse-tree-whitequark.exp
│       ├── test_send_op_asgn_conditional_0.rb
│       ├── test_send_plain_0.parse-tree-whitequark.exp
│       ├── test_send_plain_0.rb
│       ├── test_send_plain_1.parse-tree-whitequark.exp
│       ├── test_send_plain_1.rb
│       ├── test_send_plain_2.parse-tree-whitequark.exp
│       ├── test_send_plain_2.rb
│       ├── test_send_plain_cmd_0.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_0.rb
│       ├── test_send_plain_cmd_1.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_1.rb
│       ├── test_send_plain_cmd_2.parse-tree-whitequark.exp
│       ├── test_send_plain_cmd_2.rb
│       ├── test_send_self_0.parse-tree-whitequark.exp
│       ├── test_send_self_0.rb
│       ├── test_send_self_1.parse-tree-whitequark.exp
│       ├── test_send_self_1.rb
│       ├── test_send_self_2.parse-tree-whitequark.exp
│       ├── test_send_self_2.rb
│       ├── test_send_self_block_0.parse-tree-whitequark.exp
│       ├── test_send_self_block_0.rb
│       ├── test_send_self_block_1.parse-tree-whitequark.exp
│       ├── test_send_self_block_1.rb
│       ├── test_send_self_block_2.parse-tree-whitequark.exp
│       ├── test_send_self_block_2.rb
│       ├── test_send_self_block_3.parse-tree-whitequark.exp
│       ├── test_send_self_block_3.rb
│       ├── test_send_unary_op_0.parse-tree-whitequark.exp
│       ├── test_send_unary_op_0.rb
│       ├── test_send_unary_op_1.parse-tree-whitequark.exp
│       ├── test_send_unary_op_1.rb
│       ├── test_send_unary_op_2.parse-tree-whitequark.exp
│       ├── test_send_unary_op_2.rb
│       ├── test_space_args_arg_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_0.rb
│       ├── test_space_args_arg_block_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_0.rb
│       ├── test_space_args_arg_block_1.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_1.rb
│       ├── test_space_args_arg_block_2.parse-tree-whitequark.exp
│       ├── test_space_args_arg_block_2.rb
│       ├── test_space_args_arg_call_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_call_0.rb
│       ├── test_space_args_arg_newline_0.parse-tree-whitequark.exp
│       ├── test_space_args_arg_newline_0.rb
│       ├── test_space_args_block_0.parse-tree-whitequark.exp
│       ├── test_space_args_block_0.rb
│       ├── test_space_args_cmd_0.parse-tree-whitequark.exp
│       ├── test_space_args_cmd_0.rb
│       ├── test_string_FILE_0.parse-tree-whitequark.exp
│       ├── test_string_FILE_0.rb
│       ├── test_string_concat_0.parse-tree-whitequark.exp
│       ├── test_string_concat_0.rb
│       ├── test_string_dvar_0.parse-tree-whitequark.exp
│       ├── test_string_dvar_0.rb
│       ├── test_string_interp_0.parse-tree-whitequark.exp
│       ├── test_string_interp_0.rb
│       ├── test_string_plain_0.parse-tree-whitequark.exp
│       ├── test_string_plain_0.rb
│       ├── test_string_plain_1.parse-tree-whitequark.exp
│       ├── test_string_plain_1.rb
│       ├── test_super_0.parse-tree-whitequark.exp
│       ├── test_super_0.rb
│       ├── test_super_1.parse-tree-whitequark.exp
│       ├── test_super_1.rb
│       ├── test_super_2.parse-tree-whitequark.exp
│       ├── test_super_2.rb
│       ├── test_super_block_0.parse-tree-whitequark.exp
│       ├── test_super_block_0.rb
│       ├── test_super_block_1.parse-tree-whitequark.exp
│       ├── test_super_block_1.rb
│       ├── test_symbol_interp_0.parse-tree-whitequark.exp
│       ├── test_symbol_interp_0.rb
│       ├── test_symbol_plain_0.parse-tree-whitequark.exp
│       ├── test_symbol_plain_0.rb
│       ├── test_symbol_plain_1.parse-tree-whitequark.exp
│       ├── test_symbol_plain_1.rb
│       ├── test_ternary_0.parse-tree-whitequark.exp
│       ├── test_ternary_0.rb
│       ├── test_ternary_ambiguous_symbol_0.parse-tree-whitequark.exp
│       ├── test_ternary_ambiguous_symbol_0.rb
│       ├── test_true_0.parse-tree-whitequark.exp
│       ├── test_true_0.rb
│       ├── test_unary_num_pow_precedence_0.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_0.rb
│       ├── test_unary_num_pow_precedence_1.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_1.rb
│       ├── test_unary_num_pow_precedence_2.parse-tree-whitequark.exp
│       ├── test_unary_num_pow_precedence_2.rb
│       ├── test_undef_0.parse-tree-whitequark.exp
│       ├── test_undef_0.rb
│       ├── test_unknown_percent_str_0.rb
│       ├── test_unless_0.parse-tree-whitequark.exp
│       ├── test_unless_0.rb
│       ├── test_unless_1.parse-tree-whitequark.exp
│       ├── test_unless_1.rb
│       ├── test_unless_else_0.parse-tree-whitequark.exp
│       ├── test_unless_else_0.rb
│       ├── test_unless_else_1.parse-tree-whitequark.exp
│       ├── test_unless_else_1.rb
│       ├── test_unless_mod_0.parse-tree-whitequark.exp
│       ├── test_unless_mod_0.rb
│       ├── test_until_0.parse-tree-whitequark.exp
│       ├── test_until_0.rb
│       ├── test_until_1.parse-tree-whitequark.exp
│       ├── test_until_1.rb
│       ├── test_until_mod_0.parse-tree-whitequark.exp
│       ├── test_until_mod_0.rb
│       ├── test_until_post_0.parse-tree-whitequark.exp
│       ├── test_until_post_0.rb
│       ├── test_var_and_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_and_asgn_0.rb
│       ├── test_var_op_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_0.rb
│       ├── test_var_op_asgn_1.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_1.rb
│       ├── test_var_op_asgn_2.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_2.rb
│       ├── test_var_op_asgn_3.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_3.rb
│       ├── test_var_op_asgn_cmd_0.parse-tree-whitequark.exp
│       ├── test_var_op_asgn_cmd_0.rb
│       ├── test_var_op_asgn_keyword_invalid_0.rb
│       ├── test_var_or_asgn_0.parse-tree-whitequark.exp
│       ├── test_var_or_asgn_0.rb
│       ├── test_when_multi_0.parse-tree-whitequark.exp
│       ├── test_when_multi_0.rb
│       ├── test_when_splat_0.parse-tree-whitequark.exp
│       ├── test_when_splat_0.rb
│       ├── test_when_then_0.parse-tree-whitequark.exp
│       ├── test_when_then_0.rb
│       ├── test_while_0.parse-tree-whitequark.exp
│       ├── test_while_0.rb
│       ├── test_while_1.parse-tree-whitequark.exp
│       ├── test_while_1.rb
│       ├── test_while_mod_0.parse-tree-whitequark.exp
│       ├── test_while_mod_0.rb
│       ├── test_while_post_0.parse-tree-whitequark.exp
│       ├── test_while_post_0.rb
│       ├── test_xstring_interp_0.parse-tree-whitequark.exp
│       ├── test_xstring_interp_0.rb
│       ├── test_xstring_plain_0.parse-tree-whitequark.exp
│       ├── test_xstring_plain_0.rb
│       ├── test_yield_0.parse-tree-whitequark.exp
│       ├── test_yield_0.rb
│       ├── test_yield_1.parse-tree-whitequark.exp
│       ├── test_yield_1.rb
│       ├── test_yield_2.parse-tree-whitequark.exp
│       ├── test_yield_2.rb
│       ├── test_yield_3.parse-tree-whitequark.exp
│       ├── test_yield_3.rb
│       ├── test_yield_block_0.rb
│       ├── test_yield_block_1.rb
│       ├── test_zsuper_0.parse-tree-whitequark.exp
│       └── test_zsuper_0.rb
├── third_party
│   ├── BUILD
│   ├── README.md
│   ├── blake2.BUILD
│   ├── clang.BUILD
│   ├── concurrentqueue.BUILD
│   ├── cpp_subprocess.BUILD
│   ├── crcpp.BUILD
│   ├── cxxopts.BUILD
│   ├── emscripten-clang.BUILD
│   ├── emscripten-toolchain.BUILD
│   ├── externals.bzl
│   ├── gems
│   │   ├── BUILD
│   │   ├── build_defs.BUILD
│   │   ├── gemfile.bzl
│   │   ├── gems.BUILD
│   │   ├── known_gems.bzl
│   │   └── rules.bzl
│   ├── gtest.BUILD
│   ├── jemalloc.BUILD
│   ├── libb2.BUILD
│   ├── libprotobuf-mutator.BUILD
│   ├── licenses
│   │   ├── BUILD
│   │   ├── README
│   │   ├── abseil.txt
│   │   ├── blake2.txt
│   │   ├── crcpp.txt
│   │   ├── cxxopts.txt
│   │   ├── googletest.txt
│   │   ├── jemalloc.txt
│   │   ├── libb2.txt
│   │   ├── licenses.h
│   │   ├── lizard.txt
│   │   ├── lmdb.txt
│   │   ├── msgpack-c.txt
│   │   ├── pdqsort.txt
│   │   ├── progressbar.txt
│   │   ├── protobuf.txt
│   │   ├── protobufmutator.txt
│   │   ├── rang.txt
│   │   ├── rapidjson.txt
│   │   ├── spdlog.txt
│   │   ├── statsd-c-client.txt
│   │   ├── tools
│   │   │   └── generate_licenses.cc
│   │   ├── typedruby.txt
│   │   └── yamlcpp.txt
│   ├── lizard.BUILD
│   ├── lmdb.BUILD
│   ├── msgpack.BUILD
│   ├── openssl
│   │   ├── BUILD
│   │   ├── darwin.BUILD
│   │   └── linux.BUILD
│   ├── parser
│   │   ├── BUILD
│   │   ├── README.md
│   │   ├── cc
│   │   │   ├── capi.cc
│   │   │   ├── context.cc
│   │   │   ├── driver.cc
│   │   │   ├── grammars
│   │   │   │   └── typedruby.ypp
│   │   │   ├── lexer.rl
│   │   │   ├── literal.cc
│   │   │   ├── state_stack.cc
│   │   │   └── token.cc
│   │   ├── codegen
│   │   │   ├── builder.rb
│   │   │   └── generate_diagnostics.cc
│   │   └── include\\ruby_parser
│   │       ├── builder.hh
│   │       ├── capi.hh
│   │       ├── context.hh
│   │       ├── diagnostic.hh
│   │       ├── driver.hh
│   │       ├── lexer.hh
│   │       ├── literal.hh
│   │       ├── node.hh
│   │       ├── pool.hh
│   │       ├── state_stack.hh
│   │       └── token.hh
│   ├── pdqsort.BUILD
│   ├── progressbar
│   │   ├── BUILD
│   │   ├── README
│   │   ├── progressbar
│   │   │   ├── progressbar.h
│   │   │   └── statusbar.h
│   │   └── src
│   │       ├── progressbar.c
│   │       └── statusbar.c
│   ├── progressbar.BUILD
│   ├── rang.BUILD
│   ├── rapidjson.BUILD
│   ├── ruby
│   │   ├── BUILD
│   │   ├── build-ruby.bzl
│   │   └── ruby.BUILD
│   ├── spdlog.BUILD
│   ├── statsd.BUILD
│   ├── yaml_cpp.BUILD
│   └── zlib.BUILD
├── tools
│   ├── BUILD
│   ├── bazel
│   ├── buildstamp
│   │   └── get_workspace_status
│   ├── clang.bzl
│   ├── config
│   │   └── BUILD
│   ├── scripts
│   │   ├── build_compilation_db.sh
│   │   ├── cfg-view.sh
│   │   ├── check_using_namespace_std.sh
│   │   ├── ci_checks.sh
│   │   ├── format_build_files.sh
│   │   ├── format_cxx.sh
│   │   ├── format_website.sh
│   │   ├── fuzz.sh
│   │   ├── fuzz_minimize_all.sh
│   │   ├── fuzz_minimize_crash.sh
│   │   ├── generate_compdb_targets.sh
│   │   ├── import_whitequark.rb
│   │   ├── import_whitequark.sh
│   │   ├── lint_cxx.sh
│   │   ├── lint_sh.sh
│   │   ├── make_worktree.sh
│   │   ├── regen-emscripten-cache.sh
│   │   ├── try_fast_path_tests.sh
│   │   ├── update-sorbet.run.sh
│   │   ├── update_exp_files.sh
│   │   └── update_testdata_exp.sh
│   └── toolchain
│       ├── webasm-darwin
│       │   ├── BUILD
│       │   ├── ar.sh
│       │   ├── cc_toolchain_config.bzl
│       │   ├── em_cache_existing.tar.gz
│       │   └── emcc.sh
│       └── webasm-linux
│           ├── BUILD
│           ├── ar.sh
│           ├── cc_toolchain_config.bzl
│           ├── em_cache_existing.tar.gz
│           └── emcc.sh
└── website
    ├── README.md
    ├── blog
    │   ├── 2019-05-16-state-of-sorbet-spring-2019.md
    │   ├── 2019-06-20-open-sourcing-sorbet.md
    │   └── 2019-12-20-announcing-sorbet-0.5.md
    ├── core
    │   ├── Footer.js
    │   └── PageSection.js
    ├── docs
    │   ├── abstract.md
    │   ├── adopting.md
    │   ├── attached-class.md
    │   ├── class-of.md
    │   ├── class-types.md
    │   ├── cli.md
    │   ├── error-reference.md
    │   ├── exhaustiveness.md
    │   ├── faq.md
    │   ├── final.md
    │   ├── flow-sensitive.md
    │   ├── gradual.md
    │   ├── intersection-types.md
    │   ├── legal
    │   │   └── trademark-policy.md
    │   ├── metaprogramming-plugins.md
    │   ├── metrics.md
    │   ├── nilable-types.md
    │   ├── non-forcing-constants.md
    │   ├── noreturn.md
    │   ├── override-checking.md
    │   ├── overview.md
    │   ├── procs.md
    │   ├── quickref.md
    │   ├── rbi.md
    │   ├── runtime.md
    │   ├── sealed.md
    │   ├── self-type.md
    │   ├── shapes.md
    │   ├── sigs.md
    │   ├── static.md
    │   ├── stdlib-generics.md
    │   ├── talks
    │   │   ├── curry-on-2019.md
    │   │   ├── jvm-ls-2019.md
    │   │   ├── ruby-conf-2019.md
    │   │   ├── ruby-kaigi-2018.md
    │   │   ├── ruby-kaigi-2019.md
    │   │   └── strange-loop-2018.md
    │   ├── tconfiguration.md
    │   ├── tenum.md
    │   ├── troubleshooting.md
    │   ├── tstruct.md
    │   ├── tuples.md
    │   ├── type-aliases.md
    │   ├── type-annotations.md
    │   ├── type-assertions.md
    │   ├── union-types.md
    │   └── untyped.md
    ├── i18n
    ├── package.json
    ├── pages\\en
    │   ├── community.js
    │   └── index.js
    ├── sidebars.json
    ├── siteConfig.js
    ├── static
    │   ├── blog\\2019\\05\\16
    │   │   ├── State-of-Sorbet-May-2019
    │   │   │   └── index.html
    │   │   └── State-of-Sorbet-May-2019.html
    │   ├── css
    │   │   ├── PageSection.css
    │   │   ├── custom.css
    │   │   └── overrides.css
    │   ├── docs
    │   │   ├── attr_reader.html
    │   │   ├── bad-rbi.html
    │   │   ├── include-kernel.html
    │   │   ├── index.html
    │   │   ├── rake.html
    │   │   ├── ruby-3.html
    │   │   └── why-nil.html
    │   ├── img
    │   │   ├── atrium-logo.jpg
    │   │   ├── autocompleteAndDocs1.gif
    │   │   ├── coinbase-logo.png
    │   │   ├── czi-logo.svg
    │   │   ├── editor_autocomplete.gif
    │   │   ├── editor_error_squiggles.gif
    │   │   ├── editor_go_to_definition.gif
    │   │   ├── factorial-logo.png
    │   │   ├── favicon.ico
    │   │   ├── gusto-logo.jpg
    │   │   ├── kickstarter-logo.png
    │   │   ├── shopify-logo.svg
    │   │   ├── sorbet-logo-card@2x.png
    │   │   ├── sorbet-logo-purple-sparkles.svg
    │   │   ├── sorbet-logo-white-sparkles.svg
    │   │   ├── sorbet-logo.svg
    │   │   ├── sourcegraph_github.gif
    │   │   ├── stripe-logo.svg
    │   │   ├── talk-thumb.png
    │   │   ├── testimonial_once_every_never.png
    │   │   ├── testimonial_pair_programming.png
    │   │   └── vonage-logo.png
    │   ├── js
    │   │   └── error-reference.js
    │   ├── slack
    │   │   └── index.html
    │   └── talks
    │       └── index.html
    └── style-guide.md";
    super::common_test(paths, expected);
}
