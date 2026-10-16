<?php

declare(strict_types=1);

namespace Groundmail\Tests;

use PHPUnit\Framework\TestCase;
use Groundmail\Client;
use Groundmail\GroundmailError;

final class ClientTest extends TestCase
{
    public function testRequiresStxPrefix(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('gm_');

        new Client('bad');
    }

    public function testEmailsSend(): void
    {
        $lastRequest = null;

        $client = new Client(
            'gm_test_key',
            'https://api.example.com',
            function (string $method, string $path, ?array $body, array $headers) use (&$lastRequest): array {
                $lastRequest = compact('method', 'path', 'body', 'headers');

                return ['id' => 'msg_1', 'status' => 'sent'];
            },
        );

        $result = $client->emails->send([
            'from' => 'a@example.com',
            'to' => 'b@example.com',
            'subject' => 'Hi',
            'html' => '<p>Hi</p>',
        ]);

        self::assertSame(['id' => 'msg_1', 'status' => 'sent'], $result);
        self::assertSame('POST', $lastRequest['method']);
        self::assertSame('/emails', $lastRequest['path']);
        self::assertSame('Bearer gm_test_key', $lastRequest['headers']['Authorization'] ?? null);
    }

    public function testEmailsSendIncludesAttachmentsAndIdempotency(): void
    {
        $lastRequest = null;

        $client = new Client(
            'gm_test_key',
            'https://api.example.com',
            function (string $method, string $path, ?array $body, array $headers) use (&$lastRequest): array {
                $lastRequest = compact('method', 'path', 'body', 'headers');

                return ['id' => 'msg_2', 'status' => 'queued'];
            },
        );

        $client->emails->send([
            'from' => 'a@example.com',
            'to' => 'b@example.com',
            'subject' => 'Hi',
            'html' => '<p>Hi</p>',
            'attachments' => [
                [
                    'filename' => 'note.txt',
                    'content_type' => 'text/plain',
                    'content' => base64_encode('hello'),
                ],
            ],
            'idempotency_key' => 'idem-1',
        ]);

        self::assertSame('idem-1', $lastRequest['headers']['Idempotency-Key'] ?? null);
        self::assertSame(
            [
                [
                    'filename' => 'note.txt',
                    'content_type' => 'text/plain',
                    'content' => base64_encode('hello'),
                ],
            ],
            $lastRequest['body']['attachments'] ?? null,
        );
    }

    public function testHttpErrorRaisesGroundmailError(): void
    {
        $client = new Client(
            'gm_test_key',
            'https://api.example.com',
            function (): array {
                throw GroundmailError::fromResponse(401, ['error' => ['message' => 'Invalid API key']]);
            },
        );

        $this->expectException(GroundmailError::class);
        $this->expectExceptionMessage('Invalid API key');

        $client->emails->send([
            'from' => 'a@example.com',
            'to' => 'b@example.com',
            'subject' => 'Hi',
            'html' => '<p>Hi</p>',
        ]);
    }
}
