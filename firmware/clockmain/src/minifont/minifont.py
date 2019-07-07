import sys
import os.path
import struct


def custom_character_code(ch):
    if ch >= 0x41 and ch <= 0x5a:
        return ch - 0x41
    if ch >= 0x30 and ch <= 0x39:
        return ch - 0x30 + 26
    if ch >= 0x20 and ch <= 0x21:
        return ch - 0x20 + 26 + 10
    if ch >= 0x7b:
        return ch - 0x7b + 26 + 10 + 2


descriptors = []
for i in xrange(26 + 10 + 2 + 5):
    descriptors.append(None)


print "There are %d characters in our character set" % len(descriptors)


cmds = []


def main():
    current_ch = None
    bitmap = None
    width = None
    shift = 0
    for l in file(os.path.join(os.path.dirname(__file__), "minifont.bdf")):
        l = l.strip()
        if l.startswith("ENCODING "):
            current_ch = int(l[9:])
        if l.startswith("DWIDTH "):
            width = int(l[7:8])
        if l.startswith("BBX "):
            parts = l[4:].split(" ")
            shift = int(parts[2])
        elif l == "BITMAP":
            bitmap = []
        elif l == "ENDCHAR":
            process_char(current_ch, width, bitmap)
            bitmap = None
            current_ch = None
            width = None
            shift = 0
        elif bitmap is not None:
            # We're expecting a hex representation of the character, then.
            v = int(l, 16) >> shift
            bitmap.append(v)

    print "After analysis we have %d commands total" % len(cmds)

    write_files()


def process_char(ascii_code, width, bitmap):
    global descriptors, cmds

    custom_code = custom_character_code(ascii_code)

    # Our replacement character at ASCII 127 is just a full square, so we'll
    # special case it here and thus make it take up only three command positions,
    # since our algorithm below isn't designed to deal with big rectangles.
    if ascii_code == 127:
        cmds.append((5, 5))  # Fill 5x5 rectangle (and consume 5 rows)
        # Consume as many pixels as we can fit in one command.
        cmds.append((0, 15))
        cmds.append((0, 5))  # Now consume the ones that are left.
        descriptors[custom_code] = (len(cmds)-3, len(cmds))
        return

    for i in xrange(len(bitmap), 5):
        bitmap.append(0)

    print "=== Analyzing character %d ===" % ascii_code
    print "character is %s" % chr(ascii_code)
    print "custom code is %d" % custom_code
    used = same_size_bitmap(bitmap)
    for y in xrange(0, 5):
        for x in xrange(0, width):
            if bitmap_has_bit(bitmap, x, y):
                sys.stdout.write("#")
            else:
                sys.stdout.write(".")
        print ""

    start_cmd_idx = len(cmds)

    x = 0
    y = 0
    skip = 0
    height = 5
    while x < width:
        on = bitmap_has_bit(bitmap, x, y)
        already = bitmap_has_bit(used, x, y)
        advance = 1
        if on and not already:
            while skip > 0:
                # Insert a skip command (width=0) for the number of positions
                # we skipped.
                to_skip = skip
                if to_skip > 15:
                    to_skip = 15  # Our encoding format only allows us to skip 15 at a time
                cmds.append((0, to_skip))
                print "- skip %d rows" % skip
                skip -= to_skip

            # We will now choose whether we're going to prefer to fill
            # downwards or to fill to the right, by seeing which has the
            # most pixels for us to fill in.
            fill_right = 1
            fill_down = 1
            for x2 in xrange(x + 1, width):
                if bitmap_has_bit(used, x2, y) or not bitmap_has_bit(bitmap, x2, y):
                    break
                if y > 0 and x2 > x and bitmap_has_bit(bitmap, x2, y - 1):
                    # If we find a filled pixel above our span here then we'll
                    # stop and allow a future span to fill into it downwards.
                    # This is kinda arbitrary but is consistent with our
                    # general preference for filling spans downwards, because
                    # it requires less skipping then.
                    break
                fill_right += 1
            for y2 in xrange(y + 1, height):
                if bitmap_has_bit(used, x, y2) or not bitmap_has_bit(bitmap, x, y2):
                    break
                fill_down += 1

            cmd = None
            # We prefer to fill down if both are equal, because we are less
            # likely to need to skip over remaining columns to complete the
            # glyph that way.
            if fill_right > fill_down:
                cmd = (fill_right, 1)
                for x2 in range(x, x+fill_right):
                    bitmap_set_bit(used, x2, y)
            else:
                cmd = (1, fill_down)
                for y2 in range(y, y+fill_down):
                    bitmap_set_bit(used, x, y2)

            cmds.append(cmd)
            advance = cmd[1]
            print "- draw rectangle of size (%d, %d)" % cmd

        else:
            skip += 1

        y += advance
        if y >= height:
            y = y % height
            # Note that advance can never be greater than height because
            # that would suggest we had a fill command that fills below
            # the character cell, and that makes no sense.
            x += 1

    # We might have some blanks at the end, so we'll need an explicit skip
    # for those if so.
    while skip > 0:
        # Insert a skip command (width=0) for the number of positions
        # we skipped.
        to_skip = skip
        if to_skip > 15:
            to_skip = 15  # Our encoding format only allows us to skip 15 at a time
        cmds.append((0, to_skip))
        print "- skip %d rows" % skip
        skip -= to_skip

    end_cmd_index = len(cmds)
    descriptors[custom_code] = (start_cmd_idx, end_cmd_index)


def write_files():
    global descriptors, cmds

    descs_f = open(os.path.join(
        os.path.dirname(__file__), "descriptors.bin"), "wb")
    cmds_f = open(os.path.join(
        os.path.dirname(__file__), "commands.bin"), "wb")

    for cmd in cmds:
        packed = (cmd[0] & 0xf) | ((cmd[1] & 0xf) << 4)
        cmds_f.write(struct.pack(">B", packed))

    for (i, desc) in enumerate(descriptors):
        descs_f.write(struct.pack(">HH", desc[0], desc[1]))


def bitmap_has_bit(bitmap, x, y):
    return (bitmap[y] & (1 << (7 - x))) != 0


def bitmap_set_bit(bitmap, x, y):
    bitmap[y] |= (1 << (7 - x))


def same_size_bitmap(bitmap):
    new = []
    for r in bitmap:
        new.append(0)
    return new


main()
