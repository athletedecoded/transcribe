#!/bin/bash

# ./transcribe.sh <path/to/vid_dir>

if [ -z "$1" ]; then
  echo "Usage: ./transcribe.sh <path/to/vid_dir>"
  exit 1
fi

input_dir="$1"
parent_dir=$(dirname "$input_dir")
output_dir="${parent_dir}/transcripts"

# Create the output directory only if it does not exist
if [ ! -d "$output_dir" ]; then
  mkdir -p "$output_dir"
fi

find "$input_dir" -name '*.mp4' -print0 | while IFS= read -r -d '' video; do
  filename=$(basename "$video" .${video##*.})
  ffmpeg -loglevel error -i "$video" -f wav -ac 1 -acodec pcm_s16le -ar 16000 - | ./whisper.cpp/main -m whisper.cpp/models/ggml-base.en.bin -f - > "$output_dir/$filename.txt"
done

echo "DONE converting videos to transcripts."