<?php

namespace App\Http\Controllers;

class DashboardController {
    public function index() {
        $this->renderView('dashboard', [
            'title' => 'Cassiopeia ISS Data Dashboard'
        ]);
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

