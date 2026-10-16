# frozen_string_literal: true

require "minitest/autorun"
require "groundmail"

class ClientTest < Minitest::Test
  def test_requires_gm_prefix
    error = assert_raises(ArgumentError) { Groundmail::Client.new("bad") }
    assert_match(/gm_/, error.message)
  end

  def test_emails_send
    last_request = nil

    client = Groundmail::Client.new(
      "gm_test_key",
      base_url: "https://api.example.com",
      transport: lambda do |method, path, body, headers|
        last_request = { method: method, path: path, body: body, headers: headers }
        { "id" => "msg_1", "status" => "sent" }
      end
    )

    result = client.emails.send(
      from: "a@example.com",
      to: "b@example.com",
      subject: "Hi",
      html: "<p>Hi</p>"
    )

    assert_equal({ "id" => "msg_1", "status" => "sent" }, result)
    assert_equal "POST", last_request[:method]
    assert_equal "/emails", last_request[:path]
    assert_equal "Bearer gm_test_key", last_request[:headers]["Authorization"]
  end

  def test_http_error_raises_super_send_tx_error
    client = Groundmail::Client.new(
      "gm_test_key",
      base_url: "https://api.example.com",
      transport: lambda do |_method, _path, _body, _headers|
        raise Groundmail::Error.from_response(401, { "error" => { "message" => "Invalid API key" } })
      end
    )

    error = assert_raises(Groundmail::Error) do
      client.emails.send(
        from: "a@example.com",
        to: "b@example.com",
        subject: "Hi",
        html: "<p>Hi</p>"
      )
    end

    assert_equal 401, error.status
    assert_equal "Invalid API key", error.message
  end
end
