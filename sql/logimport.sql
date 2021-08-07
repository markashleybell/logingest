USE LogAnalysis
GO

BULK INSERT dbo.entries
FROM 'C:\Temp\logingest\csv\test.csv'
WITH (FORMATFILE = 'C:\Src\logingest\sql\logformat.fmt', CODEPAGE = 'RAW', FIRSTROW = 2);

SELECT COUNT(*) AS TotalRows FROM dbo.entries

SELECT TOP 100 * FROM dbo.entries ORDER BY [timestamp] DESC

-- ADD INDEXES?
