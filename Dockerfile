FROM --platform=x86_64 fedora as base

RUN dnf group install -y "Development Tools" 

RUN dnf install -y git wget cmake libxml2-devel openssl-devel clang mingw64-gcc xz

RUN mkdir /osxcross

WORKDIR /osxcross

RUN git clone https://github.com/tpoechtrager/osxcross .

RUN wget -nc https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX11.3.sdk.tar.xz \
  && mv MacOSX11.3.sdk.tar.xz tarballs/

RUN UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh

RUN groupadd -g 1001 build-group
RUN useradd -u 1001 -g 1001 build-user

RUN mkdir /polymath

RUN chown 1001:1001 /polymath

USER 1001:1001

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y 

WORKDIR /polymath

ADD entrypoint.sh /opt/entrypoint.sh

ENTRYPOINT [ "/opt/entrypoint.sh" ]
