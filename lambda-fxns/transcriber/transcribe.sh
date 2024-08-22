#!/bin/bash

# ./transcribe.sh <path/to/vid_dir>

if [ -z "$1" ]; then
  echo "Usage: ./transcribe.sh <path/to/vid_dir>"
  exit 1
fi

# check ffmpeg version
echo "ffmpeg version: $(ffmpeg -version)"

# i.e. /tmp/videos
vid_dir=$1
echo "Input directory: $vid_dir"
# i.e. /tmp/
root_dir=$(dirname "$vid_dir")
# i.e. /tmp/transcripts
output_dir="${root_dir}/transcripts"

for video in $(find "$vid_dir" -name '*.mp4'); do
  echo "Video: $video..."

  # Get the path relative to /tmp/videos i.e. /tmp/videos/path/to/video
  rel_path=$(realpath --relative-to="$vid_dir" "$video")

  # Get the sub dirs
  sub_dirs=$(dirname "$rel_path")

  # Make the relative dirs if DNE
  mkdir -p "$output_dir/$sub_dirs"

  # Get the base name of the video file without extension
  filename=$(basename "$video" .${video##*.})

  # Define the full path for the output transcription file
  output_file="$output_dir/$sub_dirs/$filename.txt"

  # Perform the transcription and save the output
  ffmpeg -loglevel error -i "$video" -f wav -ac 1 -acodec pcm_s16le -ar 16000 - | ./main -m models/ggml-base.en.bin -f - > "$output_file"

  echo "Transcription saved to $output_file"
done