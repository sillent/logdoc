package main

func main() {
	// hello
	// bye
	// qw
	log.With(log.Pis{"hello": "mallo"}).Info("hello")
	// bye
	// qwe
	log.Infol("hello")
	// zxcz
	// asd
	log.Info("bye")
	log.Info("q")
	// test
}
