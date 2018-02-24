

PORT='/dev/ttyUSB0'

import serial

fd = serial.Serial(port=PORT, baudrate=9600)

while True:
    print ord(fd.read())
