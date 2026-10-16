<?php

declare(strict_types=1);

namespace Groundmail\Laravel\Tests;

use PHPUnit\Framework\TestCase;
use Groundmail\Laravel\GroundmailManager;

final class GroundmailManagerTest extends TestCase
{
    public function testRequiresApiKey(): void
    {
        $manager = new GroundmailManager(['api_key' => null]);

        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('GROUNDMAIL_API_KEY is not configured.');

        $manager->client();
    }

    public function testBuildsAndCachesClient(): void
    {
        $manager = new GroundmailManager([
            'api_key' => 'gm_test_key',
            'base_url' => 'https://api.example.com',
        ]);

        $client = $manager->client();

        self::assertSame($client, $manager->client());
    }
}
