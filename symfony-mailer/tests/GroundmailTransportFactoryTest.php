<?php

declare(strict_types=1);

namespace Groundmail\Symfony\Tests;

use PHPUnit\Framework\TestCase;
use Groundmail\Symfony\Transport\GroundmailTransport;
use Groundmail\Symfony\Transport\GroundmailTransportFactory;
use Symfony\Component\Mailer\Transport\Dsn;

final class GroundmailTransportFactoryTest extends TestCase
{
    public function testCreatesTransportFromApiScheme(): void
    {
        $factory = new GroundmailTransportFactory();
        $transport = $factory->create(Dsn::fromString('groundmail+api://gm_test_key@default'));

        self::assertInstanceOf(GroundmailTransport::class, $transport);
        self::assertSame('groundmail', (string) $transport);
    }

    public function testCreatesTransportFromQueryApiKey(): void
    {
        $factory = new GroundmailTransportFactory();
        $transport = $factory->create(Dsn::fromString('groundmail://default?api_key=gm_query_key'));

        self::assertInstanceOf(GroundmailTransport::class, $transport);
    }

    public function testRequiresApiKey(): void
    {
        $prev = getenv('GROUNDMAIL_API_KEY');
        putenv('GROUNDMAIL_API_KEY');
        try {
            $factory = new GroundmailTransportFactory();
            $this->expectException(\InvalidArgumentException::class);
            $factory->create(Dsn::fromString('groundmail://default'));
        } finally {
            if ($prev === false) {
                putenv('GROUNDMAIL_API_KEY');
            } else {
                putenv('GROUNDMAIL_API_KEY='.$prev);
            }
        }
    }
}
