#!/usr/bin/env python3
"""
Генератор данных в стиле Pascal Legacy
Создает CSV файл с правильными форматами данных
"""
import csv
import os
from datetime import datetime, timedelta
import random

def generate_data():
    """Генерация тестовых данных"""
    data = []
    base_time = datetime.now()
    
    for i in range(100):
        timestamp = base_time + timedelta(hours=i)
        is_active = (i % 2 == 0)  # Логическое значение
        value = round(random.uniform(0, 1000), 2)  # Числовое значение
        description = f'Record {i+1}'  # Строка
        
        data.append({
            'timestamp': timestamp,
            'is_active': is_active,
            'value': value,
            'description': description
        })
    
    return data

def create_csv(data, filename):
    """Создание CSV файла с правильными форматами"""
    output_dir = 'output'
    os.makedirs(output_dir, exist_ok=True)
    
    filepath = os.path.join(output_dir, filename)
    
    with open(filepath, 'w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        
        # Заголовок
        writer.writerow(['Timestamp', 'IsActive', 'Value', 'Description'])
        
        # Данные
        for record in data:
            # Timestamp в формате даты и времени
            timestamp_str = record['timestamp'].strftime('%Y-%m-%d %H:%M:%S')
            
            # Логические блоки: ИСТИНА и ЛОЖЬ
            is_active_str = 'ИСТИНА' if record['is_active'] else 'ЛОЖЬ'
            
            # Числа - числовой формат
            value_str = str(record['value'])
            
            # Строки - текст
            description_str = record['description']
            
            writer.writerow([timestamp_str, is_active_str, value_str, description_str])
    
    print(f'CSV файл создан: {filepath}')
    return filepath

if __name__ == '__main__':
    print('Генерация данных...')
    data = generate_data()
    
    filename = f"data_{datetime.now().strftime('%Y%m%d_%H%M%S')}.csv"
    csv_file = create_csv(data, filename)
    
    # Конвертация в XLSX
    try:
        from convert_to_xlsx import convert_csv_to_xlsx
        xlsx_file = csv_file.replace('.csv', '.xlsx')
        convert_csv_to_xlsx(csv_file, xlsx_file)
    except Exception as e:
        print(f'Ошибка конвертации в XLSX: {e}')

