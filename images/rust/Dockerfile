FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
  curl \
  wget \
  git \
  build-essential \
  vim \
  pkg-config \
  libssl-dev \
  && apt-get clean

# Rust installation:
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add colors to the terminal:
RUN echo "PS1='${debian_chroot:+($debian_chroot)}\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> /root/.bashrc

CMD ["/bin/bash"]