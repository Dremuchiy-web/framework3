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
        <h1 class="mb-4">OSDR Data - NASA Open Science Data Repository</h1>
        
        <div class="filter-section">
            <div class="row">
                <div class="col-md-4">
                    <input type="text" id="searchInput" class="form-control mb-2" placeholder="Поиск по названию...">
                </div>
                <div class="col-md-4">
                    <select id="sortColumn" class="form-select mb-2">
                        <option value="dataset_id">ID набора данных</option>
                        <option value="title">Название</option>
                        <option value="file_count">Количество файлов</option>
                        <option value="size_bytes">Размер</option>
                        <option value="created_at">Дата создания</option>
                    </select>
                </div>
                <div class="col-md-4">
                    <select id="sortOrder" class="form-select mb-2">
                        <option value="asc">По возрастанию</option>
                        <option value="desc">По убыванию</option>
                    </select>
                </div>
            </div>
        </div>

        <div class="table-responsive">
            <table class="table table-hover table-striped mt-3">
                <thead class="table-dark">
                    <tr>
                        <th>ID набора данных</th>
                        <th>Название</th>
                        <th>Описание</th>
                        <th>Количество файлов</th>
                        <th>Размер (байты)</th>
                        <th>Дата создания</th>
                    </tr>
                </thead>
                <tbody id="osdrTableBody">
                    <tr>
                        <td colspan="6" class="text-center">Загрузка данных...</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>

    <script>
        document.addEventListener('DOMContentLoaded', function () {
            const osdrTableBody = document.getElementById('osdrTableBody');
            const searchInput = document.getElementById('searchInput');
            const sortColumn = document.getElementById('sortColumn');
            const sortOrder = document.getElementById('sortOrder');
            let allOsdrData = [];

            async function fetchOsdrData() {
                try {
                    const response = await fetch('http://rust_iss:3000/api/osdr');
                    const data = await response.json();
                    allOsdrData = data;
                    renderTable();
                } catch (error) {
                    console.error('Error fetching OSDR data:', error);
                    osdrTableBody.innerHTML = `<tr><td colspan="6" class="text-center text-danger">Ошибка загрузки данных: ${error.message}</td></tr>`;
                }
            }

            function renderTable() {
                let filteredData = allOsdrData.filter(item => {
                    const searchTerm = searchInput.value.toLowerCase();
                    const matchesSearch = (item.title || '').toLowerCase().includes(searchTerm) ||
                                         (item.dataset_id || '').toLowerCase().includes(searchTerm) ||
                                         (item.description || '').toLowerCase().includes(searchTerm);
                    return matchesSearch;
                });

                filteredData.sort((a, b) => {
                    let aVal = a[sortColumn.value];
                    let bVal = b[sortColumn.value];
                    
                    if (sortColumn.value === 'created_at') {
                        aVal = new Date(aVal).getTime();
                        bVal = new Date(bVal).getTime();
                    } else if (sortColumn.value === 'size_bytes' || sortColumn.value === 'file_count') {
                        aVal = Number(aVal) || 0;
                        bVal = Number(bVal) || 0;
                    } else {
                        aVal = String(aVal || '').toLowerCase();
                        bVal = String(bVal || '').toLowerCase();
                    }

                    if (sortOrder.value === 'asc') {
                        return aVal > bVal ? 1 : -1;
                    } else {
                        return aVal < bVal ? 1 : -1;
                    }
                });

                osdrTableBody.innerHTML = '';
                if (filteredData.length === 0) {
                    osdrTableBody.innerHTML = `<tr><td colspan="6" class="text-center">Нет данных для отображения.</td></tr>`;
                    return;
                }

                filteredData.forEach(item => {
                    const row = osdrTableBody.insertRow();
                    row.insertCell().textContent = item.dataset_id || 'N/A';
                    row.insertCell().textContent = item.title || 'N/A';
                    row.insertCell().textContent = (item.description || 'N/A').substring(0, 100) + (item.description && item.description.length > 100 ? '...' : '');
                    row.insertCell().textContent = item.file_count !== null ? item.file_count : 'N/A';
                    row.insertCell().textContent = item.size_bytes !== null ? item.size_bytes.toLocaleString() : 'N/A';
                    row.insertCell().textContent = item.created_at ? new Date(item.created_at).toLocaleDateString('ru-RU') : 'N/A';
                });
            }

            searchInput.addEventListener('input', renderTable);
            sortColumn.addEventListener('change', renderTable);
            sortOrder.addEventListener('change', renderTable);

            fetchOsdrData();
        });
    </script>
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
