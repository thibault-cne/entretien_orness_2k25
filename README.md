# orness interview 2k25

This is an interview I had for Orness in 2k25.

## Lauch the project

You can now launch the project using `Docker`. To do so follow the next steps:

```
docker build -t orness_interview_2k25 .
docker run --rm -p 8080:8080 orness_interview_2k25
```

This builds and then launches a docker container with the API. You can then send a `GET` request to http://127.0.0.1:8080/?code=<your-sound-code>

You should receive a binary you can execute to play the code.
