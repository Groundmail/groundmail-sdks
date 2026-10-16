<?php

declare(strict_types=1);

namespace Groundmail\Symfony\Transport;

use Groundmail\Client;
use Symfony\Component\Mailer\Exception\UnsupportedSchemeException;
use Symfony\Component\Mailer\Transport\AbstractTransportFactory;
use Symfony\Component\Mailer\Transport\Dsn;
use Symfony\Component\Mailer\Transport\TransportInterface;

/**
 * DSN schemes: groundmail://default?api_key=gm_…  or  groundmail+api://gm_…@default
 */
final class GroundmailTransportFactory extends AbstractTransportFactory
{
    public function create(Dsn $dsn): TransportInterface
    {
        if (!\in_array($dsn->getScheme(), $this->getSupportedSchemes(), true)) {
            throw new UnsupportedSchemeException($dsn, 'groundmail', $this->getSupportedSchemes());
        }

        $apiKey = $dsn->getUser() ?: $dsn->getOption('api_key');
        if (!\is_string($apiKey) || $apiKey === '') {
            $apiKey = getenv('GROUNDMAIL_API_KEY') ?: '';
        }
        if ($apiKey === '') {
            throw new \InvalidArgumentException(
                'Groundmail Mailer DSN requires an API key (userinfo, api_key query, or GROUNDMAIL_API_KEY).',
            );
        }

        $baseUrl = $dsn->getOption('base_url');
        if (!\is_string($baseUrl) || $baseUrl === '') {
            $baseUrl = 'https://api.groundmail.dev';
        }

        return new GroundmailTransport(new Client($apiKey, $baseUrl));
    }

    protected function getSupportedSchemes(): array
    {
        return ['groundmail', 'groundmail+api'];
    }
}
