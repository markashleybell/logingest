USE LogAnalysis
GO

DROP TABLE IF EXISTS entries;

CREATE TABLE entries (
    [timestamp] DATETIME NULL,
    server_ip NVARCHAR(2048) NULL,
    method NVARCHAR(2048) NULL,
    uri NVARCHAR(2048) NULL,
    query NVARCHAR(MAX) NULL,
    server_port SMALLINT NULL,
    client_username NVARCHAR(2048) NULL,
    client_ip NVARCHAR(2048) NULL,
    user_agent NVARCHAR(MAX) NULL,
    referer NVARCHAR(MAX) NULL,
    status SMALLINT NULL,
    substatus SMALLINT NULL,
    win32_status SMALLINT NULL,
    time_taken INT NULL
);