#!/bin/bash
# unset password
IFS=

while true; do
    read -p "$prompt" -r -s -n 1 char

    if [[ $char == $'\0' ]]; then
        break
    elif [[ $char == $'\177' ]]; then
        text="${text%?}"
    else
        text+="$char"
    fi

    clear
    figlet $text
done
