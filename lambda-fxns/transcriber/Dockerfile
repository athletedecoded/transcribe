####################################
#   STAGE 0: Build transcribe-fxn
####################################
FROM rust:1-bullseye as builder

WORKDIR /usr/src/app
COPY .. .

RUN cargo build --release

####################################
#   STAGE 1: Build Amazon 2023 Base OS Image
####################################
FROM public.ecr.aws/lambda/provided:al2023

## Install dependencies
RUN dnf update -y && \
      dnf install -y git make clang

# Install whisper-cpp
RUN git clone https://github.com/ggerganov/whisper.cpp.git && cd whisper.cpp && make

# Download base model
RUN ./whisper.cpp/models/download-ggml-model.sh base.en

# Copy transcribe-fxn binary & shell script
COPY --from=builder /usr/src/app/target/release/transcriber /usr/local/bin/transcriber
COPY --from=builder /usr/src/app/transcribe.sh /usr/local/bin/transcribe.sh

# Define entrypoint
ENTRYPOINT ["/usr/local/bin/transcriber"]