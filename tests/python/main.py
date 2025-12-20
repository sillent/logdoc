import "logger"
# outer log
# subject of outer log
logger.info("outer log")
def main:
    # test message from python
    # just info message
    # nothing to do
    logger.info("info test message")
    # debug second test message from python
    # also just info message
    logger.warn("warn test message from python")
    # trace message from python
    # emergency, do something
    logger.trace("trace message from python")
    # fatal log
    logger.fatal("fatal message")
