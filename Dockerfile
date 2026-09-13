FROM ubuntu:22.04
COPY ./target/release/s3-viewer ./target/release/s3-viewer
COPY ./wwwroot ./wwwroot
ENTRYPOINT ["./target/release/s3-viewer"]