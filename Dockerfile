FROM scorpil/rust:beta

RUN cargo install cargo-watch

VOLUME /project
WORKSPACE /project

CMD [ "cargo", "build", "/project" ]
