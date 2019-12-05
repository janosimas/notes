import json
from pathlib import Path

compile_commands = 'compile_commands.json'
out_json = []
for filename in Path('./').rglob(compile_commands):
	with open(filename) as json_file:
		data = json.load(json_file)
		for d in  data:
			out_json.append(d)

with open(compile_commands, 'w') as outfile:
    json.dump(out_json, outfile,  indent=4)