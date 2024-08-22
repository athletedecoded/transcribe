####################################
#   STAGE 0: Build transcribe-fxn
####################################
FROM public.ecr.aws/docker/library/rust:slim-bullseye as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

####################################
#   STAGE 1: Build Amazon 2023 Base OS Image
####################################
FROM public.ecr.aws/lambda/provided:al2023

# /var/task
WORKDIR ${LAMBDA_TASK_ROOT}

## Install dependencies
RUN dnf update -y && \
      dnf install -y make clang xz tar wget

# Install ffmpeg from source
RUN wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz && \
    tar xvf ffmpeg-release-amd64-static.tar.xz && \
    mv ffmpeg-*-amd64-static/ff* /usr/local/bin/ && \
    ln -sf /usr/local/bin/ffmpeg/ffmpeg*/ffmpeg /usr/bin/ffmpeg && \
    rm ffmpeg-release-amd64-static.tar.xz && rm -rf ffmpeg-*-amd64-static

# Copy transcriber binary & shell script
COPY --from=builder /usr/src/app/target/release/transcriber /usr/local/bin/transcriber
COPY --from=builder /usr/src/app/transcribe.sh ./transcribe.sh

# Copy core whisper.cpp files
COPY --chmod=777 whisper.cpp/main ./main
COPY --chmod=777 whisper.cpp/models/ggml-base.en.bin ./models/ggml-base.en.bin
COPY --chmod=777 whisper.cpp/samples ./samples

# Define entrypoint
ENTRYPOINT ["/usr/local/bin/transcriber"]