FROM nixos/nix:2.26.2 AS builder

# Update Nix channels and enable experimental features for flakes
RUN nix-channel --update
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

WORKDIR /kfs-src

COPY flake.nix flake.nix
COPY flake.lock flake.lock

RUN nix develop --command rustup install nightly-x86_64-unknown-linux-gnu
RUN nix develop --command rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

CMD [ "nix", "develop", "--command", "./make_iso.sh" ]
