FROM ubuntu:18.04
ENV RELEASE=release
COPY target/x86_64-unknown-linux-musl/${RELEASE}/webserver /app/webserver
CMD /app/webserver

