USE LogAnalysis
GO

BULK INSERT dbo.entries
FROM 'C:\Temp\logingest\u_ex210802.csv'
WITH (FORMATFILE = 'C:\Src\logingest\sql\logformat.fmt', CODEPAGE = 'RAW', FIRSTROW = 2);

SELECT TOP 100 * FROM dbo.entries

-- ADD INDEXES
