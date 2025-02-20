# slowcat

Demo TCP client

In one terminal session - start `tcpstates` from [bcc-tools](https://github.com/iovisor/bcc):

```console
% sudo /usr/share/bcc/tools/tcpstates -4 -L 8080 -tT
```

In another terminal session - build and run this demo server:

```console
% cargo run --example server
socket()
bind()
listen()
accept()
read()
[rx_len=4] <- PING
write()
[tx_len=4] -> PONG
close()
close()
```

In another terminal session - run netcat client

```console
% cargo run --example client
socket()
connect()
write()
[tx_len=4] -> PING
read()
[rx_len=4] <- PONG
close()
```

You should see the TCP states as they change in the first terminal session:

```console
% sudo /usr/share/bcc/tools/tcpstates -4 -L 8080 -tT
TIME(s)   SKADDR           C-PID C-COMM     LADDR           LPORT RADDR           RPORT OLDSTATE    -> NEWSTATE    MS
0.000000  ffff9b7900eea600 120835 server     127.0.0.1       8080  0.0.0.0         0     CLOSE       -> LISTEN      0.000
2.234027  ffff9b7610712f80 120862 client     127.0.0.1       8080  0.0.0.0         0     LISTEN      -> SYN_RECV    0.000
2.234035  ffff9b7610712f80 120862 client     127.0.0.1       8080  127.0.0.1       56778 SYN_RECV    -> ESTABLISHED 0.004
2.234080  ffff9b7610712f80 120835 server     127.0.0.1       8080  127.0.0.1       56778 ESTABLISHED -> FIN_WAIT1   0.044
2.234091  ffff9b7610712f80 120835 server     127.0.0.1       8080  127.0.0.1       56778 FIN_WAIT1   -> CLOSING     0.009
2.234094  ffff9b7610712f80 120835 server     127.0.0.1       8080  127.0.0.1       56778 CLOSING     -> CLOSE       0.001
2.234100  ffff9b7900eea600 120835 server     127.0.0.1       8080  0.0.0.0         0     LISTEN      -> CLOSE       2234.095

```