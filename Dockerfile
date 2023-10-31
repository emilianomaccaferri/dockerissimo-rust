FROM rust:1.73
RUN useradd --user-group --system --create-home --no-log-init poglo
WORKDIR /code
RUN chown poglo:poglo -R /code
USER poglo
COPY --chown=poglo:poglo ./src /code/src
COPY --chown=poglo:poglo ./Cargo.toml /code/Cargo.toml
RUN cargo build
ENTRYPOINT ["cargo", "run", "--", "--conf-path=2", "--templates-path=2"]
# ENTRYPOINT ["./wait-for", "db:", "--", "nodemon"] # for dev purposes only