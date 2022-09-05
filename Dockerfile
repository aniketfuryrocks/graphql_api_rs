FROM ghcr.io/aniketfuryrocks/fedora_rust_dev_container as init 
WORKDIR /app


# INFO: dev 
FROM init as dev 

CMD cargo watch -x run --features "cors"


# INFO: prod 
FROM init as prod 

ADD . .
RUN cargo install --path .
RUN rm -rf ./*
RUN which graphql_api_rs 
CMD graphql_api_rs 

