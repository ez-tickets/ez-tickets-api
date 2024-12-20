CREATE TABLE categories(
    id   TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE categories_ordering(
    category TEXT    NOT NULL PRIMARY KEY,
    ordering INTEGER NOT NULL UNIQUE,

    FOREIGN KEY (category) REFERENCES categories (id) ON DELETE CASCADE
);

CREATE TABLE products(
    id    TEXT    NOT NULL PRIMARY KEY,
    name  TEXT    NOT NULL,
    image TEXT    NOT NULL,
    
    FOREIGN KEY (image) REFERENCES images (id) ON DELETE CASCADE
);

CREATE TABLE images(
    id    TEXT NOT NULL PRIMARY KEY,
    image BLOB NOT NULL
);

CREATE TABLE catalogs(
    id    TEXT    NOT NULL PRIMARY KEY,
    name  TEXT    NOT NULL,
    image TEXT,
    desc  TEXT    NOT NULL,
    price INTEGER NOT NULL,
    
    FOREIGN KEY (image) REFERENCES images (id) ON DELETE CASCADE
);

CREATE TABLE category_catalogs(
    catalog  TEXT    NOT NULL,
    category TEXT    NOT NULL,
    ordering INTEGER NOT NULL,
    
    PRIMARY KEY (catalog, category),
    UNIQUE      (category, ordering),
    
    FOREIGN KEY (catalog)  REFERENCES catalogs   (id) ON DELETE CASCADE,
    FOREIGN KEY (category) REFERENCES categories (id) ON DELETE CASCADE
);

CREATE TABLE product_catalogs(
    catalog  TEXT    NOT NULL,
    product  TEXT    NOT NULL,
    
    PRIMARY KEY (catalog, product),
    
    FOREIGN KEY (catalog) REFERENCES catalogs (id) ON DELETE CASCADE,
    FOREIGN KEY (product) REFERENCES products (id) ON DELETE CASCADE
);