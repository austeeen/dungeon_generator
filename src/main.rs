use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use weighted_rand::table::WalkerTable;
use weighted_rand::builder::*;

const SEED: u64 = 10;
const USE_SEED: bool = false;
const MAP_SIZE: usize = 20;
const TOTAL_TILES: usize = MAP_SIZE * MAP_SIZE;

const TILE_TYPES: [char; 16] = [
    ' ', // 0000
    '╨', // 0001
    '╞', // 0010
    '╚', // 0011

    '╥', // 0100
    '║', // 0101
    '╔', // 0110
    '╠', // 0111

    '╡', // 1000
    '╝', // 1001
    '═', // 1010
    '╩', // 1011

    '╗', // 1100
    '╣', // 1101 
    '╦', // 1110
    '╬', // 1111
];

#[derive(Clone)]
struct TileType {
    ci: usize,
    ch: char,
    w: u32
}

const WEIGHTED_TILE_TYPES: [TileType; 16] = [
    TileType { ci: 0, ch: ' ', w: 1 }, // 0000
    TileType { ci: 1, ch: '╨', w: 8 }, // 0001
    TileType { ci: 2, ch: '╞', w: 8 }, // 0010
    TileType { ci: 3, ch: '╚', w: 6 }, // 0011

    TileType { ci: 4, ch: '╥', w: 8 }, // 0100
    TileType { ci: 5, ch: '║', w: 4 }, // 0101
    TileType { ci: 6, ch: '╔', w: 6 }, // 0110
    TileType { ci: 7, ch: '╠', w: 8 }, // 0111

    TileType { ci: 8,  ch: '╡', w: 8 }, // 1000
    TileType { ci: 9,  ch: '╝', w: 6 }, // 1001
    TileType { ci: 10, ch: '═', w: 6 }, // 1010
    TileType { ci: 11, ch: '╩', w: 4 }, // 1011

    TileType { ci: 12, ch: '╗', w: 6 }, // 1100
    TileType { ci: 13, ch: '╣', w: 4 }, // 1101 
    TileType { ci: 14, ch: '╦', w: 4 }, // 1110
    TileType { ci: 15, ch: '╬', w: 2 }, // 1111
];

const START_CHAR_INDEX: usize = 15;
// const START_CHAR: char = TILE_TYPES[START_CHAR_INDEX];
const START_CHAR: char = WEIGHTED_TILE_TYPES[START_CHAR_INDEX].ch;

const EMPTY_CHAR_INDEX: usize = 0;
// const EMPTY_CHAR: char = TILE_TYPES[EMPTY_CHAR_INDEX];
const EMPTY_CHAR: char = WEIGHTED_TILE_TYPES[EMPTY_CHAR_INDEX].ch;

type Tileset = Vec<usize>;
type WeightedTileset = Vec<TileType>;

fn bit_on(mask: u8, bit: u8) -> bool {
    bit & mask == bit
}

fn init_tiles(bit: u8) -> Tileset {
    let mut tiles: Tileset = vec![];
    for i in 0..TILE_TYPES.len() {
        if bit_on(i as u8, bit) {
            tiles.push(i);
        }
    }
    tiles
}

fn init_wtiles(bit: u8) -> WeightedTileset {
    WEIGHTED_TILE_TYPES
        .into_iter()
        .filter(|wt| bit_on(wt.ci as u8, bit))
        .collect()
}

fn has_top_connection(ci: u8) -> bool {
    bit_on(ci as u8, 1)
}

fn has_right_connection(ci: u8) -> bool {
    bit_on(ci as u8, 2)
}

fn has_bottom_connection(ci: u8) -> bool {
    bit_on(ci as u8, 4)
}

fn has_left_connection(ci: u8) -> bool {
    bit_on(ci as u8, 8)
}

#[derive(Clone)]
struct Coord {
    x: usize, y: usize
}
impl Coord {
    pub fn to_index(x: usize, y: usize) -> usize {
        x + y * MAP_SIZE
    }

    pub fn new(i: usize) -> Coord {
        Coord { x: i % MAP_SIZE, y: i / MAP_SIZE }
    }
}

#[derive(Clone)]
struct Tile {
    indx: usize,
    pos: Coord,
    ch: char,
    ch_i: u8
}
impl Tile {
    pub fn new(i: usize, t: usize) -> Self {
        Tile {
            indx: i,
            pos: Coord::new(i),
            ch: WEIGHTED_TILE_TYPES[t].ch,
            ch_i: t as u8
        }
    }

    pub fn empty(i: usize) -> Self {
        Tile::new(i, 0)
    }
    
    pub fn set_char(&mut self, t: usize) {
        self.ch = WEIGHTED_TILE_TYPES[t].ch;
        self.ch_i = t as u8;
    }

    pub fn print(&self) {
        println!("i: {}, ci: {}", self.indx, self.ch_i);
        println!("   top: {}", (self.ch_i & (1 as u8)));
        println!(" right: {}", (self.ch_i & (2 as u8)));
        println!("bottom: {}", (self.ch_i & (4 as u8)));
        println!("  left: {}", (self.ch_i & (8 as u8)));
    }

}

#[derive(Clone)]
struct TileGenerator {
    rng: Pcg32,
    seeded_rng: Pcg32,
    wa_table: WalkerTable
}
impl TileGenerator {
    fn new() -> Self {
        let index_weights: Vec<u32> = WEIGHTED_TILE_TYPES
            .into_iter()
            .map(|wt| wt.w)
            .collect();
        let builder = WalkerTableBuilder::new(&index_weights);

        TileGenerator {
            rng: Pcg32::from_entropy(),
            seeded_rng: Pcg32::seed_from_u64(SEED),
            wa_table: builder.build()
        }
    }

    fn random_tile_index(&mut self) -> usize {
        if USE_SEED {
            self.wa_table.next_rng(&mut self.seeded_rng)
        } else {
            self.wa_table.next_rng(&mut self.rng)
        }
    }

    fn tile_index_from(&mut self, w_tileset: &WeightedTileset) -> usize {
        let index_weights: Vec<u32> = w_tileset
            .into_iter()
            .map(|wt| wt.w)
            .collect();
        let builder = WalkerTableBuilder::new(&index_weights);
        let wa_table = builder.build();
        if USE_SEED {
            wa_table.next_rng(&mut self.seeded_rng)
        } else {
            wa_table.next_rng(&mut self.rng)
        }
    }

}

#[derive(Clone)]
struct Map {
    tiles: Vec<Tile>,
    bottom_tiles: WeightedTileset,
    left_tiles: WeightedTileset,
    top_tiles: WeightedTileset,
    right_tiles: WeightedTileset,
    // rng: Pcg32,
    // seeded_rng: Pcg32,
    tile_gen: TileGenerator
}

impl Map {
    fn new() -> Self {
        Map { 
            tiles: Vec::new(),
            bottom_tiles: init_wtiles(1),
            left_tiles: init_wtiles(2),
            top_tiles: init_wtiles(4),
            right_tiles: init_wtiles(8),
            // rng: Pcg32::from_entropy(),
            // seeded_rng: Pcg32::seed_from_u64(SEED)
            tile_gen: TileGenerator::new()
        }
    }

    fn empty() -> Self {
        let mut map = Map::new();
        for i in 0..TOTAL_TILES {
            map.tiles.push(Tile::empty(i));
        }
        map
    }

    fn random() -> Self {
        let mut map = Map::new();
        for i in 0..TOTAL_TILES {
            let ti = map.tile_gen.random_tile_index();
            map.tiles.push(Tile::new(i, ti));
        }
        map
    }

    fn print(&self) {
        println!("Map ({}x{}), Seed = {}", MAP_SIZE, MAP_SIZE, SEED);
        println!("+{:-<1$}+", "", MAP_SIZE);
        print!("|");
        for tile in &self.tiles {      
            print!("{}", tile.ch);
            if tile.pos.x == MAP_SIZE - 1 {
                print!("|");
                print!("\n");
                if tile.pos.y != MAP_SIZE - 1 {
                    print!("|");
                }
            }
        }
        println!("+{:-<1$}+", "", MAP_SIZE);
    }

    fn get_neighbors(ti: usize) -> (i32, i32, i32, i32) {
        (ti as i32 - MAP_SIZE as i32, ti as i32 + 1, ti as i32 + MAP_SIZE as i32, ti as i32 - 1)
    }

    fn get_tile(&mut self, ti: usize, mut tile_vec: WeightedTileset) -> usize {
        let (ui, ri, bi, li) = Map::get_neighbors(ti);

        if ui < 0 {
            // filter out top-connectable tiles
            tile_vec = tile_vec
                .into_iter()
                .filter(|t| !has_top_connection(t.ci as u8))
                .collect();
        } else if self.tiles[ui as usize].ch != EMPTY_CHAR {
            if has_bottom_connection(self.tiles[ui as usize].ch_i) {
                // filter out non-top-connectable tiles
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| has_top_connection(t.ci as u8))
                    .collect();
            } else {
                // filter out top-connectable tiles
                tile_vec = tile_vec
                .into_iter()
                .filter(|t| !has_top_connection(t.ci as u8))
                .collect();
            }
        }
        if (ti % MAP_SIZE) == MAP_SIZE - 1 {
            // filter out right-connectable tiles
            tile_vec = tile_vec
                .into_iter()
                .filter(|t| !has_right_connection(t.ci as u8))
                .collect();
        } else if self.tiles[ri as usize].ch != EMPTY_CHAR {
            if has_left_connection(self.tiles[ri as usize].ch_i) {
                // filter out non-right-connectable tiles in tile_vec
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| has_right_connection(t.ci as u8))
                    .collect();
            } else {
                // filter out right-connectable tiles
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| !has_right_connection(t.ci as u8))
                    .collect();
            }
        }
        if bi >= TOTAL_TILES as i32 {
            // filter out bottom-connectable tiles
            tile_vec = tile_vec
                .into_iter()
                .filter(|t| !has_bottom_connection(t.ci as u8))
                .collect();
        } else if self.tiles[bi as usize].ch != EMPTY_CHAR {
            if has_top_connection(self.tiles[bi as usize].ch_i) {
                // filter out non-bottom-connectable tiles in tile_vec
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| has_bottom_connection(t.ci as u8))
                    .collect();
            } else {
                // filter out bottom-connectable tiles
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| !has_bottom_connection(t.ci as u8))
                    .collect();
            }
        }
        if (ti % MAP_SIZE) == 0 {
            // filter out left-connectable tiles
            tile_vec = tile_vec
                .into_iter()
                .filter(|t| !has_left_connection(t.ci as u8))
                .collect();
        } else if ((ti % MAP_SIZE) > 0) && self.tiles[li as usize].ch != EMPTY_CHAR {
            if has_right_connection(self.tiles[li as usize].ch_i) {
                // filter out non-left-connectable tiles in tile_vec
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| has_left_connection(t.ci as u8))
                    .collect();
            } else {
                // filter out left-connectable tiles
                tile_vec = tile_vec
                    .into_iter()
                    .filter(|t| !has_left_connection(t.ci as u8))
                    .collect();
            }
        }
        
        let i = self.tile_gen.tile_index_from(&tile_vec);
        tile_vec[i].ci

    }

    fn generate_tiles(&mut self, cur_index: usize) {
        let mut tile_indexes: Tileset = vec![];
        let ti = self.tiles[cur_index].indx;
        let chi = self.tiles[cur_index].ch_i;

        let (ui, ri, bi, li) = Map::get_neighbors(ti);

        if (ui as i32 >= 0) && has_top_connection(chi) && self.tiles[ui as usize].ch == EMPTY_CHAR {
            let ui = ui as usize;
            tile_indexes.push(ui);
            let char_index = self.get_tile(ui, self.top_tiles.to_vec());
            self.tiles[ui].set_char(char_index);
        }
        if ((ti % MAP_SIZE) != MAP_SIZE - 1) && has_right_connection(chi) && self.tiles[ri as usize].ch == EMPTY_CHAR {
            let ri = ri as usize;
            tile_indexes.push(ri);
            let char_index = self.get_tile(ri, self.right_tiles.to_vec());
            self.tiles[ri].set_char(char_index);
        }
        if (bi < TOTAL_TILES as i32) && has_bottom_connection(chi) && self.tiles[bi as usize].ch == EMPTY_CHAR {
            let bi = bi as usize;
            tile_indexes.push(bi);
            let char_index = self.get_tile(bi, self.bottom_tiles.to_vec());
            self.tiles[bi].set_char(char_index);
        }
        if ((ti % MAP_SIZE) > 0) && has_left_connection(chi) && self.tiles[li as usize].ch == EMPTY_CHAR {
            let li = li as usize;
            tile_indexes.push(li);
            let char_index = self.get_tile(li, self.left_tiles.to_vec());
            self.tiles[li].set_char(char_index);
        }
        
        for i in tile_indexes {
            self.generate_tiles(i);
        }
    }

}

fn create_dungeon() {
    let mut map = Map::empty();
    let center_index = Coord::to_index(MAP_SIZE / 2, MAP_SIZE / 2);
    map.tiles[center_index].set_char(START_CHAR_INDEX);
    map.generate_tiles(center_index);

    println!("Done generating map:");
    map.print();
}

fn create_random_dungeon() {
    let map = Map::random();
    map.print();
}


fn main() {
    // create_random_dungeon();
    create_dungeon();
}
