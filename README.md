# text_http_client

Is a personal project and there are probably a better tool to do its job, but I wanted a lightweigth tool that could send http requests based on a text file and have support to variables so it is possible to version control my api requests. 

If you stumbled on this page, maybe consider using [hurl](https://github.com/Orange-OpenSource/hurl) instead.

### How to use

A basic command looks like this

> thc myfile.toml

This command will look for a file named environment.toml in the current workdir and use it as a base file, the file that you passed as argument on the command will override the base file content before sendind the request.

the file format if very simple, a complete file looks like this:
        
    method = "GET"
    host = "http://localhost:5000"
    path = "/"
    body = '{"fake_json":"value"}'
    
    [headers]
    accept = "application/json"
    authorization = "simple_token"

you can omit any value, but the sum of the the base file and the executed file must contain at least the host, path and method.

It's algo possible to use another file as the base file with the -b flag.

> thc myfile.toml -b other.toml

---

I'm learning rust so if there is a better way of doing things, feel free to contact me.