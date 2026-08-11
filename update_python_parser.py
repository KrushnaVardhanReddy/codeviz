import re

with open("codeviz-python/src/parser.rs", "r") as f:
    content = f.read()

# I will just write a python script that uses regex or string replacement to add parent_id and call edges.
