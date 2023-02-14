#!/bin/bash

source ~/.env.sh # Set $API_KEY

API_URL="https://owlbot.info/api/v4/dictionary"

define() {
    curl -s -H "Authorization: Token $API_KEY" $API_URL/$1 | jq '.definitions[].definition'
}

for word in $(cat ./words.txt); do
    definition=$(define $word)
    echo "<p><b>$word:</b> $definition</p>"
done
