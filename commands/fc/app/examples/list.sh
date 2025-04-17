#!/bin/bash

echo -e "\n\033[1;34mLIST OF AVAILABLES EXAMPLES\033[0m"

find examples -type f -name "main.rs" -exec dirname {} \; | sed 's|examples/||'

echo