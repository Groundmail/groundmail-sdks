<?php

declare(strict_types=1);

namespace Groundmail\Symfony\Transport;

use Groundmail\Client;
use Groundmail\GroundmailError;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mailer\Transport\AbstractTransport;
use Symfony\Component\Mime\MessageConverter;

class GroundmailTransport extends AbstractTransport
{
    public function __construct(
        private readonly Client $client,
        private readonly MessageMapper $mapper = new MessageMapper(),
    ) {
        parent::__construct();
    }

    protected function doSend(SentMessage $message): void
    {
        $email = MessageConverter::toEmail($message->getOriginalMessage());

        try {
            $result = $this->client->emails->send($this->mapper->toSendParams($email));
        } catch (GroundmailError $e) {
            throw new TransportException(
                'Groundmail API error: '.$e->getMessage(),
                $e->getCode(),
                $e,
            );
        } catch (\InvalidArgumentException $e) {
            throw new TransportException($e->getMessage(), 0, $e);
        }

        $id = $result['id'] ?? null;
        if (is_string($id) && $id !== '') {
            $message->setMessageId($id);
        }
    }

    public function __toString(): string
    {
        return 'groundmail';
    }
}
