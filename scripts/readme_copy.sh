# Small script to copy README.md to lib.rs

#!/bin/zsh
cd src/
sed -i '/\/\/!.*$/d' lib.rs
top=`sed 's|.*|//! &|g' ../README.md`
echo $top | cat - lib.rs > tmp && mv tmp lib.rs
