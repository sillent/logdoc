def main
  # test info
  # test subject 1
  # test subject 2
  logger.info("test")
  # test debug
  logger.debug("debug")
  # test error
  # test subject - message with argument
  logger.error("error message", argument)
  # this test not appear
  logger.fiction("fiction")
end

# outer log
logger.warn("warning message")
