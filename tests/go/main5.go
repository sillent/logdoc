package main

func main() {
	s := "hello"
	b := "bye"
	func() {
		// bye
		// qw
		log.Info(s)
	}
	// bye
	// qwe
	log.Infol(b)
	// log
	log.Info("hello")
}
