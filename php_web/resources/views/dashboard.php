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
            <a class="navbar-brand" href="/">Cassiopeia ISS Dashboard</a>
            <div class="navbar-nav flex-row">
                <a class="nav-link me-3" href="/iss">ISS</a>
                <a class="nav-link me-3" href="/osdr">OSDR</a>
                <a class="nav-link me-3" href="/jwst">JWST</a>
                <a class="nav-link" href="/astronomy">Astronomy</a>
            </div>
        </div>
    </nav>

    <div class="container mt-4">
        <div class="row mb-4">
            <div class="col-md-3">
                <div class="card mb-3 animate-fade-in">
                    <div class="card-body">
                        <h5 class="card-title">ISS Data</h5>
                        <p class="card-text">Международная космическая станция</p>
                        <a href="/iss" class="btn btn-primary">Перейти</a>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card mb-3 animate-fade-in" style="animation-delay: 0.1s">
                    <div class="card-body">
                        <h5 class="card-title">OSDR Data</h5>
                        <p class="card-text">NASA Open Science Data Repository</p>
                        <a href="/osdr" class="btn btn-primary">Перейти</a>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card mb-3 animate-fade-in" style="animation-delay: 0.2s">
                    <div class="card-body">
                        <h5 class="card-title">JWST Data</h5>
                        <p class="card-text">James Webb Space Telescope</p>
                        <a href="/jwst" class="btn btn-primary">Перейти</a>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card mb-3 animate-fade-in" style="animation-delay: 0.3s">
                    <div class="card-body">
                        <h5 class="card-title">Astronomy Data</h5>
                        <p class="card-text">Астрономические данные</p>
                        <a href="/astronomy" class="btn btn-primary">Перейти</a>
                    </div>
                </div>
            </div>
        </div>
        
        <div class="row">
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
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <script>
        // Инициализация карты
        const map = L.map('issMap').setView([0, 0], 2);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '© OpenStreetMap contributors'
        }).addTo(map);

        // Загрузка данных ISS
        fetch('/iss/data')
            .then(response => response.json())
            .then(data => {
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
                            Velocity: ${latest.velocity.toFixed(2)} km/h
                        `);
                    map.setView([latest.latitude, latest.longitude], 3);
                    
                    // Показываем траекторию (последние 10 точек)
                    const recent = data.slice(0, 10).reverse();
                    const polyline = L.polyline(
                        recent.map(p => [p.latitude, p.longitude]),
                        {color: 'red', weight: 2}
                    ).addTo(map);
                }
            })
            .catch(error => {
                console.error('Error loading ISS data:', error);
            });
    </script>
</body>
</html>

