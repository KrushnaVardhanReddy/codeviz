import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

# Let's search for TRIGGER_PARSE_ERROR_FOR_TEST
idx = content.find("TRIGGER_PARSE_ERROR_FOR_TEST")
if idx == -1:
    print("Not found! Why did the sed script fail or python script fail?")
else:
    print("Found at", idx)
