package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func (s TileSet) Add(t Tile) {
	s[t] = struct{}{}
}

func main() {
	tileSet := findTiles("blockfont/blockfont.bdf", "digitfont/digitfont.bdf")
	tileBlock := makeTileBlock(tileSet)

	fmt.Printf("There are %d distinct tiles\n", len(tileBlock))
	for _, tile := range tileBlock {
		fmt.Printf("%#v:\n%s\n", tile, tile.Bitmap().DebugString())
	}
	if len(tileBlock) > 16 {
		log.Fatal("Too many tiles! Only 16 are allowed.")
	}

	err := tileBlock.Write("tiles.bin")
	if err != nil {
		log.Fatal(err)
	}

	err = writeBigDigitDefs("digitfont/digits.bin")
	if err != nil {
		log.Fatal(err)
	}
}

func findTiles(filenames ...string) TileSet {
	ret := make(TileSet)

	for _, fn := range filenames {
		font, err := loadFont(fn)
		if err != nil {
			panic(err)
		}

		for _, ch := range font {
			ts := ch.Bitmap.SizeTiles()
			for ty := 0; ty < ts[1]; ty++ {
				for tx := 0; tx < ts[0]; tx++ {
					tile := ch.Bitmap.Tile(tx, ty)

					ret.Add(tile)
				}
			}
		}
	}

	return ret
}

func makeTileBlock(s TileSet) TileBlock {
	ret := make(TileBlock, 0, len(s))
	for t := range s {
		ret = append(ret, t)
	}
	sort.Slice(ret, func(i, j int) bool {
		return ret[i] < ret[j]
	})
	return ret
}

type Tile uint16

const FullTile = Tile(0xffff)
const EmptyTile = Tile(0x0000)

func (t Tile) GoString() string {
	return fmt.Sprintf("Tile(0x%04x)", uint16(t))
}

func (t Tile) Bitmap() Bitmap {
	ret := make(Bitmap, 4)
	ret[0] = []byte{byte((t >> 0) & 0xf)}
	ret[1] = []byte{byte((t >> 4) & 0xf)}
	ret[2] = []byte{byte((t >> 8) & 0xf)}
	ret[3] = []byte{byte((t >> 12) & 0xf)}
	return ret
}

type TileSet map[Tile]struct{}

type TileBlock []Tile

func (b TileBlock) Index(t Tile) int {
	for i, ft := range b {
		if ft == t {
			return i
		}
	}
	return -1
}

func (b TileBlock) Write(fn string) error {
	tf, err := os.Create(fn)
	if err != nil {
		return err
	}
	defer tf.Close()

	for _, tile := range b {
		_, err = tf.Write([]byte{byte(tile), byte(tile >> 8)})
		if err != nil {
			return err
		}
	}

	return nil
}

type Bitmap [][]byte // even though these are bytes, each one only stores 4 bits

func (b Bitmap) IsSet(x, y int) bool {
	strip := b.tileStrip(x/4, y)
	return ((strip >> (3 - uint(x%4))) & 1) != 0
}

func (b Bitmap) Tile(tx, ty int) Tile {
	y := ty * 4
	return Tile(b.tileStrip(tx, y) | b.tileStrip(tx, y+1)<<4 | b.tileStrip(tx, y+2)<<8 | b.tileStrip(tx, y+3)<<12)
}

func (b Bitmap) tileStrip(tx, y int) int {
	return int(b[y][tx])
}

func (b Bitmap) SizePixels() [2]int {
	return [2]int{len(b[0]) * 4, len(b)}
}

func (b Bitmap) SizeTiles() [2]int {
	return [2]int{len(b[0]), len(b) / 4}
}

func (b Bitmap) DebugString() string {
	size := b.SizePixels()
	buf := make([]byte, 0, size[1]*(size[0]+1))
	for y := 0; y < size[1]; y++ {
		for x := 0; x < size[0]; x++ {
			if b.IsSet(x, y) {
				buf = append(buf, '#')
			} else {
				buf = append(buf, '-')
			}
		}
		buf = append(buf, '\n')
	}
	return string(buf)
}

type Font map[byte]Character

type Character struct {
	WidthTiles int
	Bitmap     Bitmap
}

func loadFont(fn string) (Font, error) {
	ret := make(Font)
	f, err := os.Open(fn)
	if err != nil {
		return nil, err
	}
	r := bufio.NewScanner(f)

	height := 0
	minWidthTiles := 0
	currentChNum := byte(0)
	padLeft := 0
	var emptyRow []byte
	var currentCh Character

	for r.Scan() {
		line := r.Text()
		space := strings.IndexByte(line, ' ')
		cmd := line
		var args []int
		if space >= 0 {
			cmd = line[:space]
			rawArgs := strings.Split(line[space+1:], " ")
			args = make([]int, len(rawArgs))
			for i, raw := range rawArgs {
				v, _ := strconv.Atoi(raw)
				args[i] = v // we only care about args that are numbers, so we ignore errors
			}
		}

		switch cmd {
		case "FONT_ASCENT":
			height = args[0]
		case "AVERAGE_WIDTH":
			minWidthTiles = args[0] / 4
		case "ENCODING":
			currentChNum = byte(args[0])
		case "BBX":
			w := (args[0] + args[2]) / 4
			x := args[2] / 4
			y := args[3]
			if w < minWidthTiles {
				w = minWidthTiles
			}
			padLeft = x
			emptyRow = make([]byte, w)
			currentCh.Bitmap = make([][]byte, 0, height)
			for i := 0; i < y; i++ { // Pre-pad the top
				currentCh.Bitmap = append(currentCh.Bitmap, emptyRow)
			}
			currentCh.WidthTiles = w
		case "BITMAP":
			for r.Scan() {
				l := r.Text()
				if l == "ENDCHAR" {
					break
				}

				// A sequence of nibbles encoded as hex digits, then.
				row := make([]byte, padLeft, currentCh.WidthTiles)
				for _, nh := range l {
					var n byte
					switch {
					case nh >= '0' && nh <= '9':
						n = byte(nh - '0')
					case nh >= 'A' && nh <= 'F':
						n = byte(nh - 'A' + 10)
					case nh >= 'a' && nh <= 'f':
						n = byte(nh - 'a' + 10)
					}
					row = append(row, n)
				}
				row = row[:currentCh.WidthTiles] // pad right to the expected width
				currentCh.Bitmap = append(currentCh.Bitmap, row)
			}
			for i := len(currentCh.Bitmap); i < cap(currentCh.Bitmap); i++ {
				currentCh.Bitmap = append(currentCh.Bitmap, emptyRow)
			}
			ret[currentChNum] = currentCh
			currentCh = Character{}
			currentChNum = 0
		}
	}

	return ret, nil
}

func writeBigDigitDefs(fn string) error {
	tf, err := os.Create(fn)
	if err != nil {
		return err
	}
	defer tf.Close()

	const (
		A uint16 = 1 << 0
		B uint16 = 1 << 1
		C uint16 = 1 << 2
		D uint16 = 1 << 3
		E uint16 = 1 << 4
		F uint16 = 1 << 5
		G uint16 = 1 << 6

		AB uint16 = 1 << 8
		BC uint16 = 1 << 9
		CD uint16 = 1 << 10
		DE uint16 = 1 << 11
		EF uint16 = 1 << 12
		FA uint16 = 1 << 13
	)

	/*
		      A
		FA .-----. AB
		   |     |
		 F |     | B
		   |  G  |
		EF >-----< BC
		   |     |
		 E |     | C
		   |     |
		   '-----'
		DE    D    CD

		The two-letter symbols representing segment intersections are set to
		1 to indicate that the intersection should be "curved". How exactly it
		will be curved depends on which of the surrounding segments are also
		lit. This encoding therefore doesn't encode _all_ of the details
		required for rendering; some further logic is required to select
		suitable symbols at the intersections, and to draw the special serif
		on the digit 1 that isn't directly represented here.
		The "curve" flag is meaningful only if at least two segments arrive
		at a particular intersection; if not, it should always be zero.
	*/

	defs := []uint16{
		0x0: A | B | C | D | E | F | AB | CD | DE | FA,
		0x1: B | C,
		0x2: A | B | G | E | D | AB | BC | EF | DE,
		0x3: A | B | C | D | G | AB | BC | CD,
		0x4: F | G | B | C | EF,
		0x5: A | F | G | C | D | BC | CD,
		0x6: A | F | G | C | D | E | BC | CD | DE | FA,
		0x7: A | B | C,
		0x8: A | B | C | D | E | F | G | AB | BC | CD | DE | EF | FA,
		0x9: A | B | C | D | F | G | AB | CD | EF | FA,
		0xA: A | B | C | E | F | G | AB | FA,
		0xB: A | B | C | D | E | F | G | AB | BC | CD,
		0xC: A | F | E | D | FA | DE,
		0xD: A | B | C | D | E | F | AB | CD,
		0xE: A | F | G | E | D,
		0xF: A | F | G | E,
	}

	for _, dig := range defs {
		_, err = tf.Write([]byte{byte(dig), byte(dig >> 8)})
		if err != nil {
			return err
		}
	}

	return nil
}
