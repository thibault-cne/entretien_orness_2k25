import base64
from flask import Flask, request

app = Flask(__name__)

@app.route("/", methods = ["GET"])
def endpoint():
    args = request.args
    if 'code' not in args:
        return "code not found in query param"

    # Get base64 encoded sound
    code = args.get("code", str)
    decoded_code = base64.b64decode(code).decode('utf-8')
    commands = parse_input(decoded_code)

    # Concatenate the commands into one string
    payload = ';'.join([str(x) for x in commands])

    return payload

class Command:
    # _type can have 3 values
    #    note
    #    interval
    #    sleep
    
    def __init__(self, type: str, value: str):
        self._type = type
        self._value = value

    def __str__(self):
        return self._type + " " + self._value

    def __repr__(self):
        return "Command: " + self._type + " " + self._value


def parse_input(input: str) -> [Command]:
    commands = []
    
    i = 0
    while i < len(input):
        c = input[i]
        if c == '(':
            j = i
            while input[j] != ')':
                j += 1
            command = input[i+1:j].split(' ')
            commands.append(Command(command[0], command[1]))
            i = j + 1
        elif c == ' ':
            i += 1
        else:
            note = input[i:i+2]
            commands.append(Command("note", note))
            i += 2
   
    return commands


if __name__ == '__main__':
    app.run()
