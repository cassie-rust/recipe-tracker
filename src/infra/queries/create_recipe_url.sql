WITH recipe_id AS (
  INSERT INTO recipes (name)
  VALUES ($1)
  RETURNING id
),
url_id AS (
  INSERT INTO recipe_urls (name, url)
  VALUES ($2, $3)
  RETURNING id
),
url_parts AS (
  INSERT INTO recipe_parts (recipe, url)
  SELECT recipe_id.id,
    url_id.id
  FROM recipe_id,
    url_id
)
SELECT recipe_id.id
FROM recipe_id