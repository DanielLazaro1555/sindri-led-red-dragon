#!/usr/bin/env python3
# K671 (258a:013b): deja solo las teclas azules encendidas (efecto Custom).
# Protocolo sacado de OemDrv.exe: reporte feature 0x0A (efecto) y 0x0B (teclas encendidas).
import fcntl, os, time

def IOC(n, nr): return (3 << 30) | (n << 16) | (0x48 << 8) | nr
GET = lambda n: IOC(n, 0x07)   # HIDIOCGFEATURE
SET = lambda n: IOC(n, 0x06)   # HIDIOCSFEATURE

# Posiciones del reporte 0x0B que corresponden a la franja azul del teclado
# (F10 F11 F12 = Backspace ] \ Enter RShift RCtrl), sin la tecla "-" junto al Shift.
LIT = [66, 72, 73, 74, 76, 77, 78, 79, 80, 81]

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
print(f"{len(LIT)} teclas azules encendidas")
