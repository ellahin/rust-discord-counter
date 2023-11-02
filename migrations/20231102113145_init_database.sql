-- Add migration script here
CREATE TABLE servers (
  realmID INT NOT NULL UNIQUE,
  item varchar(255) NOT NULL,
  PRIMARY KEY(realmID)
);

CREATE TABLE usercounts (
  ID INT NOT Null UNIQUE,
  realmID INT NOT NULL,
  userID INT NOT NULL,
  count INT DEFAULT 0,
  PRIMARY KEY(ID),
  FOREIGN KEY(realmID) REFERENCES servers(realmID)
);
