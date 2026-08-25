package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"
)

func TestHealthEndpoint(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	recorder := httptest.NewRecorder()
	newRouter("test").ServeHTTP(recorder, req)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", recorder.Code)
	}

	var payload HealthResponse
	if err := json.Unmarshal(recorder.Body.Bytes(), &payload); err != nil {
		t.Fatalf("decode health response: %v", err)
	}
	if payload.Status != "ok" || payload.Service != "cat-gateway" || payload.Version != "test" {
		t.Fatalf("unexpected health payload: %+v", payload)
	}
	if _, err := time.Parse(time.RFC3339Nano, payload.Timestamp); err != nil {
		t.Fatalf("invalid health timestamp: %v", err)
	}
}

func TestReadinessEndpoint(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/ready", nil)
	recorder := httptest.NewRecorder()
	newRouter("test").ServeHTTP(recorder, req)

	if recorder.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", recorder.Code)
	}
}

func TestNotFoundAndSecurityHeaders(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/missing", nil)
	recorder := httptest.NewRecorder()
	newRouter("test").ServeHTTP(recorder, req)

	if recorder.Code != http.StatusNotFound {
		t.Fatalf("expected 404, got %d", recorder.Code)
	}
	if recorder.Header().Get("X-Content-Type-Options") != "nosniff" {
		t.Fatal("missing nosniff header")
	}
	if recorder.Header().Get("X-Frame-Options") != "DENY" {
		t.Fatal("missing frame protection header")
	}
	if recorder.Header().Get("Referrer-Policy") != "no-referrer" {
		t.Fatal("missing referrer policy")
	}
}

func TestRequestIDIsPreservedOrGenerated(t *testing.T) {
	router := newRouter("test")

	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	req.Header.Set("X-Request-ID", "req-123")
	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, req)
	if recorder.Header().Get("X-Request-ID") != "req-123" {
		t.Fatal("request id was not preserved")
	}

	req = httptest.NewRequest(http.MethodGet, "/health", nil)
	recorder = httptest.NewRecorder()
	router.ServeHTTP(recorder, req)
	if recorder.Header().Get("X-Request-ID") == "" {
		t.Fatal("request id was not generated")
	}
}

func TestLoadConfigEnvironmentOverride(t *testing.T) {
	t.Setenv("CAT_GATEWAY_ADDR", "127.0.0.1:9090")
	t.Setenv("CAT_GATEWAY_READ_TIMEOUT_MS", "1200")
	config := LoadConfig()

	if config.Address != "127.0.0.1:9090" {
		t.Fatalf("unexpected address: %s", config.Address)
	}
	if config.ReadTimeout != 1200*time.Millisecond {
		t.Fatalf("unexpected read timeout: %s", config.ReadTimeout)
	}

	_ = os.Getenv("CAT_GATEWAY_ADDR")
}
