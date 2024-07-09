package main

// glob omment
func main() {
	// INFO: this comment correct
	// test vas
	// piter
	// jj
	log.WithFields(logrus.Fields{"hello": "test"}).Print("hello, druzya %v", "text")
	log.Info("hello, piter")
	// DEBUG: mest bam
	// hello
	log.Debug("mest bam")
	// hello
	// test
	// qweqwe
	// qweqwe
	log.Info("mest bam")
	d := "qwe"
	var s string = "qwe"
	log.Info(d)

}
