#!/bin/bash
# Easy html syntax
# ./ht.sh "p.header.heading#id" hello

element=$(echo "$1" | tr '.' '\n' | tr '#' '\n' | head -n 1)
mod_list=$(echo "$1" | sed -E 's,([\.#]),\n\1,g')
classes=$(echo "$mod_list" | grep '\.' | tr -d '.' | tr '\n' ' ' | sed 's/\(.*\) /\1/')
ids=$(echo "$mod_list" | grep '#' | tr -d '#' | tr '\n' ' ' | sed 's/\(.*\) /\1/')
echo "<$element class=\"$classes\" id=\"$ids\">$2</$element>"
