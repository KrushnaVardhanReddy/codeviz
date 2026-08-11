import re

with open("codeviz-web/components/GraphCanvas.tsx", "r") as f:
    content = f.read()

# Add import dagre from 'dagre';
content = content.replace("import { CallPathExplorer } from './CallPathExplorer';", 
                          "import { CallPathExplorer } from './CallPathExplorer';\nimport dagre from 'dagre';")

# Replace useEffect body
use_effect_match = re.search(r'useEffect\(\(\) => \{.*?\n  \}, \[graph, setNodes, setEdges\]\);', content, re.DOTALL)
if not use_effect_match:
    print("Could not find useEffect!")
else:
    new_use_effect = """useEffect(() => {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setDefaultEdgeLabel(() => ({}));
    dagreGraph.setGraph({ rankdir: 'TB', nodesep: 50, ranksep: 100 });

    graph.nodes.forEach((n) => {
      dagreGraph.setNode(n.id, { width: 170, height: 70 });
    });

    graph.edges.forEach((e) => {
      dagreGraph.setEdge(e.from_id, e.to_id);
    });

    dagre.layout(dagreGraph);

    setNodes((currentNodes) => {
      const nodePositionMap = new Map(currentNodes.map(n => [n.id, n.position]));

      return graph.nodes.map((node) => {
        const dNode = dagreGraph.node(node.id);
        const defaultPosition = dNode 
          ? { x: dNode.x - 170 / 2, y: dNode.y - 70 / 2 }
          : { x: 0, y: 0 };
          
        const position = nodePositionMap.get(node.id) || defaultPosition;

        return {
          id: node.id,
          position,
          data: { label: node.label, kind: node.kind, testId: `node-${node.id}`, control_flow: node.control_flow },
          style: {
            ...getNodeStyle(node.kind),
            padding: '10px',
            width: 150,
            textAlign: 'center' as const,
            boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
          }
        };
      });
    });

    setEdges(
      graph.edges.map((edge, index) => ({
        id: `e${index}-${edge.from_id}-${edge.to_id}`,
        source: edge.from_id,
        target: edge.to_id,
        style: getEdgeStyle(edge.kind),
        animated: edge.kind === 'Calls',
        data: { kind: edge.kind }
      }))
    );
  }, [graph, setNodes, setEdges]);"""
    content = content.replace(use_effect_match.group(0), new_use_effect)

    with open("codeviz-web/components/GraphCanvas.tsx", "w") as f:
        f.write(content)
