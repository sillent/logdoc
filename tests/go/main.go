package main

// glob omment
func main() {
	// Test log subj
	// Test log descr 1
	// test log descr 2
	log.Info("test log")
	// Test debug subj
	// Test debug log descr 1
	log.Debug("test debug log")
	// Test debug too subj
	// this message is debug level
	log.Debug("test debug too")
	// this part is skipped
	log.Info("skipped log")
	// undefined behavior
	// call admin
	log.Fatal("just fatal log")
	// only subject
	log.Info("other info log")
}
