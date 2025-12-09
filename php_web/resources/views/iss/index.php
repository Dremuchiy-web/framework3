<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?= $title ?></title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
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

        <div class="row mb-4">
            <div class="col-12">
                <div class="card">
                    <div class="card-header">
                        <h5 class="mb-0">ISS Location Map</h5>
                    </div>
                    <div class="card-body">
                        <div id="issMap" style="height: 500px; width: 100%;"></div>
                    </div>
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

        // Инициализация карты Leaflet
        const map = L.map('issMap').setView([0, 0], 2);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '© OpenStreetMap contributors'
        }).addTo(map);

        let markers = [];
        let polyline = null;

        function updateMap(data) {
            // Удаляем старые маркеры
            markers.forEach(m => map.removeLayer(m));
            markers = [];
            
            if (polyline) {
                map.removeLayer(polyline);
            }

            if (data && data.length > 0) {
                // Показываем последнюю позицию
                const latest = data[0];
                const marker = L.marker([latest.latitude, latest.longitude])
                    .addTo(map)
                    .bindPopup(`
                        <b>ISS Position</b><br>
                        Latitude: ${latest.latitude.toFixed(4)}<br>
                        Longitude: ${latest.longitude.toFixed(4)}<br>
                        Altitude: ${latest.altitude.toFixed(2)} km<br>
                        Velocity: ${latest.velocity.toFixed(2)} km/h<br>
                        Time: ${new Date(latest.timestamp).toLocaleString('ru-RU')}
                    `);
                markers.push(marker);
                map.setView([latest.latitude, latest.longitude], 3);
                
                // Показываем траекторию (последние 20 точек)
                const recent = data.slice(0, 20).reverse();
                polyline = L.polyline(
                    recent.map(p => [p.latitude, p.longitude]),
                    {color: 'red', weight: 2, opacity: 0.7}
                ).addTo(map);
            }
        }

        // Обновляем карту при загрузке данных
        const originalLoadData = loadData;
        loadData = function() {
            originalLoadData();
            fetch('/iss/data')
                .then(response => response.json())
                .then(data => {
                    updateMap(data);
                })
                .catch(error => {
                    console.error('Error loading map data:', error);
                });
        };

        loadData();
    </script>
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
</body>
</html>

