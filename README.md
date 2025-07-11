# orness interview 2k25

This is an interview I had for Orness in 2k25.

## Lauch the project

In order to launch the project you first have to build the rust project:

```
cd sound
cargo build
```

Then we need to copy the binary created in order for the API to work correctly (from the root of the repo):

```
cp sound/target/release/sound .
```

Then you can create a new python virtualenv (from the root of the repo):

```
python3 -m venv .venv
source .venv/bin/activate
pip3 install -r requirements.txt
python3 main.py
```

Then you can send a `GET` request to http://127.0.0.1/?code=<your-sound-code>

You should receive a binary you can execute to play the code.
