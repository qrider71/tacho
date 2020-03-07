#!/bin/zsh

echo "This is some test Output[${(l:0::0:)$((100+${RANDOM}%20+1))}ms] and should match"
echo "[${(l:0::0:)$((100000+${RANDOM}%20+1))},00ns] this should match as well"
echo "This should not match [${(l:0::0:)$((100+${RANDOM}%20+1))} xs] because of the xs"
echo "[${(l:0::0:)$((100+${RANDOM}%20+1))}.0 ms] this should match"
sleep 1

