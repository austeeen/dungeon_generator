import sys
import random
import pygame
import queue
from dataclasses import dataclass

def i2c(i: int, width: int = 0) -> tuple[int, int]:
    if not width:
        width = Map.WIDTH
    return i % width, int(i / width)


class Tileset:
    TILE_SIZE = 8
    FILE = "tileset.png"
    WIDTH = 4
    SURFACE = pygame.image.load(FILE)
    RECT = SURFACE.get_rect()

    @classmethod
    def i2r(cls, i: int, size: int = 0, width: int = 0) -> 'pygame.Rect':
        if not size:
            size = cls.TILE_SIZE
        if not width:
            width = cls.WIDTH

        x, y = i2c(i, width=width)
        return pygame.Rect(x * size, y * size, size, size)


TOTAL_TILE_TYPES = 16
START_CHAR_INDEX = 15
EMPTY_CHAR_INDEX = 0



TILE_TYPES = [
    ' ', # 0000
    '╨', # 0001
    '╞', # 0010
    '╚', # 0011

    '╥', # 0100
    '║', # 0101
    '╔', # 0110
    '╠', # 0111

    '╡', # 1000
    '╝', # 1001
    '═', # 1010
    '╩', # 1011

    '╗', # 1100
    '╣', # 1101 
    '╦', # 1110
    '╬', # 1111
]


@dataclass
class TileType:
    char_index: int
    char: str
    weight: int


WEIGHTED_TILE_TYPES: list[TileType] = [
    TileType(0, ' ', 1), # 0000
    TileType(1, '╨', 16), # 0001
    TileType(2, '╞', 16), # 0010
    TileType(3, '╚', 12), # 0011

    TileType(4, '╥', 16), # 0100
    TileType(5, '║', 12), # 0101
    TileType(6, '╔', 12), # 0110
    TileType(7, '╠', 6), # 0111

    TileType(8,  '╡', 16), # 1000
    TileType(9,  '╝', 12), # 1001
    TileType(10, '═', 12), # 1010
    TileType(11, '╩', 6), # 1011

    TileType(12, '╗', 12), # 1100
    TileType(13, '╣', 6), # 1101 
    TileType(14, '╦', 6), # 1110
    TileType(15, '╬', 2), # 1111
]
    

class Neighbors:

    def __init__(self, index: int):    
        self.top = index - Map.WIDTH
        self.bottom = index + Map.WIDTH
        self.left = index - 1
        self.right = index + 1

    def __str__(self) -> str:
        return f"[{self.top},{self.left},{self.bottom},{self.right}]"


def print_separator(n: int = 0, c: str = "-"):
    if not n:
        n = Map.WIDTH
    print(f"+{c * n}+")

    
def bit_on(mask: int, bit: int) -> bool:
    return (bit & mask) == bit


def weighted_tiles(bit: int) -> list[TileType]:
    return [
        tile_type for tile_type in WEIGHTED_TILE_TYPES
        if bit_on(tile_type.char_index, bit)
    ]


def has_top_connection(tile_index: int):
    return bit_on(tile_index, 1)


def has_right_connection(tile_index: int):
    return bit_on(tile_index, 2)


def has_bottom_connection(tile_index: int):
    return bit_on(tile_index, 4)


def has_left_connection(tile_index: int):
    return bit_on(tile_index, 8)

def filter_connection(has_connection, tile_vec: list[TileType]) -> list[TileType]:
    return list(filter(lambda t: has_connection(t.char_index), tile_vec))

def filter_no_connection(has_connection, tile_vec: list[TileType]) -> list[TileType]:
    return list(filter(lambda t: not has_connection(t.char_index), tile_vec))


class Tile:

    def __init__(self, index: int, tile_type: TileType):
        self._index = index
        self.tile_type = tile_type
        self.rect = Tileset.i2r(self.index, width=Map.WIDTH)
        self.texture_rect = Tileset.i2r(self.tile_type.char_index)

    @property
    def index(self) -> int:
        return self._index
    
    @index.setter
    def index(self, i: int):
        self._index = i
        self.rect = Tileset.i2r(self.index, width=Map.WIDTH)

    @property
    def char(self) -> str:
        return self.tile_type.char
    
    @property
    def char_index(self) -> int:
        return self.tile_type.char_index
    
    @char_index.setter
    def char_index(self, index: int):
        self.tile_type = WEIGHTED_TILE_TYPES[index]
        self.texture_rect = Tileset.i2r(self.tile_type.char_index)

    def render(self, surf: 'pygame.Surface'):
        surf.blit(Tileset.SURFACE, self.rect, self.texture_rect)


class Map:
    WIDTH = 20
    TOTAL_SIZE = WIDTH * WIDTH
    MIN_SIZE = 35
    RECT = pygame.Rect(0, 0, Tileset.TILE_SIZE * WIDTH, Tileset.TILE_SIZE * WIDTH)

    def __init__(self):
        self._next_tile_queue = queue.Queue()
        self.tiles: list[Tile] = []
        self.bottom_tiles = weighted_tiles(1)
        self.left_tiles = weighted_tiles(2)
        self.top_tiles = weighted_tiles(4)
        self.right_tiles = weighted_tiles(8)
        self.surface = pygame.Surface(self.RECT.size, flags=pygame.SRCALPHA)
        self._generating_tiles = False

    @property
    def map_size(self) -> int:
        return len([t for t in self.tiles if t.char_index != EMPTY_CHAR_INDEX])

    def update(self):
        if not self._next_tile_queue.empty():
            self._generating_tiles = True
            self._generate_tiles(self._next_tile_queue.get(),
                                 use_queue=True)
        elif self._generating_tiles:
            self._generating_tiles = False
            if not self.good_map_size():
                self.generate()

    def render(self, surf: 'pygame.Surface'):
        for t in self.tiles:
            t.render(self.surface)
        # surf.blit(self.surface, (0, 0))
        surf.blit(pygame.transform.scale(self.surface, surf.get_rect().size), (0, 0))
    
    def print(self):
        print(f"MAP {self.WIDTH}x{self.WIDTH} has len {len(self.tiles)}")
        print_separator()
        row = "|"
        for tile in self.tiles:
            row += tile.char
            if (tile.index % self.WIDTH) == (self.WIDTH - 1):
                row += "|"
                print(row)
                row = ""
                if tile.index < (self.TOTAL_SIZE - 1):
                    row += "|"
        print_separator()
    
    def randomize(self):
        self.tiles = [
            Tile(i, WEIGHTED_TILE_TYPES[random.randrange(TOTAL_TILE_TYPES)])
            for i in range(self.TOTAL_SIZE)
        ]
    
    def good_map_size(self) -> bool:
        map_size = self.map_size
        if map_size < self.MIN_SIZE:
            print("Map is too small...")
            return False
        else:
            print(f"Made a map with {map_size} tiles.")
            return True

    def generate(self):
        print("Generating...")
        self.tiles = [
            Tile(i, WEIGHTED_TILE_TYPES[EMPTY_CHAR_INDEX])
            for i in range(self.TOTAL_SIZE)
        ]
        center_index = int(self.WIDTH / 2 + self.WIDTH / 2 * self.WIDTH)
        self.tiles[center_index].char_index = START_CHAR_INDEX
        self._next_tile_queue.put(center_index)

    def create(self):
        print("Creating a new map")
        self.tiles = [
            Tile(i, WEIGHTED_TILE_TYPES[EMPTY_CHAR_INDEX])
            for i in range(self.TOTAL_SIZE)
        ]
        center_index = int(self.WIDTH / 2 + self.WIDTH / 2 * self.WIDTH)
        self.tiles[center_index].char_index = START_CHAR_INDEX
        self._generate_tiles(center_index)

        if not self.good_map_size():
            self.create()

    def _filter_tile_vec(self, 
                         tile_vec: list[TileType], 
                         in_bounds: bool,
                         char_index: int,
                         connection_check,
                         has_connection) -> list[TileType]:
        if not in_bounds:
            tile_vec = filter_no_connection(has_connection, tile_vec)
        elif self.tiles[char_index].char_index != EMPTY_CHAR_INDEX:
            if connection_check(self.tiles[char_index].char_index):
                tile_vec = filter_connection(has_connection, tile_vec)
            else:
                tile_vec = filter_no_connection(has_connection, tile_vec)   
        return list(tile_vec)

    def _get_tile(self, ti: int, tile_vec: list[TileType]) -> int:
        n = Neighbors(ti)

        tile_vec = self._filter_tile_vec(
            tile_vec=tile_vec,
            in_bounds=n.top > 0,
            char_index=n.top,
            connection_check=has_bottom_connection,
            has_connection=has_top_connection
        )
        tile_vec = self._filter_tile_vec(
            tile_vec=tile_vec,
            in_bounds=(ti % self.WIDTH) != self.WIDTH - 1,
            char_index=n.right,
            connection_check=has_left_connection,
            has_connection=has_right_connection
        )
        tile_vec = self._filter_tile_vec(
            tile_vec=tile_vec,
            in_bounds=n.bottom < self.TOTAL_SIZE,
            char_index=n.bottom,
            connection_check=has_top_connection,
            has_connection=has_bottom_connection
        )
        tile_vec = self._filter_tile_vec(
            tile_vec=tile_vec,
            in_bounds=(ti % self.WIDTH) > 0,
            char_index=n.left,
            connection_check=has_right_connection,
            has_connection=has_left_connection
        )
        
        new_tile_char = random.choices(
            population=range(len(tile_vec)),
            weights=[t.weight for t in tile_vec],
            k=1
        )
        return tile_vec[new_tile_char[0]].char_index

    def _generate_tiles(self, cur_index: int, use_queue: bool = False):
        tile_indexes: list[int] = []
        ti = self.tiles[cur_index].index
        chi = self.tiles[cur_index].tile_type.char_index

        n = Neighbors(cur_index)

        if n.top >= 0 and has_top_connection(chi) and self.tiles[n.top].char_index == EMPTY_CHAR_INDEX:
            tile_indexes.append(n.top)
            self.tiles[n.top].char_index = self._get_tile(n.top, self.top_tiles.copy())
        
        if (ti % self.WIDTH) != self.WIDTH - 1 and has_right_connection(chi) and self.tiles[n.right].char_index == EMPTY_CHAR_INDEX:
            tile_indexes.append(n.right)
            self.tiles[n.right].char_index = self._get_tile(n.right, self.right_tiles.copy())
        
        if n.bottom < self.TOTAL_SIZE and has_bottom_connection(chi) and self.tiles[n.bottom].char_index == EMPTY_CHAR_INDEX:
            tile_indexes.append(n.bottom)
            self.tiles[n.bottom].char_index = self._get_tile(n.bottom, self.bottom_tiles.copy())
        
        if (ti % self.WIDTH) > 0 and has_left_connection(chi) and self.tiles[n.left].char_index == EMPTY_CHAR_INDEX:
            tile_indexes.append(n.left)
            self.tiles[n.left].char_index = self._get_tile(n.left, self.left_tiles.copy())

        if not use_queue:
            for i in tile_indexes:
                self._generate_tiles(i)
        else:
            for i in tile_indexes:
                self._next_tile_queue.put(i)

def set_seed(s=None):
    if s:
        print(f"RNG is seeded to {s}.")
    else:
        print("RNG not seeded.")
    random.seed(s)


def main(argv: list):

    pygame.init()
    screen = pygame.display.set_mode((600, 600), pygame.RESIZABLE)
    clock = pygame.time.Clock()
    running = True
    
    seeded = True
    seed = 10

    if seeded:
        set_seed(seed)
    map = Map()
    # map.randomize()
    # map.create()
    # map.print()
    map.generate()

    while running:
        # poll for events
        # pygame.QUIT event means the user clicked X to close your window
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False
            elif event.type == pygame.KEYDOWN:
                if event.key == pygame.K_q:
                    running = False
                elif event.key == pygame.K_s:
                    seeded = not seeded
                    set_seed(seed if seeded else None)
                    map.generate()
                elif event.key == pygame.K_r:
                    map.generate()


        # fill the screen with a color to wipe away anything from last frame
        screen.fill("black")

        map.update()

        # RENDER YOUR GAME HERE
        map.render(screen)

        # flip() the display to put your work on screen
        pygame.display.flip()

        clock.tick(10)  # limits FPS to 60

    pygame.quit()


if __name__ == "__main__":
    main(sys.argv)