USE LogAnalysis
GO

BULK INSERT dbo.entries
FROM 'C:\Temp\u_ex210802.csv'
WITH (FORMATFILE = 'C:\Src\logingest\sql\logformat.fmt', CODEPAGE = 'RAW', FIRSTROW = 2);

-- ADD INDEXES
