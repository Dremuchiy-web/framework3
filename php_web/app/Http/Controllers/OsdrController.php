<?php

namespace App\Http\Controllers;

use App\Services\ApiClient;

class OsdrController {
    private $apiClient;

    public function __construct() {
        $this->apiClient = new ApiClient();
    }

    public function index() {
        $this->renderView('osdr/index', [
            'title' => 'OSDR Data'
        ]);
    }

    public function getData() {
        header('Content-Type: application/json');
        $data = $this->apiClient->get('http://rust_iss:3000/api/osdr');
        echo json_encode($data);
    }

    private function renderView($view, $data = []) {
        extract($data);
        include __DIR__ . '/../../resources/views/' . $view . '.php';
    }
}

