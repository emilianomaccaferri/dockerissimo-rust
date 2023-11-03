FROM rust:1.73
RUN useradd --user-group --system --create-home --no-log-init poglo
WORKDIR /code
RUN chown poglo:poglo -R /code
USER poglo
COPY --chown=poglo:poglo ./src /code/src
COPY --chown=poglo:poglo ./Cargo.toml /code/Cargo.toml
RUN cargo install cargo-watch
RUN cargo build
# ENTRYPOINT ["./wait-for", "db:", "--", "nodemon"] # for dev purposes only