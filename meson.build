project('rust-analyzer-issue', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

rust = import('rust')

generated_bindings = rust.bindgen(
  input: 'main.h',
  output: 'bindings.rs',
)

# custom_target = custom_target('generated_bindings',
#   input: generated_bindings,
#   output: 'bindings.rs',
#   command: ['cp', '@INPUT@', '@OUTPUT@'],
# )
cargo = find_program('cargo')

build_lib = custom_target(
  'build_mylib',
  input: generated_bindings,
  output: 'dummy',
  command: [
    cargo,
    'build',
    '--release',
    '--manifest-path',
    meson.current_source_dir() + '/proj/Cargo.toml',
  ],
  env: {
    'CARGO_TARGET_DIR': meson.current_build_dir(),
    'BINDINGS': meson.current_build_dir() + '/bindings.rs',
  },
  build_always_stale: true,
)

lib = custom_target(
  'libmylib',
  input: build_lib,
  output: 'libmylib.a',
  command: [
    'cp',
    meson.current_build_dir() + '/release/libmylib.a',
    '@OUTPUT@',
  ],
  build_always_stale: true,
)


dep = declare_dependency(
  link_with: lib,
)

executable(
  'myexe',
  'main.c',
  # link_with: 'libmylib.a'
  dependencies: [dep],
)
