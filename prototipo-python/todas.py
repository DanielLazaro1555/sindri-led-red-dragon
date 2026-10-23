#!/usr/bin/env python3
# K671 (258a:013b): enciende todas las teclas (efecto Custom).
# Protocolo sacado de OemDrv.exe: reporte feature 0x0A (efecto) y 0x0B (teclas encendidas).
import fcntl, os, time

def IOC(n, nr): return (3 << 30) | (n << 16) | (0x48 << 8) | nr
GET = lambda n: IOC(n, 0x07)   # HIDIOCGFEATURE
SET = lambda n: IOC(n, 0x06)   # HIDIOCSFEATURE

# Todas las teclas; el color lo pone el LED de cada zona (arcoiris).
LIT = list(range(1, 125))   # todas las posiciones del reporte 0x0B

fd = os.open("/dev/hidraw6", os.O_RDWR)
a = bytearray(42); a[0] = 0x0A
fcntl.ioctl(fd, GET(42), a)
a[1] = 12                                    # efecto 12 = Custom
fcntl.ioctl(fd, SET(42), a); time.sleep(0.3)

c = bytearray(127); c[0] = 0x0B; c[1] = 1    # c[1] = grupo de color
for idx in LIT: c[1 + idx] = 7
for _ in range(2):
    fcntl.ioctl(fd, SET(127), c); time.sleep(0.5)
os.close(fd)
print(f"{len(LIT)} teclas encendidas")
