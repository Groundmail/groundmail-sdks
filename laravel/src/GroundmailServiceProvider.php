<?php

declare(strict_types=1);

namespace Groundmail\Laravel;

use Illuminate\Support\Facades\Mail;
use Illuminate\Support\ServiceProvider;
use Groundmail\Client;
use Groundmail\Symfony\Transport\GroundmailTransport;

class GroundmailServiceProvider extends ServiceProvider
{
    public function register(): void
    {
        $this->mergeConfigFrom(__DIR__.'/../config/groundmail.php', 'groundmail');

        $this->app->singleton(GroundmailManager::class, function ($app) {
            return new GroundmailManager($app['config']['groundmail']);
        });

        $this->app->alias(GroundmailManager::class, 'groundmail');

        $this->app->bind(Client::class, function ($app) {
            return $app->make(GroundmailManager::class)->client();
        });
    }

    public function boot(): void
    {
        if ($this->app->runningInConsole()) {
            $this->publishes([
                __DIR__.'/../config/groundmail.php' => config_path('groundmail.php'),
            ], 'groundmail-config');
        }

        if (!$this->app->bound('mail.manager')) {
            return;
        }

        Mail::extend('groundmail', function () {
            return new GroundmailTransport(
                $this->app->make(GroundmailManager::class)->client(),
            );
        });
    }
}
