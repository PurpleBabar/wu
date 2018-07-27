FROM rust:1.27-slim
LABEL maintainer="lalung.alexandre@gmail.com"

RUN set -x \
	&& apt-get update \
	&& apt-get install --no-install-recommends --no-install-suggests -y curl git \
    && curl https://sh.rustup.rs -sSyf | sh \
    && git clone https://github.com/wu-lang/wu \
    && cd wu/ \
    && cargo install \
    && apt-get remove -y curl \
	&& apt-get autoremove -y

WORKDIR /wu

ENTRYPOINT ["wu"]
