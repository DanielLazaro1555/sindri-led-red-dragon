import fcntl, os, time
def IOC(n, nr): return (3 << 30) | (n << 16) | (0x48 << 8) | nr
GET = lambda n: IOC(n, 0x07); SET = lambda n: IOC(n, 0x06)
orig = bytearray.fromhex(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "led_report_original.hex")).read())
fd = os.open("/dev/hidraw6", os.O_RDWR)
fcntl.ioctl(fd, SET(42), orig); time.sleep(0.3)
b = bytearray(42); b[0] = 0x0A; fcntl.ioctl(fd, GET(42), b)
print("restaurado:", b[:6].hex(" "), "IGUAL" if b == orig else "DIFERENTE")
os.close(fd)
