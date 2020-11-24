# Webservice
## Makefile
 
```bash
make help
```


## Build and deploy to kubernetes
```bash
make debug
```
make debug  


## Remote debugging
You need gnu debugger installed to use this. 
To debug a container running on kubernetes we can use gdb remote debugger. 
Here is an example of how to try that out
```bash
cd tutorials/webserver/src
gdb
(gdb) set sysroot target:/
```

```bash

```

```bash
❯ k port-forward svc/webserver-debug 1234
Forwarding from 127.0.0.1:1234 -> 1234
Forwarding from [::1]:1234 -> 1234
Handling connection for 1234
```

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
break src/server.rs 

break main

show

list

bt

continue
```
Here are a few good examples https://visualgdb.com/gdbreference/commands/set_sysroot  
