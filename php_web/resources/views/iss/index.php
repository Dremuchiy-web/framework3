<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?= $title ?></title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="/css/style.css">
</head>
<body>
    <nav class="navbar navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="/">Cassiopeia</a>
            <a class="nav-link text-white" href="/">Главная</a>
        </div>
    </nav>

    <div class="container mt-4">
        <h1 class="mb-4">ISS Data</h1>
        
        <div class="filter-section">
            <div class="row">
                <div class="col-md-4">
                    <div class="search-box">
                        <input type="text" id="searchInput" class="form-control" placeholder="Поиск...">
                    </div>
                </div>
                <div class="col-md-3">
                    <select id="sortColumn" class="form-select">
                        <option value="timestamp">Дата</option>
                        <option value="latitude">Широта</option>
                        <option value="longitude">Долгота</option>
                        <option value="altitude">Высота</option>
                        <option value="velocity">Скорость</option>
                    </select>
                </div>
                <div class="col-md-3">
                    <select id="sortOrder" class="form-select">
                        <option value="asc">По возрастанию</option>
                        <option value="desc">По убыванию</option>
                    </select>
                </div>
                <div class="col-md-2">
                    <button class="btn btn-primary w-100" onclick="loadData()">Обновить</button>
                </div>
            </div>
        </div>

        <div class="table-container">
            <table class="table table-striped table-hover" id="dataTable">
                <thead class="table-dark">
                    <tr>
                        <th>Дата</th>
                        <th>Широта</th>
                        <th>Долгота</th>
                        <th>Высота</th>
                        <th>Скорость</th>
                        <th>Видимость</th>
                    </tr>
                </thead>
                <tbody id="tableBody">
                    <tr>
                        <td colspan="6" class="text-center">Загрузка данных...</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <script>
        let allData = [];

        function loadData() {
            fetch('/iss/data')
                .then(response => response.json())
                .then(data => {
                    allData = data;
                    renderTable(data);
                })
                .catch(error => {
                    console.error('Error:', error);
                    document.getElementById('tableBody').innerHTML = 
                        '<tr><td colspan="6" class="text-center text-danger">Ошибка загрузки данных</td></tr>';
                });
        }

        function renderTable(data) {
            const tbody = document.getElementById('tableBody');
            if (!data || data.length === 0) {
                tbody.innerHTML = '<tr><td colspan="6" class="text-center">Нет данных</td></tr>';
                return;
            }

            tbody.innerHTML = data.map(item => `
                <tr>
                    <td>${new Date(item.timestamp).toLocaleString('ru-RU')}</td>
                    <td>${item.latitude.toFixed(4)}</td>
                    <td>${item.longitude.toFixed(4)}</td>
                    <td>${item.altitude.toFixed(2)}</td>
                    <td>${item.velocity.toFixed(2)}</td>
                    <td>${item.visibility}</td>
                </tr>
            `).join('');
        }

        function filterAndSort() {
            const searchTerm = document.getElementById('searchInput').value.toLowerCase();
            const sortColumn = document.getElementById('sortColumn').value;
            const sortOrder = document.getElementById('sortOrder').value;

            let filtered = allData.filter(item => {
                return Object.values(item).some(val => 
                    String(val).toLowerCase().includes(searchTerm)
                );
            });

            filtered.sort((a, b) => {
                let aVal = a[sortColumn];
                let bVal = b[sortColumn];
                
                if (sortColumn === 'timestamp') {
                    aVal = new Date(aVal).getTime();
                    bVal = new Date(bVal).getTime();
                }

                if (sortOrder === 'asc') {
                    return aVal > bVal ? 1 : -1;
                } else {
                    return aVal < bVal ? 1 : -1;
                }
            });

            renderTable(filtered);
        }

        document.getElementById('searchInput').addEventListener('input', filterAndSort);
        document.getElementById('sortColumn').addEventListener('change', filterAndSort);
        document.getElementById('sortOrder').addEventListener('change', filterAndSort);

        loadData();
    </script>
</body>
</html>

