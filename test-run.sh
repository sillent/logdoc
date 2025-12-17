#!/bin/bash
export LANGUAGES="c cpp golang rust ruby python javascript java"
export PROJECT="logdoc-test"
for LANGUAGE in $LANGUAGES
do
  mkdir -p ./tests/$LANGUAGE/out
  cargo run -- -l $LANGUAGE -d ./tests/$i -p $PROJECT -s ./tests/$LANGUAGE/out --new-line-separator=";"
done
#cargo run -- -l rust  -d tests/rust -p logdoc-project -s ./tests/rust/out --new-line-separator="<br/>"
#cargo run -- -l golang -d tests/go -p logdoc-project -s ./tests/golang/out --new-line-separator="<br/>"
#cargo run -- -l c  -d tests/c -p logdoc-project -s ./tests/c/out --new-line-separator="<br/>"
#cargo run -- -l cpp  -d tests/cpp -p logdoc-project -s ./tests//cpp/out --new-line-separator="<br/>"
#cargo run -- -l python  -d tests/python -p logdoc-project -s ./tests//python/out --new-line-separator="<br/>"
#cargo run -- -l ruby -d tests/ruby -p logdoc-project -s ./tests/ruby/out --new-line-separator="<br/>"
