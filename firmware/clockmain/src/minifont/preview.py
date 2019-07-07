import os.path
import struct
import sys

descs_b = open(os.path.join(
    os.path.dirname(__file__), "descriptors.bin",
), "rb").read()
cmds_b = open(os.path.join(
    os.path.dirname(__file__), "commands.bin",
), "rb").read()

descs = [
    struct.unpack('>HH', descs_b[i:i+4]) for i in xrange(0, len(descs_b), 4)
]
cmds = [
    struct.unpack('>B', cmds_b[i])[0] for i in xrange(0, len(cmds_b))
]

for ch, desc in enumerate(descs):
    print "### Character %d (commands %d-%d)\n" % (ch, desc[0], desc[1])

    px = []
    for y in xrange(5):
        px.append([False] * 6)
    x = 0
    y = 0

    for cmd in (cmds[i] for i in xrange(desc[0], desc[1])):
        width = cmd & 0xf
        height = cmd >> 4
        advance = height
        if width == 0:
            print "- Skip %d rows" % height
        else:
            print "- Fill rect of size (%d, %d)" % (width, height)
            for yf in xrange(y, y + height):
                for xf in xrange(x, x + width):
                    px[yf][xf] = True

        for n in xrange(advance):
            y += 1
            if y == 5:
                y = 0
                x += 1

    print ""

    width = x  # After we're finished, our cursor sitting just to the right of the character box
    for row in px:
        for col in row[:width]:
            sys.stdout.write("O" if col else ".")
        sys.stdout.write("\n")

    print ""
