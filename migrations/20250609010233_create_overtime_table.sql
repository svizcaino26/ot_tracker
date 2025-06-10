-- Add migration script here
CREATE TABLE IF NOT EXISTS overtime (
  ot_id INTEGER PRIMARY KEY, 
  user_id INTEGER,
  start_time TEXT,
  end_time TEXT,
  description TEXT,
  FOREIGN KEY (user_id) REFERENCES users (user_id)
  ON DELETE CASCADE
  ON UPDATE NO ACTION
  );
