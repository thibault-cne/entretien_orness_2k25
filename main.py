# from base64 import decode
# from flask import Flask, request

# app = Flask(__name__)

# @app.route("/", methods = ["GET"])
# def endpoint():
#     args = request.args

#     # Get base64 encoded sound
#     code = args.get("code", str)

#     if code is None:
#         return "code not found in query param"

#     decoded_code = decode(code)
#     
#     pass

class Command:
    # _type can have 3 values
    #    note
    #    interval
    #    sleep
    
    def __init__(self, type: str, value: str):
        self._type = type
        self._value = value

    def __repr__(self):
        return "Command: " + self._type + " " + self._value


def parse_input(input: str):
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
    print(parse_input("(interval 0.5) C4 C4 C4 D4 E4 D4 C4 E4 D4 D4 C4 (sleep 1) (interval 0.8) C4 C4 C4 D4 E4 D4 C4 E4 D4 D4 C4"))
