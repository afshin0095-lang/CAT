package main

import (
	"os"
	"strconv"
	"time"
)

type Config struct {
	Address           string
	ReadHeaderTimeout time.Duration
	ReadTimeout       time.Duration
	WriteTimeout      time.Duration
	IdleTimeout       time.Duration
	ShutdownTimeout   time.Duration
}

func LoadConfig() Config {
	return Config{
		Address:           envString("CAT_GATEWAY_ADDR", ":8080"),
		ReadHeaderTimeout: envDuration("CAT_GATEWAY_READ_HEADER_TIMEOUT_MS", 5000),
		ReadTimeout:       envDuration("CAT_GATEWAY_READ_TIMEOUT_MS", 15000),
		WriteTimeout:      envDuration("CAT_GATEWAY_WRITE_TIMEOUT_MS", 15000),
		IdleTimeout:       envDuration("CAT_GATEWAY_IDLE_TIMEOUT_MS", 60000),
		ShutdownTimeout:   envDuration("CAT_GATEWAY_SHUTDOWN_TIMEOUT_MS", 10000),
	}
}

func envString(key, fallback string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return fallback
}

func envDuration(key string, fallbackMS int) time.Duration {
	value := os.Getenv(key)
	if value == "" {
		return time.Duration(fallbackMS) * time.Millisecond
	}
	ms, err := strconv.Atoi(value)
	if err != nil || ms <= 0 {
		return time.Duration(fallbackMS) * time.Millisecond
	}
	return time.Duration(ms) * time.Millisecond
}
