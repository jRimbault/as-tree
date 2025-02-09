#[test]
fn test() {
    let paths = "\
ACKNOWLEDGEMENTS.md
CODE_OF_CONDUCT.md
CONTRIBUTING.md
README.md
docs/JRuby.md
docs/README.md
docs/compressors.md
docs/internals.md
docs/logo/README.md
docs/pipeline.md
docs/suggest-sig.md
docs/tracing.md
gems/sorbet/README.md
gems/sorbet-runtime/README.md
third_party/README.md
third_party/parser/README.md
website/README.md
website/blog/2019-05-16-state-of-sorbet-spring-2019.md
website/blog/2019-06-20-open-sourcing-sorbet.md
website/blog/2019-12-20-announcing-sorbet-0.5.md
website/docs/abstract.md
website/docs/adopting.md
website/docs/attached-class.md
website/docs/class-of.md
website/docs/class-types.md
website/docs/cli.md
website/docs/error-reference.md
website/docs/exhaustiveness.md
website/docs/faq.md
website/docs/final.md
website/docs/flow-sensitive.md
website/docs/gradual.md
website/docs/intersection-types.md
website/docs/legal/trademark-policy.md
website/docs/metaprogramming-plugins.md
website/docs/metrics.md
website/docs/nilable-types.md
website/docs/non-forcing-constants.md
website/docs/noreturn.md
website/docs/override-checking.md
website/docs/overview.md
website/docs/procs.md
website/docs/quickref.md
website/docs/rbi.md
website/docs/runtime.md
website/docs/sealed.md
website/docs/self-type.md
website/docs/shapes.md
website/docs/sigs.md
website/docs/static.md
website/docs/stdlib-generics.md
website/docs/talks/curry-on-2019.md
website/docs/talks/jvm-ls-2019.md
website/docs/talks/ruby-conf-2019.md
website/docs/talks/ruby-kaigi-2018.md
website/docs/talks/ruby-kaigi-2019.md
website/docs/talks/strange-loop-2018.md
website/docs/tconfiguration.md
website/docs/tenum.md
website/docs/troubleshooting.md
website/docs/tstruct.md
website/docs/tuples.md
website/docs/type-aliases.md
website/docs/type-annotations.md
website/docs/type-assertions.md
website/docs/union-types.md
website/docs/untyped.md
website/style-guide.md
";
    let expected = "\
.
├── ACKNOWLEDGEMENTS.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── README.md
├── docs
│   ├── JRuby.md
│   ├── README.md
│   ├── compressors.md
│   ├── internals.md
│   ├── logo
│   │   └── README.md
│   ├── pipeline.md
│   ├── suggest-sig.md
│   └── tracing.md
├── gems
│   ├── sorbet
│   │   └── README.md
│   └── sorbet-runtime
│       └── README.md
├── third_party
│   ├── README.md
│   └── parser
│       └── README.md
└── website
    ├── README.md
    ├── blog
    │   ├── 2019-05-16-state-of-sorbet-spring-2019.md
    │   ├── 2019-06-20-open-sourcing-sorbet.md
    │   └── 2019-12-20-announcing-sorbet-0.5.md
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
    └── style-guide.md
";
    super::common_test(paths, expected);
}
