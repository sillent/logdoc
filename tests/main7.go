package main
// glob omment
func main() {
	// Info: this comment correct
    // test vas
    // 
    // jj
    log.WithFields(logrus.Fields{"hello": "test"}).Print("hello, druzya %v", "text")
    log.Info("hello, piter")
    // Debug: mest bam
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

