<?php

require_once __DIR__ . '/../vendor/autoload.php';

use App\Router;

$router = new Router();

// ISS Routes
$router->get('/iss', 'IssController@index');
$router->get('/iss/data', 'IssController@getData');
$router->get('/iss/latest', 'IssController@getLatest');

// OSDR Routes
$router->get('/osdr', 'OsdrController@index');
$router->get('/osdr/data', 'OsdrController@getData');

// JWST Routes
$router->get('/jwst', 'JwstController@index');
$router->get('/jwst/data', 'JwstController@getData');

// Astronomy Routes
$router->get('/astronomy', 'AstronomyController@index');
$router->get('/astronomy/data', 'AstronomyController@getData');

// Dashboard
$router->get('/', 'DashboardController@index');

$router->dispatch();

