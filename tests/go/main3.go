package main

func main() {
	log.With(log.Pis{"hello": "mallo"}).Info("hello")
	// test
}
