# Registering an Application to a URI Scheme

## Windows

On Windows these are known as Asynchronous Pluggable Protocols. The simplest
approach is to add a registry key that invokes a command-line tool via the
shell.

There are several places within Windows where a custom URI might be useful:

- resolving links within browsers
- resolving links from the window "Run..." start-menu action. (Win+R)
- adding links to the Windows Search interface

Protocol handlers for Windows Search use a different mechanism and are
out-of-scope for this exercise.

### command line invocation

```data
HKEY_CLASSES_ROOT
   alert
      (Default) = "URL:Alert Protocol"
      URL Protocol = ""
      DefaultIcon
         (Default) = "alert.exe,1"
      shell
         open
            command
               (Default) = "C:\Program Files\Alert\alert.exe" "%1"
```

Will invoke:

```powershell
"C:\Program Files\Alert\alert.exe" "alert:Hello%20World"
or
"C:\Program Files\Alert\alert.exe" "alert:Hello World"
```

Apparently, whether the argument is url-encoded or not depends on where the link
is coming from. *Applications have to handle both*.

> By adding the above settings to the registry, navigating to URIs such as
> alert:Hello%20World would cause an attempt to launch alert.exe with the
> complete URI on the command line. Internet Explorer percent-decodes the URI,
> but the Windows Run... command does not. If a URI contains percent-encoded
> spaces, it may be split across more than one argument on the command line.

### Other mechanisms

There are more complex options:

- *pluggable mime filters* receive data through a stream, transforming and returning a stream.
- *pluggable async handler* Registers a COM object which is able to return a response.

### Questions

- [ ] use of "UseOriginalUrlEncoding" by e.g. zoommtg

## References

1. [MSDN](https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa767914(v=vs.85)?redirectedfrom=MSDN)
2. Windows specific [stackoverflow thread](https://docs.microsoft.com/en-us/previous-versions/windows/internet-explorer/ie-developer/platform-apis/aa767914(v=vs.85))
3. Cross platform [guide](https://support.shotgunsoftware.com/hc/en-us/articles/219031308-Launching-applications-using-custom-browser-protocols)
