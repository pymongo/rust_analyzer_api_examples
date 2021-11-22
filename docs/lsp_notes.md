## 术语定义和术语缩写

- LSP = Language Server Protocol, 常用于 IDE 静态分析客户端服务端的通信协议
- RA = rust analyzer

## LSP 通信的信道

通信的数据格式是 JSON, 通信信道是 管道/pipe，当然也能用 socket

只不过 LSP 的客户端和服务端一般都在同一台机器上用 pipe 通信性能会比 socket 好

## LSP 客户端和服务端建立通信的过程

首先服务端的可执行文件必须事先编译或下载到磁盘中，

客户端(一般是各种 IDE 例如 vscode)开一个子进程 exec() 系统调用运行服务端可执行文件

然后客户端和服务端之间建立两个管道进行全双工通信(因为操作系统的管道是单向的，所以要建立两个管道)

简单来说客户端和服务端之间就是通过 STDIN 通信 就跟 redis-cli 这种命令行软件跟服务端的通信差不多

发请求时客户端通过管道往 RA 子进程的 STDIN 发字符串格式的 JSON 请求

然后 RA 服务端处理完后直接写到 STDOUT 往客户端发响应(客户端能通过另一个管道读取到响应数据)

所以 LSP 的 initialize 请求会带上 pid (也就是客户端的 process id)

## 如何监听两个管道的 IO 事件

我建议的做法是 poll/select/epoll 这种 **IO 多路复用** 系统调用让主线程读写两个管道

我个人喜欢用 select() 系统调用，select() 应对这种监听两个 fd 的 IO 事件比较简单

由于 glibc 的 select 跟 Rust 标准库的读写 API 之间交互不太方便

所以 rust-analyzer/lsp-server spawn 两个线程一个监听读(客户端请求)另一个监听写(服务端返回数据给客户端)

然后再通过 crossbeam 的 channel 在多个线程间通信

## RA 与 vscode 之间的日志原理

vscode 的 output 界面中:
- Rust Analyzer Client: 是 typescript 写的 vscode 插件，只会打印配置和版本信息，完全不用看
- Rust Analyzer Language Server Trace: 不开 RA 的 trace 配置的话不打印任何信息
- Rust Analyzer Language Server: 打印 RA 进程的 STDERR

由于 rust-analyzer/lsp-server 把自身的 STDIN/STDOUT 给 lock() 到 IoThread

所以在 RA 源码中加上 println!() 不会有任何效果!

### STDERR in LSP

我用的是 eprintln! 和 dbg! 这两个输出到 STDERR 的宏记录请求

另外补充知识说明下，Rust 的 STDOUT 有内置 buffer 所以 print! 不 flush 的话可能要等够 4096 个字节再打印一次

如果是 eprintln! dbg! 这种好像 Rust 的 STDERR 类都没 buffer 直接打出去

## LSP 的消息类型

分为 Request/Response/Notification 三种，三者的 JSON 格式都不一样

注意 Request 并不是只有 client 向 server 发，server 也会给 client 发 request 获取客户端信息

client 发的 notification 例如修改了哪个文件的第几行

server 发的 notification 例如 Indexing 12/100 当前扫描文件的进度


