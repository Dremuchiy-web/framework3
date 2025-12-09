<?php

namespace App\Http\Controllers;

use App\Services\ApiClient;

class AstronomyController {
    private $apiClient;

    public function __construct() {
        $this->apiClient = new ApiClient();
    }

    public function index() {
        $this->renderView('astronomy/index', [
            'title' => 'Astronomy Data'
        ]);
    }

    public function getData() {
        header('Content-Type: application/json');
        $data = $this->apiClient->get('http://rust_iss:3000/api/astronomy');
        echo json_encode($data);
    }

    private function renderView($view, $data = []) {
        extract($data);
        $viewPath = __DIR__ . '/../../../resources/views/' . $view . '.php';
        if (file_exists($viewPath)) {
            include $viewPath;
        } else {
            echo "View not found: $viewPath";
        }
    }
}

