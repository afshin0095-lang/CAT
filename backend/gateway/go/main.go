package main

import (
	"log/slog"
	"os"
	"os/signal"
	"syscall"
)

const version = "0.1.0"

func main() {
	config := LoadConfig()
	server := NewServer(config, version)

	go func() {
		slog.Info("CAT gateway listening", "address", config.Address, "version", version)
		if err := server.ListenAndServe(); err != nil {
			slog.Error("CAT gateway stopped", "error", err)
			os.Exit(1)
		}
	}()

	signals := make(chan os.Signal, 1)
	signal.Notify(signals, syscall.SIGINT, syscall.SIGTERM)
	<-signals

	if err := server.Shutdown(); err != nil {
		slog.Error("CAT gateway shutdown failed", "error", err)
		os.Exit(1)
	}
}
