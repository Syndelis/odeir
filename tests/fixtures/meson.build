# Lists all json files in the current folder
find = run_command(['find', '-maxdepth', '1', '-name', '*.json'], check: true)
fixture_sources = find.stdout().strip().split('\n')

# Filters empty filenames (can happen) and replaces .json with .h
fixture_headers = []
foreach filepath : fixture_sources
  if filepath != ''
    fixture_headers += fs.stem(filepath)+'.h'
  endif
endforeach

# Runs the script
fixtures = custom_target('fixtures',
  command: 'gen_headers.sh',
  output: fixture_headers,
  input: fixture_sources,
  )

