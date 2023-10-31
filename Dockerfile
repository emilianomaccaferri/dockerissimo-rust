FROM rust:1.74
RUN addgroup poglo && adduser -D -S -G poglo poglo
WORKDIR /code
RUN chown poglo /code

USER poglo
COPY --chown=poglo:poglo ./src /code/src
COPY --chown=poglo:poglo ./Cargo.toml /code/Cargo.toml
RUN cargo build
ENTRYPOINT [ "cargo", "run" ]
# ENTRYPOINT ["./wait-for", "db:", "--", "nodemon"] # for dev purposes only