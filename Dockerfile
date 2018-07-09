FROM 1.27-slim
LABEL maintainer="lalung.alexandre@gmail.com"

RUN set -x \
	&& apt-get update \
	&& apt-get install --no-install-recommends --no-install-suggests -y curl git \
    && curl https://sh.rustup.rs -sSf | sh \
    && git clone https://github.com/wu-lang/wu \
    && cd wu/ \
    && cargo install \
    && apt-remove curl
