#include <iostream>
#include <vector>
#include <stdio.h>      /* printf, scanf, puts, NULL */
#include <stdlib.h>     /* srand, rand */
#include <time.h>       /* time */
#include <algorithm>


const int SIZE = 16;
const int MAP_SIZE = SIZE * SIZE;
const int TOTAL_TILE_TYPES = 16;


void print_separator(const int n, const char c) {
    std::cout << "+";
    for (int i = 0; i < n; i ++) {
        std::cout << c;
    }
    std::cout << "+" << std::endl;
}

struct Coord {
    int x, y;
};

int c2i(Coord c) {
    // coordinates to index in the map
    return c.x + c.y * SIZE;
}
 
Coord i2c(const int i) {
    // index in the map to coordinates
    return Coord{ i % SIZE, i / SIZE };
}

const char TILE_TYPES[TOTAL_TILE_TYPES] = {
    ' ',    // 0000
    '\xd0', // '╨', // 0001
    '\xc6', // '╞', // 0010
    '\xc8', // '╚', // 0011

    '\xd2', // '╥', // 0100
    '\xba', // '║', // 0101
    '\xc9', // '╔', // 0110
    '\xcc', // '╠', // 0111

    '\xb5', // '╡', // 1000
    '\xbc', // '╝', // 1001
    '\xcd', // '═', // 1010
    '\xca', // '╩', // 1011

    '\xbb', // '╗', // 1100
    '\xb9', // '╣', // 1101 
    '\xcb', // '╦', // 1110
    '\xce', // '╬', // 1111
};

const char START_CHAR_INDEX = 15;
const char EMPTY_CHAR_INDEX = 0;

struct TileType {
    int tile_index;
    int weight;
};

typedef std::vector<TileType> TileVec;

TileType WEIGHTED_TILE_TYPES[TOTAL_TILE_TYPES] = {
    TileType { 0, 1 }, // 0000
    TileType { 1, 8 }, // 0001
    TileType { 2, 8 }, // 0010
    TileType { 3, 6 }, // 0011

    TileType { 4, 8 }, // 0100
    TileType { 5, 4 }, // 0101
    TileType { 6, 6 }, // 0110
    TileType { 7, 8 }, // 0111

    TileType { 8, 8 }, // 1000
    TileType { 9, 6 }, // 1001
    TileType { 10, 6 }, // 1010
    TileType { 11, 4 }, // 1011

    TileType { 12, 6 }, // 1100
    TileType { 13, 4 }, // 1101 
    TileType { 14, 4 }, // 1110
    TileType { 15, 2 }, // 1111
};

struct Neighbors {
    int top, bottom, left, right;

    Neighbors(const int index) : 
        top(index - SIZE), 
        bottom(index + SIZE), 
        left(index - 1), 
        right(index + 1) 
    {}
    friend std::ostream& operator<< (std::ostream& stream, const Neighbors& n) {
        stream << "Neighbors[" 
            << n.top << ", " 
            << n.bottom << ", " 
            << n.left << ", " 
            << n.right << "]";
        
        return stream;
    }
};

inline bool bit_on(const int mask, const int bit) {
    return (bit & mask) == bit;
}

void weighted_tiles(int bit, TileVec& tileset) {
    for (int i = 0; i < TOTAL_TILE_TYPES; i ++) {
        if (bit_on(WEIGHTED_TILE_TYPES[i].tile_index, bit)) {
            tileset.push_back(WEIGHTED_TILE_TYPES[i]);
        }
    }
}

bool has_top_connection(int tile_index) {
    return bit_on(tile_index, 1);
}

bool has_right_connection(int tile_index) {
    return bit_on(tile_index, 2);
}

bool has_bottom_connection(int tile_index) {
    return bit_on(tile_index, 4);
}

bool has_left_connection(int tile_index) {
    return bit_on(tile_index, 8);
}

struct Tile 
{
    int index;
    TileType tile_type;

    const char get_char() {
        return TILE_TYPES[tile_type.tile_index];
    }
    const int char_index() {
        return tile_type.tile_index;
    }
    void set_char_index(const int index) {
        tile_type.tile_index = index;
    }
};

class Map
{
public:
    Map() {
        this->tiles = std::vector<Tile>();
        weighted_tiles(1, this->bottom_tiles);
        weighted_tiles(2, this->left_tiles);
        weighted_tiles(4, this->top_tiles);
        weighted_tiles(8, this->right_tiles);
    }

    void print() {
        printf("Map %dx%d\n", SIZE, SIZE);
        print_separator(SIZE, '-');
        std::cout << "|";
        for (auto& tile: this->tiles) {
            std::cout << tile.get_char();
            if ((tile.index % SIZE) == (SIZE - 1)) {
                std::cout << "|" << std::endl;
                if (tile.index < (MAP_SIZE - 1)) {
                    std::cout << "|";
                }
            }
        }
        print_separator(SIZE, '-');
    }

    void randomize() {
        for (int i = 0; i < MAP_SIZE; i++) {
            this->tiles.push_back(Tile{ i, rand() % TOTAL_TILE_TYPES });
        }
    }

    void create() {
        for (int i = 0; i < MAP_SIZE; i++) {
            this->tiles.push_back(Tile{ i, EMPTY_CHAR_INDEX });
        }
        const int center_index = SIZE / 2 + SIZE / 2 * SIZE;
        this->tiles[SIZE / 2 + SIZE / 2 * SIZE].set_char_index(START_CHAR_INDEX);
        this->generate_tiles(center_index);
    }

private:

    TileVec filter_top_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return !has_top_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_not_top_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return has_top_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_right_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return !has_right_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_not_right_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return has_right_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_bottom_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return !has_bottom_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_not_bottom_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return has_bottom_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_left_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return !has_left_connection(t.tile_index);}
        );
        return dst;
    }

    TileVec filter_not_left_connectable(TileVec& src) {
        TileVec dst;
        std::copy_if(
            src.begin(), src.end(), std::back_inserter(dst), 
            [](TileType t) {return has_left_connection(t.tile_index);}
        );
        return dst;
    }

    int get_tile(const int ti, TileVec tile_vec) {
        Neighbors n(ti);

        if (n.top < 0) {
            tile_vec = this->filter_top_connectable(tile_vec);
        } else if (this->tiles[n.top].char_index() != EMPTY_CHAR_INDEX) {
            if (has_bottom_connection(this->tiles[n.top].char_index())) {
                tile_vec = this->filter_not_top_connectable(tile_vec);
            } else {
                tile_vec = this->filter_top_connectable(tile_vec);
            }
        }

        if ((ti % SIZE) == SIZE - 1) {
            tile_vec = this->filter_right_connectable(tile_vec);
        } else if (this->tiles[n.right].char_index() != EMPTY_CHAR_INDEX) {
            if (has_left_connection(this->tiles[n.right].char_index())) {
                tile_vec = this->filter_not_right_connectable(tile_vec);
            } else {
                tile_vec = this->filter_right_connectable(tile_vec);
            }
        }

        if (n.bottom >= MAP_SIZE) {
            tile_vec = this->filter_bottom_connectable(tile_vec);
        } else if (this->tiles[n.bottom].char_index() != EMPTY_CHAR_INDEX) {
            if (has_top_connection(this->tiles[n.bottom].char_index())) {
                tile_vec = this->filter_not_bottom_connectable(tile_vec);
            } else {
                tile_vec = this->filter_bottom_connectable(tile_vec);
            }
        }

        if ((ti % SIZE) == 0) {
            tile_vec = this->filter_left_connectable(tile_vec);
        } else if (this->tiles[n.left].char_index() != EMPTY_CHAR_INDEX) {
            if (has_right_connection(this->tiles[n.left].char_index())) {
                tile_vec = this->filter_not_left_connectable(tile_vec);
            } else {
                tile_vec = this->filter_left_connectable(tile_vec);
            }
        }
        return tile_vec[rand() % tile_vec.size()].tile_index;

    }

    void generate_tiles(const int cur_index) {
        std::vector<int> tile_indexes;
        const int ti = this->tiles[cur_index].index;
        const int chi = this->tiles[cur_index].tile_type.tile_index;

        Neighbors n(cur_index);

        if (n.top >= 0 && has_top_connection(chi) && this->tiles[n.top].char_index() == EMPTY_CHAR_INDEX) {
            tile_indexes.push_back(n.top);
            this->tiles[n.top].set_char_index(this->get_tile(n.top, this->top_tiles));
        }
        if ((ti % SIZE) != SIZE - 1 && has_right_connection(chi) && this->tiles[n.right].char_index() == EMPTY_CHAR_INDEX) {
            tile_indexes.push_back(n.right);
            this->tiles[n.right].set_char_index(this->get_tile(n.right, this->right_tiles));
        }
        if (n.bottom < MAP_SIZE && has_bottom_connection(chi) && this->tiles[n.bottom].char_index() == EMPTY_CHAR_INDEX) {
            tile_indexes.push_back(n.bottom);
            this->tiles[n.bottom].set_char_index(this->get_tile(n.bottom, this->bottom_tiles));
        }
        if ((ti % SIZE) > 0 && has_left_connection(chi) && this->tiles[n.left].char_index() == EMPTY_CHAR_INDEX) {
            tile_indexes.push_back(n.left);
            this->tiles[n.left].set_char_index(this->get_tile(n.left, this->left_tiles));
        }
        
        for (auto i : tile_indexes) {
            this->generate_tiles(i);
        }
    }

    std::vector<Tile> tiles;
    TileVec bottom_tiles;
    TileVec top_tiles;
    TileVec right_tiles;
    TileVec left_tiles;
};

int main(int argc, char *argv[])
{
    int seed = time(NULL);

    if (argc > 1) {
        std::cout << "Args:";
        for (int i = 0; i < argc; i ++) {
            std::cout << argv[i] << ", ";
        }
        std::cout << std::endl;

        if (argv[1] == std::string("-s") || 
            argv[1] == std::string("--seed")) {
            if (argc <= 2) {
                std::cout << "ERROR: Not enough args, --seed requires an integer value." << std::endl;
            } else {
                seed = atoi(argv[2]);
            }
        }
    }
    srand(seed);
    Map map = Map();
    map.create();
    // map.randomize();
    map.print();

    return 0;
}