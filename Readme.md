# Logdoc

Format special block of comments in the code into Markdown files

Support severity of logs:

- **info**
- **debug**
- **trace**
- **warn**
- **fatal**

Create special file for each severity.

Support languages:

- **golang**
- **c**
- **c++**
- **python**
- **java**
- **javascript**
- **ruby**
- **rust**

Example:

```golang
func main() {
  ...
  // Info: create special unit
  // just notify that special unit is created
  // nothing to do, it's just informational log
  logrus.Info("create special unit")
  ...
}
```

create file **info.md** with table

```markdown
|error message|subject|description|
|---|---|---|
|create special unit|just notify that special unit is created|nothing to do, it's just informational log|
```

You can add more information into document - set environment `INFO_DESC`, `DEBUG_DESC`, `TRACE_DESC`, `WARN_DESC` and `FATAL_DESC`.

Or change table header - set environment `MESSAGE_TABLE_HEADER`, `SUBJECT_TABLE_HEADER` or `DESCRIPTION_TABLE_HEADER`

# Sample usage
```bash
logdoc -l golang -p project_name  -d project_dir/ 
```


