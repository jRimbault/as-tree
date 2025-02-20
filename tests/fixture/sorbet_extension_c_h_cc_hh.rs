#[test]
fn test() {
    let paths = "\
ast/ArgParsing.cc
ast/ArgParsing.h
ast/Helpers.cc
ast/Helpers.h
ast/TreeCopying.cc
ast/TreeSanityChecks.cc
ast/Trees.cc
ast/Trees.h
ast/ast.h
ast/desugar/Desugar.cc
ast/desugar/Desugar.h
ast/desugar/test/desugar_test.cc
ast/substitute/Substitute.cc
ast/substitute/substitute.h
ast/treemap/treemap.h
ast/verifier/Verifier.cc
ast/verifier/verifier.h
cfg/CFG.cc
cfg/CFG.h
cfg/Instructions.cc
cfg/Instructions.h
cfg/builder/builder.h
cfg/builder/builder_entry.cc
cfg/builder/builder_finalize.cc
cfg/builder/builder_walk.cc
class_flatten/class_flatten.cc
class_flatten/class_flatten.h
common/ConstExprStr.h
common/Counters.cc
common/Counters.h
common/Counters_impl.h
common/Exception.h
common/FileOps.h
common/FileSystem.cc
common/FileSystem.h
common/JSON.cc
common/JSON.h
common/Levenstein.cc
common/Levenstein.h
common/Random.cc
common/Random.h
common/Subprocess.cc
common/Subprocess.h
common/Timer.cc
common/Timer.h
common/backtrace.cc
common/common.cc
common/common.h
common/concurrency/ConcurrentQueue.h
common/concurrency/WorkerPool.h
common/concurrency/WorkerPoolImpl.cc
common/concurrency/WorkerPoolImpl.h
common/crypto_hashing/crypto_hashing.h
common/formatting.h
common/kvstore/KeyValueStore-emscripten.cc
common/kvstore/KeyValueStore.cc
common/kvstore/KeyValueStore.h
common/kvstore/test/kvstore_test.cc
common/os/emscripten.cc
common/os/linux.cc
common/os/mac.cc
common/os/os.cc
common/os/os.h
common/sort.h
common/statsd/statsd-emscripten.cc
common/statsd/statsd.cc
common/statsd/statsd.h
common/test/common_test.cc
common/typecase.h
common/web_tracer_framework/tracing.cc
common/web_tracer_framework/tracing.h
core/AutocorrectSuggestion.cc
core/AutocorrectSuggestion.h
core/Context.cc
core/Context.h
core/DebugOnlyCheck.h
core/Error.cc
core/Error.h
core/ErrorFlusher.cc
core/ErrorFlusher.h
core/ErrorQueue.cc
core/ErrorQueue.h
core/ErrorQueueMessage.h
core/Files.cc
core/Files.h
core/GlobalState.cc
core/GlobalState.h
core/GlobalSubstitution.h
core/Hashing.h
core/Loc.cc
core/Loc.h
core/LocalVariable.cc
core/LocalVariable.h
core/NameHash.cc
core/NameHash.h
core/NameRef.h
core/Names.cc
core/Names.h
core/StrictLevel.h
core/SymbolRef.h
core/Symbols.cc
core/Symbols.h
core/TypeConstraint.cc
core/TypeConstraint.h
core/TypePtr.h
core/Types.h
core/TypesAndOrigins.cc
core/Unfreeze.cc
core/Unfreeze.h
core/core.h
core/errors/cfg.h
core/errors/desugar.h
core/errors/errors.h
core/errors/infer.h
core/errors/internal.h
core/errors/namer.h
core/errors/parser.h
core/errors/plugin.h
core/errors/resolver.h
core/errors/rewriter.h
core/lsp/PreemptionTaskManager.cc
core/lsp/PreemptionTaskManager.h
core/lsp/Query.cc
core/lsp/Query.h
core/lsp/QueryResponse.cc
core/lsp/QueryResponse.h
core/lsp/Task.h
core/lsp/TypecheckEpochManager.cc
core/lsp/TypecheckEpochManager.h
core/proto/proto.cc
core/proto/proto.h
core/serialize/pickler.h
core/serialize/serialize.cc
core/serialize/serialize.h
core/serialize/test/serialize_test.cc
core/test/core_test.cc
core/tools/generate_names.cc
core/types/calls.cc
core/types/printing.cc
core/types/subtyping.cc
core/types/typemaps.cc
core/types/types.cc
definition_validator/validator.cc
definition_validator/validator.h
definition_validator/variance.cc
definition_validator/variance.h
emscripten/main.cc
infer/SigSuggestion.cc
infer/SigSuggestion.h
infer/environment.cc
infer/environment.h
infer/infer.h
infer/inference.cc
infer/inference.h
infer/test/infer_test.cc
local_vars/local_vars.cc
local_vars/local_vars.h
main/autogen/autogen.cc
main/autogen/autogen.h
main/autogen/autoloader.cc
main/autogen/autoloader.h
main/autogen/subclasses.cc
main/autogen/subclasses.h
main/cache/cache-orig.cc
main/cache/cache.cc
main/cache/cache.h
main/lsp/DefLocSaver.cc
main/lsp/DefLocSaver.h
main/lsp/ErrorReporter.cc
main/lsp/ErrorReporter.h
main/lsp/LSPConfiguration.cc
main/lsp/LSPConfiguration.h
main/lsp/LSPFileUpdates.cc
main/lsp/LSPFileUpdates.h
main/lsp/LSPIndexer.cc
main/lsp/LSPIndexer.h
main/lsp/LSPInput.cc
main/lsp/LSPInput.h
main/lsp/LSPMessage.cc
main/lsp/LSPMessage.h
main/lsp/LSPOutput.cc
main/lsp/LSPOutput.h
main/lsp/LSPPreprocessor.cc
main/lsp/LSPPreprocessor.h
main/lsp/LSPTask.cc
main/lsp/LSPTask.h
main/lsp/LSPTypechecker.cc
main/lsp/LSPTypechecker.h
main/lsp/LSPTypecheckerCoordinator.cc
main/lsp/LSPTypecheckerCoordinator.h
main/lsp/LocalVarFinder.cc
main/lsp/LocalVarFinder.h
main/lsp/LocalVarSaver.cc
main/lsp/LocalVarSaver.h
main/lsp/NextMethodFinder.cc
main/lsp/NextMethodFinder.h
main/lsp/ShowOperation.cc
main/lsp/ShowOperation.h
main/lsp/UndoState.cc
main/lsp/UndoState.h
main/lsp/json_enums.h
main/lsp/json_types.cc
main/lsp/json_types.h
main/lsp/lsp.cc
main/lsp/lsp.h
main/lsp/lsp_helpers.cc
main/lsp/lsp_messages_gen_helpers.cc
main/lsp/lsp_messages_gen_helpers.h
main/lsp/notifications/cancel_request.cc
main/lsp/notifications/cancel_request.h
main/lsp/notifications/exit.cc
main/lsp/notifications/exit.h
main/lsp/notifications/initialized.cc
main/lsp/notifications/initialized.h
main/lsp/notifications/notifications.h
main/lsp/notifications/sorbet_fence.cc
main/lsp/notifications/sorbet_fence.h
main/lsp/notifications/sorbet_pause.cc
main/lsp/notifications/sorbet_pause.h
main/lsp/notifications/sorbet_resume.cc
main/lsp/notifications/sorbet_resume.h
main/lsp/notifications/sorbet_workspace_edit.cc
main/lsp/notifications/sorbet_workspace_edit.h
main/lsp/protocol.cc
main/lsp/request_dispatch.cc
main/lsp/requests/code_action.cc
main/lsp/requests/code_action.h
main/lsp/requests/completion.cc
main/lsp/requests/completion.h
main/lsp/requests/definition.cc
main/lsp/requests/definition.h
main/lsp/requests/document_highlight.cc
main/lsp/requests/document_highlight.h
main/lsp/requests/document_symbol.cc
main/lsp/requests/document_symbol.h
main/lsp/requests/get_counters.cc
main/lsp/requests/get_counters.h
main/lsp/requests/hover.cc
main/lsp/requests/hover.h
main/lsp/requests/initialize.cc
main/lsp/requests/initialize.h
main/lsp/requests/references.cc
main/lsp/requests/references.h
main/lsp/requests/requests.h
main/lsp/requests/shutdown.cc
main/lsp/requests/shutdown.h
main/lsp/requests/signature_help.cc
main/lsp/requests/signature_help.h
main/lsp/requests/sorbet_error.cc
main/lsp/requests/sorbet_error.h
main/lsp/requests/sorbet_read_file.cc
main/lsp/requests/sorbet_read_file.h
main/lsp/requests/type_definition.cc
main/lsp/requests/type_definition.h
main/lsp/requests/workspace_symbols.cc
main/lsp/requests/workspace_symbols.h
main/lsp/test/error_reporter_test.cc
main/lsp/test/generate_lsp_messages_test.cc
main/lsp/test/lsp_file_updates_test.cc
main/lsp/test/lsp_preprocessor_test.cc
main/lsp/test/lsp_test.cc
main/lsp/tools/generate_lsp_messages.cc
main/lsp/tools/generate_lsp_messages.h
main/lsp/tools/make_lsp_types.cc
main/lsp/tools/make_lsp_types.h
main/lsp/watchman/WatchmanProcess.cc
main/lsp/watchman/WatchmanProcess.h
main/lsp/wrapper.cc
main/lsp/wrapper.h
main/main.cc
main/options/ConfigParser.cc
main/options/ConfigParser.h
main/options/options.cc
main/options/options.h
main/options/test/options_test.cc
main/pipeline/ProgressIndicator.cc
main/pipeline/ProgressIndicator.h
main/pipeline/pipeline.cc
main/pipeline/pipeline.h
main/pipeline/semantic_extension/DefaultImplementation.cc
main/pipeline/semantic_extension/SemanticExtension.h
main/realmain.cc
main/realmain.h
namer/configatron/configatron.cc
namer/configatron/configatron.h
namer/namer.cc
namer/namer.h
namer/test/namer_test.cc
parser/Builder.cc
parser/Builder.h
parser/Dedenter.h
parser/Node.cc
parser/Node.h
parser/Parser.cc
parser/parser.h
parser/test/parser_test.cc
parser/tools/generate_ast.cc
payload/binary/binary.h
payload/binary/empty.cc
payload/binary/tools/gen_state_payload.cc
payload/otherwise_compdb_bugs_out.cc
payload/payload.cc
payload/payload.h
payload/text/nopopulate.cc
payload/text/populate.cc
payload/text/text.h
payload/text/tools/generate_payload.cc
plugin/Plugins.cc
plugin/Plugins.h
plugin/SubprocessTextPlugin.cc
plugin/SubprocessTextPlugin.h
rbi/tools/generate_procs.cc
resolver/CorrectTypeAlias.cc
resolver/CorrectTypeAlias.h
resolver/GlobalPass.cc
resolver/resolver.cc
resolver/resolver.h
resolver/type_syntax.cc
resolver/type_syntax.h
rewriter/AttrReader.cc
rewriter/AttrReader.h
rewriter/ClassNew.cc
rewriter/ClassNew.h
rewriter/Cleanup.cc
rewriter/Cleanup.h
rewriter/Command.cc
rewriter/Command.h
rewriter/DSLBuilder.cc
rewriter/DSLBuilder.h
rewriter/DefaultArgs.cc
rewriter/DefaultArgs.h
rewriter/Delegate.cc
rewriter/Delegate.h
rewriter/Flatfiles.cc
rewriter/Flatfiles.h
rewriter/Flatten.cc
rewriter/Flatten.h
rewriter/Initializer.cc
rewriter/Initializer.h
rewriter/InterfaceWrapper.cc
rewriter/InterfaceWrapper.h
rewriter/Mattr.cc
rewriter/Mattr.h
rewriter/Minitest.cc
rewriter/Minitest.h
rewriter/MixinEncryptedProp.cc
rewriter/MixinEncryptedProp.h
rewriter/ModuleFunction.cc
rewriter/ModuleFunction.h
rewriter/Private.cc
rewriter/Private.h
rewriter/Prop.cc
rewriter/Prop.h
rewriter/ProtobufDescriptorPool.cc
rewriter/ProtobufDescriptorPool.h
rewriter/Rails.cc
rewriter/Rails.h
rewriter/Regexp.cc
rewriter/Regexp.h
rewriter/SelfNew.cc
rewriter/SelfNew.h
rewriter/SigRewriter.cc
rewriter/SigRewriter.h
rewriter/Struct.cc
rewriter/Struct.h
rewriter/TEnum.cc
rewriter/TEnum.h
rewriter/TypeMembers.cc
rewriter/TypeMembers.h
rewriter/Util.cc
rewriter/Util.h
rewriter/rewriter.cc
rewriter/rewriter.h
sorbet_version/sorbet_version.c
sorbet_version/sorbet_version.h
test/LSPTest.cc
test/LSPTest.h
test/autocorrect-test.cc
test/backtrace-test-error.cc
test/backtrace-test-raise.cc
test/error-check-test.cc
test/fuzz/empty.cc
test/fuzz/fuzz_dash_e.cc
test/fuzz/fuzz_doc_symbols.cc
test/fuzz/fuzz_hover.cc
test/hello-test.cc
test/helpers/MockFileSystem.cc
test/helpers/MockFileSystem.h
test/helpers/expectations.h
test/helpers/lsp.cc
test/helpers/lsp.h
test/helpers/position_assertions.cc
test/helpers/position_assertions.h
test/lsp/ProtocolTest.cc
test/lsp/ProtocolTest.h
test/lsp/cache_protocol_test_corpus.cc
test/lsp/multithreaded_protocol_test_corpus.cc
test/lsp/protocol_test_corpus.cc
test/lsp/watchman_test_corpus.cc
test/print_document_symbols.cc
test/test_corpus.cc
third_party/licenses/licenses.h
third_party/licenses/tools/generate_licenses.cc
third_party/parser/cc/capi.cc
third_party/parser/cc/context.cc
third_party/parser/cc/driver.cc
third_party/parser/cc/literal.cc
third_party/parser/cc/state_stack.cc
third_party/parser/cc/token.cc
third_party/parser/codegen/generate_diagnostics.cc
third_party/parser/include/ruby_parser/builder.hh
third_party/parser/include/ruby_parser/capi.hh
third_party/parser/include/ruby_parser/context.hh
third_party/parser/include/ruby_parser/diagnostic.hh
third_party/parser/include/ruby_parser/driver.hh
third_party/parser/include/ruby_parser/lexer.hh
third_party/parser/include/ruby_parser/literal.hh
third_party/parser/include/ruby_parser/node.hh
third_party/parser/include/ruby_parser/pool.hh
third_party/parser/include/ruby_parser/state_stack.hh
third_party/parser/include/ruby_parser/token.hh
third_party/progressbar/progressbar/progressbar.h
third_party/progressbar/progressbar/statusbar.h
third_party/progressbar/src/progressbar.c
third_party/progressbar/src/statusbar.c
";
    #[cfg(not(target_os = "windows"))]
    let expected = "\
.
├── ast
│   ├── ArgParsing.cc
│   ├── ArgParsing.h
│   ├── Helpers.cc
│   ├── Helpers.h
│   ├── TreeCopying.cc
│   ├── TreeSanityChecks.cc
│   ├── Trees.cc
│   ├── Trees.h
│   ├── ast.h
│   ├── desugar
│   │   ├── Desugar.cc
│   │   ├── Desugar.h
│   │   └── test
│   │       └── desugar_test.cc
│   ├── substitute
│   │   ├── Substitute.cc
│   │   └── substitute.h
│   ├── treemap
│   │   └── treemap.h
│   └── verifier
│       ├── Verifier.cc
│       └── verifier.h
├── cfg
│   ├── CFG.cc
│   ├── CFG.h
│   ├── Instructions.cc
│   ├── Instructions.h
│   └── builder
│       ├── builder.h
│       ├── builder_entry.cc
│       ├── builder_finalize.cc
│       └── builder_walk.cc
├── class_flatten
│   ├── class_flatten.cc
│   └── class_flatten.h
├── common
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
│   │   ├── ConcurrentQueue.h
│   │   ├── WorkerPool.h
│   │   ├── WorkerPoolImpl.cc
│   │   └── WorkerPoolImpl.h
│   ├── crypto_hashing
│   │   └── crypto_hashing.h
│   ├── formatting.h
│   ├── kvstore
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
│   │   ├── statsd-emscripten.cc
│   │   ├── statsd.cc
│   │   └── statsd.h
│   ├── test
│   │   └── common_test.cc
│   ├── typecase.h
│   └── web_tracer_framework
│       ├── tracing.cc
│       └── tracing.h
├── core
│   ├── AutocorrectSuggestion.cc
│   ├── AutocorrectSuggestion.h
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
│   │   ├── proto.cc
│   │   └── proto.h
│   ├── serialize
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
│   ├── validator.cc
│   ├── validator.h
│   ├── variance.cc
│   └── variance.h
├── emscripten
│   └── main.cc
├── infer
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
│   ├── local_vars.cc
│   └── local_vars.h
├── main
│   ├── autogen
│   │   ├── autogen.cc
│   │   ├── autogen.h
│   │   ├── autoloader.cc
│   │   ├── autoloader.h
│   │   ├── subclasses.cc
│   │   └── subclasses.h
│   ├── cache
│   │   ├── cache-orig.cc
│   │   ├── cache.cc
│   │   └── cache.h
│   ├── lsp
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
│   │   ├── ConfigParser.cc
│   │   ├── ConfigParser.h
│   │   ├── options.cc
│   │   ├── options.h
│   │   └── test
│   │       └── options_test.cc
│   ├── pipeline
│   │   ├── ProgressIndicator.cc
│   │   ├── ProgressIndicator.h
│   │   ├── pipeline.cc
│   │   ├── pipeline.h
│   │   └── semantic_extension
│   │       ├── DefaultImplementation.cc
│   │       └── SemanticExtension.h
│   ├── realmain.cc
│   └── realmain.h
├── namer
│   ├── configatron
│   │   ├── configatron.cc
│   │   └── configatron.h
│   ├── namer.cc
│   ├── namer.h
│   └── test
│       └── namer_test.cc
├── parser
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
│   ├── binary
│   │   ├── binary.h
│   │   ├── empty.cc
│   │   └── tools
│   │       └── gen_state_payload.cc
│   ├── otherwise_compdb_bugs_out.cc
│   ├── payload.cc
│   ├── payload.h
│   └── text
│       ├── nopopulate.cc
│       ├── populate.cc
│       ├── text.h
│       └── tools
│           └── generate_payload.cc
├── plugin
│   ├── Plugins.cc
│   ├── Plugins.h
│   ├── SubprocessTextPlugin.cc
│   └── SubprocessTextPlugin.h
├── rbi/tools
│   └── generate_procs.cc
├── resolver
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
│   ├── sorbet_version.c
│   └── sorbet_version.h
├── test
│   ├── LSPTest.cc
│   ├── LSPTest.h
│   ├── autocorrect-test.cc
│   ├── backtrace-test-error.cc
│   ├── backtrace-test-raise.cc
│   ├── error-check-test.cc
│   ├── fuzz
│   │   ├── empty.cc
│   │   ├── fuzz_dash_e.cc
│   │   ├── fuzz_doc_symbols.cc
│   │   └── fuzz_hover.cc
│   ├── hello-test.cc
│   ├── helpers
│   │   ├── MockFileSystem.cc
│   │   ├── MockFileSystem.h
│   │   ├── expectations.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── position_assertions.cc
│   │   └── position_assertions.h
│   ├── lsp
│   │   ├── ProtocolTest.cc
│   │   ├── ProtocolTest.h
│   │   ├── cache_protocol_test_corpus.cc
│   │   ├── multithreaded_protocol_test_corpus.cc
│   │   ├── protocol_test_corpus.cc
│   │   └── watchman_test_corpus.cc
│   ├── print_document_symbols.cc
│   └── test_corpus.cc
└── third_party
    ├── licenses
    │   ├── licenses.h
    │   └── tools
    │       └── generate_licenses.cc
    ├── parser
    │   ├── cc
    │   │   ├── capi.cc
    │   │   ├── context.cc
    │   │   ├── driver.cc
    │   │   ├── literal.cc
    │   │   ├── state_stack.cc
    │   │   └── token.cc
    │   ├── codegen
    │   │   └── generate_diagnostics.cc
    │   └── include/ruby_parser
    │       ├── builder.hh
    │       ├── capi.hh
    │       ├── context.hh
    │       ├── diagnostic.hh
    │       ├── driver.hh
    │       ├── lexer.hh
    │       ├── literal.hh
    │       ├── node.hh
    │       ├── pool.hh
    │       ├── state_stack.hh
    │       └── token.hh
    └── progressbar
        ├── progressbar
        │   ├── progressbar.h
        │   └── statusbar.h
        └── src
            ├── progressbar.c
            └── statusbar.c
";
    #[cfg(target_os = "windows")]
    let expected = "\
.
├── ast
│   ├── ArgParsing.cc
│   ├── ArgParsing.h
│   ├── Helpers.cc
│   ├── Helpers.h
│   ├── TreeCopying.cc
│   ├── TreeSanityChecks.cc
│   ├── Trees.cc
│   ├── Trees.h
│   ├── ast.h
│   ├── desugar
│   │   ├── Desugar.cc
│   │   ├── Desugar.h
│   │   └── test
│   │       └── desugar_test.cc
│   ├── substitute
│   │   ├── Substitute.cc
│   │   └── substitute.h
│   ├── treemap
│   │   └── treemap.h
│   └── verifier
│       ├── Verifier.cc
│       └── verifier.h
├── cfg
│   ├── CFG.cc
│   ├── CFG.h
│   ├── Instructions.cc
│   ├── Instructions.h
│   └── builder
│       ├── builder.h
│       ├── builder_entry.cc
│       ├── builder_finalize.cc
│       └── builder_walk.cc
├── class_flatten
│   ├── class_flatten.cc
│   └── class_flatten.h
├── common
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
│   │   ├── ConcurrentQueue.h
│   │   ├── WorkerPool.h
│   │   ├── WorkerPoolImpl.cc
│   │   └── WorkerPoolImpl.h
│   ├── crypto_hashing
│   │   └── crypto_hashing.h
│   ├── formatting.h
│   ├── kvstore
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
│   │   ├── statsd-emscripten.cc
│   │   ├── statsd.cc
│   │   └── statsd.h
│   ├── test
│   │   └── common_test.cc
│   ├── typecase.h
│   └── web_tracer_framework
│       ├── tracing.cc
│       └── tracing.h
├── core
│   ├── AutocorrectSuggestion.cc
│   ├── AutocorrectSuggestion.h
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
│   │   ├── proto.cc
│   │   └── proto.h
│   ├── serialize
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
│   ├── validator.cc
│   ├── validator.h
│   ├── variance.cc
│   └── variance.h
├── emscripten
│   └── main.cc
├── infer
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
│   ├── local_vars.cc
│   └── local_vars.h
├── main
│   ├── autogen
│   │   ├── autogen.cc
│   │   ├── autogen.h
│   │   ├── autoloader.cc
│   │   ├── autoloader.h
│   │   ├── subclasses.cc
│   │   └── subclasses.h
│   ├── cache
│   │   ├── cache-orig.cc
│   │   ├── cache.cc
│   │   └── cache.h
│   ├── lsp
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
│   │   ├── ConfigParser.cc
│   │   ├── ConfigParser.h
│   │   ├── options.cc
│   │   ├── options.h
│   │   └── test
│   │       └── options_test.cc
│   ├── pipeline
│   │   ├── ProgressIndicator.cc
│   │   ├── ProgressIndicator.h
│   │   ├── pipeline.cc
│   │   ├── pipeline.h
│   │   └── semantic_extension
│   │       ├── DefaultImplementation.cc
│   │       └── SemanticExtension.h
│   ├── realmain.cc
│   └── realmain.h
├── namer
│   ├── configatron
│   │   ├── configatron.cc
│   │   └── configatron.h
│   ├── namer.cc
│   ├── namer.h
│   └── test
│       └── namer_test.cc
├── parser
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
│   ├── binary
│   │   ├── binary.h
│   │   ├── empty.cc
│   │   └── tools
│   │       └── gen_state_payload.cc
│   ├── otherwise_compdb_bugs_out.cc
│   ├── payload.cc
│   ├── payload.h
│   └── text
│       ├── nopopulate.cc
│       ├── populate.cc
│       ├── text.h
│       └── tools
│           └── generate_payload.cc
├── plugin
│   ├── Plugins.cc
│   ├── Plugins.h
│   ├── SubprocessTextPlugin.cc
│   └── SubprocessTextPlugin.h
├── rbi\\tools
│   └── generate_procs.cc
├── resolver
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
│   ├── sorbet_version.c
│   └── sorbet_version.h
├── test
│   ├── LSPTest.cc
│   ├── LSPTest.h
│   ├── autocorrect-test.cc
│   ├── backtrace-test-error.cc
│   ├── backtrace-test-raise.cc
│   ├── error-check-test.cc
│   ├── fuzz
│   │   ├── empty.cc
│   │   ├── fuzz_dash_e.cc
│   │   ├── fuzz_doc_symbols.cc
│   │   └── fuzz_hover.cc
│   ├── hello-test.cc
│   ├── helpers
│   │   ├── MockFileSystem.cc
│   │   ├── MockFileSystem.h
│   │   ├── expectations.h
│   │   ├── lsp.cc
│   │   ├── lsp.h
│   │   ├── position_assertions.cc
│   │   └── position_assertions.h
│   ├── lsp
│   │   ├── ProtocolTest.cc
│   │   ├── ProtocolTest.h
│   │   ├── cache_protocol_test_corpus.cc
│   │   ├── multithreaded_protocol_test_corpus.cc
│   │   ├── protocol_test_corpus.cc
│   │   └── watchman_test_corpus.cc
│   ├── print_document_symbols.cc
│   └── test_corpus.cc
└── third_party
    ├── licenses
    │   ├── licenses.h
    │   └── tools
    │       └── generate_licenses.cc
    ├── parser
    │   ├── cc
    │   │   ├── capi.cc
    │   │   ├── context.cc
    │   │   ├── driver.cc
    │   │   ├── literal.cc
    │   │   ├── state_stack.cc
    │   │   └── token.cc
    │   ├── codegen
    │   │   └── generate_diagnostics.cc
    │   └── include\\ruby_parser
    │       ├── builder.hh
    │       ├── capi.hh
    │       ├── context.hh
    │       ├── diagnostic.hh
    │       ├── driver.hh
    │       ├── lexer.hh
    │       ├── literal.hh
    │       ├── node.hh
    │       ├── pool.hh
    │       ├── state_stack.hh
    │       └── token.hh
    └── progressbar
        ├── progressbar
        │   ├── progressbar.h
        │   └── statusbar.h
        └── src
            ├── progressbar.c
            └── statusbar.c
";
    super::common_test(paths, expected);
}
