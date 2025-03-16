CREATE TABLE IF NOT EXISTS users (
  id INT UNIQUE AUTO_INCREMENT,
  email VARCHAR(255) COLLATE utf8mb4_bin UNIQUE NOT NULL,
  password VARCHAR(32) COLLATE utf8mb4_bin NOT NULL,
  name VARCHAR(255) COLLATE utf8mb4_bin NOT NULL,
  admin BOOLEAN DEFAULT false,
  PRIMARY KEY (id)
);