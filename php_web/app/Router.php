<?php

namespace App;

class Router {
    private $routes = [];

    public function get($path, $handler) {
        $this->routes['GET'][$path] = $handler;
    }

    public function post($path, $handler) {
        $this->routes['POST'][$path] = $handler;
    }

    public function dispatch() {
        $method = $_SERVER['REQUEST_METHOD'] ?? 'GET';
        $path = parse_url($_SERVER['REQUEST_URI'] ?? '/', PHP_URL_PATH);

        // Убираем trailing slash для консистентности
        $path = rtrim($path, '/');
        if (empty($path)) {
            $path = '/';
        }

        if (isset($this->routes[$method][$path])) {
            [$controller, $method] = explode('@', $this->routes[$method][$path]);
            $controllerClass = "App\\Http\\Controllers\\{$controller}";
            
            if (class_exists($controllerClass)) {
                $controllerInstance = new $controllerClass();
                if (method_exists($controllerInstance, $method)) {
                    return $controllerInstance->$method();
                }
            }
        }

        http_response_code(404);
        echo json_encode(['error' => 'Not found', 'path' => $path]);
    }
    
    public function getRoutes() {
        return $this->routes;
    }
}

