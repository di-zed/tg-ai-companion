FROM ubuntu:22.04

RUN apt-get update && apt-get install -y curl ca-certificates && apt-get clean

RUN curl -L -o /usr/local/bin/localai https://github.com/mudler/LocalAI/releases/download/v2.29.0/local-ai-Linux-arm64 \
    && chmod +x /usr/local/bin/localai

WORKDIR /app

CMD ["localai", "serve", "--models-path", "/models"]