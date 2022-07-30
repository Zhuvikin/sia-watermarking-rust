FROM rust:1.62.1

RUN cargo install wasm-pack

RUN apt-get update && apt-get install -y curl

# nvm environment variables
ENV NODE_VERSION 18.6.0
ENV NPM_VERSION 8.13.2

# install node and npm
# https://github.com/nvm-sh/nvm#install-script
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash && \
    export NVM_DIR="$HOME/.nvm" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \
    \. "$NVM_DIR/nvm.sh" && \
    nvm install $NODE_VERSION && \
    nvm alias default $NODE_VERSION && \
    nvm use default && \
    npm install -g npm@$NPM_VERSION
