FROM centos:centos8

RUN curl -o /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-8.repo
RUN sed -i -e"s|mirrors.cloud.aliyuncs.com|mirrors.aliyun.com|g " /etc/yum.repos.d/CentOS-*
RUN sed -i -e "s|releasever|releasever-stream|g" /etc/yum.repos.d/CentOS-*
RUN yum clean all && yum makecache

RUN dnf module list nodejs
RUN dnf module -y enable nodejs:16
RUN dnf module -y install nodejs:16/common

COPY package.json package.json
RUN npm --version
RUN npm install

COPY _build/sir /sir
COPY ir_example/hello_world.ir /sir/example/hello_world.ir

RUN chmod +x /sir/bin/ir_cli

ENV PATH="/sir/bin:${PATH}"
ENV LANG=en_US.utf8
