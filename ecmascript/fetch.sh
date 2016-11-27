exercism fetch
echo 'cleaning:'
find . -not \( -path './package.json' -or -path './node_modules/*' -prune \) -and \( -name 'package.json' -or -name 'gulpfile.js' \) -print -delete
