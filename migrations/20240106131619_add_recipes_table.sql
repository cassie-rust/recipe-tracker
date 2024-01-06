-- recipes
CREATE TABLE recipes (
  id UUID DEFAULT gen_random_uuid () NOT NULL,
  name TEXT CHECK (LENGTH(name) <= 50) NOT NULL,
  PRIMARY KEY (id)
);

-- recipe_urls
CREATE TABLE recipe_urls (
  id UUID DEFAULT gen_random_uuid() NOT NULL,
  url TEXT CHECK (LENGTH(url) <= 100) NOT NULL,
  name TEXT CHECK (LENGTH(name) <= 50) NOT NULL,
  PRIMARY KEY (id)
);

-- recipe_books
CREATE TABLE recipe_books (
  id UUID DEFAULT gen_random_uuid() NOT NULL,
  title TEXT CHECK (LENGTH(title) <= 50) NOT NULL,
  isbn TEXT CHECK (
    LENGTH(isbn) = 10
    OR LENGTH(isbn) = 13
  ),
  PRIMARY KEY (id)
);

-- recipe_parts
CREATE TABLE recipe_parts (
  recipe uuid NOT NULL,
  part uuid NOT NULL GENERATED ALWAYS AS (COALESCE("url", "book")) STORED,
  url uuid,
  book uuid,
  CONSTRAINT recipe_parts_pkey PRIMARY KEY (recipe, part),
  CONSTRAINT recipe_parts_recipe_fkey FOREIGN KEY (recipe) REFERENCES recipes ON
        DELETE CASCADE,
  CONSTRAINT recipe_parts_url_fkey FOREIGN KEY (url) REFERENCES recipe_urls ON
        DELETE CASCADE,
  CONSTRAINT recipe_parts_book_fkey FOREIGN KEY (book) REFERENCES recipe_books ON
        DELETE CASCADE
);