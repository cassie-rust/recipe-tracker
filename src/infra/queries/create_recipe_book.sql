WITH recipe_id AS (
  INSERT INTO recipes (name)
  VALUES ($1)
  RETURNING id
),
book_id AS (
  INSERT INTO recipe_books (title, isbn)
  VALUES ($2, $3)
  RETURNING id
),
book_parts AS (
  INSERT INTO recipe_parts (recipe, book)
  SELECT recipe_id.id,
    book_id.id
  FROM recipe_id,
    book_id
)
SELECT recipe_id.id
FROM recipe_id