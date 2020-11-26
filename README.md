# Proxy

A small rust project mainly to practice remote debugging and tracing. This is intended to figure out how rust ownership works in a live remote environment as well as locally 

## Makefile
To see what options there are run 
```bash
#> make 

cross                          Add dependencies needed for cross compilation
darwin                         Build a webserver debug binary on darwin
debug                          Create a docker container with a debug mode compiled binary and source code. Expose the binary via gdbserver on port 1234. Tag and push docker to registry
dev                            Cross compile a linux binary from darwin in debug mode
release                        Cross compile a linux binary in release mode (Takes longer)
run                            Build and run a docker container locally with debugger on port 1234
test                           Run cargo test on all integration tests

```


## Build a GDB enabled debug container and deploy to kubernetes
```bash
❯ make debug
```
This will build a linux binary with debug symbols, generate a docker container that holds a remote gdbdebugger and upload that container to   
a remote docker registry.  
To deploy the container to kubernetes run   
``` bash 
❯ kubectl apply -f kubernetes/manifests/
``` 
And there should we a webserver container and a webserver service available in the cluster.  


### Remote debugging
You need gnu debugger installed to use this. 
To debug a container running on kubernetes we can use gdb remote debugger. 
Here is an example of how to try that out
```bash
❯ cd tutorials/webserver/src
gdb
(gdb) set sysroot target:/
```
In another session forward the gdbserver port to localhost port 1234 
```bash
❯ kubectl port-forward svc/webserver-debug 1234
Forwarding from 127.0.0.1:1234 -> 1234
Forwarding from [::1]:1234 -> 1234
Handling connection for 1234
```

And then connect to the debug port via localhost in gdb
```bash
(gdb) target remote 127.0.0.1:1234
Remote debugging using 127.0.0.1:1234
Reading /app/webserver from remote target...
warning: File transfers from remote targets can be slow. Use "set sysroot" to access files locally instead.
Reading /app/webserver from remote target...
Reading symbols from target:/app/webserver...
0x00007ffff7d966e2 in _start ()
(gdb) list
1	mod server;
2	use server::Server;
3
4
5	fn main() {
6	    let server = Server::new("127.0.0.1:8080".to_string());
7	    server.run();
8	}
(gdb) b 6
Breakpoint 1 at 0x7ffff7d97d6b: file src/main.rs, line 6.
(gdb) n
Single stepping until exit from function _start,
which has no line number information.

Breakpoint 1, webserver::main () at src/main.rs:6
6	    let server = Server::new("127.0.0.1:8080".to_string());
(gdb) 
```

Use gdb commands such as 
```bash
❯ break src/server.rs 

❯ break main

❯ show

❯ list

❯ bt

❯ continue
```
Here are a few good examples https://visualgdb.com/gdbreference/commands/set_sysroot  
