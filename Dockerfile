FROM ubuntu:20.04

RUN apt-get update && apt-get dist-upgrade -y && DEBIAN_FRONTEND=noninteractive apt-get install -y curl clang libclang-dev libopencv-dev gdb vim

RUN echo "set disassembly-flavor intel" > /root/.gdbinit && \
    echo "set breakpoint pending on" >> /root/.gdbinit && \
    echo "break simple_struct_return_infallible" >> /root/.gdbinit

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN /root/.cargo/bin/cargo search opencv --limit=1
