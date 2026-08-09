import subprocess
import json
import time
import os

def test_mcp_e2e_list_tools():
    # Make sure we're using the built binary
    subprocess.run(["cargo", "build", "-p", "codeviz-cli"], check=True)

    binary_path = os.path.join(
        os.path.dirname(__file__), "..", "..", "target", "debug", "codeviz-cli"
    )

    process = subprocess.Popen(
        [binary_path, "serve", "--mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    }

    req_str = json.dumps(request) + "\n"
    process.stdin.write(req_str)
    process.stdin.flush()

    output_line = process.stdout.readline()
    response = json.loads(output_line)

    assert response["jsonrpc"] == "2.0"
    assert response["id"] == 1

    tools = response["result"]["tools"]
    tool_names = [tool["name"] for tool in tools]

    assert "add_language_support" in tool_names

    process.terminate()
    process.wait()
