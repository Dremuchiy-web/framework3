program GenerateData;

uses
    SysUtils, DateUtils;

type
    TDataRecord = record
        Timestamp: TDateTime;
        IsActive: Boolean;
        Value: Double;
        Description: String;
    end;

var
    Data: array[1..100] of TDataRecord;
    i: Integer;
    CSVFile: TextFile;
    CSVFileName: String;
    Row: String;

begin
    // Генерация тестовых данных
    for i := 1 to 100 do
    begin
        Data[i].Timestamp := Now + (i * 1.0 / 24.0); // Добавляем часы
        Data[i].IsActive := (i mod 2 = 0);
        Data[i].Value := Random(1000) + Random;
        Data[i].Description := 'Record ' + IntToStr(i);
    end;

    // Генерация CSV файла
    CSVFileName := 'output/data_' + FormatDateTime('yyyymmdd_hhnnss', Now) + '.csv';
    AssignFile(CSVFile, CSVFileName);
    Rewrite(CSVFile);

    // Заголовок CSV
    WriteLn(CSVFile, 'Timestamp,IsActive,Value,Description');

    // Данные CSV
    for i := 1 to 100 do
    begin
        Row := FormatDateTime('yyyy-mm-dd hh:nn:ss', Data[i].Timestamp) + ',';
        if Data[i].IsActive then
            Row := Row + 'ИСТИНА,'
        else
            Row := Row + 'ЛОЖЬ,';
        Row := Row + FloatToStr(Data[i].Value) + ',';
        Row := Row + Data[i].Description;
        WriteLn(CSVFile, Row);
    end;

    CloseFile(CSVFile);
    WriteLn('CSV файл создан: ', CSVFileName);
end.

