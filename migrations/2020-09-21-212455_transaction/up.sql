CREATE TABLE IF NOT EXISTS Transactions(
   id                       INTEGER PRIMARY KEY ASC,
   `sequence`                 INTEGER UNIQUE      ,
   tranType                     NUMERIC             ,
   `dateTime`                 NUMERIC             ,
   amount                   NUMERIC             
);