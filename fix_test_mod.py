import re

with open('codeviz-cli/src/main.rs', 'r') as f:
    content = f.read()

content = content.replace("mod tests {\n    use super::*;", "#[cfg(test)]\nmod tests {\n    use super::*;")

with open('codeviz-cli/src/main.rs', 'w') as f:
    f.write(content)
