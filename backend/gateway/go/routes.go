package main

import (
	"net/http"
	"strings"
	"time"
)

func newRouter(version string) http.Handler {
	mux := http.NewServeMux()
	mux.Handle("GET /health", healthHandler(version))
	mux.Handle("GET /ready", readinessHandler())
	mux.Handle("GET /live", readinessHandler())
	mux.HandleFunc("GET /", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/" {
			writeJSON(w, http.StatusNotFound, map[string]string{"error": "not_found"})
			return
		}
		writeJSON(w, http.StatusOK, map[string]string{"service": "cat-gateway", "status": "ok"})
	})
	return requestMiddleware(mux)
}

func requestMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		requestID := strings.TrimSpace(r.Header.Get("X-Request-ID"))
		if requestID == "" {
			requestID = time.Now().UTC().Format("20060102T150405.000000000Z07:00")
		}
		w.Header().Set("X-Request-ID", requestID)
		w.Header().Set("X-Content-Type-Options", "nosniff")
		w.Header().Set("X-Frame-Options", "DENY")
		w.Header().Set("Referrer-Policy", "no-referrer")
		next.ServeHTTP(w, r)
		_ = start
	})
}
