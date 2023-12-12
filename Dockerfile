FROM centos:centos8
COPY _build/sir /sir
COPY ir_example/hello_world.ir /sir/example/hello_world.ir

RUN chmod +x /sir/bin/ir_cli

ENV PATH="/sir/bin:${PATH}"
ENV LANG=en_US.utf8
