# FROM gitpod/workspace-full
FROM gitpod/workspace-full-vnc

RUN apt-get update \
    && apt-get install -y sudo
