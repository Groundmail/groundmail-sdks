<?php

declare(strict_types=1);

namespace Groundmail\Laravel\Facades;

use Illuminate\Support\Facades\Facade;
use Groundmail\Client;

/**
 * @method static Client client()
 *
 * @see \Groundmail\Laravel\GroundmailManager
 */
final class Groundmail extends Facade
{
    protected static function getFacadeAccessor(): string
    {
        return 'groundmail';
    }
}
