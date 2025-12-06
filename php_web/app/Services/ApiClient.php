<?php

namespace App\Services;

use GuzzleHttp\Client;

class ApiClient {
    private $client;

    public function __construct() {
        $this->client = new Client([
            'timeout' => 30,
            'verify' => false
        ]);
    }

    public function get($url) {
        try {
            $response = $this->client->get($url);
            return json_decode($response->getBody(), true);
        } catch (\Exception $e) {
            return ['error' => $e->getMessage()];
        }
    }

    public function post($url, $data = []) {
        try {
            $response = $this->client->post($url, [
                'json' => $data
            ]);
            return json_decode($response->getBody(), true);
        } catch (\Exception $e) {
            return ['error' => $e->getMessage()];
        }
    }
}

