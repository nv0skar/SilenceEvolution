-- SilenceEvolution
-- Copyright (C) 2026 Oscar Alvarez Gonzalez

-- NOTE: Each statement is executed in different queries.

CREATE TABLE IF NOT EXISTS |users_table| (
    |user_id_field| INT AUTO_INCREMENT PRIMARY KEY,
    |user_name_field| VARCHAR(64) NOT NULL UNIQUE,
    |user_email_field| VARCHAR(32) NOT NULL UNIQUE,
    |user_password_field| VARCHAR(128) NOT NULL
);

CREATE TABLE IF NOT EXISTS |roles_table| (
    user_id INT PRIMARY KEY REFERENCES |users_table| (|user_id_field|) ON DELETE CASCADE,
    role VARCHAR(16) NOT NULL
);
