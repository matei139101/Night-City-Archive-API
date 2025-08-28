USE NCADatabase;

CREATE TABLE IF NOT EXISTS gear (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  description TEXT,
  price INT NOT NULL
);

CREATE TABLE IF NOT EXISTS cyberware (
  id INT PRIMARY KEY,
  humanity_loss INT NOT NULL,
  type ENUM('fashionware', 'neuralware', 'cyberoptics', 'cyberaudio', 'internal', 'external', 'cyberlimbs', 'borgware'),
  is_foundational SMALLINT NOT NULL,
  effect TEXT,
  CONSTRAINT fk_cyberware_gear FOREIGN KEY (id) REFERENCES gear(id)
);

CREATE TABLE IF NOT EXISTS weapons (
  id INT PRIMARY KEY,
  type ENUM('very_light_melee', 'light_melee', 'medium_melee', 'heavy_melee', 'light_pistol', 'medium_pistol', 'heavy_pistol', 'very_heavy_pistol', 'smg', 'heavy_smg', 'shotgun', 'assault_rifle', 'sniper_rifle', 'grenade_launcher', 'rocket_launcher'),
  damage_die INT NOT NULL,
  is_exotic SMALLINT NOT NULL,
  CONSTRAINT fk_weapons_gear FOREIGN KEY (id) REFERENCES gear(id)
);

CREATE TABLE IF NOT EXISTS items (
  id INT PRIMARY KEY,
  information TEXT NOT NULL,
  CONSTRAINT fk_items_gear FOREIGN KEY (id) REFERENCES gear(id)
);


INSERT INTO gear(name, description, price) VALUES
('Agent', 'Self-adaptive-AI powered smartphone.', 100),
('Airhypo', 'Easy to use drug distribution platform which uses a quick burst of compressed air to force a drug through the skin', 50),
('Kibble Pack', 'One foil package of dry, pet food-like cereal or wafers equivalent to a single meal', 10)