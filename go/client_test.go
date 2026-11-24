package groundmail_test

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	groundmail "github.com/Groundmail/groundmail-sdks/go"
)

func TestNewRequiresStxPrefix(t *testing.T) {
	if _, err := groundmail.New("bad"); err == nil {
		t.Fatal("expected error for invalid api key prefix")
	}
}

func TestSendsItsUserAgent(t *testing.T) {
	var got string
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		got = r.Header.Get("User-Agent")
		_ = json.NewEncoder(w).Encode(map[string]any{})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}

	if _, err := client.Request(http.MethodGet, "/domains", nil, nil); err != nil {
		t.Fatal(err)
	}
	if want := "groundmail-go/" + groundmail.Version + " go/"; !strings.HasPrefix(got, want) {
		t.Fatalf("User-Agent = %q, want prefix %q", got, want)
	}

	if _, err := client.Request(http.MethodGet, "/domains", nil, map[string]string{"User-Agent": "my-app/1.0"}); err != nil {
		t.Fatal(err)
	}
	if got != "my-app/1.0" {
		t.Fatalf("a caller's User-Agent should win, got %q", got)
	}
}

func TestEmailsSend(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/emails" || r.Method != http.MethodPost {
			t.Fatalf("unexpected request: %s %s", r.Method, r.URL.Path)
		}
		if got := r.Header.Get("Authorization"); got != "Bearer gm_test" {
			t.Fatalf("unexpected auth header: %q", got)
		}
		_ = json.NewEncoder(w).Encode(map[string]any{"id": "msg_1", "status": "sent"})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}

	result, err := client.Emails.Send(map[string]any{
		"from":    "a@example.com",
		"to":      "b@example.com",
		"subject": "Hi",
		"html":    "<p>Hi</p>",
	})
	if err != nil {
		t.Fatal(err)
	}
	if result["id"] != "msg_1" {
		t.Fatalf("unexpected result: %#v", result)
	}
}

func TestErrorResponse(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		_ = json.NewEncoder(w).Encode(map[string]any{"error": map[string]any{"message": "Invalid API key"}})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}

	_, err = client.Emails.Send(map[string]any{
		"from":    "a@example.com",
		"to":      "b@example.com",
		"subject": "Hi",
		"html":    "<p>Hi</p>",
	})
	if err == nil {
		t.Fatal("expected error")
	}
	apiErr, ok := err.(*groundmail.Error)
	if !ok {
		t.Fatalf("expected *groundmail.Error, got %T", err)
	}
	if apiErr.Status != 401 || apiErr.Message != "Invalid API key" {
		t.Fatalf("unexpected error: %#v", apiErr)
	}
}

func TestValidate(t *testing.T) {
	var got map[string]any
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/validate" || r.Method != http.MethodPost {
			t.Fatalf("unexpected request: %s %s", r.Method, r.URL.Path)
		}
		if err := json.NewDecoder(r.Body).Decode(&got); err != nil {
			t.Fatal(err)
		}
		_ = json.NewEncoder(w).Encode(map[string]any{"email": "ada@gmial.com", "verdict": "invalid", "suggestion": "ada@gmail.com"})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}
	result, err := client.Validate("ada@gmial.com", nil)
	if err != nil {
		t.Fatal(err)
	}
	if result["verdict"] != "invalid" || result["suggestion"] != "ada@gmail.com" {
		t.Fatalf("unexpected result: %v", result)
	}
	if len(got) != 1 || got["email"] != "ada@gmial.com" {
		t.Fatalf("unexpected body: %v", got)
	}

	if _, err := client.Validate("ada@acme.com", &groundmail.ValidateOptions{MailboxCheck: true, Tenant: "cust_1"}); err != nil {
		t.Fatal(err)
	}
	if got["mailbox_check"] != true || got["tenant"] != "cust_1" {
		t.Fatalf("options not sent: %v", got)
	}
}

func TestValidateBatch(t *testing.T) {
	var got map[string]any
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/validate/batch" || r.Method != http.MethodPost {
			t.Fatalf("unexpected request: %s %s", r.Method, r.URL.Path)
		}
		if err := json.NewDecoder(r.Body).Decode(&got); err != nil {
			t.Fatal(err)
		}
		_ = json.NewEncoder(w).Encode(map[string]any{"data": []any{}})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}
	if _, err := client.ValidateBatch([]string{"a@acme.com", "b@acme.com"}, &groundmail.ValidateOptions{MailboxCheck: true}); err != nil {
		t.Fatal(err)
	}
	emails, _ := got["emails"].([]any)
	if len(emails) != 2 || emails[0] != "a@acme.com" || got["mailbox_check"] != true {
		t.Fatalf("unexpected body: %v", got)
	}
	if _, ok := got["tenant"]; ok {
		t.Fatalf("tenant should be left out when empty: %v", got)
	}
}

func TestValidateError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		_ = json.NewEncoder(w).Encode(map[string]any{"error": map[string]any{"message": "Invalid API key", "code": "unauthorized"}})
	}))
	defer server.Close()

	client, err := groundmail.NewClient("gm_test", server.URL, server.Client())
	if err != nil {
		t.Fatal(err)
	}
	if _, err := client.Validate("a@b.com", nil); err == nil {
		t.Fatal("expected an error for a 401")
	}
}
