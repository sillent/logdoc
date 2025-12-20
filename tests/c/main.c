#include <stdio.h>
// glob omment
void main() {
	// Test info log - subject
	// description 1
	// description 2
	log_info("test info log");
	// Test info log 2 - subject
	// description 1
	log_info("test info log 2");
	// just debug subject
	// description 1
	// description 2
	// description 3
	log_debug("test debug log");
	// Test error log - subject
	// this message is debug level
	log_error("test debug too");
	// Test trace log - subject
	log_trace("skipped log");
	// Test fatal log - subject
	// Test fatal description
	log_fatal("just fatal log");
	// Test warn log - subject (only subject)
	log_warn("other info log");
	// Log not supported
	log_other("error on process");
}
