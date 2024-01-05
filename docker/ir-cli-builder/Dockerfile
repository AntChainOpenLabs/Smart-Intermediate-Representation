# Copyright (c) The Ant Group Core Contributors
# Copyright (c) The Smart Intermediate Representation Contributors
# SPDX-License-Identifier: Apache-2.0

# this dockerfile requires on a cargo_config(line 91) and a demo_to_download_deps.Cargo.toml(refered at line 94) file exsiting at current directory

FROM centos:centos8

# macOS M1 --platform linux/amd64
# try fix "Problem with the SSL CA cert (path? access rights?)"
# https://issueexplorer.com/issue/docker/for-mac/5912
# https://access.redhat.com/articles/2050743
RUN touch /etc/sysconfig/64bit_strstr_via_64bit_strstr_sse2_unaligned

# https://forketyfork.medium.com/centos-8-no-urls-in-mirrorlist-error-3f87c3466faa
RUN sed -i -e "s|mirrorlist=|#mirrorlist=|g" /etc/yum.repos.d/CentOS-*
RUN sed -i -e "s|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g" /etc/yum.repos.d/CentOS-*

RUN yum -y install make
RUN yum -y install which
RUN yum -y install wget
RUN yum -y install git

# ca-certificates
RUN yum -y install ca-certificates

# rust-1.70.0
# cargo
# RUN yum -y install rust cargo rustfmt
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN rustup default 1.70

# RUN rustup default stable
RUN cargo version
RUN rustc --version

# cmake
RUN yum install -y cmake

RUN yum install -y bison


#RUN yum install -y centos-release-scl
#RUN yum install -y devtoolset-11
#RUN echo "source /opt/rh/devtoolset-11/enable" >> ~/.bash_profile 
#RUN source /opt/rh/devtoolset-11/enable
RUN yum install -y gcc-c++
RUN yum install -y gcc-toolset-11 gcc-toolset-11-gcc-c++ gcc-toolset-11-make gcc-toolset-11-libasan-devel gcc-toolset-11-gdb gcc-toolset-11-binutils
RUN mkdir -p /opt
WORKDIR /opt
# COPY linux-x64/clang+llvm-14.0.6-x86_64-linux-gnu-rhel-8.4.tar.xz /opt/
RUN wget https://github.com/llvm/llvm-project/releases/download/llvmorg-15.0.0/clang+llvm-15.0.0-x86_64-linux-gnu-rhel-8.4.tar.xz
RUN tar -xvf clang+llvm-15.0.0-x86_64-linux-gnu-rhel-8.4.tar.xz
RUN rm -rf clang+llvm-15.0.0-x86_64-linux-gnu-rhel-8.4.tar.xz
RUN mv clang+llvm-15.0.0-x86_64-linux-gnu-rhel-8.4/ llvm15
WORKDIR /opt/llvm15
RUN echo "PATH=/opt/llvm15/bin:\$PATH" >> ~/.bash_profile
RUN source ~/.bash_profile
ENV PATH=/opt/llvm15/bin:$PATH
ENV LLVM_SYS_150_PREFIX=/opt/llvm15


# clang-15
# RUN yum -y install clang
# RUN clang --version

# llvm-15
RUN yum -y install libffi-devel
RUN yum install -y lld lld-devel zlib-devel
RUN ln -s /usr/lib64/libtinfo.so.6 /usr/lib64/libtinfo.so

RUN yum install -y python3 python3-devel

#RUN mkdir -p /mnt/lld_extract_tmp
#COPY linux-x64/liblld-static-14.0.6.tar.gz /mnt/lld_extract_tmp/
#WORKDIR /mnt/lld_extract_tmp
#RUN tar -xzf liblld-static-14.0.6.tar.gz
#RUN cp /mnt/lld_extract_tmp/*.a /lib64/

RUN ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN echo 'Asia/Shanghai' >/etc/timezone

RUN mkdir -p /root/.cargo && touch /root/.cargo/env

RUN mkdir /tmp/cargo_dep
WORKDIR /tmp/cargo_dep
COPY cargo_config /tmp/cargo_config
RUN cp -rf /tmp/cargo_config ~/.cargo/config

COPY Cargo.toml /tmp/cargo_dep/Cargo.toml
RUN cargo update

# RUN rustup component add llvm-tools-preview
RUN cargo install cargo-llvm-cov
RUN rustup component add llvm-tools-preview

WORKDIR /root

CMD ["bash"]

