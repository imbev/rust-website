FROM quay.io/almalinuxorg/almalinux:10-kitten

WORKDIR /workdir

RUN dnf install -y rust-toolset

COPY . .

RUN cargo build --release

# ---

FROM quay.io/almalinuxorg/10-kitten-base:10-kitten

WORKDIR /app

COPY --from=0 /workdir/static static/
COPY --from=0 /workdir/templates templates/
COPY --from=0 /workdir/target/release/rust-website .

RUN adduser --no-create-home app
RUN chown -R app:app /app
USER app

CMD [ "/app/rust-website" ]
