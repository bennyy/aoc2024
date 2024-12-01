#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

SESSION_FILE="${SCRIPT_DIR}/.aoc-session" # Session cookie
YEAR=2024
AOC_URL="https://adventofcode.com/${YEAR}/day"
INPUT_DIR="${SCRIPT_DIR}/inputs"

mkdir -p "$INPUT_DIR"

# Check if session file exists
if [[ ! -f "$SESSION_FILE" ]]; then
    echo "Session file '$SESSION_FILE' not found!"
    exit 1
fi

# Read the session cookie
SESSION=$(cat "$SESSION_FILE")

# Initialize progress display
PROGRESS="Downloading input for day: "

# Loop from 1 to 25
for DAY in $(seq -w 1 25); do
    FILE_PATH="${INPUT_DIR}/day_${DAY}.txt"
    URL="${AOC_URL}/${DAY#0}/input"

    PROGRESS="${PROGRESS}${DAY} "
    printf "\r%s" "${PROGRESS}"

    # Skip if the file already exists
    if [[ -f "$FILE_PATH" ]]; then
        continue
    fi

    # Download the input
    CURL_OUTPUT=$(curl --silent --show-error --fail --cookie "session=${SESSION}" "$URL" -o "$FILE_PATH" 2>&1)
    CURL_EXIT_CODE=$?

    # Check if curl failed
    if [[ $CURL_EXIT_CODE -ne 0 ]]; then
        if [[ $CURL_EXIT_CODE -eq 22 ]] && [[ "$CURL_OUTPUT" == *"404"* ]]; then
            echo -e "\nInput for day $DAY not available. Skip."
            exit 0
        else
            echo -e "\nError downloading input for day $DAY:"
            echo "$CURL_OUTPUT"
            exit 1
        fi
    fi
done

echo -e "\nDone!"
