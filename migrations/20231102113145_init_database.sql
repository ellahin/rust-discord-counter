-- Add migration script here
CREATE TABLE servers (
  realmID BIGINT NOT NULL UNIQUE,
  item varchar(255) NOT NULL,
  PRIMARY KEY(realmID)
);

CREATE TABLE usercounts (
  ID INTEGER NOT Null UNIQUE PRIMARY KEY AUTOINCREMENT,
  realmID BIGINT NOT NULL,
  userID BIGINT NOT NULL,
  count	INT NOT NULL DEFAULT 0,
  FOREIGN KEY(realmID) REFERENCES servers(realmID)
);
