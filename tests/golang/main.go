package main

import (
	"github.com/sirupsen/logrus"
)

// glob omment
func main() {
	log := logrus.WithField("log", "log")
	// Test info log - subject
	// description 1
	// description 2
	log.Info("test info log")
	// Test info log 2 - subject
	// description 1
	log.WithField("test", "test").
		Info("test info log 2")
	log.WithField("field", "message").Info("invalid error")
	// just debug subject
	// description 1
	// description 2
	// description 3
	log.Debug("test debug log")
	// Test error log - subject
	// this message is debug level
	log.Error("test debug too")
	// Test trace log - subject
	log.Trace("skipped log")
	// Test fatal log - subject
	// Test fatal description
	log.Fatal("just fatal log")
	// Test warn log - subject (only subject)
	log.Warn("other info log")
	// Log not supported
	log.Other("error on process")
	log.Info("log without subject/description - not processed")
}
