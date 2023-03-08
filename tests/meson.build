project('ode-ir-tests', 'rust', 'cpp', version: '1.0')
fs = import('fs')

build_odeir = custom_target('build-odeir',
  command: ['cargo', 'build', '-Z', 'unstable-options', '--out-dir', '@BUILD_ROOT@'],
  output: 'libodeir.a',
  build_always_stale: true,
  build_by_default: true,
  console: true,
  )

subdir('fixtures')

catch2 = subproject('catch2').get_variable('catch2_dep')

executable('testcpp', fixtures, 'test_deserialization.cpp', include_directories: ['../include', 'fixtures'], dependencies: catch2, link_with: build_odeir)
