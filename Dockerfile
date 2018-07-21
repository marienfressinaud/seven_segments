FROM alpine:3.8
LABEL maintainer="dev@marienfressinaud.fr"

ENV RUSTFLAGS "-C target-feature=+crt-static"

RUN apk add --no-cache cargo rust

WORKDIR /code/
VOLUME /code/

ENTRYPOINT ["cargo"]
CMD ["build"]
